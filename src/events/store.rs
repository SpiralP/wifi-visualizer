use super::{hash_macs, is_broadcast};
use ieee80211::MacAddress;
use serde::Serialize;
use std::{
  collections::{HashMap, HashSet},
  time::{Duration, Instant},
};

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data")] // {type: "NewAddress", data: "aa:aa:aa"}
pub enum Event {
  NewAddress(MacAddress),

  AccessPoint(MacAddress, AccessPointInfo),

  Connection(MacAddress, MacAddress, ConnectionType),

  ProbeRequest(MacAddress, Vec<u8>), // from, ssid

  // Loss(MacAddress, u64, u64), // addr, # lost, # received
  Signal(MacAddress, i8),

  Rate(MacAddress, u64),

  // #received, #correct
  BeaconQuality(MacAddress, u64, u64),

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

  addresses: HashMap<MacAddress, Instant>,

  // TODO instead of String use (mac1, mac2) in hashed sorted
  connections: HashMap<String, ConnectionType>,
  access_points: HashMap<MacAddress, AccessPointInfo>,
  probes: HashMap<MacAddress, HashSet<Vec<u8>>>,
  frame_count: HashMap<MacAddress, u64>,
  beacon_count: HashMap<MacAddress, u64>,
  // last_sequence_number: HashMap<(MacAddress, MacAddress), HashMap<FrameSubtype, u16>>,
  // data_frame_loss_count: HashMap<MacAddress, u64>,
  next_signal_event_update: HashMap<MacAddress, Instant>,
  next_rate_event_update: HashMap<MacAddress, Instant>,
  rate_last_frame_count: HashMap<MacAddress, u64>,

  // interval, first beacon time
  beacon_quality_intervals: HashMap<MacAddress, (f64, Instant)>,
  next_beacon_quality_update: HashMap<MacAddress, Instant>,

  signals: HashMap<MacAddress, (i8, Instant)>,

  signal_event_update_interval: Duration,
  signal_event_timeout: Duration,
  rate_event_update_interval: Duration,
  beacon_quality_update_interval: Duration,
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
      beacon_count: HashMap::new(),
      next_signal_event_update: HashMap::new(),
      next_rate_event_update: HashMap::new(),
      rate_last_frame_count: HashMap::new(),
      beacon_quality_intervals: HashMap::new(),
      next_beacon_quality_update: HashMap::new(),
      signals: HashMap::new(),

