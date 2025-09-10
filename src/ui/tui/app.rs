use std::io;
use crossterm::event::{self, Event as CEvent, KeyEvent};
use ratatui::DefaultTerminal;
use std::time::{Duration, Instant};

use crate::ui::tui::ui::render_app;
use crate::{practice, utils};
use crate::practice::TYPING_LEVELS;
use crate::config::AppConfig;


#[derive(PartialEq, Eq)]
pub enum GameState {
    NotStarted,
    Started,
    Results,
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
    pub speed_per_second: Vec<f64>,
    pub char_number: usize,
    pub errors_per_second: Vec<f32>,
    pub tab_pressed: Instant,
    pub correct_count: usize,
    pub error_count: usize,
    pub practice_menu: bool,
    pub practice_mode: bool,
    pub selected_level: usize,
    pub timer: Duration,
    pub app_config: AppConfig,
}

impl App {
    pub fn new() -> Self {
        let app_config = AppConfig::load();
        
        Self {
            exit: false,
            reference: String::new(),
            pressed_vec: Vec::new(),
            pos1: 0,
            words_done: 0,
            is_correct: Vec::new(),
            errors_this_second: 0.0,
            test_time: app_config.test_time,
            start_time: None,
            game_state: GameState::NotStarted,
            config: false,
            punctuation: app_config.punctuation,
            numbers: app_config.numbers,
            time_mode: app_config.time_mode,
            word_mode: app_config.word_mode,
            quote: app_config.quote,
            batch_size: app_config.batch_size,
            selected_config: if app_config.time_mode { "time" } 
                else if app_config.word_mode { "words" }
                else if app_config.quote { "quote" }
                else if app_config.practice_mode { "practice" }
                else { "time" },
            speed_per_second: Vec::new(),
            char_number: 0,
            errors_per_second: Vec::new(),
            tab_pressed: Instant::now() - Duration::from_secs(5),
            correct_count: 0,
            error_count: 0,
            practice_menu: false,
            practice_mode: app_config.practice_mode,
            selected_level: app_config.selected_level,
            timer: Duration::from_secs(0),
            app_config,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let word_list = utils::read_first_n_words(500);
        self.reference = utils::get_reference(false, false, &word_list, self.batch_size);
        self.is_correct = vec![0; self.reference.chars().count()];
        let mut last_recorded_time = Instant::now();
        
        while !self.exit {
            if self.game_state != GameState::Started {
                last_recorded_time = Instant::now();
            }
            if event::poll(Duration::from_millis(16))? {
                if let CEvent::Key(key) = event::read()? {
                    self.handle_key_event(key, self.reference.clone())?;
                }
            }
            self.timer = if let Some(start_time) = self.start_time {
                if self.game_state == GameState::Started {
                    Instant::now().duration_since(start_time)
                } else if self.game_state != GameState::Results {
                    Duration::from_secs(0)
                } else {
                    self.timer
                }
            } else {
                Duration::from_secs(0)
            };

            if self.test_time - (self.timer.as_secs_f32()) < 0.0 && self.game_state == GameState::Started && self.time_mode {
                self.errors_per_second.push(self.errors_this_second);
                self.game_state = GameState::Results;
            } else if self.words_done >= self.batch_size && (self.word_mode || self.practice_mode || self.quote) {
                self.errors_per_second.push(self.errors_this_second);
                self.game_state = GameState::Results;
            }
            let now = Instant::now();
            let time_since_last = now.duration_since(last_recorded_time);

            if time_since_last >= Duration::from_secs(1) && self.game_state == GameState::Started && self.game_state != GameState::Results {
                let total_typed = self.pressed_vec.len();
                let chars_in_this_second = total_typed.saturating_sub(self.char_number);
                let cpm = chars_in_this_second as f64 * 60.0;

                self.speed_per_second.push(cpm);

                self.char_number = total_typed;

                self.errors_per_second.push(self.errors_this_second);
                self.errors_this_second = 0.0;
                last_recorded_time += Duration::from_secs(1);
            }
            terminal.draw(|frame| render_app(frame, self, self.timer))?;
        }
        Ok(())
    }

    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        reference: String,
    ) -> io::Result<()> {
        use crossterm::event::KeyCode;

        let button_states = vec![
            ("! punctuation", self.punctuation, !self.quote && !self.practice_mode),
            ("# numbers", self.numbers, !self.quote && !self.practice_mode),
            ("|", true, true),
            ("time", self.time_mode, true),
            ("words", self.word_mode, true),
            ("quote", self.quote, true),
            ("practice", self.practice_mode, true),
            ("|", true, true),
            ("15", self.test_time == 15.0, self.time_mode),
            ("30", self.test_time == 30.0, self.time_mode),
            ("60", self.test_time == 60.0, self.time_mode),
            ("120", self.test_time == 120.0, self.time_mode),
            ("25", self.batch_size == 25, self.word_mode),
            ("50", self.batch_size == 50, self.word_mode),
            ("100", self.batch_size == 100, self.word_mode),
        ];

        let reference_chars: Vec<char> = reference.chars().collect();

        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match key_event.code {
                KeyCode::Esc => {
                    self.save_config();
                    self.exit = true;
                },
                KeyCode::Backspace => {
                    if !self.pressed_vec.is_empty() && reference_chars.get(self.pos1) == Some(&' ') {
                        self.words_done = self.words_done.saturating_sub(1);
                    }
                    if self.is_correct[self.pos1] == 2 || self.is_correct[self.pos1] == 1 {
                        self.correct_count = self.correct_count.saturating_sub(1);
                    } else if self.is_correct[self.pos1] == -1 {
                        self.error_count = self.error_count.saturating_sub(1);
                    }
                    self.pressed_vec.pop();
                    if self.pos1 > 0 {
                        self.pos1 -= 1;
                    }
                    self.config = false;
                }
                KeyCode::Up => {
                    if self.game_state != GameState::Results && !self.practice_menu {
                        self.config = true;
                    } else if self.practice_menu {
                        if self.selected_level > 0 {
                            self.selected_level -= 1;
                        }
                    }
                }
                KeyCode::Down => {
                    if self.practice_menu {
                        if self.selected_level < TYPING_LEVELS.len() - 1 {
                            self.selected_level += 1;
                        }
                    } else {
                        self.config = true;
                    }
                }
                KeyCode::Tab => {
                    self.tab_pressed = Instant::now();
                },
                KeyCode::Enter => {
                    if self.tab_pressed.elapsed() < Duration::from_secs(1) {
                        if self.word_mode || self.time_mode {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(500), self.batch_size);
                        } else if self.quote {
                            self.reference = utils::get_random_quote();
                            self.batch_size = self.reference.split_whitespace().count();
                        } else if self.practice_mode {
                            self.reference = practice::create_words(TYPING_LEVELS[self.selected_level].1, self.batch_size);
                        }
                        self.is_correct = vec![0; self.reference.chars().count()];
                        self.pressed_vec.clear();
                        self.pos1 = 0;
                        self.words_done = 0;
                        self.errors_this_second = 0.0;
                        self.start_time = None;
                        self.game_state = GameState::NotStarted;
                        self.speed_per_second.clear();
                        self.char_number = 0;
                        self.errors_per_second.clear();
                        self.tab_pressed = Instant::now() - Duration::from_secs(5);
                        self.correct_count = 0;
                        self.error_count = 0;
                    }
                    if self.practice_menu {
                        self.practice_menu = false;
                        self.practice_mode = true;
                        self.time_mode = false;
                        self.word_mode = false;
                        self.quote = false;
                        self.batch_size = 50;
                        self.pressed_vec.clear();
                        self.pos1 = 0;
                        self.words_done = 0;
                        self.errors_this_second = 0.0;
                        self.start_time = None;
                        self.game_state = GameState::NotStarted;
                        self.speed_per_second.clear();
                        self.char_number = 0;
                        self.errors_per_second.clear();
                        self.tab_pressed = Instant::now() - Duration::from_secs(5);
                        self.correct_count = 0;
                        self.error_count = 0;
                        self.config = false;
                        self.reference = practice::create_words(TYPING_LEVELS[self.selected_level].1, self.batch_size);
                        self.is_correct = vec![0; self.reference.chars().count()];
                    }
                    if self.config {
                        match self.selected_config {
                            "time" => {
                                self.time_mode = true;
                                self.word_mode = false;
                                self.quote = false;
                                self.practice_mode = false;
                                self.batch_size = 50;
                            }
                            "words" => {
                                if !self.word_mode {
                                    self.batch_size = 50;
                                }
                                self.time_mode = false;
                                self.word_mode = true;
                                self.quote = false;
                                self.practice_mode = false;
                            }
                            "quote" => {
                                self.quote = true;
                                self.time_mode = false;
                                self.word_mode = false;
                                self.practice_mode = false;
                            }
                            "practice" => {
                                self.practice_menu = !self.practice_menu;
                                self.selected_level = practice::get_first_not_done();
                            }
                            "! punctuation" => {
                                self.punctuation = !self.punctuation;
                            }
                            "# numbers" => {
                                self.numbers = !self.numbers;
                            }
                            "15" => {
                                self.test_time = 15.0;
                            }
                            "30" => {
                                self.test_time = 30.0;
                            }
                            "60" => {
                                self.test_time = 60.0;
                            }
                            "120" => {
                                self.test_time = 120.0;
                            }
                            "25" => {
                                self.batch_size = 25;
                            }
                            "50" => {
                                self.batch_size = 50;
                            }
                            "100" => {
                                self.batch_size = 100;
                            }
                            _ => {}
                        }
                        if self.selected_config == "quote" {
                            self.reference = utils::get_random_quote();
                            self.batch_size = self.reference.split_whitespace().count();
                        }
                        else {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(500), self.batch_size);
                        }
                        self.is_correct = vec![0; self.reference.chars().count()];
                        self.pressed_vec.clear();
                        self.pos1 = 0;
                        self.words_done = 0;
                        self.errors_this_second = 0.0;
                        self.start_time = None;
                        self.game_state = GameState::NotStarted;
                        self.speed_per_second.clear();
                        self.char_number = 0;
                        self.errors_per_second.clear();
                        self.tab_pressed = Instant::now() - Duration::from_secs(5);
                        self.correct_count = 0;
                        self.error_count = 0;
                        self.config = false;
                        self.save_config();
                    }
                }
                KeyCode::Left => {
                    if !self.config {
                        return Ok(());
                    }
                    for (i, (label, _state_val, visible)) in button_states.iter().enumerate() {
                        if *visible && self.selected_config == *label {
                            let start_index = i;
                            let mut j = if i == 0 {
                                button_states.len() - 1
                            } else {
                                i - 1
                            };

                            while j != start_index {
                                if button_states[j].2 && button_states[j].0 != "|" {
                                    self.selected_config = button_states[j].0;
                                    break;
                                }
                                j = if j == 0 {
                                    button_states.len() - 1
                                } else {
                                    j - 1
                                };
                            }
                            break;
                        }
                    }
                }
                KeyCode::Right => {
                    if !self.config {
                        return Ok(());
                    }
                    for (i, (label, _state_val, visible)) in button_states.iter().enumerate() {
                        if *visible && self.selected_config == *label {
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
                    if self.practice_menu && ch == 'q' {
                        self.practice_menu = false;
                        return Ok(());
                    }
                    if self.is_correct[0] == 0 && ch == ' ' {
                        return Ok(());
                    }
                    let reference_chars: Vec<char> = self.reference.chars().collect();
                    if let Some(&ref_char) = reference_chars.get(self.pos1) {
                        if self.game_state == GameState::Results {
                            return Ok(());
                        }
                        if self.game_state == GameState::NotStarted {
                            self.game_state = GameState::Started;
                            self.start_time = Some(Instant::now());
                        }
                        if self.is_correct.len() > self.pos1 {
                            
                            if ref_char == ch && self.is_correct[self.pos1] != -1 && self.is_correct[self.pos1] != 1 {
                                self.is_correct[self.pos1] = 2; // Correct
                                self.correct_count += 1;
                                self.pos1 += 1;
                            } else if ref_char == ch && self.is_correct[self.pos1] == -1 {
                                self.is_correct[self.pos1] = 1; // Corrected
                                self.pos1 += 1;
                            } else {
                                self.is_correct[self.pos1] = -1; // Incorrect
                                self.errors_this_second += 1.0;
                                self.error_count += 1;
                                if !self.practice_mode {
                                    self.pos1 += 1;
                                }
                            }
                        }
                        
                        self.pressed_vec.push(ch);
                        if (reference_chars.get(self.pos1) == Some(&' ') && !self.practice_mode) || (reference_chars.get(self.pos1) == Some(&' ') && self.is_correct[self.pos1] != -1) {
                            self.words_done += 1;
                        }
                    }
                    self.config = false;

                    if self.pos1 >= self.reference.chars().count() {
                        self.words_done += 1;
                        self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(500), self.batch_size);
                        self.is_correct = vec![0; self.reference.chars().count()];
                        self.pos1 = 0;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn save_config(&mut self) {
        self.app_config = AppConfig {
            punctuation: self.punctuation,
            numbers: self.numbers,
            time_mode: self.time_mode,
            word_mode: self.word_mode,
            quote: self.quote,
            practice_mode: self.practice_mode,
            batch_size: self.batch_size,
            test_time: self.test_time,
            selected_level: self.selected_level,
        };
        
        let _ = self.app_config.save();
    }
}
