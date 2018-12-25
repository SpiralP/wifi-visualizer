use crate::ieee802_11::util::*;
use crate::ieee802_11::*;
use serde_derive::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data")] // {type: "NewAddress", data: "aa:aa:aa"}
enum Event {
  NewAddress(MacAddress),
  Connection(MacAddress, MacAddress),
  Leave(MacAddress, MacAddress), // x leaves y
  SetKind(MacAddress, Kind),
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone)]
enum Kind {
  AccessPoint,
  Station,
}

#[derive(Default, Debug)]
pub struct Store {
  addresses: HashSet<MacAddress>,
  connections: HashSet<String>,
  kinds: HashMap<MacAddress, Kind>,

  events: VecDeque<Event>,
}

impl Store {
  pub fn new() -> Store {
    Store {
      ..Default::default()
    }
  }

  fn add_address(&mut self, mac: MacAddress) {
    if self.addresses.contains(&mac) {
      return;
    }

    self.addresses.insert(mac);
    self.events.push_back(Event::NewAddress(mac));
  }

  fn add_connection(&mut self, mac1: MacAddress, mac2: MacAddress) {
    let hash = hash_macs(mac1, mac2);

    if self.connections.contains(&hash) {
      return;
    }

    self.connections.insert(hash);
    self.events.push_back(Event::Connection(mac1, mac2));
  }

  fn set_kind(&mut self, mac: MacAddress, kind: Kind) {
    if let Some(old_kind) = self.kinds.get(&mac) {
      if *old_kind == kind {
        return;
      }
    }

    self.kinds.insert(mac, kind);
    self.events.push_back(Event::SetKind(mac, kind));
  }
}

pub fn handle_frame(frame: Frame, store: &mut Store) {
  let basic_frame = match frame {
    Frame::Basic(ref frame) => &frame,
    Frame::Beacon(ref frame) => &frame.basic_frame,
  };

  if let Some(transmitter_address) = basic_frame.transmitter_address {
    store.add_address(transmitter_address);
  }

  if let Some(receiver_address) = basic_frame.receiver_address {
    // if intended for broadcast TODO
    if is_broadcast(receiver_address) {
      return;
    }

    store.add_address(receiver_address);
  }

  match basic_frame.type_ {
    FrameType::Data(ref subtype) => {
      // most likely a connection
      store.add_connection(
        basic_frame.transmitter_address.unwrap(),
        basic_frame.receiver_address.unwrap(),
      );
    }

    FrameType::Management(ref subtype) => match subtype {
      ManagementSubtype::Beacon => {
        store.set_kind(basic_frame.transmitter_address.unwrap(), Kind::AccessPoint);
      }
      _ => {}
    },

    _ => {}
  }
}