      signal_event_update_interval: Duration::from_secs(1),
      signal_event_timeout: Duration::from_secs(5),
      rate_event_update_interval: Duration::from_secs(1),
      beacon_quality_update_interval: Duration::from_secs(1),
    }
  }

  pub fn flush_buffer(&mut self) -> Vec<Event> {
    self.buffer.drain(..).collect()
  }

  pub fn add_address(&mut self, mac: MacAddress) {
    let now = Instant::now();

    if self.addresses.insert(mac, now).is_none() {
      self.buffer.push(Event::NewAddress(mac));
    }
  }

  pub fn access_point(&mut self, mac: MacAddress, info: AccessPointInfo) {
    if self.access_points.contains_key(&mac) {
      // TODO changing ssid/channel?
      return;
    }

    self.access_points.insert(mac, info.clone());
    self.buffer.push(Event::AccessPoint(mac, info));
  }

  pub fn change_connection(
    &mut self,
    transmitter_address: MacAddress,
    receiver_address: MacAddress,
    kind: ConnectionType,
  ) {
    if is_broadcast(receiver_address) {
      return;
    }

    let hash = hash_macs(transmitter_address, receiver_address);

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
    self.buffer.push(Event::Connection(
      transmitter_address,
      receiver_address,
      kind,
    ));
  }

  pub fn probe_request(&mut self, mac: MacAddress, ssid: Vec<u8>) {
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

  pub fn update_beacon_quality(&mut self, transmitter_address: MacAddress, interval: f64) {
    self
      .beacon_count
      .entry(transmitter_address)
      .and_modify(move |beacon_count| *beacon_count += 1)
      .or_insert(1);

    // TODO add timeout like signal, after 5 seconds, reset %
    if !self
      .beacon_quality_intervals
      .contains_key(&transmitter_address)
    {
      let now = Instant::now();

      self
        .beacon_quality_intervals
        .insert(transmitter_address, (interval, now)); // first beacon seen

      self.next_beacon_quality_update.insert(
        transmitter_address,
        now + self.beacon_quality_update_interval,
      );

      self
        .buffer
        .push(Event::BeaconQuality(transmitter_address, 1, 1));
    }
  }

  pub fn update_signal(&mut self, transmitter_address: MacAddress, signal: i8) {
    let now = Instant::now();

    self.signals.insert(transmitter_address, (signal, now));

    if !self
      .next_signal_event_update
      .contains_key(&transmitter_address)
    {
      // TODO maybe just insert now, let the timer handle the sending
      self
        .next_signal_event_update
        .insert(transmitter_address, now + self.signal_event_update_interval);

      self.buffer.push(Event::Signal(transmitter_address, signal));
    }
  }

  pub fn update_rate(&mut self, transmitter_address: MacAddress) {
    self
      .frame_count
      .entry(transmitter_address)
      .and_modify(move |frame_count| *frame_count += 1)
      .or_insert(1);

    if !self
      .next_rate_event_update
      .contains_key(&transmitter_address)
    {
      let now = Instant::now();

      self
        .next_rate_event_update
        .insert(transmitter_address, now + self.rate_event_update_interval);

      self.buffer.push(Event::Rate(transmitter_address, 1));
    }
  }

  pub fn check_timers(&mut self) {
    // TODO this is only called if packets are arriving!

    let now = Instant::now();

    // check beacon quality timers
    for (transmitter_address, next_interval) in &mut self.next_beacon_quality_update {
      if now < *next_interval {
        continue;
      }
      *next_interval = now + self.beacon_quality_update_interval;

      // TODO eventually reset this every (interval) ?

      let (interval, start_time) = self
        .beacon_quality_intervals
        .get(transmitter_address)
        .expect("beacon_quality_intervals.get");
      let received_count = *self
        .beacon_count
        .get(transmitter_address)
        .expect("beacon_count.get");

      #[allow(clippy::cast_sign_loss)]
      #[allow(clippy::cast_possible_truncation)]
      let real_count = ((now.duration_since(*start_time).as_secs_f64() / interval) as u64) + 1;

      self.buffer.push(Event::BeaconQuality(
        *transmitter_address,
        received_count,
        real_count,
      ));
    }

    // check rate timers
    let mut to_remove = Vec::new();
    for (transmitter_address, next_interval) in &mut self.next_rate_event_update {
      if now < *next_interval {
        continue;
      }

      let frame_count = *self
        .frame_count
        .get(transmitter_address)
        .expect("frame_count.get");

      let last_frame_count = if let Some(last_frame_count) = self
        .rate_last_frame_count
        .insert(*transmitter_address, frame_count)
      {
        last_frame_count
      } else {
        0
      };

      let rate = frame_count - last_frame_count;

      self.buffer.push(Event::Rate(*transmitter_address, rate));

      if rate == 0 {
        to_remove.push(*transmitter_address);
      } else {
        *next_interval = now + self.rate_event_update_interval;
      }
    }

    for key in to_remove {
      self.next_rate_event_update.remove(&key);
    }

    // update signals
    let mut to_remove = Vec::new();
    for (transmitter_address, next_interval) in &mut self.next_signal_event_update {
      // if time is too long, send out of range

      if now < *next_interval {
        continue;
      }

      let (signal, time) = self.signals.get(transmitter_address).expect("signals.get");

      if now.duration_since(*time) >= self.signal_event_timeout {
        self.buffer.push(Event::Signal(*transmitter_address, 0));

        to_remove.push(*transmitter_address);
      } else {
        self
          .buffer
          .push(Event::Signal(*transmitter_address, *signal));

        *next_interval = now + self.rate_event_update_interval;
      }
    }

    for key in to_remove {
      self.next_signal_event_update.remove(&key);
    }
  }
}
