use std::io;
use std::thread;
use std::sync::mpsc;
use crate::ui::tui::event::{Event, handle_input_events};
use crate ::ui::tui::app::App;
use crate::ui::tui::event;


pub fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    
    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let tx_to_input_events = event_tx.clone();
    thread::spawn(move || handle_input_events(tx_to_input_events));

    let tx_to_background_events = event_tx.clone();
    thread::spawn(move || event::run_background_thread(tx_to_background_events));

    let app_result = app.run(&mut terminal, event_rx);
    ratatui::restore();

    app_result
}