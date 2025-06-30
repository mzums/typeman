use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use crossterm::event::{self, Event as CEvent, KeyEvent};

pub enum Event {
    Input(KeyEvent),
    Progress(f64),
}

pub fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let CEvent::Key(key) = event::read().unwrap() {
                tx.send(Event::Input(key)).unwrap();
            }
        }
    }
}

pub fn run_background_thread(tx: mpsc::Sender<Event>) {
    let mut progress = 0.0f64;
    let increment = 0.01f64;
    
    loop {
        thread::sleep(Duration::from_millis(100));
        progress = (progress + increment).min(1.0);
        tx.send(Event::Progress(progress)).unwrap();
    }
}