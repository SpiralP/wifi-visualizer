use crate::error::*;
use hyper::{
  service::{make_service_fn, service_fn},
  Body, Error, Response, Server,
};
use log::info;
use std::net::SocketAddr;

pub async fn start(addr: &SocketAddr) -> Result<()> {
  info!("starting http server on http://{}/", addr);

  // And a MakeService to handle each connection...
  let make_service = make_service_fn(move |_| {
    async move {
      Ok::<_, Error>(service_fn(move |request| {
        async move {
          let path = request.uri().path();

          if let Ok(data) = parceljs::get_file(path) {
            let mut response = Response::builder();

            if let Some(content_type) = parceljs::get_content_type(path) {
              response.header(&b"Content-Type"[..], content_type);
            }

            response.body(Body::from(data))
          } else {
            Response::builder().status(404).body(Body::empty())
          }
        }
      }))
    }
  });

  // Then bind and serve...
  let server = Server::bind(addr).serve(make_service);

  server.await?;

  Ok(())
}
