mod capabilities_info;
mod fixed_parameters;
mod tagged_paramters;

pub use self::capabilities_info::*;
pub use self::fixed_parameters::*;
pub use self::tagged_paramters::*;
use super::*;

#[derive(Debug)]
pub struct ManagementFrame {
  pub basic_frame: BasicFrame,

  pub fragment_number: u8,
  pub sequence_number: u16,
}

impl ManagementFrame {
  pub fn parse(basic_frame: BasicFrame, bytes: &mut Cursor<Vec<u8>>) -> Result<ManagementFrame> {
    let byte1 = bytes.read_u8().unwrap();
    let byte2 = bytes.read_u8().unwrap();
    let fragment_number = byte1 & 0b0000_1111;
    let sequence_number = ((u16::from(byte2) << 8) | u16::from(byte1)) >> 4;

    Ok(ManagementFrame {
      basic_frame,
      fragment_number,
      sequence_number,
    })
  }
}
