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

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Kind {
  AccessPoint(Vec<u8>), // { type: "SetKind", data: { type: "AccessPoint", data: [0] } }
  Station,
}
impl Kind {
  fn is_access_point(&self) -> bool {
    match self {
      Kind::AccessPoint(_) => true,
      _ => false,
    }
  }
  fn is_station(&self) -> bool {
    match self {
      Kind::Station => true,
      _ => false,
    }
  }
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
      if old_kind.is_access_point() && kind.is_access_point() {
        return;
      }
      if old_kind.is_station() && kind.is_station() {
        return;
      }
    }

    self.add_address(mac);

    // TODO stop cloning!
    self.kinds.insert(mac, kind.clone());
    (self.event_handler)(Event::SetKind(mac, kind));
  }
}

pub fn handle_frame(frame: Frame, store: &mut Store) {
  let basic_frame = match frame {
    Frame::Basic(ref frame) => &frame,
    Frame::Beacon(ref frame) => &frame.management_frame.basic_frame,
    Frame::ProbeRequest(ref frame) => &frame.management_frame.basic_frame,
    Frame::Management(ref frame) => &frame.basic_frame,
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

    _ => {
      // other FrameType
    }
  }

  match frame {
    Frame::Beacon(ref beacon_frame) => {
      store.set_kind(
        basic_frame
          .transmitter_address
          .expect("no transmitter_address on Beacon"),
        Kind::AccessPoint(beacon_frame.ssid.clone()),
      );
    }

    _ => {}
  }
}
