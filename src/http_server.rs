use log::{debug, error, info};
use tiny_http::{Header, Response, Server};

pub fn start_blocking(addr: &str) {
  info!("starting http server on http://{}/", addr);

  let server = Server::http(addr).unwrap();

  loop {
    // blocks until the next request is received
    match server.recv() {
      Ok(request) => {
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
