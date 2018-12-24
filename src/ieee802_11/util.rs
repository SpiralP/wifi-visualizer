use serde::ser::*;
use std::mem::transmute;

#[derive(Copy, Clone)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
  pub fn from(bytes: &[u8]) -> MacAddress {
    MacAddress([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]])
  }
}

impl std::fmt::Debug for MacAddress {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    formatter.write_fmt(format_args!(
      "{:0>2X}:{:0>2X}:{:0>2X}:{:0>2X}:{:0>2X}:{:0>2X}",
      self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
    ))?;
    Ok(())
  }
}

impl Serialize for MacAddress {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    Ok(serializer.serialize_str(&format!("{:?}", self))?)
  }
}

#[inline]
pub fn bytes8_to_u64(bytes: &[u8]) -> u64 {
  let n: u64 = unsafe {
    transmute([
      bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
  };
  n.to_le()
}

#[inline]
pub fn bytes4_to_u32(bytes: &[u8]) -> u32 {
  let n: u32 = unsafe { transmute([bytes[0], bytes[1], bytes[2], bytes[3]]) };
  n.to_le()
}

#[inline]
pub fn bytes2_to_u16(bytes: &[u8]) -> u16 {
  let n: u16 = unsafe { transmute([bytes[0], bytes[1]]) };
  n.to_le()
}
