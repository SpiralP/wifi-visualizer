#![warn(clippy::pedantic)]

mod error;
mod events;
mod http_server;
mod logger;
mod packet_capture;
mod thread;
mod websocket;

use crate::{error::*, packet_capture::CaptureType};
use clap::{clap_app, crate_name, crate_version};
use log::debug;
use std::{
  net::{IpAddr, Ipv4Addr, SocketAddr},
  time::{Duration, Instant},
};

#[tokio::main]
async fn main() -> Result<()> {
  let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
  let http_server_addr = SocketAddr::new(ip, 8000);

  let matches = clap_app!(app =>
      (name: crate_name!())
      (version: crate_version!())

      (@arg debug: -v --verbose --debug ... "Show debug messages, multiple flags for higher verbosity")
      (@arg no_browser: -n --("no-browser") "Don't open browser")
      (@arg no_sleep_playback: requires[input_file] --("no-sleep-playback") "Don't play back files at original speed")

      (@arg input_file: conflicts_with[interface] -f --file [FILE] +required "File to read from")
      (@arg interface: conflicts_with[input_file] -i --interface [INTERFACE] +required "Interface to capture packets from")
  )
  .get_matches();

  #[cfg(debug_assertions)]
  logger::initialize(true, false);

  #[cfg(not(debug_assertions))]
  logger::initialize(
    matches.is_present("debug"),
    matches.occurrences_of("debug") > 1,
  );

  let capture_type = if let Some(input_file) = matches.value_of("input_file") {
    debug!("got input file {:?}", input_file);

    if input_file == "-" {
      CaptureType::Stdin
    } else {
      CaptureType::File(
        input_file.to_string(),
        !matches.is_present("no_sleep_playback"),
      )
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

  // TODO wait until packet capture begins successfully?
  let no_browser = matches.is_present("no_browser");

  if !no_browser {
    tokio::spawn(async move {
      tokio::timer::delay(Instant::now() + Duration::from_millis(100)).await;

      open::that(format!("http://{}/", http_server_addr)).unwrap();
    });
  }

  http_server::start(http_server_addr, capture_type).await;

  Ok(())
}
