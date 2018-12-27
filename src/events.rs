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

  NewConnection(MacAddress, MacAddress),
  RemoveConnection(MacAddress, MacAddress),

  ProbeRequest(MacAddress, Vec<u8>),              // from ssid
  ProbeResponse(MacAddress, MacAddress, Vec<u8>), // from to ssid
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
    if let Some(old_kind) = self.kinds.get(&mac) {
      if old_kind.is_access_point() && kind.is_access_point() {
        return;
      }
      if old_kind.is_station() && kind.is_station() {
        return;
      }
    }

    self.add_address(mac);

    self.kinds.insert(mac, kind.clone());
    (self.event_handler)(Event::SetKind(mac, kind));
  }

  fn add_connection(&mut self, mac1: MacAddress, mac2: MacAddress) {
    if is_broadcast(mac1) {
      return; // TODO
    } // TODO
    if is_broadcast(mac2) {
      return;
    }

    let hash = hash_macs(mac1, mac2);

    if self.connections.contains(&hash) {
      return;
    }

    self.add_address(mac1);
    self.add_address(mac2);

    self.connections.insert(hash);
    (self.event_handler)(Event::NewConnection(mac1, mac2));
  }

  fn remove_connection(&mut self, mac1: MacAddress, mac2: MacAddress) {
    let hash = hash_macs(mac1, mac2);

    if !self.connections.contains(&hash) {
      return;
    }

    self.add_address(mac1);
    self.add_address(mac2);

    self.connections.remove(&hash);
    (self.event_handler)(Event::RemoveConnection(mac1, mac2));
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

    if let Some(transmitter_address) = basic_frame.transmitter_address {
      store.add_connection(transmitter_address, receiver_address);
    }
  }

  // events based on frame type
  match basic_frame.type_ {
    // FrameType::Data(ref subtype) => {
    //   match subtype {
    //     DataSubtype::Data | DataSubtype::QoSData => {
    //       // most likely a connection

    //       let transmitter_address = basic_frame.transmitter_address.expect("no trans on Data");
    //       let receiver_address = basic_frame.receiver_address.expect("no recv on Data");

    //       store.add_connection(transmitter_address, receiver_address);
    //     }

    //     _ => {
    //       // other DataSubtype
    //     }
    //   }
    // }
    FrameType::Management(ref subtype) => match subtype {
      //   ManagementSubtype::Authentication
      //   | ManagementSubtype::AssociationRequest
      //   | ManagementSubtype::AssociationResponse
      //   | ManagementSubtype::ReassociationRequest
      //   | ManagementSubtype::ReassociationResponse => {
      //     // Authentication is 2 way
      //     // _Request is from STA
      //     // _Response is from AP

      //     store.add_connection(
      //       basic_frame
      //         .transmitter_address
      //         .expect("no ta on association"),
      //       basic_frame.receiver_address.expect("no ra on association"),
      //     );
      //   }
      ManagementSubtype::Disassociation | ManagementSubtype::Deauthentication => {
        // TODO broadcast? is it sent from router?
        // Disassociation is from STA
        // Deauthentication is from AP

        store.remove_connection(
          basic_frame
            .transmitter_address
            .expect("no ta on disassociation"),
          basic_frame
            .receiver_address
            .expect("no ra on disassociation"),
        );
      }

      _ => {
        // other ManagementSubtype
      }
    },

    _ => {
      // other FrameType
    }
  }

  // frames with special info that's sent
  match frame {
    Frame::Beacon(ref beacon_frame) => {
      store.change_kind(
        basic_frame
          .transmitter_address
          .expect("no transmitter_address on Beacon"),
        Kind::AccessPoint(beacon_frame.ssid.clone()),
      );
    }

    Frame::ProbeRequest(ref probe_request_frame) => {
      if !probe_request_frame.ssid.is_empty() {
        let t = probe_request_frame
          .management_frame
          .basic_frame
          .transmitter_address
          .expect("no trans on ProbeRequest");

        // (store.event_handler)(Event::ProbeRequest(t, probe_request_frame.ssid.clone()));
      }
    }

    Frame::ProbeResponse(ref probe_response_frame) => {
      store.change_kind(
        probe_response_frame
          .management_frame
          .basic_frame
          .transmitter_address
          .expect("no trans on ProbeResponse"),
        Kind::AccessPoint(probe_response_frame.ssid.clone()),
      );
    }

    _ => {}
  }
}
