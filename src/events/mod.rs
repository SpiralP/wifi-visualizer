pub mod store;

pub use self::store::*;
use crate::ieee802_11::frame_control::*;
use crate::ieee802_11::*;

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

  // TODO check for inactive nodes
  store.check_for_inactive();
}
