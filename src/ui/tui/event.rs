use crossterm::event::{self, Event as CEvent, KeyEvent};
use std::time::Duration;
use std::{sync::mpsc, thread};

pub enum Event {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler {
    rx: mpsc::Receiver<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let (tx, rx) = mpsc::channel();
        let tx_clone = tx.clone();
        
        thread::spawn(move || {
            loop {
                if event::poll(Duration::from_millis(tick_rate)).unwrap() {
                    if let Ok(CEvent::Key(key)) = event::read() {
                        tx.send(Event::Key(key)).unwrap();
                    }
                }
                tx_clone.send(Event::Tick).unwrap();
            }
        });
        
        EventHandler { rx }
    }

    pub fn next(&self) -> Result<Option<Event>, mpsc::RecvError> {
        match self.rx.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => Ok(Some(event)),
            Err(mpsc::RecvTimeoutError::Timeout) => Ok(None),
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                Err(mpsc::RecvError)
            }
        }
    }
}