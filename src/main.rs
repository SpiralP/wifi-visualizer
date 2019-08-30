mod error;
mod events;
mod http_server;
mod logger;
mod packet_capture;
mod ws_server;

use crate::{
  error::*,
  events::Store,
  packet_capture::{get_capture, CaptureType},
};
use clap::{clap_app, crate_name, crate_version};
use log::debug;
use std::thread;

const HTTP_SERVER_ADDR: &str = "127.0.0.1:8000";
const WEBSOCKET_SERVER_ADDR: &str = "127.0.0.1:8001";

fn main() -> Result<()> {
  let matches = clap_app!(app =>
      (name: crate_name!())
      (version: crate_version!())

      (@arg debug: -v --verbose --debug "Show debug messages")
      (@arg no_browser: -n --("no-browser") "Don't open browser")

      (@arg input_file: conflicts_with[interface] -f --file [FILE] +required "File to read from")
      (@arg interface: conflicts_with[input_file] -i --interface [INTERFACE] +required "Interface to capture packets from")
  )
  .get_matches();

  #[cfg(debug_assertions)]
  logger::initialize(true);

  #[cfg(not(debug_assertions))]
  logger::initialize(matches.is_present("debug"));

  let mut sleep_playback = false;
  let capture_type = if let Some(input_file) = matches.value_of("input_file") {
    debug!("got input file {:?}", input_file);

    if input_file == "-" {
      CaptureType::Stdin
    } else {
      sleep_playback = true;
      CaptureType::File(input_file.to_string())
    }
  } else if let Some(interface_name) = matches.value_of("interface") {
    debug!("got interface name {:?}", interface_name);

    #[cfg(target_os = "linux")]
    {
      use caps::{CapSet, Capability};
      use log::warn;
      use std::env;

      if !caps::has_cap(None, CapSet::Permitted, Capability::CAP_NET_RAW).unwrap() {
        warn!("WARNING: CAP_NET_RAW not permitted! live packet capture won't work!");
        warn!(
          "try running: sudo setcap cap_net_raw+ep {}",
          env::current_exe()?.display()
        );
      }
    }

    CaptureType::Interface(interface_name.to_string())
  } else {
    unreachable!()
  };

  let mut store = Store::new();
  let event_receiver = store.get_receiver().unwrap();

  let ws_server_thread = thread::spawn(move || {
    ws_server::start_blocking(WEBSOCKET_SERVER_ADDR, event_receiver).unwrap();
  });

  let http_server_thread = thread::spawn(move || {
    http_server::start_blocking(HTTP_SERVER_ADDR);
  });

  let packet_capture_thread = thread::spawn(move || {
    let capture = get_capture(capture_type).unwrap();

    packet_capture::start_blocking(capture, store, sleep_playback).unwrap();
  });

  // TODO wait until packet capture begins successfully
  let no_browser = matches.is_present("no_browser");

  if !no_browser {
    use std::time;

    thread::sleep(time::Duration::from_millis(100));
    open::that(format!("http://{}/", HTTP_SERVER_ADDR))?;
  }

  ws_server_thread.join().unwrap();
  http_server_thread.join().unwrap();
  packet_capture_thread.join().unwrap();

  Ok(())
}
