use crate::{
  error::*,
  events::{handle_frame, Event, Store},
  packet_capture::{self, get_capture_iterator, CaptureType},
};
use ieee80211::Frame;
use log::error;
use tokio::prelude::*;
use warp::{
  filters::ws::{Message, WebSocket},
  Future, Stream,
};

pub fn start(ws: WebSocket, capture_type: CaptureType) -> impl Future<Item = (), Error = ()> {
  let (ws_sender, _ws_receiver) = ws.split();

  // TODO somehow send errors through the websocket!
  start_capture_event_stream(capture_type)
    .and_then(move |event_stream| {
      event_stream
        .and_then(|event| serde_json::to_string(&event).map_err(Error::from))
        .map(Message::text)
        .forward(ws_sender)
        .map(|_| ())
    })
    .map_err(|e| {
      // ws.send(Message::text(String::from("HI")));
      error!("websocket connection: {}", e)
    })
}

fn start_capture_event_stream(
  capture_type: CaptureType,
) -> impl Future<Item = impl Stream<Item = Event, Error = Error>, Error = Error> {
  future::lazy(move || get_capture_iterator(capture_type)).and_then(|capture_iterator| {
    let mut store = Store::new();

    packet_capture::start(capture_iterator).map(move |packet_data_stream| {
      packet_data_stream
        .map(move |data| {
          let frame = Frame::new(&data);
          stream::iter_ok(handle_frame(&mut store, &frame))
        })
        .flatten()
    })
  })
}
