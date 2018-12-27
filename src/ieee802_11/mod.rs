pub mod basic_frame;
pub mod beacon_frame;
pub mod frame_control;
pub mod management_frame;
pub mod probe_request_frame;
pub mod probe_response_frame;
pub mod util;

pub use self::basic_frame::*;
pub use self::beacon_frame::*;
pub use self::frame_control::*;
pub use self::frame_control::*;
pub use self::management_frame::*;
pub use self::probe_request_frame::*;
pub use self::probe_response_frame::*;
pub use self::util::*;
use crate::error::*;
pub use byteorder::{ReadBytesExt, *};
use std::io::Cursor;

#[derive(Debug)]
pub enum Frame {
  Basic(BasicFrame),
  Management(ManagementFrame), // for untyped
  Beacon(BeaconFrame),
  ProbeRequest(ProbeRequestFrame),
  ProbeResponse(ProbeResponseFrame),
}

impl Frame {
  pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<Frame> {
    let basic_frame = BasicFrame::parse(bytes)?;

    match basic_frame.type_ {
      FrameType::Management(subtype) => {
        let management_frame = ManagementFrame::parse(basic_frame, bytes)?;
        match subtype {
          ManagementSubtype::Beacon => {
            Ok(Frame::Beacon(BeaconFrame::parse(management_frame, bytes)?))
          }

          ManagementSubtype::ProbeRequest => Ok(Frame::ProbeRequest(ProbeRequestFrame::parse(
            management_frame,
            bytes,
          )?)),

          ManagementSubtype::ProbeResponse => Ok(Frame::ProbeResponse(ProbeResponseFrame::parse(
            management_frame,
            bytes,
          )?)),

          _ => Ok(Frame::Management(management_frame)),
        }
      }

      _ => Ok(Frame::Basic(basic_frame)),
    }
  }
}
