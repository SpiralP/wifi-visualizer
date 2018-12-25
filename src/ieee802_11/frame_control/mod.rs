mod flags;
mod type_;
mod version;

pub use self::flags::*;
pub use self::type_::{ControlSubtype, DataSubtype, FrameType, ManagementSubtype};
pub use self::version::Version;
use crate::error::*;
use serde_derive::*;

#[derive(Serialize, Debug)]
pub struct FrameControl {
  #[serde(skip)]
  pub version: Version,

  pub type_: FrameType,

  #[serde(skip)]
  pub flags: Flags,
}

impl FrameControl {
  pub fn parse(bytes: &[u8]) -> Result<FrameControl> {
    let version = bytes[0] & 0b0000_0011;
    let type_ = (bytes[0] & 0b0000_1100) >> 2;
    let subtype = (bytes[0] & 0b1111_0000) >> 4;
    let flags = Flags::parse(bytes[1])?;

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
