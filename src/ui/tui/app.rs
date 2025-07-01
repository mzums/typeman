use std::io;
use crossterm::event::{self, Event as CEvent, KeyEvent};
use std::time::Duration;
use ratatui::DefaultTerminal;

use crate::ui::tui::ui::render_app;

pub struct App {
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { exit: false }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal, _reference: String) -> io::Result<()> {
        while !self.exit {
            if event::poll(Duration::from_millis(16))? {
                if let CEvent::Key(key) = event::read()? {
                    self.handle_key_event(key)?;
                }
            }
            
            terminal.draw(|frame| render_app(frame, self))?;
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match key_event.code {
                crossterm::event::KeyCode::Char('q') => self.exit = true,
                _ => {}
            }
        }
        Ok(())
    }
}