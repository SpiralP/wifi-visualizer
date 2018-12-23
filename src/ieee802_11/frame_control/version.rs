use crate::error::*;

#[derive(Debug)]
pub enum Version {
  Standard,
}

impl Version {
  pub fn parse(n: u8) -> Result<Version> {
    match n {
      0 => Ok(Version::Standard),
      _ => bail!("invalid Version"),
    }
  }
}
