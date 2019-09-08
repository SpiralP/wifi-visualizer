mod get_capture;
mod strip_radiotap;

use self::{get_capture::*, strip_radiotap::strip_radiotap};
use crate::error::*;
use pcap::{linktypes, Activated, Capture, Error as PcapError};
use std::{thread, time::Duration};
use tokio::prelude::*;

#[derive(Clone)]
pub enum CaptureType {
  Stdin,
  File(String),
  Interface(String),
}

pub fn get_capture_iterator(capture_type: CaptureType) -> Result<CaptureIterator> {
  let mut sleep_playback = false;

  let capture = match capture_type {
    CaptureType::Stdin => get_stdin_capture()?.into(),
    CaptureType::File(path) => {
      sleep_playback = true;
      get_file_capture(path)?.into()
    }
    CaptureType::Interface(interface_name) => {
      let device = get_interface(&interface_name)?;
      get_live_capture(device)?.into()
    }
  };

  CaptureIterator::new(capture, sleep_playback)
}

pub fn start(
  capture_iterator: CaptureIterator,
) -> impl Future<Item = impl Stream<Item = Vec<u8>, Error = Error>, Error = Error> {
  future::lazy(move || Ok(stream::iter_result(capture_iterator)))
}

pub struct CaptureIterator {
  capture: Capture<dyn Activated>,
  is_radiotap: bool,
  sleep_playback: bool,
  maybe_last_time: Option<Duration>,
}

impl CaptureIterator {
  fn new(capture: Capture<dyn Activated>, sleep_playback: bool) -> Result<Self> {
    let maybe_last_time: Option<Duration> = None;

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

    Ok(Self {
      capture,
      is_radiotap,
      sleep_playback,
      maybe_last_time,
    })
  }
}

impl Iterator for CaptureIterator {
  type Item = Result<Vec<u8>>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.capture.next() {
      Err(err) => match err {
        PcapError::NoMorePackets => None,
        // PcapError::TimeoutExpired => {
        //   // this is called on windows at least!
        //   Some(Ok(None))
        // }
        other => {
          // panic!("{}", err);
          Some(Err(other.into()))
        }
      },

      Ok(ref packet) => {
        // TODO move out of sniff thread
        let data = if self.is_radiotap {
          strip_radiotap(packet.data)
        } else {
          packet.data
        };

        if self.sleep_playback {
          #[allow(clippy::cast_possible_truncation)]
          #[allow(clippy::cast_sign_loss)]
          let current_time = std::time::Duration::new(
            packet.header.ts.tv_sec as u64,
            (packet.header.ts.tv_usec * 1000) as u32,
          );
          if let Some(last_time) = self.maybe_last_time {
            if current_time > last_time {
              thread::sleep(current_time - last_time);
            }
          }
          self.maybe_last_time = Some(current_time);
        }

        Some(Ok(data.to_vec()))
      }
    }
  }
}
