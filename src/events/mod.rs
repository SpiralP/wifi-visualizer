pub mod store;
mod util;

pub use self::store::*;
pub use self::util::*;
use packet::ieee802_11::*;

pub fn handle_frame(frame: &IEEE802_11Frame, store: &mut Store) {
  let their_address = frame.receiver_address();
  store.add_address(their_address);

  let my_address;
  match frame.next_layer() {
    IEEE802_11FrameLayer::Management(management_frame) => {
      my_address = management_frame.transmitter_address();
    }
    IEEE802_11FrameLayer::Control(control_frame) => {
      my_address = control_frame.transmitter_address();
    }
    IEEE802_11FrameLayer::Data(data_frame) => {
      my_address = data_frame.transmitter_address();
    }
  }

  if let Some(transmitter_address) = my_address {
    store.add_address(transmitter_address);
  }

  // check for connections
  let mut is_associated = false;
  match frame.subtype() {
    FrameSubtype::Data(ref subtype) => {
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

    FrameSubtype::Management(ref subtype) => match subtype {
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
          their_address,
          my_address.expect("no my_address on disassociation"),
          ConnectionType::Disassociated,
        );
      }

      _ => {
        // other ManagementSubtype
      }
    },

    FrameSubtype::Control(_) => {
      // anyone in range will use these
    }
  }

  // if two nodes are communicating
  if let Some(my_address) = my_address {
    if is_associated {
      store.change_connection(my_address, their_address, ConnectionType::Associated);
    } else {
      // ra & ta have communicated!
      store.change_connection(my_address, their_address, ConnectionType::InRange);
    }
  }

  // frames with special info that's sent
  if let IEEE802_11FrameLayer::Management(ref management_frame) = frame.next_layer() {
    if let Some(management_frame_layer) = management_frame.next_layer() {
      match management_frame_layer {
        ManagementFrameLayer::Beacon(ref beacon_frame) => {
          if let Some(ssid) = beacon_frame.ssid() {
            store.change_kind(
              my_address.expect("no ta on Beacon"),
              Kind::AccessPoint(ssid),
            );
          }
        }

        ManagementFrameLayer::ProbeResponse(ref probe_response_frame) => {
          if let Some(ssid) = probe_response_frame.ssid() {
            store.change_kind(
              my_address.expect("no ta on ProbeResponse"),
              Kind::AccessPoint(ssid),
            );
          }
        }

        ManagementFrameLayer::ProbeRequest(ref probe_request_frame) => {
          if let Some(ssid) = probe_request_frame.ssid() {
            if !ssid.is_empty() {
              store.probe_request(my_address.expect("no ta on ProbeRequest"), ssid);
            }
          }
        }
      }
    }
  }

  store.check_for_inactive();
}
