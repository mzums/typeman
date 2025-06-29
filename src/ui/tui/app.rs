use crate::ui::tui::event::Event;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub counter: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            counter: 0,
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(key) => {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    self.should_quit = true;
                }
                if key.code == crossterm::event::KeyCode::Char('c') {
                    self.counter += 1;
                }
                if key.code == crossterm::event::KeyCode::Char('r') {
                    self.counter = 0;
                }
            }
            _ => {}
        }
        self.should_quit
    }
}