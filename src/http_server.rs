use crate::error::*;
use helpers::{check_notified_return, notify::Notify};
use log::{debug, info};
use std::time::Duration;
use tiny_http::{Header, Response, Server};

pub fn start_blocking(addr: &str, stop_notify: &Notify) -> Result<()> {
  info!("starting http server on http://{}/", addr);

  let server = Server::http(addr).map_err(Error::from_boxed_compat)?;

  loop {
    check_notified_return!(stop_notify, Ok(()));

    // blocks until the next request is received
    if let Some(request) = server.recv_timeout(Duration::from_millis(1000))? {
      debug!("{:#?}", request);

      let url = request.url();

      if let Ok(data) = parceljs::get_file(&url) {
        let mut response = Response::from_data(data);

        if let Some(content_type) = parceljs::get_content_type(&url) {
          let header = Header::from_bytes(&b"Content-Type"[..], content_type)
            .map_err(|_| err_msg("couldn't convert content-type"))?;
          response.add_header(header);
        }

        let _ = request.respond(response);
      } else {
        let response = Response::empty(404);

        let _ = request.respond(response);
      }
    }
  }
}
