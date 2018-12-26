use crate::error::*;
use std::slice::Iter;

#[derive(Debug)]
pub struct Flags {
  pub to_ds: bool,
  pub from_ds: bool,
  pub more_fragments: bool,
  pub retry: bool,
  pub pwr_mgt: bool,
  pub more_data: bool,
  pub protected: bool,
  pub order: bool,
}

impl Flags {
  pub fn parse(bytes: &mut Iter<u8>) -> Result<Flags> {
    let byte = bytes.next().unwrap();

    let to_ds = byte & 0b0000_0001 != 0; // to Distribution System
    let from_ds = byte & 0b0000_0010 != 0; // from Distribution System

    // 00 Not leaving DS or network is operating in AD-HOC mode
    // 10 Frame from DS to a STA via AP
    // 01 Frame from STA to DS via an AP

    let more_fragments = (byte & 0b0000_0100) != 0; // 0: This is the last fragment
    let retry = (byte & 0b0000_1000) != 0; // 0: Frame is not being retransmitted
    let pwr_mgt = (byte & 0b0001_0000) != 0; // 0: STA will stay up
    let more_data = (byte & 0b0010_0000) != 0; // 0: No data buffered
    let protected = (byte & 0b0100_0000) != 0; // 0: Data is not protected
    let order = (byte & 0b1000_0000) != 0; // 0: Not strictly ordered

    Ok(Flags {
      to_ds,
      from_ds,
      more_fragments,
      retry,
      pwr_mgt,
      more_data,
      protected,
      order,
    })
  }
}
