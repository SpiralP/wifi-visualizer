use crate::ieee802_11::frame_control::*;
use crate::ieee802_11::util::*;
use crate::ieee802_11::*;
use serde_derive::*;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data")] // {type: "NewAddress", data: "aa:aa:aa"}
pub enum Event {
  NewAddress(MacAddress),

  Connection(MacAddress, MacAddress),
  SetKind(MacAddress, Kind),

  ProbeRequest(MacAddress, String),
  ProbeResponse(),

  // x joins y
  Join(MacAddress, MacAddress),
  Leave(MacAddress, MacAddress),
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone)]
pub enum Kind {
  AccessPoint,
  Station,
}

pub struct Store {
  addresses: HashSet<MacAddress>,
  connections: HashSet<String>,
  kinds: HashMap<MacAddress, Kind>,

  event_handler: Box<FnMut(Event)>,
}

impl Store {
  pub fn new(event_handler: Box<FnMut(Event)>) -> Store {
    Store {
      addresses: HashSet::new(),
      connections: HashSet::new(),
      kinds: HashMap::new(),

      event_handler,
    }
  }

  fn add_address(&mut self, mac: MacAddress) {
    if self.addresses.contains(&mac) {
      return;
    }

    self.addresses.insert(mac);
    (self.event_handler)(Event::NewAddress(mac));
  }

  fn add_connection(&mut self, mac1: MacAddress, mac2: MacAddress) {
    let hash = hash_macs(mac1, mac2);

    if self.connections.contains(&hash) {
      return;
    }

    self.add_address(mac1);
    self.add_address(mac2);

    self.connections.insert(hash);
    (self.event_handler)(Event::Connection(mac1, mac2));
  }

  fn set_kind(&mut self, mac: MacAddress, kind: Kind) {
    if let Some(old_kind) = self.kinds.get(&mac) {
      if *old_kind == kind {
        return;
      }
    }

    self.add_address(mac);

    self.kinds.insert(mac, kind);
    (self.event_handler)(Event::SetKind(mac, kind));
  }
}

pub fn handle_frame(frame: Frame, store: &mut Store) {
  let basic_frame = match frame {
    Frame::Basic(ref frame) => &frame,
    Frame::Beacon(ref frame) => &frame.basic_frame,
  };

  match basic_frame.type_ {
    FrameType::Data(ref subtype) => {
      // most likely a connection

      match subtype {
        DataSubtype::Data | DataSubtype::QoSData => {
          let transmitter_address = basic_frame.transmitter_address.expect("no trans on Data");
          let receiver_address = basic_frame.receiver_address.expect("no recv on Data");

          if is_broadcast(receiver_address) {
            return;
          }

          store.add_connection(transmitter_address, receiver_address);
          // if let Some(bssid) = basic_frame.bssid {
          //   if transmitter_address == bssid {
          //     // we are an AP
          //     store.set_kind(transmitter_address, Kind::AccessPoint);
          //   } else if receiver_address == bssid {
          //     // we are a station
          //     store.set_kind(receiver_address, Kind::Station);
          //   }
          // }
        }

        _ => {
          // other DataSubtype
        }
      }
    }

    FrameType::Management(ref subtype) => {
      //
      match subtype {
        ManagementSubtype::Beacon => {
          store.set_kind(
            basic_frame
              .transmitter_address
              .expect("no transmitter_address on Beacon"),
            Kind::AccessPoint,
          );
        }
        _ => {
          // other ManagementSubtype
        }
      }
    }

    _ => {
      // other FrameType
    }
  }
}
