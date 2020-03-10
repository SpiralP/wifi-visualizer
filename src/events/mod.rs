pub mod store;
mod util;

pub use self::{store::*, util::*};
use crate::{
  error::{err_msg, Result},
  packet_capture::FrameWithRadiotap,
};
use ieee80211::{
  BeaconFixedParametersTrait, ControlFrameTrait, DataFrameTrait, FrameLayer, FrameSubtype,
  FrameTrait, MacAddress, ManagementFrameLayer, ManagementFrameTrait, ManagementSubtype,
  TaggedParametersTrait,
};
use log::warn;

pub fn handle_frame(
  store: &mut Store,
  frame_with_radiotap: &FrameWithRadiotap,
) -> Result<Vec<Event>> {
  let frame = &frame_with_radiotap.frame;

  let receiver_address = frame.receiver_address();
  if !is_broadcast(receiver_address) {
    store.add_address(receiver_address);
  }

  let layer = &frame
    .next_layer()
    .ok_or_else(|| err_msg("frame.next_layer"))?;

  let transmitter_address = match layer {
    FrameLayer::Management(management_frame) => management_frame.transmitter_address(),
    FrameLayer::Control(control_frame) => control_frame.transmitter_address(),
    FrameLayer::Data(data_frame) => data_frame.transmitter_address(),
  };

  if let Some(transmitter_address) = transmitter_address {
    store.add_address(transmitter_address);

    handle_transmitter(
      store,
      frame_with_radiotap,
      transmitter_address,
      receiver_address,
    )?;
  } else {
    match layer {
      FrameLayer::Control(_control_frame) => {}
      FrameLayer::Management(management_frame) => {
        warn!(
          "no transmitter_address on {:?} {:?}",
          frame.type_(),
          management_frame.subtype()
        );
      }
      FrameLayer::Data(data_frame) => {
        warn!(
          "no transmitter_address on {:?} {:?}",
          frame.type_(),
          data_frame.subtype()
        );
      }
    }
  }

  store.check_timers();

  Ok(store.flush_buffer())
}

fn handle_transmitter(
  store: &mut Store,
  frame_with_radiotap: &FrameWithRadiotap,
  transmitter_address: MacAddress,
  receiver_address: MacAddress,
) -> Result<()> {
  let frame = &frame_with_radiotap.frame;

  check_association(
    store,
    frame_with_radiotap,
    transmitter_address,
    receiver_address,
  )?;

  let layer = &frame
    .next_layer()
    .ok_or_else(|| err_msg("frame.next_layer"))?;

  // frames with special info that's sent
  if let FrameLayer::Management(ref management_frame) = layer {
    if let Some(management_frame_layer) = management_frame.next_layer() {
      match management_frame_layer {
        ManagementFrameLayer::Beacon(ref beacon_frame) => {
          let tagged_parameters = beacon_frame.tagged_parameters()?;

          store.access_point(
            transmitter_address,
            AccessPointInfo {
              ssid: tagged_parameters.ssid().ok_or_else(|| err_msg("ssid"))?,
              channel: tagged_parameters.channel(),
            },
          );

          store.update_beacon_quality(transmitter_address, beacon_frame.beacon_interval());
        }

        ManagementFrameLayer::ProbeResponse(ref probe_response_frame) => {
          let tagged_parameters = probe_response_frame.tagged_parameters()?;

          store.access_point(
            transmitter_address,
            AccessPointInfo {
              ssid: tagged_parameters.ssid().ok_or_else(|| err_msg("ssid"))?,
              channel: tagged_parameters.channel(),
            },
          );
        }

        ManagementFrameLayer::ProbeRequest(ref probe_request_frame) => {
          let tagged_parameters = probe_request_frame.tagged_parameters()?;

          let ssid = tagged_parameters.ssid().ok_or_else(|| err_msg("ssid"))?;
          if !ssid.is_empty() {
            store.probe_request(transmitter_address, ssid);
          }
        }

        _ => {}
      }
    }
  }

  store.update_rate(transmitter_address);

  if let Some(radiotap) = &frame_with_radiotap.radiotap {
    if let Some(signal) = &radiotap.antenna_signal {
      store.update_signal(transmitter_address, signal.value);
    }
  }

  // store.update_loss(transmitter_address, receiver_address, &layer);

  Ok(())
}

fn check_association(
  store: &mut Store,
  frame_with_radiotap: &FrameWithRadiotap,
  transmitter_address: MacAddress,
  receiver_address: MacAddress,
) -> Result<()> {
  let frame = &frame_with_radiotap.frame;

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

          store.change_connection(
            transmitter_address,
            receiver_address,
            ConnectionType::Authentication,
          );
        }

        ManagementSubtype::Disassociate | ManagementSubtype::Deauthentication => {
          // TODO broadcast? is it sent from router?
          // Disassociation is from STA
          // Deauthentication is from AP

          store.change_connection(
            transmitter_address,
            receiver_address,
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

  Ok(())
}
