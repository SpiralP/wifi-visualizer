use crate::{packet_capture::CaptureType, websocket};
use futures::prelude::*;
use helpers::thread;
use http::Response;
use hyper::Body;
use log::{debug, info};
use std::net::SocketAddr;
use tokio::prelude::*;
use warp::{path::FullPath, Filter, Future, Reply};

pub async fn start(addr: SocketAddr, capture_type: CaptureType) {
  info!("starting http/websocket server on http://{}/", addr);

  let routes = warp::path("ws")
    .and(warp::ws2())
    .map(move |ws: warp::ws::Ws2| {
      let capture_type = capture_type.clone();
      ws.on_upgrade(move |ws| {
        // we don't want to use tokio here because iterator streams
        // block the other http request futures by taking from the pool
        thread::spawn("websocket future thread", move || {
          let _ = websocket::start(ws, capture_type).wait();
        });

        future::ok(())
      })
    })
    .or(warp::path::full().map(|path: FullPath| {
      debug!("http {}", path.as_str());
      ParceljsResponder { path }
    }));

  warp::serve(routes).bind(addr)
}

struct ParceljsResponder {
  path: FullPath,
}

impl Reply for ParceljsResponder {
  fn into_response(self) -> Response<Body> {
    let path = self.path.as_str();

    if let Ok(data) = parceljs::get_file(path) {
      let mut response = Response::builder();

      if let Some(content_type) = parceljs::get_content_type(path) {
        response.header(&b"Content-Type"[..], content_type);
      }

      response.body(Body::from(data)).unwrap()
    } else {
      Response::builder().status(404).body(Body::empty()).unwrap()
    }
  }
}
