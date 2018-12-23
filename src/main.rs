use error_chain::*;
use pcap;
use pcap::{Active, Capture, Device, Offline, Packet, PacketHeader};
use std::env;

error_chain! {}

fn get_interface(maybe_search: Option<String>) -> Result<Device> {
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

fn start_capture(interface_name: Option<String>) -> Capture<Offline> {
  // let dev = get_interface(interface_name).unwrap();

  // println!("listening on {}", dev.name);

  Capture::from_file(r"D:\wpa\school\c1-02 (2).cap").unwrap()
  // .promisc(true)
  // .open()
  // .unwrap()
  // .immediate_mode(true)
}

struct PacketWithHeader {
  header: PacketHeader,
  data: Vec<u8>,
}

enum Status<T> {
  Active(T),
  Finished,
}

fn main() {
  let mut cap = start_capture(env::args().nth(1));

  let (sender, receiver) = std::sync::mpsc::channel();

  let work_thread = std::thread::spawn(move || loop {
    let status: Status<PacketWithHeader> = receiver.recv().unwrap();
    match status {
      Status::Active(packet) => {
        println!("{:#?}", packet.header);
      }
      Status::Finished => {
        break;
      }
    }
  });

  loop {
    match cap.next() {
      Err(err) => match err {
        pcap::Error::NoMorePackets => break,
        _ => {
          panic!("{}", err);
        }
      },
      Ok(packet) => {
        sender
          .send(Status::Active(PacketWithHeader {
            header: *packet.header,
            data: packet.data.to_vec(),
          }))
          .unwrap();
      }
    }
  }

  sender.send(Status::Finished).unwrap();
  work_thread.join().unwrap()
}
