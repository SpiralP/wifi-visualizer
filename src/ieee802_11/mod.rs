mod frame_control;
mod util;

pub use self::frame_control::FrameControl;
pub use self::frame_control::*;
pub use self::util::*;
use crate::error::*;
use error_chain::*;
use serde_derive::*;

#[derive(Serialize, Debug)]
pub struct BeaconFrame {
  pub frame_control: FrameControl,
  pub duration: u16, // microseconds

  pub receiver_address: Option<MacAddress>,
  pub transmitter_address: Option<MacAddress>,

  pub destination_address: Option<MacAddress>,
  pub source_address: Option<MacAddress>,

  pub bssid: Option<MacAddress>,

  pub fragment_number: u8,
  pub sequence_number: u16,
}

#[derive(Serialize, Debug)]
pub struct BasicFrame {
  pub frame_control: FrameControl,
  pub duration: u16, // microseconds

  pub receiver_address: Option<MacAddress>,
  pub transmitter_address: Option<MacAddress>,

  pub destination_address: Option<MacAddress>,
  pub source_address: Option<MacAddress>,

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

    let receiver_address = Some(addr1);
    let mut transmitter_address = None;

    let mut destination_address = None;
    let mut source_address = None;

    let mut bssid = None;

    let mut other = false;
    match frame_control.type_ {
      Type::Control(ref subtype) => {
        match subtype {
          ControlSubtype::ACK | ControlSubtype::CTS => {
            // only receiver
          }

          ControlSubtype::RTS | ControlSubtype::BlockAck | ControlSubtype::BlockAckRequest => {
            // only receiver + transmitter
            let addr2 = MacAddress::from(&bytes[10..16]);
            transmitter_address = Some(addr2);
          }

          ControlSubtype::CFEnd => {
            let addr2 = MacAddress::from(&bytes[10..16]);
            bssid = Some(addr2);
          }

          _ => {
            other = true;
          }
        }
      }

      _ => {
        other = true;
      }
    }

    if other {
      let addr2 = MacAddress::from(&bytes[10..16]);
      let addr3 = MacAddress::from(&bytes[16..22]);
      transmitter_address = Some(addr2);
      // https://networkengineering.stackexchange.com/questions/25100/four-layer-2-addresses-in-802-11-frame-header
      match (frame_control.flags.to_ds, frame_control.flags.from_ds) {
        (false, false) => {
          // from one STA to another STA, plus all management/control type frames
          destination_address = Some(addr1);
          source_address = Some(addr2);
          bssid = Some(addr3);
        }
        (false, true) => {
          // exiting the DS
          destination_address = Some(addr1);
          bssid = Some(addr2);
          source_address = Some(addr3);
        }
        (true, false) => {
          // destined for the DS
          bssid = Some(addr1);
          source_address = Some(addr2);
          destination_address = Some(addr3);
        }
        (true, true) => {
          // one AP to another AP
          let addr4 = MacAddress::from(&bytes[22..28]);

          destination_address = Some(addr3);
          source_address = Some(addr4);
        }
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
