#![warn(clippy::pedantic)]

mod error;
mod events;
mod http_server;
mod logger;
mod packet_capture;
mod thread;
mod websocket;

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use clap::{ArgAction, Parser};
use log::debug;

use crate::{error::Result, packet_capture::CaptureType};

/// wifi-visualizer
#[derive(Debug, Parser)]
#[command(author, version)]
pub struct Args {
    /// Show debug messages, multiple flags for higher verbosity
    #[clap(short, long, action(ArgAction::Count))]
    pub verbose: u8,

    /// Don't open browser
    #[arg(short, long)]
    pub no_browser: bool,

    /// Don't play back files at original speed
    #[arg(long, requires("file"))]
    pub no_sleep_playback: bool,

    /// File to read from
    #[arg(short, long, required(true), conflicts_with("interface"))]
    pub file: Option<String>,

    /// Interface to capture packets from
    #[arg(short, long, required(true), conflicts_with("file"))]
    pub interface: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let http_server_addr = SocketAddr::new(ip, 8000);

    let args = Args::parse();

    #[cfg(debug_assertions)]
    logger::initialize(true, false);

    #[cfg(not(debug_assertions))]
    logger::initialize(args.verbose >= 1, args.verbose >= 2);

    let capture_type = if let Some(file) = args.file {
        debug!("got input file {:?}", file);

        if file == "-" {
            CaptureType::Stdin
        } else {
            CaptureType::File(file, !args.no_sleep_playback)
        }
    } else if let Some(interface_name) = args.interface {
        debug!("got interface name {:?}", interface_name);

        #[cfg(target_os = "linux")]
        {
            use std::env;

            use caps::{CapSet, Capability};
            use log::warn;

            if !caps::has_cap(None, CapSet::Permitted, Capability::CAP_NET_RAW).unwrap() {
                warn!("WARNING: CAP_NET_RAW not permitted! live packet capture won't work!");
                warn!(
                    "try running: sudo setcap cap_net_raw+ep {}",
                    env::current_exe()?.display()
                );
            }
        }

        CaptureType::Interface(interface_name)
    } else {
        unreachable!()
    };

    // TODO wait until packet capture begins successfully?
    if !args.no_browser {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;

            open::that(format!("http://{}/", http_server_addr)).unwrap();
        });
    }

    http_server::start(http_server_addr, capture_type).await;

    Ok(())
}
