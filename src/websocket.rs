use crate::{
  error::*,
  events::{handle_frame, Event, Store},
  packet_capture::{get_capture_stream, CaptureType},
};
use futures::prelude::*;
use log::{error, info};
use std::pin::Pin;
use warp::filters::ws::{Message, WebSocket};

pub async fn start(ws: WebSocket, capture_type: CaptureType) -> Result<()> {
  let (mut ws_sender, _ws_receiver) = ws.split();

  let mut events_stream: Pin<Box<dyn Stream<Item = Result<Vec<Event>>>>> =
    match start_capture_event_stream(capture_type).await {
      Ok(events_stream) => events_stream.boxed(),
      Err(e) => stream::iter(vec![Err(e)]).boxed(),
    };

  while let Some(result) = events_stream.next().await {
    let was_error = result.is_err();

    let message = result
      .or_else(|err| Ok(vec![Event::Error(format!("{}", err))]))
      .and_then(|events| serde_json::to_string(&events).map_err(Error::from))
      .map(|json| Message::text(json))?;

    if let Err(err) = ws_sender.send(message).await {
      error!("websocket sink error: {}", err);
      return Ok(());
    }

    if was_error {
      break;
    }
  }

  ws_sender.close().await?;
  info!("websocket closed");

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
