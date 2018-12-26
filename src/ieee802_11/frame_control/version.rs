use crate::error::*;

#[derive(Debug)]
pub enum Version {
  Standard,
}

impl Version {
  pub fn parse(byte: u8) -> Result<Version> {
    match byte {
      0 => Ok(Version::Standard),
      _ => bail!("invalid Version {}", byte),
    }
  }
}
