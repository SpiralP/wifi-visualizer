mod flags;
mod type_;
mod version;

pub use self::flags::Flags;
pub use self::type_::{ControlSubtype, DataSubtype, FrameType, ManagementSubtype};
pub use self::version::Version;
use crate::error::*;
use std::slice::Iter;

#[derive(Debug)]
pub struct FrameControl {
  pub version: Version,

  pub type_: FrameType,

  pub flags: Flags,
}

impl FrameControl {
  pub fn parse(bytes: &mut Iter<u8>) -> Result<FrameControl> {
    let byte = bytes.next().unwrap();
    let version = byte & 0b0000_0011;
    let type_ = (byte & 0b0000_1100) >> 2;
    let subtype = (byte & 0b1111_0000) >> 4;
    let flags = Flags::parse(bytes)?;

    let type_ = match type_ {
      0 => FrameType::Management(ManagementSubtype::parse(subtype)?),
      1 => FrameType::Control(ControlSubtype::parse(subtype)?),
      2 => FrameType::Data(DataSubtype::parse(subtype)?),
      _ => bail!("invalid type"),
    };

    Ok(FrameControl {
      version: Version::parse(version)?,
      type_,
      flags,
    })
  }
}
