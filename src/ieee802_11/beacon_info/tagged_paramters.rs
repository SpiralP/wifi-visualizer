use crate::error::*;
use serde_derive::*;

#[derive(Serialize, Debug)]
pub struct Tag {
  pub number: u8,
  pub length: u8,
  pub data: Vec<u8>,
}

#[derive(Serialize, Debug)]
pub struct TaggedParameters {
  pub tags: Vec<Tag>,
}

impl TaggedParameters {
  pub fn parse(bytes: &[u8]) -> Result<TaggedParameters> {
    let mut tags = Vec::new();

    let mut bytes = bytes.iter();
    loop {
      let number = {
        let maybe_number = bytes.next();
        if maybe_number.is_none() {
          break;
        }
        *maybe_number.unwrap()
      };
      let length = *bytes.next().unwrap();

      let mut data = Vec::new();
      for _ in 0..length {
        data.push(*bytes.next().unwrap());
      }

      tags.push(Tag {
        number,
        length,
        data,
      });
    }

    Ok(TaggedParameters { tags })
  }
}
