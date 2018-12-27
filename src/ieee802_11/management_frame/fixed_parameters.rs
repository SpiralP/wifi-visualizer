use super::*;

#[derive(Debug)]
pub struct FixedParameters {
  pub timestamp: u64,
  pub beacon_interval: f64, // seconds
  pub capabilities_info: CapabilitiesInfo,
}

impl FixedParameters {
  pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<FixedParameters> {
    let timestamp: u64 = bytes.read_u64::<LE>().unwrap();
    let beacon_interval = f64::from(bytes.read_u16::<LE>().unwrap()) * 0.001_024f64;
    let capabilities_info = CapabilitiesInfo::parse(bytes)?;

    Ok(FixedParameters {
      timestamp,
      beacon_interval,
      capabilities_info,
    })
  }
}
