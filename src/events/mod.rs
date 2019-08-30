pub mod store;
mod util;

pub use self::{store::*, util::*};
use ieee80211::*;

pub fn handle_frame(store: &mut Store, frame: &Frame) {
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
          let tagged_parameters = beacon_frame.tagged_parameters();

          store.access_point(
            transmitter_address.expect("no ta on Beacon"),
            AccessPointInfo {
              ssid: tagged_parameters.ssid().unwrap(),
              channel: tagged_parameters.channel(),
            },
          );
        }

        ManagementFrameLayer::ProbeResponse(ref probe_response_frame) => {
          let tagged_parameters = probe_response_frame.tagged_parameters();

          store.access_point(
            transmitter_address.expect("no ta on ProbeResponse"),
            AccessPointInfo {
              ssid: tagged_parameters.ssid().unwrap(),
              channel: tagged_parameters.channel(),
            },
          );
        }

        ManagementFrameLayer::ProbeRequest(ref probe_request_frame) => {
          let ssid = probe_request_frame.ssid().unwrap();
          if !ssid.is_empty() {
            store.probe_request(transmitter_address.expect("no ta on ProbeRequest"), ssid);
          }
        }

        ManagementFrameLayer::Authentication(ref authentication_frame) => {
          println!("Authentication");
        }

        _ => {}
      }
    }
  }

  store.check_for_inactive();
}
