mod frame_control;
mod util;

pub use self::frame_control::FrameControl;
use self::frame_control::*;
pub use self::util::*;
use crate::error::*;
use error_chain::*;
use serde_derive::*;

#[derive(Serialize, Debug)]
pub struct BeaconFrame {
  pub frame_control: FrameControl,
  pub duration: u16, // microseconds

  pub receiver_address: MacAddress,
  pub transmitter_address: MacAddress,

  pub destination_address: MacAddress,
  pub source_address: MacAddress,

  pub bssid: Option<MacAddress>,

  pub fragment_number: u8,
  pub sequence_number: u16,
}

#[derive(Serialize, Debug)]
pub struct BasicFrame {
  pub frame_control: FrameControl,
  pub duration: u16, // microseconds

  pub receiver_address: MacAddress,
  pub transmitter_address: MacAddress,

  pub destination_address: MacAddress,
  pub source_address: MacAddress,

  pub bssid: Option<MacAddress>,
}

#[derive(Serialize, Debug)]
pub enum Frame {
  Basic(BasicFrame),
  Beacon(BeaconFrame),
}

impl Frame {
  pub fn parse(bytes: &[u8]) -> Result<Frame> {
    let frame_control = FrameControl::parse(&bytes[0..2])?;
    let duration = (u16::from(bytes[3]) << 8) | u16::from(bytes[2]);

    let addr1 = MacAddress::from(&bytes[4..10]);
    let addr2 = MacAddress::from(&bytes[10..16]);
    let addr3 = MacAddress::from(&bytes[16..22]);

    let receiver_address = addr1;
    let transmitter_address = addr2;

    let destination_address;
    let source_address;

    let mut bssid = None;

    // https://networkengineering.stackexchange.com/questions/25100/four-layer-2-addresses-in-802-11-frame-header
    match (frame_control.flags.to_ds, frame_control.flags.from_ds) {
      (false, false) => {
        // from one STA to another STA, plus all management/control type frames
        destination_address = addr1;
        source_address = addr2;
        bssid = Some(addr3);
      }
      (false, true) => {
        // exiting the DS
        destination_address = addr1;
        bssid = Some(addr2);
        source_address = addr3;
      }
      (true, false) => {
        // destined for the DS
        bssid = Some(addr1);
        source_address = addr2;
        destination_address = addr3;
      }
      (true, true) => {
        // one AP to another AP
        let addr4 = MacAddress::from(&bytes[22..28]);

        destination_address = addr3;
        source_address = addr4;
      }
    }

    match frame_control.type_ {
      Type::Management(ref subtype) => match subtype {
        ManagementSubtype::Beacon => {
          let fragment_number = bytes[22] & 0b0000_1111;
          let sequence_number = ((u16::from(bytes[23]) << 8) | u16::from(bytes[22])) >> 4;

          Ok(Frame::Beacon(BeaconFrame {
            frame_control,
            duration,
            receiver_address,
            destination_address,
            transmitter_address,
            source_address,
            bssid,
            fragment_number,
            sequence_number,
          }))
        }
        _ => Ok(Frame::Basic(BasicFrame {
          frame_control,
          duration,
          receiver_address,
          destination_address,
          transmitter_address,
          source_address,
          bssid,
        })),
      },
      _ => Ok(Frame::Basic(BasicFrame {
        frame_control,
        duration,
        receiver_address,
        destination_address,
        transmitter_address,
        source_address,
        bssid,
      })),
    }
  }
}
