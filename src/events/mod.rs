pub mod store;
mod util;

pub use self::store::*;
pub use self::util::*;
use ieee80211::*;

pub fn handle_frame(frame: &Frame, store: &mut Store) {
  let receiver_address = frame.receiver_address();
  store.add_address(receiver_address);

  let layer = frame.next_layer().unwrap();

  let transmitter_address;
  match layer {
    FrameLayer::Management(ref management_frame) => {
      transmitter_address = management_frame.transmitter_address();
    }
    FrameLayer::Control(ref control_frame) => {
      transmitter_address = control_frame.transmitter_address();
    }
    FrameLayer::Data(ref data_frame) => {
      transmitter_address = data_frame.transmitter_address();
    }
  }

  if let Some(transmitter_address) = transmitter_address {
    store.add_address(transmitter_address);
  }

  // check for connections
  let mut is_associated = false;
  match frame.subtype() {
    FrameSubtype::Data(ref _subtype) => {
      is_associated = true;
    }

    FrameSubtype::Management(ref subtype) => {
      match subtype {
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

        ManagementSubtype::Disassociate | ManagementSubtype::Deauthentication => {
          // TODO broadcast? is it sent from router?
          // Disassociation is from STA
          // Deauthentication is from AP

          store.change_connection(
            receiver_address,
            transmitter_address.expect("no transmitter_address on disassociation"),
            ConnectionType::Disassociated,
          );
        }

        _ => {
          // other ManagementSubtype
        }
      }
    }
    _ => {}
  }

  // if two nodes are communicating
  if let Some(transmitter_address) = transmitter_address {
    if is_associated {
      store.change_connection(
        receiver_address,
        transmitter_address,
        ConnectionType::Associated,
      );
    } else {
      // ra & ta have communicated!
      store.change_connection(
        receiver_address,
        transmitter_address,
        ConnectionType::InRange,
      );
    }
  }

  // frames with special info that's sent
  if let FrameLayer::Management(ref management_frame) = layer {
    if let Some(management_frame_layer) = management_frame.next_layer() {
      match management_frame_layer {
        ManagementFrameLayer::Beacon(ref beacon_frame) => {
          if let Some(ssid) = beacon_frame.ssid() {
            store.change_kind(
              transmitter_address.expect("no ta on Beacon"),
              Kind::AccessPoint(ssid),
            );
          }
        }

        ManagementFrameLayer::ProbeResponse(ref probe_response_frame) => {
          if let Some(ssid) = probe_response_frame.ssid() {
            store.change_kind(
              transmitter_address.expect("no ta on ProbeResponse"),
              Kind::AccessPoint(ssid),
            );
          }
        }

        ManagementFrameLayer::ProbeRequest(ref probe_request_frame) => {
          if let Some(ssid) = probe_request_frame.ssid() {
            if !ssid.is_empty() {
              store.probe_request(transmitter_address.expect("no ta on ProbeRequest"), ssid);
            }
          }
        }
      }
    }
  }

  store.check_for_inactive();
}
