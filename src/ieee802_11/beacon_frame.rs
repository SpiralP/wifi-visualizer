use super::*;

#[derive(Debug)]
pub struct BeaconFrame {
  pub management_frame: ManagementFrame,

  pub fixed_parameters: FixedParameters,
  pub tagged_parameters: TaggedParameters,

  pub ssid: Vec<u8>,
}

impl BeaconFrame {
  pub fn parse(management_frame: ManagementFrame, bytes: &mut Iter<u8>) -> Result<BeaconFrame> {
    let fixed_parameters = FixedParameters::parse(bytes)?;
    let tagged_parameters = TaggedParameters::parse(bytes)?;

    let mut ssid = Vec::new();

    for tag in &tagged_parameters.tags {
      if tag.number == 0 {
        // SSID
        ssid = tag.data.clone();
      }
    }

    Ok(BeaconFrame {
      management_frame,
      fixed_parameters,
      tagged_parameters,
      ssid,
    })
  }
}
