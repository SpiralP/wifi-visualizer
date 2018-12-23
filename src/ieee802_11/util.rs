use serde::ser::*;

#[derive(Copy, Clone)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
  pub fn from(bytes: &[u8]) -> MacAddress {
    MacAddress([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]])
  }
}

impl std::fmt::Debug for MacAddress {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    formatter.write_fmt(format_args!(
      "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}",
      self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
    ))?;
    Ok(())
  }
}

impl Serialize for MacAddress {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    Ok(serializer.serialize_str(&format!("{:?}", self))?)
  }
}
