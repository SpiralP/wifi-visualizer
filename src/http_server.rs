use crate::{packet_capture::CaptureType, thread, websocket};
use futures::executor::block_on;
use log::{debug, info};
use parceljs::warp::ParceljsResponder;
use std::net::SocketAddr;
use warp::{path::FullPath, Filter};

pub async fn start(addr: SocketAddr, capture_type: CaptureType) {
  info!("starting http/websocket server on http://{}/", addr);

  let routes = warp::path("ws")
    .and(warp::ws2())
    .map(move |ws: warp::ws::Ws2| {
      let capture_type = capture_type.clone();
      ws.on_upgrade(move |ws| {
        async {
          // we don't want to use tokio here because iterator streams
          // block the other http request futures by taking from the pool
          thread::spawn("websocket future thread", move || {
            block_on(websocket::start(ws, capture_type)).expect("block_on");
          });
        }
      })
    })
    .or(warp::path::full().map(|path: FullPath| {
      debug!("http {}", path.as_str());
      ParceljsResponder::new(path)
    }));

  warp::serve(routes).bind(addr).await;
}
