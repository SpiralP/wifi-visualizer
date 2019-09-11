use crate::{
  error::*,
  events::{handle_frame, Event, Store},
  inject_stream::InjectBeforeErrorStreamExt,
  packet_capture::{get_capture_iterator, CaptureType},
};
use ieee80211::Frame;
use log::{error, info};
use tokio::prelude::*;
use warp::{
  filters::ws::{Message, WebSocket},
  Future, Stream,
};

pub fn start(ws: WebSocket, capture_type: CaptureType) -> impl Future<Item = (), Error = ()> {
  let (ws_sender, _ws_receiver) = ws.split();

  start_capture_event_stream(capture_type)
    .then(move |result| {
      // check for pre-stream errors
      // TODO don't do it this way
      let ag: Box<dyn Stream<Item = Vec<Event>, Error = Error> + Send> = match result {
        Ok(events_stream) => Box::new(events_stream),
        Err(e) => {
          error!("{}", e);
          Box::new(stream::once(Ok(vec![Event::Error(format!("{}", e))])))
        }
      };

      ag.and_then(|event| serde_json::to_string(&event).map_err(Error::from))
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
  future::lazy(move || get_capture_iterator(capture_type)).map(|capture_iterator| {
    let mut store = Store::new();

    stream::iter_result(capture_iterator)
      .and_then(move |data| {
        let frame = Frame::new(&data);
        handle_frame(&mut store, &frame)
      })
      .filter(|events| !events.is_empty())
      .inject_before_error(|e| vec![Event::Error(format!("{}", e))])
  })
}
