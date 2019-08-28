pub fn start(addr: &str) {
  println!("starting http server on {}", addr);

  let server = tiny_http::Server::http(addr).unwrap();

  loop {
    // blocks until the next request is received
    match server.recv() {
      Ok(request) => {
        println!("{:#?}", request);

        let url = request.url();

        match parceljs::get_file(&url) {
          Ok(data) => {
            let mut response = tiny_http::Response::from_data(data);

            if let Some(content_type) = parceljs::get_content_type(&url) {
              let header =
                tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type).unwrap();
              response.add_header(header);
            }

            let _ = request.respond(response);
          }
          Err(e) => {
            println!("{}", e);
          }
        }
      }
      Err(e) => {
        println!("error: {}", e);
        break;
      }
    };
  }
}
