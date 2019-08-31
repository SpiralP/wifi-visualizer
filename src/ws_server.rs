use crate::events::Event;
use crossbeam_channel::*;
use log::{debug, info};
use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread,
  time::Duration,
};
use ws::{Builder, CloseCode, Handler, Handshake, Message, Result, Sender, Settings};

// Server WebSocket handler
struct Server {
  sender: ws::Sender,
  event_receiver: Option<Receiver<Event>>,
  stop_notify: Arc<AtomicBool>,
}

impl Handler for Server {
  fn on_open(&mut self, _shake: Handshake) -> Result<()> {
    debug!("ws on_open");

    let event_receiver = self.event_receiver.take().unwrap();
    let sender = self.sender.clone();
    let stop_notify = self.stop_notify.clone();

    thread::Builder::new()
      .name("event_receiver_thread".into())
      .spawn(move || {
        for event in event_receiver {
          if stop_notify.load(Ordering::SeqCst) {
            return;
          }

          sender.send(serde_json::to_string(&event).unwrap()).unwrap();
        }

        info!("event loop done");

        let _ = sender.close(CloseCode::Normal);
      })
      .unwrap();

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

    self.stop_notify.store(true, Ordering::SeqCst);
  }
}

pub fn start_blocking(
  addr: &str,
  event_receiver: Receiver<Event>,
  stop_notify: Arc<AtomicBool>,
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

  thread::spawn(move || loop {
    thread::sleep(Duration::from_millis(100));

    if stop_notify.load(Ordering::SeqCst) {
      let _ = broadcaster.shutdown();
      break;
    }
  });

  websocket.listen(addr)?;

  Ok(())
}
