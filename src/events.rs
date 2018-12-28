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

  SetKind(MacAddress, Kind),

  Connection(MacAddress, MacAddress, ConnectionType),

  ProbeRequest(MacAddress, Vec<u8>), // from ssid
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
  addresses: HashSet<MacAddress>,
  connections: HashMap<String, ConnectionType>,
  kinds: HashMap<MacAddress, Kind>,
  probes: HashMap<MacAddress, HashSet<Vec<u8>>>,

  event_handler: Box<FnMut(Event)>,
}

impl Store {
  pub fn new(event_handler: Box<FnMut(Event)>) -> Store {
    Store {
      addresses: HashSet::new(),
      connections: HashMap::new(),
      kinds: HashMap::new(),
      probes: HashMap::new(),

      event_handler,
    }
  }

  fn add_address(&mut self, mac: MacAddress) {
    if is_broadcast(mac) {
      return;
    }

    if self.addresses.contains(&mac) {
      return;
    }

    self.addresses.insert(mac);
    (self.event_handler)(Event::NewAddress(mac));
  }

  fn change_kind(&mut self, mac: MacAddress, kind: Kind) {
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

  fn change_connection(&mut self, mac1: MacAddress, mac2: MacAddress, kind: ConnectionType) {
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

  fn probe_request(&mut self, mac: MacAddress, ssid: Vec<u8>) {
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

pub fn handle_frame(frame: Frame, store: &mut Store) {
  let basic_frame = match frame {
    Frame::Basic(ref frame) => &frame,
    Frame::Beacon(ref frame) => &frame.management_frame.basic_frame,
    Frame::ProbeRequest(ref frame) => &frame.management_frame.basic_frame,
    Frame::ProbeResponse(ref frame) => &frame.management_frame.basic_frame,
    Frame::Management(ref frame) => &frame.basic_frame,
  };

  if let Some(transmitter_address) = basic_frame.transmitter_address {
    store.add_address(transmitter_address);
  }

  if let Some(receiver_address) = basic_frame.receiver_address {
    store.add_address(receiver_address);
  }

  // check for connections
  let mut is_associated = false;
  match basic_frame.type_ {
    FrameType::Data(ref subtype) => {
      match subtype {
        DataSubtype::Data | DataSubtype::QoSData => {
          // most likely a connection
          is_associated = true;
        }

        _ => {
          // other DataSubtype
        }
      }
    }

    FrameType::Management(ref subtype) => match subtype {
      ManagementSubtype::Authentication
      | ManagementSubtype::AssociationRequest
      | ManagementSubtype::AssociationResponse
      | ManagementSubtype::ReassociationRequest
      | ManagementSubtype::ReassociationResponse => {
        // Authentication is 2 way
        // _Request is from STA
        // _Response is from AP

        is_associated = true;
      }

      ManagementSubtype::Disassociation | ManagementSubtype::Deauthentication => {
        // TODO broadcast? is it sent from router?
        // Disassociation is from STA
        // Deauthentication is from AP

        store.change_connection(
          basic_frame
            .transmitter_address
            .expect("no ta on disassociation"),
          basic_frame
            .receiver_address
            .expect("no ra on disassociation"),
          ConnectionType::Disassociated,
        );
      }

      _ => {
        // other ManagementSubtype
      }
    },

    FrameType::Control(_) => {
      // anyone in range will use these
    }
  }

  // if two nodes are communicating
  if let Some(receiver_address) = basic_frame.receiver_address {
    if let Some(transmitter_address) = basic_frame.transmitter_address {
      if is_associated {
        store.change_connection(
          transmitter_address,
          receiver_address,
          ConnectionType::Associated,
        );
      } else {
        // ra & ta have communicated!
        store.change_connection(
          transmitter_address,
          receiver_address,
          ConnectionType::InRange,
        );
      }
    }
  }

  // frames with special info that's sent
  match frame {
    Frame::Beacon(ref beacon_frame) => {
      store.change_kind(
        basic_frame.transmitter_address.expect("no ta on Beacon"),
        Kind::AccessPoint(beacon_frame.ssid.clone()),
      );
    }

    Frame::ProbeResponse(ref probe_response_frame) => {
      store.change_kind(
        basic_frame
          .transmitter_address
          .expect("no ta on ProbeResponse"),
        Kind::AccessPoint(probe_response_frame.ssid.clone()),
      );
    }

    Frame::ProbeRequest(ref probe_request_frame) => {
      if !probe_request_frame.ssid.is_empty() {
        let ta = probe_request_frame
          .management_frame
          .basic_frame
          .transmitter_address
          .expect("no ta on ProbeRequest");

        store.probe_request(ta, probe_request_frame.ssid.clone());
      }
    }

    _ => {}
  }
}
