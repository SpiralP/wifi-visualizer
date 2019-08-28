use crate::error::*;
use boxfnonce::BoxFnOnce;
use crossbeam_channel::*;
use parking_lot::Mutex;
use pcap::{linktypes, Active, Capture, Device, Error as PcapError, Offline, PacketHeader};
use radiotap::Radiotap;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;
use std::{io, sync::Arc};

pub fn get_interface(maybe_search: Option<String>) -> Result<Device> {
  for interface in Device::list().unwrap() {
    match maybe_search {
      None => {
        return Ok(interface);
      }
      Some(ref search) => {
        if interface.name == *search {
          return Ok(interface);
        }
      }
    }
  }

  Err("No interface found".into())
}

#[test]
fn test_get_interface() {
  println!("{:#?}", get_interface(None).unwrap());
}

pub struct PacketWithHeader {
  pub header: PacketHeader,
  pub data: Vec<u8>,
}

pub enum Status<T> {
  Active(T),
  Finished,
}

fn get_live_capture(dev: Device) -> Result<Capture<Active>> {
  println!("listening on {}", dev.name);

  Ok(
    Capture::from_device(dev)?
      .immediate_mode(true)
      .promisc(true)
      .timeout(1000)
      .open()?,
  )
}

#[test]
fn test_live_capture() {
  #[cfg(windows)]
  let iface = Some(r"\Device\NPF_{1207900A-6848-40D9-B1C2-860E3F27FE74}".to_string());

  #[cfg(not(windows))]
  let iface: Option<String> = None;

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

pub fn start_live_capture(
  interface_name: Option<String>,
) -> Result<(Receiver<Status<PacketWithHeader>>, BoxFnOnce<'static, ()>)> {
  let dev = get_interface(interface_name)?;

  let cap = get_live_capture(dev)?;

  Ok(start_capture(cap)?)
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

fn get_file_capture(file_path: String) -> Result<Capture<Offline>> {
  Ok(Capture::from_file(file_path)?)
}

#[test]
fn test_file_capture() {
  let cap = get_file_capture("./caps/bap.cap".to_string()).unwrap();
  println!("{:#?}", cap.list_datalinks().unwrap());
}

pub fn start_file_capture(
  file_path: String,
) -> Result<(Receiver<Status<PacketWithHeader>>, BoxFnOnce<'static, ()>)> {
  let cap = get_file_capture(file_path)?;

  Ok(start_capture(cap)?)
}

pub fn start_stdin_capture() -> Result<(Receiver<Status<PacketWithHeader>>, BoxFnOnce<'static, ()>)>
{
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

  Ok(start_capture(capture)?)
}

fn strip_radiotap(bytes: &[u8]) -> &[u8] {
  let (radiotap, rest) = Radiotap::parse(bytes).unwrap();
  // println!("{:#?}", element);

  let has_fcs = radiotap.flags.map(|flags| flags.fcs).unwrap_or(false);

  if has_fcs {
    // remove last 4 bytes (uint32_t)
    let (data, _fcs) = rest.split_at(rest.len() - 4);
    data
  } else {
    rest
  }
}

#[test]
fn test_strip_radiotap() {
  let frame_with_fcs = [
    0x00, 0x00, 0x38, 0x00, 0x2f, 0x40, 0x40, 0xa0, 0x20, 0x08, 0x00, 0xa0, 0x20, 0x08, 0x00, 0x00,
    0xbd, 0x20, 0x83, 0x26, 0x00, 0x00, 0x00, 0x00, 0x10, 0x0c, 0x6c, 0x09, 0xc0, 0x00, 0xd5, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x1f, 0x83, 0x26, 0x00, 0x00, 0x00, 0x00,
    0x16, 0x00, 0x11, 0x03, 0xd5, 0x00, 0xce, 0x01, 0x80, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xb0, 0xee, 0x7b, 0x98, 0x3a, 0x79, 0xb0, 0xee, 0x7b, 0x98, 0x3a, 0x79, 0x60, 0x2d,
    0x3a, 0x80, 0xbe, 0x13, 0x35, 0x00, 0x00, 0x00, 0x64, 0x00, 0x11, 0x05, 0x00, 0x16, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x01, 0x08, 0x8c, 0x12, 0x98, 0x24, 0xb0, 0x48, 0x60, 0x6c, 0x03, 0x01,
    0x01, 0x05, 0x04, 0x01, 0x03, 0x00, 0x00, 0x07, 0x06, 0x55, 0x53, 0x20, 0x01, 0x0b, 0x1e, 0x20,
    0x01, 0x00, 0x23, 0x02, 0x12, 0x00, 0x2a, 0x01, 0x00, 0x2f, 0x01, 0x00, 0x30, 0x14, 0x01, 0x00,
    0x00, 0x0f, 0xac, 0x04, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x04, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x02,
    0x08, 0x00, 0x2d, 0x1a, 0xbc, 0x19, 0x16, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3d, 0x16,
    0x01, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x08, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
    0xdd, 0x09, 0x00, 0x10, 0x18, 0x02, 0x00, 0x00, 0x4c, 0x00, 0x00, 0xdd, 0x18, 0x00, 0x50, 0xf2,
    0x02, 0x01, 0x01, 0x80, 0x00, 0x03, 0xa4, 0x00, 0x00, 0x27, 0xa4, 0x00, 0x00, 0x42, 0x43, 0x5e,
    0x00, 0x62, 0x32, 0x2f, 0x00, 0xdd, 0x05, 0x00, 0x50, 0xf2, 0x05, 0x00, 0xdd, 0x52, 0x00, 0x50,
    0xf2, 0x04, 0x10, 0x4a, 0x00, 0x01, 0x10, 0x10, 0x44, 0x00, 0x01, 0x02, 0x10, 0x47, 0x00, 0x10,
    0x22, 0x21, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0xb0, 0xee, 0x7b, 0x98, 0x3a, 0x79,
    0x10, 0x49, 0x00, 0x06, 0x00, 0x37, 0x2a, 0x00, 0x01, 0x20, 0x10, 0x54, 0x00, 0x08, 0x00, 0x06,
    0x00, 0x50, 0xf2, 0x04, 0x00, 0x01, 0x10, 0x11, 0x00, 0x16, 0x44, 0x49, 0x52, 0x45, 0x43, 0x54,
    0x2d, 0x72, 0x6f, 0x6b, 0x75, 0x2d, 0x39, 0x35, 0x30, 0x2d, 0x41, 0x36, 0x39, 0x42, 0x33, 0x35,
    0xdd, 0x12, 0x50, 0x6f, 0x9a, 0x09, 0x02, 0x02, 0x00, 0x37, 0x2b, 0x03, 0x06, 0x00, 0xb2, 0xee,
    0x7b, 0x98, 0x3a, 0x77, 0xdd, 0x0d, 0x50, 0x6f, 0x9a, 0x0a, 0x00, 0x00, 0x06, 0x01, 0x11, 0x1c,
    0x44, 0x00, 0x96, 0x66, 0xaf, 0x6f, 0x76,
  ];

  let frame = strip_radiotap(&frame_with_fcs);
  assert_eq!(
    frame[0..4],
    [0x80, 0x00, 0x00, 0x00],
    "radiotap header not removed correctly"
  );

  assert_eq!(frame[frame.len() - 1], 0x96, "fcs at end not removed");
}

fn start_capture<T: ::pcap::Activated + Send + 'static>(
  mut cap: Capture<T>,
) -> Result<(Receiver<Status<PacketWithHeader>>, BoxFnOnce<'static, ()>)> {
  #[allow(clippy::mutex_atomic)]
  let stop = Arc::new(Mutex::new(false));

  let (sender, receiver) = unbounded();

  let datalink = cap.get_datalink();
  let is_radiotap = match datalink.0 {
    linktypes::IEEE802_11 => false,
    linktypes::IEEE802_11_RADIOTAP => true,
    _ => {
      bail!(
        "bad datalink type {}",
        datalink
          .get_name()
          .unwrap_or_else(|_| format!("(couldn't get_name for {})", datalink.0).to_string())
      );
    }
  };

  let sniff_thread = {
    let stop = stop.clone();
    std::thread::spawn(move || {
      loop {
        if *stop.lock() {
          break;
        }
        match cap.next() {
          Err(ref err) => match err {
            PcapError::NoMorePackets => break,
            PcapError::TimeoutExpired => {
              // this is called on windows at least!
            }
            _ => {
              panic!("{}", err);
            }
          },
          Ok(ref packet) => {
            // TODO move out of sniff thread
            let data = if is_radiotap {
              strip_radiotap(packet.data)
            } else {
              packet.data
            };

            sender
              .send(Status::Active(PacketWithHeader {
                header: *packet.header,
                data: data.to_vec(),
              }))
              .unwrap();
          }
        }

        // let stats = cap.stats();
        // println!("{:#?}", stats);
      }

      println!("sniff loop done");

      sender.send(Status::Finished).unwrap();
    })
  };

  let stop_thread = BoxFnOnce::from(move || {
    {
      let mut stop = stop.lock();
      *stop = true;
    }
    sniff_thread.join().unwrap();
  });

  Ok((receiver, stop_thread))
}
