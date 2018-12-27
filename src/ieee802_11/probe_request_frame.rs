use super::*;

#[derive(Debug)]
pub struct ProbeRequestFrame {
  pub management_frame: ManagementFrame,

  pub tagged_parameters: TaggedParameters,

  pub ssid: Vec<u8>,
}

impl ProbeRequestFrame {
  pub fn parse(
    management_frame: ManagementFrame,
    bytes: &mut Cursor<Vec<u8>>,
  ) -> Result<ProbeRequestFrame> {
    let tagged_parameters = TaggedParameters::parse(bytes)?;

    let mut ssid = Vec::new();

    for tag in &tagged_parameters.tags {
      if tag.number == 0 {
        // SSID
        ssid = tag.data.clone();
      }
    }

    Ok(ProbeRequestFrame {
      management_frame,
      tagged_parameters,
      ssid,
    })
  }
}
