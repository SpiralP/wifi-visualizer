use boxfnonce::BoxFnOnce;
use ieee80211::*;
use std::sync::mpsc::Receiver;
use wifi_visualizer::events::*;
use wifi_visualizer::pcap_parser::{
  start_file_capture, start_live_capture, PacketWithHeader, Status,
};
use wifi_visualizer::test_packets::*;
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
          let (sender, receiver) = std::sync::mpsc::channel();
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
        _ => {
          return self.out.close(ws::CloseCode::Error);
        }
      };

    self.stop_sniff = Some(stop_sniff);

    {
      let out = self.out.clone();
      std::thread::spawn(move || {
        let mut maybe_last_time = None;
        let mut store = {
          let out = out.clone();
          Store::new(Box::new(move |event| {
            println!("{:?}", event);
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
    if let Some(stop_sniff) = self.stop_sniff.take() {
      stop_sniff.call();
    }
  }
}

fn main() {
  println!("websocket starting");
  if let Err(err) = listen("127.0.0.1:3012", |out| Server {
    out,
    stop_sniff: None,
  }) {
    println!("listen error: {}", err);
  }
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
