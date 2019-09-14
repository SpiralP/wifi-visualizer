use super::*;
use ieee80211::MacAddress;
use serde_derive::*;
use std::collections::{HashMap, HashSet};
use time::{get_time, Duration, Timespec};

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data")] // {type: "NewAddress", data: "aa:aa:aa"}
pub enum Event {
  NewAddress(MacAddress),

  AccessPoint(MacAddress, AccessPointInfo),

  Connection(MacAddress, MacAddress, ConnectionType),

  ProbeRequest(MacAddress, Vec<u8>), // from, ssid

  // InactiveAddress(Vec<MacAddress>),
  // Loss(MacAddress, u64, u64), // addr, # lost, # received
  Signal(MacAddress, i8),

  Rate(MacAddress, u64),

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

pub struct Store {
  buffer: Vec<Event>,

  addresses: HashMap<MacAddress, Timespec>,
  connections: HashMap<String, ConnectionType>,
  access_points: HashMap<MacAddress, AccessPointInfo>,
  probes: HashMap<MacAddress, HashSet<Vec<u8>>>,
  frame_count: HashMap<MacAddress, u64>,
  // last_sequence_number: HashMap<(MacAddress, MacAddress), HashMap<FrameSubtype, u16>>,
  // data_frame_loss_count: HashMap<MacAddress, u64>,
  next_signal_event_update: HashMap<MacAddress, Timespec>,
  next_rate_event_update: HashMap<MacAddress, Timespec>,
  rate_last_frame_count: HashMap<MacAddress, u64>,

  signal_event_update_interval: Duration,
  rate_event_update_interval: Duration,
}

impl Store {
  pub fn new() -> Self {
    Self {
      buffer: Vec::new(),
      addresses: HashMap::new(),
      connections: HashMap::new(),
      access_points: HashMap::new(),
      probes: HashMap::new(),
      frame_count: HashMap::new(),
      next_signal_event_update: HashMap::new(),
      next_rate_event_update: HashMap::new(),
      rate_last_frame_count: HashMap::new(),

      signal_event_update_interval: Duration::seconds(1),
      rate_event_update_interval: Duration::seconds(1),
    }
  }

  pub fn flush_buffer(&mut self) -> Vec<Event> {
    self.buffer.drain(..).collect()
  }

  // pub fn check_for_inactive(&mut self) {
  //   let mut macs_to_remove: Vec<MacAddress> = Vec::new();

  //   let now = get_time();
  //   for (a, b) in &self.addresses {
  //     if (now - *b) > self.todo {
  //       macs_to_remove.push(*a);
  //     }
  //   }

  //   if macs_to_remove.is_empty() {
  //     return;
  //   }

  //   for mac in &macs_to_remove {
  //     self.addresses.remove(&mac);
  //   }

  //   self.buffer.push(Event::InactiveAddress(macs_to_remove));
  // }

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

  // pub fn update_loss(
  //   &mut self,
  //   transmitter_address: Option<MacAddress>,
  //   receiver_address: MacAddress,
  //   layer: &FrameLayer,
  // ) {
  //   fn get_loss(from: u16, to: u16, retry: bool) -> u16 {
  //     // TODO split data packets from the other types
  //     // for beacons, use beacon interval and current time

  //     if retry && from == to {
  //       // real 2, retry 2
  //       // retry 2, retry 2
  //       return 0;
  //     }

  //     let lost = if to > from {
  //       to - from - 1
  //     } else {
  //       // 4095 0 = 0
  //       // 4095 1 = 1 (0)
  //       // 4094 1 = 2 (4095, 0)

  //       // 12 bit number, we wrapped around 4096, 0-4095
  //       (4096 + to) - from - 1
  //     };

  //     if retry {
  //       // real 2, retry 3 (lost real 3)
  //       lost + 1
  //     } else {
  //       lost
  //     }
  //   }

  //   let frame = match &layer {
  //     FrameLayer::Control(_) | FrameLayer::Management(_) => {
  //       // Control frames don't have sequence numbers

