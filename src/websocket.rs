use crate::{
  events::{Event, Store},
  packet_capture::{self, get_capture, CaptureType},
  websocket,
};
use crossbeam_channel::{bounded, Receiver, Sender};
use futures::prelude::*;
use helpers::thread;
use log::error;
use std::sync::{Arc, Mutex};
use tokio::prelude::*;
use warp::{
  filters::ws::{Message, WebSocket},
  Future, Stream,
};

pub fn start(ws: WebSocket, capture_type: CaptureType) -> impl Future<Item = (), Error = ()> {
  future::lazy(move || {
    let event_receiver = start_capture(capture_type);

    stream::iter_ok(event_receiver.into_iter())
      .map(|event| Message::text(serde_json::to_string(&event).unwrap()))
      .forward(ws.sink_map_err(|err| {
        error!("websocket sink error: {}", err);
      }))
      .map(|_| ())
  })
}

fn start_capture(capture_type: CaptureType) -> Receiver<Event> {
  let mut store = Store::new();
  let event_receiver = store.get_receiver().unwrap();

  thread::spawn("packet_capture thread", move || {
    let mut sleep_playback = false;
    if let CaptureType::File(_) = capture_type {
      sleep_playback = true;
    }

    let capture = get_capture(capture_type).unwrap();

    packet_capture::start_blocking(capture, store, sleep_playback).unwrap();
  });

  event_receiver
}
