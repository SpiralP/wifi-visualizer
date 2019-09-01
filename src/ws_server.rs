use crate::events::Event;
use crossbeam_channel::*;
use helpers::{check_notified_return, notify::Notify, thread};
use log::{debug, info};
use ws::{Builder, CloseCode, Handler, Handshake, Message, Result, Sender, Settings};

// Server WebSocket handler
struct Server {
  sender: ws::Sender,
  event_receiver: Option<Receiver<Event>>,
  stop_notify: Notify,
}

impl Handler for Server {
  fn on_open(&mut self, _shake: Handshake) -> Result<()> {
    debug!("ws on_open");

    let event_receiver = self.event_receiver.take().unwrap();
    let sender = self.sender.clone();
    let stop_notify = self.stop_notify.clone();

    thread::spawn("event_receiver_thread", move || {
      for event in event_receiver {
        check_notified_return!(stop_notify);

        sender.send(serde_json::to_string(&event).unwrap()).unwrap();
      }

      info!("event loop done");

      let _ = sender.close(CloseCode::Normal);
    });

    Ok(()) // don't close yet
  }

  fn on_message(&mut self, msg: Message) -> Result<()> {
    debug!("incoming message {:?}", msg);

    Ok(())
  }

  fn on_close(&mut self, _code: CloseCode, _reason: &str) {
    debug!("ws on_close");

    info!("websocket closed");
    let _ = self.sender.shutdown();

    self.stop_notify.notify();
  }
}

pub fn start_blocking(
  addr: &str,
  event_receiver: Receiver<Event>,
  mut stop_notify: Notify,
) -> Result<()> {
  debug!("starting websocket server on {}", addr);

  let websocket = {
    let mut event_receiver = Some(event_receiver);
    let mut stop_notify = Some(stop_notify.clone());

    Builder::new()
      .with_settings(Settings {
        max_connections: 1,
        ..Settings::default()
      })
      .build(move |sender: Sender| Server {
        sender,
        event_receiver: Some(event_receiver.take().unwrap()),
        stop_notify: stop_notify.take().unwrap(),
      })?
  };

  let broadcaster = websocket.broadcaster();

  stop_notify.wait(move || {
    let _ = broadcaster.shutdown();
  });

  websocket.listen(addr)?;

  Ok(())
}
