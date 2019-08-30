use crate::events::Event;
use crossbeam_channel::*;
use log::{debug, info};
use std::{process, thread};
use ws::{listen, CloseCode, Handler, Handshake, Message, Result};

// Server WebSocket handler
struct Server {
  sender: Option<ws::Sender>,
  event_receiver: Option<Receiver<Event>>,
}

impl Handler for Server {
  fn on_open(&mut self, _shake: Handshake) -> Result<()> {
    debug!("ws on_open");

    // self.stop_sniff =
    //   Some(packet_capture::start(self.capture.take().unwrap(), self.sender.clone()).unwrap());

    let event_receiver = self.event_receiver.take().unwrap();
    let sender = self.sender.take().unwrap();

    thread::spawn(move || {
      for event in event_receiver {
        sender.send(serde_json::to_string(&event).unwrap()).unwrap();
      }

      info!("event loop done");

      process::exit(0);
    });

    Ok(()) // don't close yet
  }

  fn on_message(&mut self, msg: Message) -> Result<()> {
    debug!("incoming message {:?}", msg);

    Ok(())
  }

  fn on_close(&mut self, _code: CloseCode, _reason: &str) {
    debug!("ws on_close");

    process::exit(0);

    // if let Some(stop_sniff) = self.stop_sniff.take() {
    //   stop_sniff.call();
    // }
  }
}

pub fn start_blocking(addr: &str, event_receiver: Receiver<Event>) -> Result<()> {
  debug!("starting websocket server on {}", addr);

  let mut event_receiver = Some(event_receiver);

  listen(addr, |sender| Server {
    sender: Some(sender),
    event_receiver: Some(event_receiver.take().unwrap()),
  })?;

  Ok(())
}
