use serde::ser::*;
use std::mem::transmute;
use std::slice::Iter;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
  pub fn from(bytes: &mut Iter<u8>) -> MacAddress {
    MacAddress([
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
    ])
  }
}

impl std::fmt::Display for MacAddress {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    formatter.write_fmt(format_args!(
      "{:0>2X}:{:0>2X}:{:0>2X}:{:0>2X}:{:0>2X}:{:0>2X}",
      self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
    ))?;
    Ok(())
  }
}

impl std::fmt::Debug for MacAddress {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    formatter.write_fmt(format_args!("\"{}\"", self))?;
    Ok(())
  }
}

impl Serialize for MacAddress {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    Ok(serializer.serialize_str(&format!("{}", self))?)
  }
}

#[inline]
pub fn bytes8_to_u64(bytes: &mut Iter<u8>) -> u64 {
  let n: u64 = unsafe {
    transmute([
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
    ])
  };
  n.to_le()
}

#[inline]
pub fn bytes4_to_u32(bytes: &mut Iter<u8>) -> u32 {
  let n: u32 = unsafe {
    transmute([
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
      *bytes.next().unwrap(),
    ])
  };
  n.to_le()
}

#[inline]
pub fn bytes2_to_u16(bytes: &mut Iter<u8>) -> u16 {
  let n: u16 = unsafe { transmute([*bytes.next().unwrap(), *bytes.next().unwrap()]) };
  n.to_le()
}

pub fn hash_macs(mac1: MacAddress, mac2: MacAddress) -> String {
  if mac1 >= mac2 {
    format!("{}{}", mac1, mac2).to_string()
  } else {
    format!("{}{}", mac2, mac1).to_string()
  }
}

pub fn is_broadcast(mac: MacAddress) -> bool {
  (mac.0[0] & 0b01) != 0
  // multicast
}
