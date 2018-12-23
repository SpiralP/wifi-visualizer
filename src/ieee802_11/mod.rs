mod frame_control;
mod util;

pub use self::frame_control::FrameControl;
use self::frame_control::*;
pub use self::util::*;
use crate::error::*;
use error_chain::*;

#[derive(Debug)]
pub struct BeaconFrame {
  pub frame_control: FrameControl,
  pub duration: u16, // microseconds
  pub receiver_address: MacAddress,
  pub destination_address: MacAddress,
  pub transmitter_address: MacAddress,
  pub source_address: MacAddress,
  pub bss_id: MacAddress,
  pub fragment_number: u8,
  pub sequence_number: u16,
}

#[derive(Debug)]
pub struct BasicFrame {
  pub frame_control: FrameControl,
  pub duration: u16, // microseconds
}

#[derive(Debug)]
pub enum Frame {
  Basic(BasicFrame),
  Beacon(BeaconFrame),
}

impl Frame {
  pub fn parse(bytes: &[u8]) -> Result<Frame> {
    let frame_control = FrameControl::parse(&bytes[0..2])?;
    let duration = (u16::from(bytes[3]) << 8) | u16::from(bytes[2]);

    match frame_control.type_ {
      Type::Management(ref subtype) => match subtype {
        ManagementSubtype::Beacon => {
          let destination_address = MacAddress::from(&bytes[4..10]);
          let source_address = MacAddress::from(&bytes[10..16]);

          let fragment_number = bytes[22] & 0b0000_1111;
          let sequence_number = ((u16::from(bytes[23]) << 8) | u16::from(bytes[22])) >> 4;

          Ok(Frame::Beacon(BeaconFrame {
            frame_control,
            duration,
            receiver_address: destination_address,
            destination_address,
            transmitter_address: source_address,
            source_address,
            bss_id: MacAddress::from(&bytes[16..22]),
            fragment_number,
            sequence_number,
          }))
        }
        _ => Ok(Frame::Basic(BasicFrame {
          frame_control,
          duration,
        })),
      },
      _ => Ok(Frame::Basic(BasicFrame {
        frame_control,
        duration,
      })),
    }
  }
}
