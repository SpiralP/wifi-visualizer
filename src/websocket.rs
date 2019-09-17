use crate::{
  error::*,
  events::{handle_frame, Event, Store},
  inject_stream::InjectBeforeErrorTryStreamExt,
  packet_capture::{get_capture_stream, CaptureType},
};
use futures::prelude::*;
use log::{error, info};
use std::pin::Pin;
use warp::filters::ws::{Message, WebSocket};

pub async fn start(ws: WebSocket, capture_type: CaptureType) -> Result<()> {
  let ws = futures::compat::Compat01As03Sink::new(ws);
  let (ws_sender, _ws_receiver) = ws.split();

  let events_stream: Pin<Box<dyn Stream<Item = Result<Vec<Event>>>>> =
    match start_capture_event_stream(capture_type).await {
      Ok(events_stream) => events_stream.boxed(),
      Err(e) => stream::iter(vec![Err(e)]).boxed(),
    };

  InjectBeforeErrorTryStreamExt::inject_before_error(events_stream, |e| {
    vec![Event::Error(format!("{}", e))]
  })
  .map(|result| {
    let events = result?;
    let json = serde_json::to_string(&events).map_err(Error::from)?;
    Ok::<_, Error>(Message::text(json))
  })
  .map_err(|e| error!("websocket: {}", e))
  .forward(ws_sender.sink_map_err(|e| error!("websocket sink error: {}", e)))
  .map(|_| ())
  .inspect(|_| {
    info!("websocket closed");
  })
  .await;

  Ok(())
}

async fn start_capture_event_stream(
  capture_type: CaptureType,
) -> Result<impl Stream<Item = Result<Vec<Event>>>> {
  let capture_stream = get_capture_stream(capture_type).await?;
  let mut store = Store::new();

  Ok(
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
      .try_filter(|events| future::ready(!events.is_empty())),
  )
}
