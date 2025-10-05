use std::io;
use crossterm::event::{self, Event as CEvent, KeyEvent};
use ratatui::DefaultTerminal;
use std::time::{Duration, Instant};
use chrono;

use crate::ui::tui::ui::render_app;
use crate::{practice, utils};
use crate::practice::TYPING_LEVELS;
use crate::language::Language;
use crate::color_scheme::ColorScheme;
use crate::config::AppConfig;
use crate::button_states::{ButtonStates, ButtonState};


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
    pub selected_config: String,
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
    pub language: Language,
    pub language_popup_open: bool,
    pub language_popup_selected: usize,
    pub color_scheme: ColorScheme,
    pub theme_popup_open: bool,
    pub theme_popup_selected: usize,
    pub app_config: AppConfig,
    pub leaderboard_open: bool,
    pub leaderboard_entries: Vec<crate::leaderboard::LeaderboardEntry>,
    pub leaderboard_selected: usize,
    pub button_states: ButtonStates,
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
            selected_config: if app_config.time_mode { "time".into() }
                else if app_config.word_mode { "words".into() }
                else if app_config.quote { "quote".into() }
                else if app_config.practice_mode { "practice".into() }
                else { "time".into() },
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
            language: app_config.language,
            language_popup_open: false,
            language_popup_selected: 0,
            color_scheme: app_config.color_scheme,
            theme_popup_open: false,
            theme_popup_selected: 0,
            app_config,
            leaderboard_open: false,
            leaderboard_entries: crate::leaderboard::load_entries().unwrap_or_default(),
            leaderboard_selected: 0,
            button_states: ButtonStates::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        if self.quote {
            self.reference = utils::get_random_quote();
            self.batch_size = self.reference.split_whitespace().count();
        } else if self.practice_mode {
            let level = practice::get_first_not_done();
            self.reference = practice::create_words(TYPING_LEVELS[level].1, 50);
        } else {
            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(500, self.language), self.batch_size);
        }
   
        self.is_correct = vec![0; self.reference.chars().count()];
        let mut last_recorded_time = Instant::now();
        
        while !self.exit {
            self.button_states = ButtonStates {
                punctuation: ButtonState::new("punctuation", "punctuation", "punct", self.punctuation, !self.quote && !self.practice_mode),
                numbers: ButtonState::new("numbers", "numbers", "num", self.numbers, !self.quote && !self.practice_mode),
                divider1: ButtonState::new("|", "|", "|", true, self.time_mode || self.word_mode),
                language: ButtonState::new("language", "language", "lang", false, !self.quote && !self.practice_mode),
                theme: ButtonState::new("theme", "theme", "theme", false, true),
                divider2: ButtonState::new("|", "|", "|", true, true),
                time: ButtonState::new("time", "time", "time", self.time_mode, true),
                words: ButtonState::new("words", "words", "words", self.word_mode, true),
                quote: ButtonState::new("quote", "quote", "quote", self.quote, true),
                practice: ButtonState::new("practice", "practice", "practice", self.practice_mode, true),
                divider3: ButtonState::new("|", "|", "|", true, self.time_mode || self.word_mode),
                time_15: ButtonState::new("15", "15", "15", self.test_time == 15.0, self.time_mode),
                time_30: ButtonState::new("30", "30", "30", self.test_time == 30.0, self.time_mode),
                time_60: ButtonState::new("60", "60", "60", self.test_time == 60.0, self.time_mode),
                time_120: ButtonState::new("120", "120", "120", self.test_time == 120.0, self.time_mode),
                batch_25: ButtonState::new("25", "25", "25", self.batch_size == 25, self.word_mode),
                batch_50: ButtonState::new("50", "50", "50", self.batch_size == 50, self.word_mode),
                batch_100: ButtonState::new("100", "100", "100", self.batch_size == 100, self.word_mode),
            };

            if self.game_state != GameState::Started {
                last_recorded_time = Instant::now();
            }
            if event::poll(Duration::from_millis(16))? {
                if let CEvent::Key(key) = event::read()? {
                    // Pass mutable reference to button_states
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

            if (self.test_time - self.timer.as_secs_f32() < 0.0 
                && self.game_state == GameState::Started 
                && self.time_mode) 
                || (self.words_done >= self.batch_size 
                    && (self.word_mode || self.quote) 
                    && self.game_state != GameState::Results)
                || ((self.words_done >= 50 || self.pos1 >= self.reference.chars().count()) && self.practice_mode && self.game_state != GameState::Results)

            {
                self.errors_per_second.push(self.errors_this_second);
                let total_typed = self.pressed_vec.len();
                let chars_in_this_second = total_typed.saturating_sub(self.char_number);
                let cpm = chars_in_this_second as f64 * 60.0;
                self.speed_per_second.push(cpm);
                self.game_state = GameState::Results;

                let (correct_words, _, _) = utils::count_correct_words(&self.reference, &std::collections::VecDeque::from(self.is_correct.clone()));
                let wpm = (correct_words as f32 / self.timer.as_secs_f32()) * 60.0;

                let accuracy = if self.words_done > 0 {
                    (self.correct_count as f32 / self.pressed_vec.len() as f32) * 100.0
                } else {
                    0.0
                };

                if self.practice_mode {
                    practice::save_results(
                        self.test_time as f64,
                        accuracy as f64,
                        wpm as f64,
                        self.selected_level + 1,
                    );
                }
                
                // Save result to leaderboard
                self.save_to_leaderboard();
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
            terminal.draw(|frame| render_app(frame, self, self.timer, &self.button_states))?;
        }
        Ok(())
    }

    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        reference: String,
    ) -> io::Result<()> {
        use crossterm::event::KeyCode;

        let reference_chars: Vec<char> = reference.chars().collect();

        if key_event.kind == crossterm::event::KeyEventKind::Press {
            // Handle theme popup first if it's open
            if self.theme_popup_open {
                match key_event.code {
                    KeyCode::Esc => {
                        self.theme_popup_open = false;
                        return Ok(());
                    }
                    KeyCode::Up => {
                        if self.theme_popup_selected > 0 {
                            self.theme_popup_selected -= 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Down => {
                        let schemes = ColorScheme::all();
                        if self.theme_popup_selected < schemes.len() - 1 {
                            self.theme_popup_selected += 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        let schemes = ColorScheme::all();
                        if self.theme_popup_selected < schemes.len() {
                            self.color_scheme = schemes[self.theme_popup_selected];
                        }
                        self.theme_popup_open = false;
                        self.save_config();
                        return Ok(());
                    }
                    _ => return Ok(()),
                }
            }

            // Handle leaderboard if it's open
            if self.leaderboard_open {
                match key_event.code {
                    KeyCode::Esc => {
                        self.leaderboard_open = false;
                        return Ok(());
                    }
                    KeyCode::Up => {
                        if self.leaderboard_selected > 0 {
                            self.leaderboard_selected -= 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Down => {
                        if self.leaderboard_selected < self.leaderboard_entries.len().saturating_sub(1) {
                            self.leaderboard_selected += 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Tab => {
                        self.tab_pressed = Instant::now();
                        return Ok(());
                    }
                    KeyCode::Char('l') | KeyCode::Char('L') => {
                        if self.tab_pressed.elapsed() < Duration::from_secs(1) {
                            self.leaderboard_open = false;
                            self.tab_pressed = Instant::now() - Duration::from_secs(5);
                        }
                        return Ok(());
                    }
                    _ => return Ok(()),
                }
            }

            // Handle language popup if it's open
            if self.language_popup_open {
                match key_event.code {
                    KeyCode::Esc => {
                        self.language_popup_open = false;
                        return Ok(());
                    }
                    KeyCode::Up => {
                        if self.language_popup_selected > 0 {
                            self.language_popup_selected -= 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Down => {
                        if self.language_popup_selected < Language::count() - 1 { 
                            self.language_popup_selected += 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        self.language = match self.language_popup_selected {
                            0 => Language::English,
                            1 => Language::Indonesian,
                            2 => Language::Italian,
                            _ => Language::English,
                        };
                        self.language_popup_open = false;
                        // Update reference text with new language
                        if self.word_mode || self.time_mode {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(500, self.language), self.batch_size);
                            self.is_correct = vec![0; self.reference.chars().count()];
                            self.pressed_vec.clear();
                            self.pos1 = 0;
                            self.words_done = 0;
                        }
                        self.save_config();
                        return Ok(());
                    }
                    _ => return Ok(()),
                }
            }

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
                    } else if self.practice_menu && self.selected_level > 0 {
                        self.selected_level -= 1;
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
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(500, self.language), self.batch_size);
                        } else if self.quote {
                            self.reference = utils::get_random_quote();
                            self.batch_size = self.reference.split_whitespace().count();
                        } else if self.practice_mode {
                            self.reference = practice::create_words(TYPING_LEVELS[self.selected_level].1, 50);
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
                        self.reference = practice::create_words(TYPING_LEVELS[self.selected_level].1, 50);
                        self.is_correct = vec![0; self.reference.chars().count()];
                    }
                    if self.config {
                        match self.selected_config.as_str() {
                            "time" => {
                                self.time_mode = true;
                                self.word_mode = false;
                                self.quote = false;
                                self.practice_mode = false;
                                self.batch_size = 50;
                                self.practice_mode = false;
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
                            "punctuation" => {
                                self.punctuation = !self.punctuation;
                            }
                            "numbers" => {
                                self.numbers = !self.numbers;
                            }
                            "language" => {
                                self.language_popup_open = true;
                                self.language_popup_selected = match self.language {
                                    Language::English => 0,
                                    Language::Indonesian => 1,
                                    Language::Italian=> 2,
                                };
                            }
                            "theme" => {
                                self.theme_popup_open = true;
                                let schemes = ColorScheme::all();
                                self.theme_popup_selected = schemes.iter().position(|&s| s == self.color_scheme).unwrap_or(0);
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
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(500, self.language), self.batch_size);
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
                    let buttons = self.button_states.as_vec();
                    for (i, btn) in buttons.iter().enumerate() {
                        if btn.visible && self.selected_config == btn.label {
                            let start_index = i;
                            let mut j = if i == 0 {
                                buttons.len() - 1
                            } else {
                                i - 1
                            };
                            while j != start_index {
                                if buttons[j].visible && buttons[j].label != "|" {
                                    self.selected_config = buttons[j].label.clone(); // Clone the string
                                    break;
                                }
                                j = if j == 0 {
                                    buttons.len() - 1
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
                    let buttons = self.button_states.as_vec();
                    for (i, btn) in buttons.iter().enumerate() {
                        if btn.visible && self.selected_config == btn.label {
                            let start_index = i;
                            let mut j = if i == buttons.len() - 1 {
                                0
                            } else {
                                i + 1
                            };
                            while j != start_index {
                                if buttons[j].visible && buttons[j].label != "|" {
                                    self.selected_config = buttons[j].label.clone(); // Clone the string
                                    break;
                                }
                                j = if j == buttons.len() - 1 {
                                    0
                                } else {
                                    j + 1
                                };
                            }
                            break;
                        }
                    }
                }
                KeyCode::Char(ch) => {
                    // Handle Tab+L leaderboard toggle
                    if (ch == 'l' || ch == 'L') && self.tab_pressed.elapsed() < Duration::from_secs(1) {
                        self.leaderboard_open = !self.leaderboard_open;
                        self.tab_pressed = Instant::now() - Duration::from_secs(5);
                        // Reload entries when opening leaderboard
                        if self.leaderboard_open {
                            self.leaderboard_entries = crate::leaderboard::load_entries().unwrap_or_default();
                            self.leaderboard_selected = 0;
                        }
                        return Ok(());
                    }
                    
                    if self.practice_menu && ch == 'q' {
                        self.practice_menu = false;
                        self.practice_mode = false;
                        self.time_mode = true;
                        self.selected_config = "time".into();
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
                            } else if ref_char == ch && (self.is_correct[self.pos1] == -1 || self.is_correct[self.pos1] == 1) {
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
                        // If we've reached the end of reference text, count the final word for word/quote modes
                        if (self.word_mode || self.quote) && self.pos1 > 0 {
                            // Check if we just completed a word (not already counted)
                            let previous_char = reference_chars.get(self.pos1 - 1);
                            if previous_char.is_some() && previous_char != Some(&' ') {
                                self.words_done += 1;
                            }
                        }
                        
                        // Only generate new reference if we haven't reached target word count yet
                        if self.time_mode {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(500, self.language), self.batch_size);
                            self.is_correct = vec![0; self.reference.chars().count()];
                            self.pos1 = 0;
                        }
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
            language: self.language,
            color_scheme: self.color_scheme,
        };
        
        let _ = self.app_config.save();
    }

    fn save_to_leaderboard(&mut self) {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed().as_secs_f64();
            let total_chars = self.pressed_vec.len();
            
            // Calculate WPM (words per minute) - using original formula: words_done / time

            let (correct_words, _, _) = utils::count_correct_words(&self.reference, &std::collections::VecDeque::from(self.is_correct.clone()));
            let wpm = (correct_words as f32 / self.timer.as_secs_f32()) * 60.0;
            
            // Calculate accuracy
            let correct_chars = self.correct_count;
            let accuracy = if total_chars > 0 {
                (correct_chars as f64 / total_chars as f64) * 100.0
            } else {
                0.0
            };
            
            // Determine test type
            let test_type = if self.practice_mode {
                crate::leaderboard::TestType::Practice(self.selected_level + 1)
            } else if self.time_mode {
                crate::leaderboard::TestType::Time(self.test_time as u32)
            } else if self.word_mode {
                crate::leaderboard::TestType::Word(self.batch_size)
            } else if self.quote {
                crate::leaderboard::TestType::Quote
            } else {
                crate::leaderboard::TestType::Time(30) // Default fallback
            };
            
            // Create leaderboard entry
            let entry = crate::leaderboard::LeaderboardEntry {
                wpm: wpm as f64,
                accuracy,
                test_type,
                test_mode: if self.practice_mode { "practice".to_string() }
                          else if self.time_mode { "time".to_string() }
                          else if self.word_mode { "word".to_string() }
                          else if self.quote { "quote".to_string() }
                          else { "time".to_string() },
                word_count: self.words_done, // Actual completed words
                test_duration: elapsed,
                timestamp: chrono::Local::now().to_rfc3339(),
                language: self.language,
            };
            
            // Save entry
            if let Err(e) = crate::leaderboard::save_entry(&entry) {
                // Enhanced error logging with specific error types
                match e {
                    crate::leaderboard::LeaderboardError::ValidationError(ref validation_err) => {
                        eprintln!("Invalid leaderboard entry data: {:?}", validation_err);
                        eprintln!("Entry not saved due to validation failure");
                    }
                    crate::leaderboard::LeaderboardError::IoError(ref io_err) => {
                        eprintln!("Failed to save leaderboard entry due to file system error: {}", io_err);
                        eprintln!("Check file permissions and disk space");
                    }
                    crate::leaderboard::LeaderboardError::SerializationError(ref serde_err) => {
                        eprintln!("Failed to serialize leaderboard data: {}", serde_err);
                        eprintln!("This may indicate data corruption");
                    }
                    crate::leaderboard::LeaderboardError::LockTimeout => {
                        eprintln!("Failed to save leaderboard entry: file lock timeout");
                        eprintln!("Another instance may be writing to the leaderboard");
                    }
                    crate::leaderboard::LeaderboardError::LockError(ref lock_err) => {
                        eprintln!("Failed to acquire leaderboard file lock: {}", lock_err);
                        eprintln!("Check file permissions and system resources");
                    }
                }
            }
            
            // Always update in-memory entries to ensure synchronization
            // This ensures the leaderboard immediately reflects the latest game results
            self.leaderboard_entries = crate::leaderboard::load_entries().unwrap_or_default();
        }
    }
}
