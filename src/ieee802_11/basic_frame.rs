use super::*;

#[derive(Debug)]
pub struct BasicFrame {
  pub type_: FrameType,

  pub duration: u16, // microseconds

  pub receiver_address: Option<MacAddress>,
  pub transmitter_address: Option<MacAddress>,

  pub destination_address: Option<MacAddress>,
  pub source_address: Option<MacAddress>,

  pub bssid: Option<MacAddress>,
}

impl BasicFrame {
  pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<BasicFrame> {
    let frame_control = FrameControl::parse(bytes)?;
    let duration = bytes.read_u16::<LE>().unwrap();

    let addr1 = MacAddress::from(bytes);

    let receiver_address = Some(addr1);
    let mut transmitter_address = None;

    let mut destination_address = None;
    let mut source_address = None;

    let mut bssid = None;

    let mut other = false;
    match frame_control.type_ {
      FrameType::Control(ref subtype) => {
        match subtype {
          ControlSubtype::ACK | ControlSubtype::CTS => {
            // only receiver
          }

          ControlSubtype::RTS | ControlSubtype::BlockAck | ControlSubtype::BlockAckRequest => {
            // only receiver + transmitter
            let addr2 = MacAddress::from(bytes);
            transmitter_address = Some(addr2);
          }

          ControlSubtype::CFEnd => {
            let addr2 = MacAddress::from(bytes);
            bssid = Some(addr2);
          }

          _ => {
            other = true;
          }
        }
      }

      _ => {
        other = true;
      }
    }

    if other {
      let addr2 = MacAddress::from(bytes);
      let addr3 = MacAddress::from(bytes);
      transmitter_address = Some(addr2);
      // https://networkengineering.stackexchange.com/questions/25100/four-layer-2-addresses-in-802-11-frame-header
      match (frame_control.flags.to_ds, frame_control.flags.from_ds) {
        (false, false) => {
          // from one STA to another STA, plus all management/control type frames
          destination_address = Some(addr1);
          source_address = Some(addr2);
          bssid = Some(addr3);
        }
        (false, true) => {
          // exiting the DS
          destination_address = Some(addr1);
          bssid = Some(addr2);
          source_address = Some(addr3);
        }
        (true, false) => {
          // destined for the DS
          bssid = Some(addr1);
          source_address = Some(addr2);
          destination_address = Some(addr3);
        }
        (true, true) => {
          // one AP to another AP
          let addr4 = MacAddress::from(bytes);

          destination_address = Some(addr3);
          source_address = Some(addr4);
        }
      }
    }

    Ok(BasicFrame {
      type_: frame_control.type_,
      duration,
      receiver_address,
      destination_address,
      transmitter_address,
      source_address,
      bssid,
    })
  }
}