  //       // Management frames such as probes have weird start numbers
  //       // also sometimes different types use the same number

  //       return;
  //     }

  //     FrameLayer::Data(frame) => frame,
  //   };

  //   if let DSStatus::FromSTAToDS = frame.ds_status() {
  //     // only clients, not access points
  //   } else {
  //     return;
  //   }

  //   let transmitter_address = if let Some(a) = transmitter_address {
  //     a
  //   } else {
  //     return;
  //   };

  //   let frame_type = frame.subtype();
  //   let sequence_number = frame.sequence_number();
  //   let retry = frame.retry();

  //   // {
  //   //   ("98-d6-f7-01-01-00", "54-a0-50-79-19-54"): {
  //   //     QoSData: 1,
  //   //     Data: 3,
  //   //   }
  //   // }

  //   let frame_types = self
  //     .last_sequence_number
  //     .entry((transmitter_address, receiver_address))
  //     .or_insert_with(HashMap::new);

  //   let data_frame_loss_count = self
  //     .data_frame_loss_count
  //     .entry(transmitter_address)
  //     .or_insert(0);

  //   frame_types
  //     .entry(frame_type)
  //     .and_modify(|ag| {
  //       let old_sequence_number = *ag;

  //       // TODO maybe have a set of all, then find all missing in the set in range min-max
  //       // or just use -10 - -100 dBm from radiotap?

  //       if retry && (sequence_number < old_sequence_number) {
  //         // if the seq is lower than the previous and we're retry
  //         // then we're retrying an old frame so who cares
  //         return;
  //       }

  //       *ag = sequence_number;

  //       if sequence_number == 0 {
  //         // could mark the beginning of the counter reset
  //         return;
  //       }

  //       if retry {
  //         // we can't trust getting loss from retry
  //         // sometimes it goes like real 200, retry 400
  //         return;
  //       }

  //       let lost = get_loss(old_sequence_number, sequence_number, retry);

  //       if lost != 0 {
  //         let lost: u64 = lost.into();

  //         debug!(
  //           "{:?} {} {} - {} lost {}",
  //           frame_type, transmitter_address, old_sequence_number, sequence_number, lost
  //         );

  //         *data_frame_loss_count += lost
  //       }
  //     })
  //     .or_insert(sequence_number);

  //   let data_frame_count_todo_this_is_the_old_number = self
  //     .data_frame_count
  //     .entry(transmitter_address)
  //     .and_modify(|count| *count += 1)
  //     .or_insert(1);

  //   // TODO interval this so no spam
  //   self.buffer.push(Event::Loss(
  //     transmitter_address,
  //     *data_frame_loss_count,
  //     *data_frame_count,
  //   ));
  // }

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

  pub fn update_signal(&mut self, transmitter_address: MacAddress, signal: i8) {
    let now = get_time();

    if let Some(next_time) = self.next_signal_event_update.get(&transmitter_address) {
      if now < *next_time {
        // not yet
        return;
      }
    }

    self
      .next_signal_event_update
      .insert(transmitter_address, now + self.signal_event_update_interval);
    self.buffer.push(Event::Signal(transmitter_address, signal));
  }

  pub fn update_rate(&mut self, transmitter_address: MacAddress) {
    let frame_count = *self
      .frame_count
      .entry(transmitter_address)
      .and_modify(move |frame_count| *frame_count += 1)
      .or_insert(1);

    let now = get_time();

    if let Some(next_time) = self.next_rate_event_update.get(&transmitter_address) {
      if now < *next_time {
        // not yet
        return;
      }
    }

    let last_frame_count = if let Some(last_frame_count) = self
      .rate_last_frame_count
      .insert(transmitter_address, frame_count)
    {
      last_frame_count
    } else {
      0
    };

    let rate = frame_count - last_frame_count;

    self
      .next_rate_event_update
      .insert(transmitter_address, now + self.rate_event_update_interval);

    self.buffer.push(Event::Rate(transmitter_address, rate));
  }
}
