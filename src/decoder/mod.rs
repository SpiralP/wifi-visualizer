use crate::error::*;
use enum_primitive::*;
use error_chain::*;

enum_from_primitive! {
  #[derive(Debug)]
  pub enum Version {
    Standard,
  }
}

#[derive(Debug)]
pub enum Type {
  Management(ManagementSubtype),
  Control(ControlSubtype),
  Data,
}

enum_from_primitive! {
  #[derive(Debug)]
  pub enum ManagementSubtype {
    AssociationRequest,
    AssociationResponse,
    ReassociationRequest,
    ReassociationResponse,
    ProbeRequest,
    ProbeResponse,
    Beacon = 8,
    ATIM,
    Disassociation,
    Authentication,
    Deauthentication,
  }
}

enum_from_primitive! {
  #[derive(Debug)]
  pub enum ControlSubtype {
    PSPoll = 10,
    RTS, // Request To Send
    CTS, // Clear To Send
    ACK,
    CFEnd, // Contention Free
    CFEndCFACK,
  }
}

#[derive(Debug)]
pub struct FrameControl {
  pub version: Version,
  pub type_: Type,
  pub flags: u8,
}

impl FrameControl {
  pub fn parse(bytes: [u8; 2]) -> Result<FrameControl> {
    let version = bytes[0] & 0b0000_0011;
    let type_ = (bytes[0] & 0b0000_1100) >> 2;
    let subtype = (bytes[0] & 0b1111_0000) >> 4;
    let flags = bytes[1];

    let type_ = match type_ {
      0 => Type::Management(ManagementSubtype::from_u8(subtype).ok_or("invalid subtype")?),
      1 => Type::Control(ControlSubtype::from_u8(subtype).ok_or("invalid subtype")?),
      2 => Type::Data,
      _ => bail!("invalid type"),
    };

    Ok(FrameControl {
      version: Version::from_u8(version).ok_or("invalid subtype")?,
      type_,
      flags,
    })
  }
}

#[derive(Debug)]
pub struct Frame {
  pub frame_control: FrameControl,
}

impl Frame {
  pub fn parse(bytes: &[u8]) -> Result<Frame> {
    Ok(Frame {
      frame_control: FrameControl::parse([bytes[0], bytes[1]])?,
    })
  }
}
