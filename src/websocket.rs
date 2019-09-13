use crate::{
  error::*,
  events::{handle_frame, Event, Store},
  inject_stream::InjectBeforeErrorStreamExt,
  packet_capture::{get_capture_stream, CaptureType},
};
use log::{error, info};
use tokio::prelude::*;
use warp::filters::ws::{Message, WebSocket};

pub fn start(ws: WebSocket, capture_type: CaptureType) -> impl Future<Item = (), Error = ()> {
  let (ws_sender, _ws_receiver) = ws.split();

  start_capture_event_stream(capture_type)
    .then(|result| {
      let events_stream: Box<dyn Stream<Item = _, Error = _>> = match result {
        Ok(events_stream) => Box::new(events_stream),
        Err(e) => Box::new(stream::iter_result(vec![Err(e)])),
      };

      Ok(events_stream)
    })
    .and_then(move |events_stream| {
      events_stream
        .inject_before_error(|e| vec![Event::Error(format!("{}", e))])
        .and_then(|events| serde_json::to_string(&events).map_err(Error::from))
        .map(Message::text)
        .map_err(|e| error!("websocket: {}", e))
        .forward(ws_sender.sink_map_err(|e| error!("websocket sink error: {}", e)))
        .map(|_| ())
    })
    .inspect(|_| {
      info!("websocket closed");
    })
}

fn start_capture_event_stream(
  capture_type: CaptureType,
) -> impl Future<Item = impl Stream<Item = Vec<Event>, Error = Error>, Error = Error> {
  future::lazy(move || get_capture_stream(capture_type)).map(|capture_stream| {
    let mut store = Store::new();

    capture_stream
      .and_then(move |frame_with_radiotap| handle_frame(&mut store, &frame_with_radiotap))
      .filter(|events| !events.is_empty())
  })
}
