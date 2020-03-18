mod get_capture;

use self::get_capture::{get_file_capture, get_interface, get_live_capture, get_stdin_capture};
use crate::error::{bail, Result};
use futures::prelude::*;
use ieee80211::Frame;
use pcap::{linktypes, Activated, Capture, Error as PcapError};
use radiotap::Radiotap;
use std::{borrow::Cow, thread, time::Duration};

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

pub struct FrameWithRadiotap<'a> {
  pub id: u64,
  pub frame: Frame<'a>,
  pub radiotap: Option<Radiotap>,
}

pub async fn get_capture_stream(
  capture_type: CaptureType,
) -> Result<impl Stream<Item = Result<FrameWithRadiotap<'static>>>> {
  let capture_iterator = get_capture_iterator(capture_type)?;
  let is_radiotap = capture_iterator.is_radiotap;

  let mut id = 0;

  Ok(stream::iter(capture_iterator).map(move |result| {
    match result {
      Err(e) => Err(e),

      Ok(bytes) => {
        let (radiotap, bytes) = if is_radiotap {
          let (radiotap, rest) = Radiotap::parse(&bytes)?;

          let has_fcs = radiotap.flags.map_or(false, |flags| flags.fcs);

          let frame_bytes = if has_fcs {
            // remove last 4 bytes (uint32_t)
            let (data, _fcs) = rest.split_at(rest.len() - 4);
            data
          } else {
            rest
          };

          (Some(radiotap), Cow::Borrowed(frame_bytes))
        } else {
          (None, Cow::Owned(bytes))
        };

        let frame = Frame::new(bytes.into_owned());
        id += 1;

        Ok(FrameWithRadiotap {
          id,
          frame,
          radiotap,
        })
      }
    }
  }))
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

      Ok(packet) => {
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

        Some(Ok(packet.data.to_owned()))
      }
    }
  }
}
