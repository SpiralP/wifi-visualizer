#![warn(clippy::pedantic)]
#![allow(clippy::needless_return)]

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
use helpers::{check_err_return, check_notified_return, notify::Notify, thread};
use log::{debug, error};
use std::{
  net::{IpAddr, Ipv4Addr, SocketAddr},
  time::Duration,
};

#[tokio::main]
async fn main() -> Result<()> {
  let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
  let http_server_addr = SocketAddr::new(ip, 8000);
  let websocket_server_addr = SocketAddr::new(ip, 8001);

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

  let stop_notify = Notify::new();

  let ws_server_thread = {
    let mut stop_notify = stop_notify.clone();

    thread::spawn("ws_server_thread", move || {
      check_err_return!(
        ws_server::start_blocking(&websocket_server_addr, event_receiver, &mut stop_notify),
        stop_notify
      );
    })
  };

  tokio::spawn(async move {
    http_server::start(&http_server_addr).await.unwrap(); // TODO just error!()
  });

  let packet_capture_thread = {
    let mut stop_notify = stop_notify.clone();

    thread::spawn("packet_capture_thread", move || {
      let capture = check_err_return!(get_capture(capture_type), stop_notify);
      check_err_return!(
        packet_capture::start_blocking(capture, store, sleep_playback, &stop_notify),
        stop_notify
      );
    })
  };

  // TODO wait until packet capture begins successfully?
  let no_browser = matches.is_present("no_browser");

  if !no_browser {
    thread::spawn("open http thread", move || {
      thread::sleep(Duration::from_millis(100));

      check_notified_return!(stop_notify);

      open::that(format!("http://{}/", http_server_addr)).unwrap();
    });
  }

  ws_server_thread.join().unwrap();
  packet_capture_thread.join().unwrap();

  Ok(())
}
