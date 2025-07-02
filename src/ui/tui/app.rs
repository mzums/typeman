use std::io;
use crossterm::event::{self, Event as CEvent, KeyEvent};
use eframe::egui::Key;
use ratatui::DefaultTerminal;
use std::time::{Duration, Instant};

use crate::ui::tui::ui::render_app;
use crate::utils;

use std::fs::OpenOptions;
use std::io::Write;


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
    pub test_time: f32,
    pub start_time: Option<Instant>,
    pub game_state: GameState,
    pub config: bool,
    pub punctuation: bool,
    pub numbers: bool,
    pub time_mode: bool,
    pub word_mode: bool,
    pub quote: bool,
    pub batch_size: usize,
    pub selected_config: &'static str,
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
            test_time: 15.0,
            start_time: None,
            game_state: GameState::NotStarted,
            config: false,
            punctuation: false,
            numbers: false,
            time_mode: true,
            word_mode: false,
            quote: false,
            batch_size: 50,
            selected_config: "time",
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let word_list = utils::read_first_n_words(500);
        self.reference = utils::get_reference(false, false, &word_list, self.batch_size);
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

        let button_states = vec![
            ("! punctuation", self.punctuation, !self.quote),
            ("# numbers", self.numbers, !self.quote),
            ("|", true, true),
            ("time", self.time_mode, true),
            ("words", self.word_mode, true),
            ("quote", self.quote, true),
            ("|", true, true),
            ("15", self.test_time == 15.0, self.time_mode),
            ("30", self.test_time == 30.0, self.time_mode),
            ("60", self.test_time == 60.0, self.time_mode),
            ("120", self.test_time == 120.0, self.time_mode),
            ("25", self.batch_size == 25, self.word_mode),
            ("50", self.batch_size == 50, self.word_mode),
            ("100", self.batch_size == 100, self.word_mode),
        ];

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
                KeyCode::Up => {
                    self.config = true;
                }
                KeyCode::Down => {
                    self.config = false;
                }
                KeyCode::Left => {
                    if ! self.config {
                        return Ok(());
                    }
                    for (i, (label, state_val, visible)) in button_states.iter().enumerate() {
                        if *visible && self.selected_config == *label {
                            /*let mut file = OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open("lposition.log")
                                .unwrap();
                            writeln!(file, "pos1: {}", i).unwrap();*/
                            if i == 0 {
                                self.selected_config = button_states.last().unwrap().0;
                            } else {
                                let mut prev = i - 1;

                                if button_states[prev].0 == "|" {
                                    prev -= 1;
                                }
                                if !button_states[prev].2 {
                                    prev -= 1;
                                }
                                self.selected_config = button_states[prev].0;
                            }
                            break;
                        }
                    }
                }
                KeyCode::Right => {
                    if ! self.config {
                        return Ok(());
                    }
                    for (i, (label, state_val, visible)) in button_states.iter().enumerate() {
                        if *visible && self.selected_config == *label {
                            
                            let mut file = OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open("lposition.log")
                                .unwrap();
                            writeln!(file, "pos1: {}", i).unwrap();

                            if i == button_states.len() - 1 {
                                self.selected_config = button_states[0].0;
                            } else {
                                let mut next = i + 1;
                                if button_states[next].0 == "|" {
                                    next += 1;
                                }
                                while next != i {
                                    if next >= button_states.len() {
                                        next = 0;
                                    }
                                    if button_states[next].2 {
                                        self.selected_config = button_states[next].0;
                                        break;
                                    }
                                    next += 1;
                                }
                            }
                            break;
                        }
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
