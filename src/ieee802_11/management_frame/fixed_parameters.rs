pub use super::capabilities_info::*;
use crate::error::*;
use crate::ieee802_11::util::*;
use std::slice::Iter;

#[derive(Debug)]
pub struct FixedParameters {
  pub timestamp: u64,
  pub beacon_interval: f64, // seconds
  pub capabilities_info: CapabilitiesInfo,
}

impl FixedParameters {
  pub fn parse(bytes: &mut Iter<u8>) -> Result<FixedParameters> {
    let timestamp: u64 = bytes8_to_u64(bytes);
    let beacon_interval = f64::from(bytes2_to_u16(bytes)) * 0.001_024f64;
    let capabilities_info = CapabilitiesInfo::parse(bytes)?;

    Ok(FixedParameters {
      timestamp,
      beacon_interval,
      capabilities_info,
    })
  }
}
