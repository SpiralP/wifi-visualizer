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
  use caps::{CapSet, Capability};

  if !caps::has_cap(None, CapSet::Permitted, Capability::CAP_NET_RAW).unwrap() {
    println!("WARNING: CAP_NET_RAW not permitted! live packet capture won't work!");
    println!(
      "try running: sudo setcap cap_net_raw+ep {}",
      std::env::current_exe().unwrap().display()
    );
  }

  let http_server_thread = thread::spawn(move || {
    http_server::start(HTTP_SERVER_ADDR);
  });

  let ws_server_thread = thread::spawn(move || {
    ws_server::start(WEBSOCKET_SERVER_ADDR);
  });

  open::that(format!("http://{}/", HTTP_SERVER_ADDR)).unwrap();

  ws_server_thread.join().unwrap();
  http_server_thread.join().unwrap();
}
