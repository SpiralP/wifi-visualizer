use super::*;

#[derive(Debug)]
pub struct BeaconFrame {
  pub management_frame: ManagementFrame,

  pub fixed_parameters: FixedParameters,
  pub tagged_parameters: TaggedParameters,
}

impl BeaconFrame {
  pub fn parse(management_frame: ManagementFrame, bytes: &mut Iter<u8>) -> Result<BeaconFrame> {
    let fixed_parameters = FixedParameters::parse(bytes)?;
    let tagged_parameters = TaggedParameters::parse(bytes)?;

    Ok(BeaconFrame {
      management_frame,
      fixed_parameters,
      tagged_parameters,
    })
  }
}
