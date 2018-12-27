mod flags;
mod types;
mod version;

pub use self::flags::Flags;
pub use self::types::{ControlSubtype, DataSubtype, FrameType, ManagementSubtype};
pub use self::version::Version;
use super::*;

#[derive(Debug)]
pub struct FrameControl {
  pub version: Version,

  pub type_: FrameType,

  pub flags: Flags,
}

impl FrameControl {
  pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<FrameControl> {
    let byte = bytes.read_u8().unwrap();
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
