use std::io;
use std::sync::mpsc;
use ratatui::prelude::Color;
use ratatui::DefaultTerminal;
use crate::ui::tui::event::Event;
use crate::ui::tui::ui::render_app;

pub struct ProgressBar {
    pub progress: f64,
    pub color: Color,
}

pub struct App {
    pub exit: bool,
    pub progress_bars: Vec<ProgressBar>,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            progress_bars: vec![ProgressBar { progress: 0.0, color: Color::Green }],
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> io::Result<()> {
        while !self.exit {
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
                Event::Progress(progress) => self.update_progress(progress),
            }
            terminal.draw(|frame| render_app(frame, self))?;
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match key_event.code {
                crossterm::event::KeyCode::Char('q') => self.exit = true,
                crossterm::event::KeyCode::Char('c') => self.toggle_last_progress_color(),
                _ => {}
            }
        }
        Ok(())
    }

    fn toggle_last_progress_color(&mut self) {
        if let Some(last) = self.progress_bars.last_mut() {
            last.color = if last.color == Color::Green {
                Color::Red
            } else {
                Color::Green
            };
        }
    }

    fn update_progress(&mut self, new_progress: f64) {
        if let Some(last) = self.progress_bars.last_mut() {
            last.progress = new_progress;
            if new_progress >= 1.0 {
                self.progress_bars.push(ProgressBar { progress: 0.0, color: Color::Green });
            }
        }
    }
}