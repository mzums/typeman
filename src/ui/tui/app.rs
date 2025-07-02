use std::io;
use crossterm::event::{self, Event as CEvent, KeyEvent};
use std::time::Duration;
use ratatui::DefaultTerminal;
use std::fs::OpenOptions;
use std::io::Write;

use crate::ui::tui::ui::render_app;

pub struct App {
    pub exit: bool,
    pub reference: String,
    pub pressed_vec: Vec<char>,
    pub pos1: usize,
    pub words_done: usize,
    pub is_correct: Vec<i32>,
    pub errors_this_second: f32,
}

impl App {
    pub fn new() -> Self {
        Self { exit: false, 
               reference: String::new(),
               pressed_vec: Vec::new(),
               pos1: 0,
               words_done: 0,
               is_correct: Vec::new(),
               errors_this_second: 0.0 }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal, reference: String) -> io::Result<()> {
        /*if let Ok(mut file) = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("key_events.log")
                    {
                        let _ = writeln!(file, "ref: {}", reference);
                    }*/
        self.is_correct = vec![0; reference.chars().count()];
        while !self.exit {
            if event::poll(Duration::from_millis(16))? {
                if let CEvent::Key(key) = event::read()? {
                    self.handle_key_event(key, &reference)?;
                }
            }
            
            terminal.draw(|frame| render_app(frame, self, &reference, &self.is_correct, &self.pos1))?;
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        reference: &String,
    ) -> io::Result<()> {
        use crossterm::event::KeyCode;

        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match key_event.code {
                KeyCode::Esc => self.exit = true,
                KeyCode::Tab | KeyCode::Enter => {
                    //
                }
                KeyCode::Backspace => {
                    if !self.pressed_vec.is_empty() && self.reference.chars().nth(self.pos1) == Some(' ') {
                        self.words_done = self.words_done.saturating_sub(1);
                    }
                    self.pressed_vec.pop();
                    if self.pos1 > 0 {
                        self.pos1 -= 1;
                    }
                    /*if let Ok(mut file) = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("key_events.log")
                    {
                        let _ = writeln!(file, "pos1: {:?}", self.pos1);
                    }*/
                }
                KeyCode::Char(ch) => {
                    let ref_char: Option<char> = reference.chars().nth(self.pos1);
                    //println!("{}", self.reference);
                    if self.is_correct.len() > self.pos1 && ref_char == Some(ch) && self.is_correct[self.pos1] != -1 && self.is_correct[self.pos1] != 1 {
                        self.is_correct[self.pos1] = 2; // Correct
                    } else if ref_char == Some(ch) && self.is_correct[self.pos1] == -1 {
                        self.is_correct[self.pos1] = 1; // Corrected
                    } else {
                        if self.is_correct.len() > self.pos1 {
                            self.is_correct[self.pos1] = -1; // Incorrect
                        }
                        self.errors_this_second += 1.0;
                    }
                    self.pos1 += 1;
                    self.pressed_vec.push(ch);
                    if self.reference.chars().nth(self.pos1) == Some(' ') {
                        self.words_done += 1;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
