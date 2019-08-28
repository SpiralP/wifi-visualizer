mod error;
mod events;
mod pcap_parser;
mod test_packets;

use self::{events::*, pcap_parser::*, test_packets::*};
use boxfnonce::BoxFnOnce;
use crossbeam_channel::{unbounded, Receiver};
use ieee80211::*;
use std::thread;
use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};

// Server WebSocket handler
struct Server {
  out: Sender,
  stop_sniff: Option<BoxFnOnce<'static, ()>>,
}

impl Handler for Server {
  fn on_open(&mut self, shake: Handshake) -> Result<()> {
    println!("ws on_open");

    let mut parts = shake.request.resource().split('/');
    parts.next(); // skip first /

    let root = parts.next().unwrap_or("");
    let rest = {
      let ag = parts.collect::<Vec<&str>>().join("/");
      if ag == "" {
        None
      } else {
        Some(ag)
      }
    };

    let mut is_file = false;
    let (receiver, stop_sniff): (Receiver<Status<PacketWithHeader>>, BoxFnOnce<'static, ()>) =
      match root {
        "test" => {
          let (sender, receiver) = unbounded();
          for data in &[&BEACON[..], &PROBE_RESPONSE_RETRY[..], &DATA_FROM_DS[..]] {
            sender
              .send(Status::Active(PacketWithHeader {
                header: unsafe { std::mem::zeroed() },
                data: data.to_vec(),
              }))
              .unwrap();
          }
          sender.send(Status::Finished).unwrap();

          (receiver, BoxFnOnce::from(|| {}))
        }
        "file" => {
          is_file = true;
          start_file_capture(rest.expect("no filename given")).unwrap()
        }
        "live" => start_live_capture(rest).unwrap(),
        "stdin" => start_stdin_capture().unwrap(),
        _ => {
          return self.out.close(ws::CloseCode::Error);
        }
      };

    self.stop_sniff = Some(stop_sniff);

    {
      let out = self.out.clone();
      std::thread::spawn(move || {
        let mut maybe_last_time: Option<std::time::Duration> = None;
        let mut store = {
          let out = out.clone();
          Store::new(Box::new(move |event| {
            // println!("{:?}", event);
            out.send(serde_json::to_string(&event).unwrap()).unwrap();
          }))
        };

        loop {
          let status = receiver.recv().unwrap();
          match status {
            Status::Active(packet) => {
              // println!("{:#?}", packet.header);

              if is_file {
                let current_time = std::time::Duration::new(
                  packet.header.ts.tv_sec as u64,
                  (packet.header.ts.tv_usec * 1000) as u32,
                );
                if let Some(last_time) = maybe_last_time {
                  if current_time > last_time {
                    std::thread::sleep(current_time - last_time);
                  }
                }
                maybe_last_time = Some(current_time);
              }

              let frame = Frame::new(&packet.data);
              handle_frame(&frame, &mut store);
            }
            Status::Finished => {
              break;
            }
          }
        }

        println!("work loop done");
        // if let Err(err) = out.close(ws::CloseCode::Normal) {
        //   println!("error closing: {}", err);
        // }
      });
    }

    Ok(()) // don't close yet
  }

  fn on_message(&mut self, msg: Message) -> Result<()> {
    // incoming message
    println!("incoming message {:?}", msg);
    Ok(())
  }

  fn on_close(&mut self, _code: CloseCode, _reason: &str) {
    println!("ws on_close");

    if let Some(stop_sniff) = self.stop_sniff.take() {
      stop_sniff.call();
    }
  }
}

const HTTP_SERVER_ADDR: &str = "127.0.0.1:8080";
const WEBSOCKET_SERVER_ADDR: &str = "127.0.0.1:3012";

fn main() {
  let http_server_thread = thread::spawn(move || {
    println!("starting http server on {}", HTTP_SERVER_ADDR);

    let server = tiny_http::Server::http(HTTP_SERVER_ADDR).unwrap();

    loop {
      // blocks until the next request is received
      match server.recv() {
        Ok(request) => {
          println!("{:#?}", request);

          let url = match request.url() {
            "/" => "index.html",
            other => &other[1..],
          };

          match parceljs::get_file(&url) {
            Ok(data) => {
              let mut response = tiny_http::Response::from_data(data);

              if url.ends_with(".css") {
                let header = tiny_http::Header::from_bytes(
                  &b"Content-Type"[..],
                  &b"text/css; charset=utf-8"[..],
                )
                .unwrap();
                response.add_header(header);
              } else if url.ends_with(".js") {
                let header = tiny_http::Header::from_bytes(
                  &b"Content-Type"[..],
                  &b"application/javascript; charset=utf-8"[..],
                )
                .unwrap();
                response.add_header(header);
              } else if url.ends_with(".html") {
                let header = tiny_http::Header::from_bytes(
                  &b"Content-Type"[..],
                  &b"text/html; charset=utf-8"[..],
                )
                .unwrap();
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
  });

  println!("starting websocket server on {}", WEBSOCKET_SERVER_ADDR);
  listen(WEBSOCKET_SERVER_ADDR, |out| Server {
    out,
    stop_sniff: None,
  })
  .unwrap();

  http_server_thread.join().unwrap();
}

#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn test_live_frame_parse() {
  let (receiver, _stop_sniff) = start_live_capture(None).unwrap();
  let status = receiver.recv().unwrap();
  if let Status::Active(packet) = status {
    let frame = Frame::new(&packet.data);
    println!("{:#?}", frame.receiver_address());
  } else {
    panic!("not Status::Active");
  }
}
