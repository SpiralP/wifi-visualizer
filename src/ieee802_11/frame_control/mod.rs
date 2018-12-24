mod flags;
mod type_;
mod version;

pub use self::flags::*;
pub use self::type_::*;
pub use self::version::*;
use crate::error::*;
use serde_derive::*;

#[derive(Serialize, Debug)]
pub struct FrameControl {
  #[serde(skip)]
  pub version: Version,

  pub type_: Type,

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
      0 => Type::Management(ManagementSubtype::parse(subtype)?),
      1 => Type::Control(ControlSubtype::parse(subtype)?),
      2 => Type::Data(DataSubtype::parse(subtype)?),
      _ => bail!("invalid type"),
    };

    Ok(FrameControl {
      version: Version::parse(version)?,
      type_,
      flags,
    })
  }
}
