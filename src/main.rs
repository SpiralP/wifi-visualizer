use error_chain::*;
use packet;
use packet::ether;
use packet::ip;
use packet::tcp;
use packet::Packet;
use pcap;
use pcap::{Active, Capture, Device};
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

fn start_capture(interface_name: Option<String>) -> Capture<Active> {
  let dev = get_interface(interface_name).unwrap();

  println!("listening on {}", dev.name);

  Capture::from_device(dev)
    .unwrap()
    .promisc(true)
    .snaplen(100)
    .open()
    .unwrap()
}

fn main() {
  let mut cap = start_capture(env::args().nth(1));

  loop {
    let frame = cap.next().unwrap();

    let parsed_ether = ether::Packet::new(frame.data).unwrap();

    let proto = parsed_ether.protocol();

    println!(
      "{:?}: {} -> {}",
      proto,
      parsed_ether.source(),
      parsed_ether.destination()
    );

    if proto == ether::Protocol::Ipv4 {
      let parsed_ip = ip::v4::Packet::new(parsed_ether.payload()).unwrap();
      println!(
        "\t{:?}: {} -> {}",
        parsed_ip.protocol(),
        parsed_ip.source(),
        parsed_ip.destination()
      );

      if parsed_ip.protocol() == ip::Protocol::Tcp {
        let parsed_tcp = tcp::Packet::new(parsed_ip.payload()).unwrap();
        println!(
          "\t\tTcp: {} -> {}",
          parsed_tcp.source(),
          parsed_tcp.destination()
        );

        break;
      }
    }
  }
}
