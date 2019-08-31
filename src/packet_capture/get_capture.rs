use crate::error::*;
use log::info;
use pcap::{Active, Capture, Device, Offline};
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;
use std::{io, path::Path};

pub fn get_interface(search: String) -> Result<Device> {
  for interface in Device::list()? {
    if interface.name == search {
      return Ok(interface);
    }
  }

  bail!("No interface found")
}

pub fn get_live_capture(dev: Device) -> Result<Capture<Active>> {
  info!("listening on {}", dev.name);

  Ok(
    Capture::from_device(dev)?
      .immediate_mode(true)
      .promisc(true)
      .timeout(1000)
      .open()?,
  )
}

pub fn get_file_capture<P: AsRef<Path>>(file_path: P) -> Result<Capture<Offline>> {
  Ok(Capture::from_file(file_path)?)
}

pub fn get_stdin_capture() -> Result<Capture<Offline>> {
  let capture = {
    #[cfg(windows)]
    {
      Capture::from_raw_handle(io::stdin().as_raw_handle())?
    }

    #[cfg(not(windows))]
    {
      Capture::from_raw_fd(io::stdin().as_raw_fd())?
    }
  };

  Ok(capture)
}

#[test]
fn test_file_capture() {
  let cap = get_file_capture("./caps/bap.cap".to_string()).unwrap();
  println!("{:#?}", cap.list_datalinks().unwrap());
}

#[test]
fn test_live_capture() {
  use pcap::Error as PcapError;

  #[cfg(windows)]
  let iface = r"\Device\NPF_{1207900A-6848-40D9-B1C2-860E3F27FE74}".to_string();

  #[cfg(not(windows))]
  let iface = "mon0".to_string();

  let mut cap = get_live_capture(get_interface(iface).unwrap()).unwrap();
  println!("{:#?}", cap.get_datalink());
  println!("{:#?}", cap.list_datalinks().unwrap());

  loop {
    match cap.next() {
      Err(ref err) => match err {
        PcapError::TimeoutExpired => {
          // this is called on windows at least!
        }
        _ => {
          panic!("{}", err);
        }
      },
      Ok(ref packet) => {
        println!("packet {:#?}", packet);
        break;
      }
    }
  }
}

// #[test]
// #[cfg_attr(target_os = "windows", ignore)]
// fn test_live_frame_parse() {
//   use ieee80211::*;

//   let (receiver, _stop_sniff) = start_live_capture(None).unwrap();
//   let status = receiver.recv().unwrap();
//   if let Status::Active(packet) = status {
//     let frame = Frame::new(&packet.data);
//     println!("{:#?}", frame.receiver_address());
//   } else {
//     panic!("not Status::Active");
//   }
// }

// #[test]
// fn test_get_interface() {
//   println!("{:#?}", get_interface(None).unwrap());
// }
