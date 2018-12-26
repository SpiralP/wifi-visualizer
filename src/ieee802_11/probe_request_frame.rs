use super::*;

#[derive(Debug)]
pub struct ProbeRequestFrame {
  pub management_frame: ManagementFrame,

  pub tagged_parameters: TaggedParameters,
}

impl ProbeRequestFrame {
  pub fn parse(
    management_frame: ManagementFrame,
    bytes: &mut Iter<u8>,
  ) -> Result<ProbeRequestFrame> {
    let tagged_parameters = TaggedParameters::parse(bytes)?;

    Ok(ProbeRequestFrame {
      management_frame,
      tagged_parameters,
    })
  }
}
