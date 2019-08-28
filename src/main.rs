mod error;
mod events;
mod http_server;
mod pcap_parser;
mod test_packets;
mod ws_server;

use std::thread;

const HTTP_SERVER_ADDR: &str = "127.0.0.1:8080";
const WEBSOCKET_SERVER_ADDR: &str = "127.0.0.1:3012";

fn main() {
  let http_server_thread = thread::spawn(move || {
    http_server::start(HTTP_SERVER_ADDR);
  });

  ws_server::start(WEBSOCKET_SERVER_ADDR);

  http_server_thread.join().unwrap();
}

#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn test_live_frame_parse() {
  use crate::pcap_parser::*;
  use ieee80211::*;

  let (receiver, _stop_sniff) = start_live_capture(None).unwrap();
  let status = receiver.recv().unwrap();
  if let Status::Active(packet) = status {
    let frame = Frame::new(&packet.data);
    println!("{:#?}", frame.receiver_address());
  } else {
    panic!("not Status::Active");
  }
}
