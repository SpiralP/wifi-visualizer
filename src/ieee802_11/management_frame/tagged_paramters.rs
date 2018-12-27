use super::*;

#[derive(Debug)]
pub struct Tag {
  pub number: u8,
  pub length: u8,
  pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct TaggedParameters {
  pub tags: Vec<Tag>,
}

impl TaggedParameters {
  pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<TaggedParameters> {
    let mut tags = Vec::new();

    loop {
      let number = {
        match bytes.read_u8() {
          Err(_) => break,
          Ok(number) => number,
        }
      };
      let length = bytes.read_u8().unwrap();

      let mut data = Vec::new();
      for _ in 0..length {
        data.push(bytes.read_u8().unwrap());
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
