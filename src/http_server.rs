use log::{debug, error, info};
use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  time::Duration,
};
use tiny_http::{Header, Response, Server};

pub fn start_blocking(addr: &str, stop_notify: Arc<AtomicBool>) {
  info!("starting http server on http://{}/", addr);

  let server = Server::http(addr).unwrap();

  loop {
    if stop_notify.load(Ordering::SeqCst) {
      break;
    }

    // blocks until the next request is received
    match server.recv_timeout(Duration::from_millis(1000)) {
      Ok(None) => {
        // timeout hit
      }

      Ok(Some(request)) => {
        debug!("{:#?}", request);

        let url = request.url();

        match parceljs::get_file(&url) {
          Ok(data) => {
            let mut response = Response::from_data(data);

            if let Some(content_type) = parceljs::get_content_type(&url) {
              let header = Header::from_bytes(&b"Content-Type"[..], content_type).unwrap();
              response.add_header(header);
            }

            let _ = request.respond(response);
          }
          Err(_) => {
            let response = Response::empty(404);

            let _ = request.respond(response);
          }
        }
      }

      Err(e) => {
        error!("http server error: {}", e);
        break;
      }
    };
  }
}
