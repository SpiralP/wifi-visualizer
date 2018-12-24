mod capabilities_info;
mod fixed_parameters;
mod tagged_paramters;

pub use self::capabilities_info::*;
pub use self::fixed_parameters::*;
pub use self::tagged_paramters::*;
use crate::error::*;
use serde_derive::*;

#[derive(Serialize, Debug)]
pub struct BeaconInfo {
  pub fixed_parameters: FixedParameters,
  pub tagged_parameters: TaggedParameters,
}

impl BeaconInfo {
  pub fn parse(bytes: &[u8]) -> Result<BeaconInfo> {
    let fixed_parameters = FixedParameters::parse(&bytes[0..12])?;
    let tagged_parameters = TaggedParameters::parse(&bytes[12..])?;

    Ok(BeaconInfo {
      fixed_parameters,
      tagged_parameters,
    })
  }
}
