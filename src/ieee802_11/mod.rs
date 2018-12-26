pub mod basic_frame;
pub mod beacon_frame;
pub mod frame_control;
pub mod management_frame;
pub mod probe_request_frame;
pub mod util;

pub use self::basic_frame::BasicFrame;
pub use self::beacon_frame::BeaconFrame;
pub use self::frame_control::FrameControl;
pub use self::frame_control::{ControlSubtype, FrameType, ManagementSubtype};
pub use self::management_frame::{FixedParameters, ManagementFrame, TaggedParameters};
pub use self::probe_request_frame::ProbeRequestFrame;
pub use self::util::{bytes2_to_u16, MacAddress};
use crate::error::*;
use std::slice::Iter;

#[derive(Debug)]
pub enum Frame {
  Basic(BasicFrame),
  Management(ManagementFrame), // for untyped
  Beacon(BeaconFrame),
  ProbeRequest(ProbeRequestFrame),
}

impl Frame {
  pub fn parse(bytes: &mut Iter<u8>) -> Result<Frame> {
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

          _ => Ok(Frame::Management(management_frame)),
        }
      }

      _ => Ok(Frame::Basic(basic_frame)),
    }
  }
}
