use crate::{packet_capture::CaptureType, websocket};
use futures::{executor::block_on, prelude::*};
use crate::helpers::thread;
use http::Response;
use hyper::Body;
use log::{debug, info};
use std::net::SocketAddr;
use warp::{path::FullPath, Filter, Reply};

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
          block_on(websocket::start(ws, capture_type)).expect("block_on");
        });

        future::ok(()).compat()
      })
    })
    .or(warp::path::full().map(|path: FullPath| {
      debug!("http {}", path.as_str());
      ParceljsResponder { path }
    }));

  futures::compat::Compat01As03::new(warp::serve(routes).bind(addr))
    .await
    .expect("warp::serve().bind()");
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

      response
        .body(Body::from(data))
        .expect("response.body(Body::from(data))")
    } else {
      Response::builder()
        .status(404)
        .body(Body::empty())
        .expect("Resposne::builder()..body(empty())")
    }
  }
}
