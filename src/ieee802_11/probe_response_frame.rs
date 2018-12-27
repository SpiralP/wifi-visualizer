use super::*;

#[derive(Debug)]
pub struct ProbeResponseFrame {
  pub management_frame: ManagementFrame,

  pub fixed_parameters: FixedParameters,
  pub tagged_parameters: TaggedParameters,

  pub ssid: Vec<u8>,
}

impl ProbeResponseFrame {
  pub fn parse(
    management_frame: ManagementFrame,
    bytes: &mut Cursor<Vec<u8>>,
  ) -> Result<ProbeResponseFrame> {
    let fixed_parameters = FixedParameters::parse(bytes)?;
    let tagged_parameters = TaggedParameters::parse(bytes)?;

    let mut ssid = Vec::new();

    for tag in &tagged_parameters.tags {
      if tag.number == 0 {
        // SSID
        ssid = tag.data.clone();
      }
    }

    Ok(ProbeResponseFrame {
      management_frame,
      fixed_parameters,
      tagged_parameters,
      ssid,
    })
  }
}
