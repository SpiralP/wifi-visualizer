mod get_capture;
mod strip_radiotap;

use self::{get_capture::*, strip_radiotap::strip_radiotap};
use crate::{
  error::*,
  events::{handle_frame, store::Store},
};
use helpers::{check_notified_return, notify::Notify};
use ieee80211::Frame;
use log::debug;
use pcap::{linktypes, Activated, Capture, Error as PcapError};
use std::{thread, time::Duration};

pub enum CaptureType {
  Stdin,
  File(String),
  Interface(String),
}

pub fn get_capture(capture_type: CaptureType) -> Result<Capture<dyn Activated>> {
  Ok(match capture_type {
    CaptureType::Stdin => get_stdin_capture()?.into(),
    CaptureType::File(path) => get_file_capture(path)?.into(),
    CaptureType::Interface(interface_name) => {
      let device = get_interface(&interface_name)?;
      get_live_capture(device)?.into()
    }
  })
}

pub fn start_blocking(
  mut capture: Capture<dyn Activated>,
  mut store: Store,
  sleep_playback: bool,
  stop_notify: &Notify,
) -> Result<()> {
  let mut maybe_last_time: Option<Duration> = None;

  let datalink = capture.get_datalink();
  let is_radiotap = match datalink.0 {
    linktypes::IEEE802_11 => false,
    linktypes::IEEE802_11_RADIOTAP => true,
    _ => {
      bail!(
        "bad datalink type {}",
        datalink
          .get_name()
          .unwrap_or_else(|_| format!("(couldn't get_name for {})", datalink.0).to_string())
      );
    }
  };

  loop {
    check_notified_return!(stop_notify, Ok(()));

    match capture.next() {
      Err(ref err) => match err {
        PcapError::NoMorePackets => break,
        PcapError::TimeoutExpired => {
          // this is called on windows at least!
        }
        _ => {
          panic!("{}", err);
        }
      },

      Ok(ref packet) => {
        // TODO move out of sniff thread
        let data = if is_radiotap {
          strip_radiotap(packet.data)
        } else {
          packet.data
        };

        if sleep_playback {
          #[allow(clippy::cast_possible_truncation)]
          #[allow(clippy::cast_sign_loss)]
          let current_time = std::time::Duration::new(
            packet.header.ts.tv_sec as u64,
            (packet.header.ts.tv_usec * 1000) as u32,
          );
          if let Some(last_time) = maybe_last_time {
            if current_time > last_time {
              thread::sleep(current_time - last_time);
            }
          }
          maybe_last_time = Some(current_time);
        }

        let frame = Frame::new(&data);
        handle_frame(&mut store, &frame);
      }
    }
  }

  debug!("packet capture loop done");

  Ok(())
}
