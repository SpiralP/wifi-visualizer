use crate::{
  error::*,
  events::{handle_frame, Event, Store},
  packet_capture::{get_capture_stream, CaptureType},
};
use futures::prelude::*;
use log::{error, info};
use warp::filters::ws::{Message, WebSocket};

pub async fn start(ws: WebSocket, capture_type: CaptureType) -> Result<()> {
  let (ws_sender, _ws_receiver) = ws.split();

  let events_stream: Box<dyn Stream<Item = _>> =
    match start_capture_event_stream(capture_type).await {
      Ok(events_stream) => Box::new(events_stream),
      Err(e) => Box::new(stream::iter(vec![Err(e)])),
    };

  // .inject_before_error(|e| vec![Event::Error(format!("{}", e))])

  events_stream
    .map(|result| {
      let events = result?;
      let json = serde_json::to_string(&events).map_err(Error::from)?;
      Ok(Message::text(json))
    })
    .map_err(|e| error!("websocket: {}", e))
    .forward(ws_sender.sink_map_err(|e| error!("websocket sink error: {}", e)))
    .map(|_| ())
    .inspect(|_| {
      info!("websocket closed");
    })
}

async fn start_capture_event_stream(
  capture_type: CaptureType,
) -> Result<impl Stream<Item = Result<Vec<Event>>>> {
  let capture_stream = get_capture_stream(capture_type).await?;
  let mut store = Store::new();

  capture_stream
    .map(move |result| {
      let frame_with_radiotap = result?;
      Ok(handle_frame(&mut store, &frame_with_radiotap)?)
    })
    .map(|result| {
      if let Err(e) = result {
        error!("packet parse error: {:?}", e);
        Ok(vec![])
      } else {
        result
      }
    })
    .filter(|result| Ok(result.map(|events| !events.is_empty()).unwrap_or(false)))
}
