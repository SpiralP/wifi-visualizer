use super::*;
use ieee80211::MacAddress;
use serde_derive::*;
use std::collections::{HashMap, HashSet};
use time::{get_time, Timespec};

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data")] // {type: "NewAddress", data: "aa:aa:aa"}
pub enum Event {
  NewAddress(MacAddress),

  AccessPoint(MacAddress, AccessPointInfo),

  Connection(MacAddress, MacAddress, ConnectionType),

  ProbeRequest(MacAddress, Vec<u8>), // from, ssid

  InactiveAddress(Vec<MacAddress>),
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct AccessPointInfo {
  pub ssid: Vec<u8>,
  pub channel: Option<u8>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum ConnectionType {
  Associated,
  Authentication,
  Disassociated,
  InRange,
}

pub struct Store {
  addresses: HashMap<MacAddress, Timespec>,
  connections: HashMap<String, ConnectionType>,
  access_points: HashMap<MacAddress, AccessPointInfo>,
  probes: HashMap<MacAddress, HashSet<Vec<u8>>>,

  buffer: Vec<Event>,
}

impl Store {
  pub fn new() -> Self {
    Self {
      addresses: HashMap::new(),
      connections: HashMap::new(),
      access_points: HashMap::new(),
      probes: HashMap::new(),

      buffer: Vec::new(),
    }
  }

  pub fn flush_buffer(&mut self) -> Vec<Event> {
    self.buffer.drain(..).collect()
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

    self.buffer.push(Event::InactiveAddress(macs_to_remove));
  }

  pub fn add_address(&mut self, mac: MacAddress) {
    if is_broadcast(mac) {
      return;
    }

    let now = get_time();

    let is_new_addr = self.addresses.get(&mac).is_none();

    self.addresses.insert(mac, now);
    if is_new_addr {
      self.buffer.push(Event::NewAddress(mac));
    }
  }

  pub fn access_point(&mut self, mac: MacAddress, info: AccessPointInfo) {
    if is_broadcast(mac) {
      return; // TODO
    } // TODO

    if self.access_points.contains_key(&mac) {
      return;
    }

    self.add_address(mac);

    self.access_points.insert(mac, info.clone());
    self.buffer.push(Event::AccessPoint(mac, info));
  }

  pub fn change_connection(&mut self, mac1: MacAddress, mac2: MacAddress, kind: ConnectionType) {
    self.add_address(mac1); // TODO these go higher level
    self.add_address(mac2);

    if is_broadcast(mac1) || is_broadcast(mac2) {
      return;
    }

    let hash = hash_macs(mac1, mac2);

    if let Some(old_kind) = self.connections.get(&hash) {
      if kind == *old_kind {
        return;
      }

      match kind {
        ConnectionType::Associated => {
          if let ConnectionType::Authentication = old_kind {
            // keep Authentication over basic Associated
            return;
          }
        }
        ConnectionType::InRange => {
          // if we had a better type that means we were already in range!
          return;
        }
        ConnectionType::Authentication | ConnectionType::Disassociated => {}
      }
    }

    self.connections.insert(hash, kind.clone());
    self.buffer.push(Event::Connection(mac1, mac2, kind));
  }

  pub fn probe_request(&mut self, mac: MacAddress, ssid: Vec<u8>) {
    if is_broadcast(mac) {
      return;
    }

    if let Some(ssid_list) = self.probes.get_mut(&mac) {
      if !ssid_list.contains(&ssid) {
        ssid_list.insert(ssid.clone());
        self.buffer.push(Event::ProbeRequest(mac, ssid));
      }
    } else {
      // new list
      let mut ssid_list = HashSet::new();
      ssid_list.insert(ssid.clone());
      self.probes.insert(mac, ssid_list);
      self.buffer.push(Event::ProbeRequest(mac, ssid));
    }
  }
}
