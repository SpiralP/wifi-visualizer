mod get_capture;

use self::get_capture::*;
use crate::error::*;
use bytes::Bytes;
use ieee80211::Frame;
use pcap::{linktypes, Activated, Capture, Error as PcapError};
use radiotap::Radiotap;
use std::{thread, time::Duration};
use tokio::prelude::*;

#[derive(Clone)]
pub enum CaptureType {
  Stdin,
  File(String, bool), // path, sleep_playback
  Interface(String),
}

fn get_capture_iterator(capture_type: CaptureType) -> Result<CaptureIterator> {
  let mut sleep_playback = false;

  let capture = match capture_type {
    CaptureType::Stdin => get_stdin_capture()?.into(),
    CaptureType::File(path, sleep_playback2) => {
      sleep_playback = sleep_playback2;
      get_file_capture(path)?.into()
    }
    CaptureType::Interface(interface_name) => {
      let device = get_interface(&interface_name)?;
      get_live_capture(device)?.into()
    }
  };

  CaptureIterator::new(capture, sleep_playback)
}

pub struct FrameWithRadiotap {
  pub frame: Frame,
  pub radiotap: Option<Radiotap>,
}

pub fn get_capture_stream(
  capture_type: CaptureType,
) -> impl Future<Item = impl Stream<Item = FrameWithRadiotap, Error = Error>, Error = Error> {
  future::lazy(move || get_capture_iterator(capture_type)).map(|capture_iterator| {
    let is_radiotap = capture_iterator.is_radiotap;

    stream::iter_result(capture_iterator).map(move |bytes| {
      let (radiotap, bytes) = if is_radiotap {
        let (radiotap, rest) = Radiotap::parse(&bytes).unwrap();

        let has_fcs = radiotap.flags.map_or(false, |flags| flags.fcs);

        let frame_bytes = if has_fcs {
          // remove last 4 bytes (uint32_t)
          let (data, _fcs) = rest.split_at(rest.len() - 4);
          data
        } else {
          rest
        };

        (Some(radiotap), frame_bytes.into())
      } else {
        (None, bytes)
      };

      let frame = Frame::new(bytes);

      FrameWithRadiotap { frame, radiotap }
    })
  })
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
            .unwrap_or_else(|_| format!("(couldn't datalink.get_name() for {})", datalink.0))
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
  type Item = Result<Bytes>;

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

        Some(Ok(Bytes::from(packet.data)))
      }
    }
  }
}
