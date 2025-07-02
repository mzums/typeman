use std::io;
use crossterm::event::{self, Event as CEvent, KeyEvent};
use ratatui::DefaultTerminal;
use std::time::{Duration, Instant};

use crate::ui::tui::ui::render_app;
use crate::utils;


#[derive(PartialEq, Eq)]
pub enum GameState {
    NotStarted,
    Started,
    _Results,
}

pub struct App {
    pub exit: bool,
    pub reference: String,
    pub pressed_vec: Vec<char>,
    pub pos1: usize,
    pub words_done: usize,
    pub is_correct: Vec<i32>,
    pub errors_this_second: f32,
    pub test_time: Duration,
    pub start_time: Option<Instant>,
    pub game_state: GameState,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            reference: String::new(),
            pressed_vec: Vec::new(),
            pos1: 0,
            words_done: 0,
            is_correct: Vec::new(),
            errors_this_second: 0.0,
            test_time: Duration::from_secs(10),
            start_time: None,
            game_state: GameState::NotStarted,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let word_list = utils::read_first_n_words(500);
        self.reference = utils::get_reference(false, false, &word_list, 50);
        self.is_correct = vec![0; self.reference.chars().count()];
        let reference_chars: Vec<char> = self.reference.chars().collect();
        
        while !self.exit {
            if event::poll(Duration::from_millis(16))? {
                if let CEvent::Key(key) = event::read()? {
                    self.handle_key_event(key, &reference_chars)?;
                }
            }
            let timer = if let Some(start_time) = self.start_time {
                if self.game_state == GameState::Started {
                    Instant::now().duration_since(start_time)
                } else {
                    Duration::from_secs(0)
                }
            } else {
                Duration::from_secs(0)
            };
            //println!("Timer: {:?}", timer);
            terminal.draw(|frame| render_app(frame, self, timer))?;
        }
        Ok(())
    }

    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        reference_chars: &[char],
    ) -> io::Result<()> {
        use crossterm::event::KeyCode;

        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match key_event.code {
                KeyCode::Esc => self.exit = true,
                KeyCode::Tab | KeyCode::Enter => {}
                KeyCode::Backspace => {
                    if !self.pressed_vec.is_empty() && reference_chars.get(self.pos1) == Some(&' ') {
                        self.words_done = self.words_done.saturating_sub(1);
                    }
                    self.pressed_vec.pop();
                    if self.pos1 > 0 {
                        self.pos1 -= 1;
                    }
                }
                KeyCode::Char(ch) => {
                    if let Some(&ref_char) = reference_chars.get(self.pos1) {
                        if self.game_state == GameState::NotStarted {
                            self.game_state = GameState::Started;
                            self.start_time = Some(Instant::now());
                        }
                        if self.is_correct.len() > self.pos1 {
                            if ref_char == ch && self.is_correct[self.pos1] != -1 && self.is_correct[self.pos1] != 1 {
                                self.is_correct[self.pos1] = 2; // Correct
                            } else if ref_char == ch && self.is_correct[self.pos1] == -1 {
                                self.is_correct[self.pos1] = 1; // Corrected
                            } else {
                                self.is_correct[self.pos1] = -1; // Incorrect
                                self.errors_this_second += 1.0;
                            }
                        }
                        self.pos1 += 1;
                        self.pressed_vec.push(ch);
                        if reference_chars.get(self.pos1) == Some(&' ') {
                            self.words_done += 1;
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
