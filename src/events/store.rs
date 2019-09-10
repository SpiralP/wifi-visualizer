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

  Error(String),
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

#[derive(Default)]
pub struct Store {
  addresses: HashMap<MacAddress, Timespec>,
  connections: HashMap<String, ConnectionType>,
  access_points: HashMap<MacAddress, AccessPointInfo>,
  probes: HashMap<MacAddress, HashSet<Vec<u8>>>,
  last_sequence_number: HashMap<(MacAddress, MacAddress), HashMap<FrameSubtype, u16>>,

  buffer: Vec<Event>,
}

impl Store {
  pub fn new() -> Self {
    Self::default()
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

  pub fn update_loss(
    &mut self,
    transmitter_address: Option<MacAddress>,
    receiver_address: MacAddress,
    layer: &FrameLayer,
  ) {
    use std::str::FromStr;

    fn get_lost(from: u16, to: u16, retry: bool) -> u16 {
      // TODO split data packets from the other types
      // for beacons, use beacon interval and current time

      if retry && from == to {
        // real 2, retry 2
        // retry 2, retry 2
        return 0;
      }

      let lost = if to > from {
        to - from - 1
      } else if to < from {
        // 4095 0 = 0
        // 4095 1 = 1 (0)
        // 4094 1 = 2 (4095, 0)

        // 12 bit number, we wrapped around 4096, 0-4095
        (4096 + to) - from - 1
      } else {
        // more likely to be a retransmission than a full cycle around 4096
        0
      };

      if retry {
        // real 2, retry 3 (lost real 3)
        lost + 1
      } else {
        lost
      }
    }

    let transmitter_address = if let Some(a) = transmitter_address {
      a
    } else {
      return;
    };

    if transmitter_address != MacAddress::from_str("98-d6-f7-01-01-00").unwrap() {
      return;
    }

    let frame = match &layer {
      FrameLayer::Control(_) | FrameLayer::Management(_) => {
        // Control frames don't have sequence numbers

        // Management frames such as probes have weird start numbers
        // also sometimes different types use the same number

        return;
      }

      FrameLayer::Data(frame) => frame,
    };

    let frame_type = frame.subtype();
    let sequence_number = frame.sequence_number();
    let retry = frame.retry();

    // {
    //   ("98-d6-f7-01-01-00", "54-a0-50-79-19-54"): {
    //     QoSData: 1,
    //     Data: 3,
    //   }
    // }

    let frame_types = self
      .last_sequence_number
      .entry((transmitter_address, receiver_address))
      .or_insert_with(HashMap::new);

    frame_types
      .entry(frame_type)
      .and_modify(move |old_sequence_number| {
        if sequence_number != 0 {
          let lost = get_lost(*old_sequence_number, sequence_number, retry);

          if lost != 0 {
            println!("{:?} {} lost {}", frame_type, sequence_number, lost);
          }
        }

        *old_sequence_number = sequence_number;
      })
      .or_insert(sequence_number);
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
