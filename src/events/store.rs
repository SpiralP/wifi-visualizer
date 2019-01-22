use super::*;
use ieee802_11::MacAddress;
use serde_derive::*;
use std::collections::HashMap;
use std::collections::HashSet;
use time::{get_time, Timespec};

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data")] // {type: "NewAddress", data: "aa:aa:aa"}
pub enum Event {
  NewAddress(MacAddress),

  SetKind(MacAddress, Kind),

  Connection(MacAddress, MacAddress, ConnectionType),

  ProbeRequest(MacAddress, Vec<u8>), // from ssid

  InactiveAddress(Vec<MacAddress>),
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum ConnectionType {
  Associated,
  Disassociated,
  InRange,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum Kind {
  AccessPoint(Vec<u8>), // { type: "SetKind", data: { type: "AccessPoint", data: [0] } }
}

pub struct Store {
  addresses: HashMap<MacAddress, Timespec>,
  connections: HashMap<String, ConnectionType>,
  kinds: HashMap<MacAddress, Kind>,
  probes: HashMap<MacAddress, HashSet<Vec<u8>>>,

  event_handler: Box<FnMut(Event)>,
}

impl Store {
  pub fn new(event_handler: Box<FnMut(Event)>) -> Store {
    Store {
      addresses: HashMap::new(),
      connections: HashMap::new(),
      kinds: HashMap::new(),
      probes: HashMap::new(),

      event_handler,
    }
  }

  pub fn check_for_inactive(&mut self) {
    let mut macs_to_remove: Vec<MacAddress> = Vec::new();

    let now = get_time();
    for (a, b) in &self.addresses {
      if (now - *b) > time::Duration::seconds(5) {
        macs_to_remove.push(*a);
      }
    }

    if macs_to_remove.is_empty() {
      return;
    }

    for mac in &macs_to_remove {
      self.addresses.remove(&mac);
    }

    (self.event_handler)(Event::InactiveAddress(macs_to_remove));
  }

  pub fn add_address(&mut self, mac: MacAddress) {
    if is_broadcast(mac) {
      return;
    }

    let now = get_time();

    let new = self.addresses.get(&mac).is_none();

    self.addresses.insert(mac, now);
    if new {
      (self.event_handler)(Event::NewAddress(mac));
    }
  }

  pub fn change_kind(&mut self, mac: MacAddress, kind: Kind) {
    if is_broadcast(mac) {
      return; // TODO
    } // TODO

    if let Some(old_kind) = self.kinds.get(&mac) {
      if kind == *old_kind {
        return;
      }
    }

    self.add_address(mac);

    self.kinds.insert(mac, kind.clone());
    (self.event_handler)(Event::SetKind(mac, kind));
  }

  pub fn change_connection(&mut self, mac1: MacAddress, mac2: MacAddress, kind: ConnectionType) {
    if is_broadcast(mac1) {
      return; // TODO
    } // TODO
    if is_broadcast(mac2) {
      return;
    }

    let hash = hash_macs(mac1, mac2);

    if let Some(old_kind) = self.connections.get(&hash) {
      if kind == *old_kind {
        return;
      }

      if let ConnectionType::InRange = kind {
        // if we were associated/disassociated that means we were in range!

        match old_kind {
          ConnectionType::Associated | ConnectionType::Disassociated => {
            return;
          }

          ConnectionType::InRange => {}
        }
      }
    }

    self.add_address(mac1);
    self.add_address(mac2);

    self.connections.insert(hash, kind.clone());
    (self.event_handler)(Event::Connection(mac1, mac2, kind));
  }

  pub fn probe_request(&mut self, mac: MacAddress, ssid: Vec<u8>) {
    if is_broadcast(mac) {
      return;
    }

    if let Some(ssid_list) = self.probes.get_mut(&mac) {
      if ssid_list.contains(&ssid) {
        return;
      } else {
        ssid_list.insert(ssid.clone());
        (self.event_handler)(Event::ProbeRequest(mac, ssid));
      }
    } else {
      // new list
      let mut ssid_list = HashSet::new();
      ssid_list.insert(ssid.clone());
      self.probes.insert(mac, ssid_list);
      (self.event_handler)(Event::ProbeRequest(mac, ssid));
    }
  }
}
