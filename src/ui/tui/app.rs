use std::io;
use crossterm::event::{self, Event as CEvent, KeyEvent, KeyCode};
use ratatui::DefaultTerminal;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use chrono;

use crate::batch_size_selection::BatchSizeSelection;
use crate::ui::tui::ui::render_app;
use crate::{practice, utils};
use crate::practice::TYPING_LEVELS;
use crate::language::Language;
use crate::color_scheme::ColorScheme;
use crate::config::AppConfig;
use crate::button_states::{ButtonStates, ButtonState};
use crate::ui::tui::popup::{PopupStates, PopupState};
use crate::time_selection::TimeSelection;
use crate::word_number_selection::WordNumberSelection;
use crate::top_words_selection::TopWordsSelection;
use crate::settings::Settings;
use crate::leaderboard::LeaderboardData;


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
    pub wiki_mode: bool,
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
    pub color_scheme: ColorScheme,
    pub word_number: usize,
    pub top_words: usize,
    pub app_config: AppConfig,
    pub button_states: ButtonStates,
    pub popup_states: PopupStates,
    pub menu_buttons_times: HashMap<String, Instant>,
    pub leaderboard: LeaderboardData,
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
            wiki_mode: app_config.wiki_mode,
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
            color_scheme: app_config.color_scheme,
            word_number: app_config.word_number,
            top_words: app_config.top_words,
            app_config,
            button_states: ButtonStates::new(),
            popup_states: PopupStates {
                language: PopupState { open: false, selected: 0 },
                color_scheme: PopupState { open: false, selected: 0 },
                time_selection: PopupState { open: false, selected: 0 },
                word_number_selection: PopupState { open: false, selected: 0 },
                settings: PopupState { open: false, selected: 0 },
                batch_size_selection: PopupState { open: false, selected: 0 },
                top_words_selection: PopupState { open: false, selected: 0 },
            },
            menu_buttons_times: HashMap::from([
                ("settings".to_string(), Instant::now() - Duration::from_secs(5)),
                ("time".to_string(), Instant::now() - Duration::from_secs(5)),
                ("words".to_string(), Instant::now() - Duration::from_secs(5)),
                ("quote".to_string(), Instant::now() - Duration::from_secs(5)),
                ("practice".to_string(), Instant::now() - Duration::from_secs(5)),
                ("punctuation".to_string(), Instant::now() - Duration::from_secs(5)),
                ("numbers".to_string(), Instant::now() - Duration::from_secs(5)),
                ("wiki".to_string(), Instant::now() - Duration::from_secs(5)),
            ]),
            leaderboard: LeaderboardData {
                open: false,
                entries: crate::leaderboard::load_entries().unwrap_or_default(),
                selected: 0,
            },
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        if self.quote {
            self.reference = utils::get_random_quote();
            self.batch_size = self.reference.split_whitespace().count();
        } else if self.practice_mode {
            let level = practice::get_first_not_done();
            self.reference = practice::create_words(TYPING_LEVELS[level].1, 50);
        } else if self.time_mode {
            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), self.batch_size);
        } else if self.wiki_mode {
            self.reference = utils::get_wiki_summary();
        } else {
            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), usize::min(self.batch_size, self.word_number));
        }
   
        self.is_correct = vec![0; self.reference.chars().count()];
        let mut last_recorded_time = Instant::now();
        
        while !self.exit {
            self.button_states = ButtonStates {
                settings: ButtonState::new("settings", "settings", "...", false, true),
                divider0: ButtonState::new("|", "|", "|", true, true),
                punctuation: ButtonState::new("punctuation", "! punctuation", "! punct", self.punctuation, !self.quote && !self.practice_mode && !self.wiki_mode),
                numbers: ButtonState::new("numbers", "# numbers", "# num", self.numbers, !self.quote && !self.practice_mode && !self.wiki_mode),
                divider1: ButtonState::new("|", "|", "|", true, self.time_mode || self.word_mode),
                time: ButtonState::new("time", "⌄ time", "⌄ time", self.time_mode, true),
                words: ButtonState::new("words", "⌄ words", "⌄ words", self.word_mode, true),
                quote: ButtonState::new("quote", "quote", "quote", self.quote, true),
                wiki_mode: ButtonState::new("wiki", "wikipedia", "wiki", self.wiki_mode, true),
                practice: ButtonState::new("practice", "practice", "practice", self.practice_mode, true),
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

            if self.game_state != GameState::Results && ((self.test_time - self.timer.as_secs_f32() < 0.0 && self.game_state == GameState::Started && self.time_mode)
                || (self.words_done >= self.word_number && self.word_mode)
                || (self.words_done >= self.reference.split_whitespace().count() && (self.quote || self.wiki_mode) && self.game_state != GameState::Results)
                || (self.words_done >= self.word_number 
                    && (self.word_mode|| self.practice_mode)
                    && self.game_state != GameState::Results)
                || ((self.words_done >= 50 || self.pos1 >= self.reference.chars().count()) && self.practice_mode && self.game_state != GameState::Results))

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
            terminal.draw(|frame| render_app(frame, self))?;
        }
        Ok(())
    }

    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        reference: String,
    ) -> io::Result<()> {
        let reference_chars: Vec<char> = reference.chars().collect();

        if key_event.kind == crossterm::event::KeyEventKind::Press {
            let schemes = ColorScheme::all();
            if self.popup_states.color_scheme.open {
                match key_event.code {
                    KeyCode::Esc => {
                        self.popup_states.color_scheme.open = false;
                        return Ok(());
                    }
                    KeyCode::Up => {
                        if self.popup_states.color_scheme.selected > 0 {
                            self.popup_states.color_scheme.selected -= 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Down => {
                        if self.popup_states.color_scheme.selected < schemes.len() - 1 {
                            self.popup_states.color_scheme.selected += 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        if self.popup_states.color_scheme.selected < schemes.len() {
                            self.color_scheme = schemes[self.popup_states.color_scheme.selected];
                        }
                        self.popup_states.color_scheme.open = false;
                        self.save_config();
                        return Ok(());
                    }
                    _ => return Ok(()),
                }
            }

            if self.popup_states.time_selection.open {
                let schemes = TimeSelection::all();
                match key_event.code {
                    KeyCode::Esc => {
                        self.popup_states.time_selection.open = false;
                        return Ok(());
                    }
                    KeyCode::Up => {
                        if self.popup_states.time_selection.selected > 0 {
                            self.popup_states.time_selection.selected -= 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Down => {
                        if self.popup_states.time_selection.selected < schemes.len() - 1 {
                            self.popup_states.time_selection.selected += 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        if self.popup_states.time_selection.selected < schemes.len() {
                            self.test_time = schemes[self.popup_states.time_selection.selected].to_seconds() as f32;
                        }
                        self.popup_states.time_selection.open = false;
                        self.save_config();
                        return Ok(());
                    }
                    _ => return Ok(()),
                }
            }

            if self.popup_states.language.open {
                let schemes = Language::all();
                match key_event.code {
                    KeyCode::Esc => {
                        self.popup_states.language.open = false;
                        return Ok(());
                    }
                    KeyCode::Up => {
                        if self.popup_states.language.selected > 0 {
                            self.popup_states.language.selected -= 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Down => {
                        if self.popup_states.language.selected < schemes.len() - 1 {
                            self.popup_states.language.selected += 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        self.language = match self.popup_states.language.selected {
                            0 => Language::English,
                            1 => Language::Indonesian,
                            2 => Language::Italian,
                            _ => Language::English,
                        };
                        self.popup_states.language.open = false;
                        if self.word_mode || self.time_mode {
                            if self.time_mode {
                                self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), self.batch_size);
                            } else if self.word_mode {
                                self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), usize::min(self.batch_size, self.word_number));
                            }
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

            if self.popup_states.word_number_selection.open {
                let schemes = WordNumberSelection::all();
                match key_event.code {
                    KeyCode::Esc => {
                        self.popup_states.word_number_selection.open = false;
                        return Ok(());
                    }
                    KeyCode::Up => {
                        if self.popup_states.word_number_selection.selected > 0 {
                            self.popup_states.word_number_selection.selected -= 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Down => {
                        if self.popup_states.word_number_selection.selected < WordNumberSelection::count() - 1 {
                            self.popup_states.word_number_selection.selected += 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        if self.popup_states.word_number_selection.selected < schemes.len() {
                            self.word_number = schemes[self.popup_states.word_number_selection.selected].to_words() as usize;
                        }
                        if self.time_mode {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), self.batch_size);
                        } else if self.word_mode {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), usize::min(self.batch_size, self.word_number));
                        }
                        self.is_correct = vec![0; self.reference.chars().count()];
                        self.pressed_vec.clear();
                        self.pos1 = 0;
                        self.words_done = 0;
                        self.popup_states.word_number_selection.open = false;
                        self.save_config();
                        return Ok(());
                    }
                    _ => return Ok(()),
                }
            }
            
            if self.popup_states.batch_size_selection.open {
                let schemes = BatchSizeSelection::all();
                match key_event.code {
                    KeyCode::Esc => {
                        self.popup_states.batch_size_selection.open = false;
                        return Ok(());
                    }
                    KeyCode::Up => {
                        self.popup_states.batch_size_selection.selected -= 1;
                        if self.popup_states.batch_size_selection.selected > 0 {
                        }
                        return Ok(());
                    }
                    KeyCode::Down => {
                        self.popup_states.batch_size_selection.selected += 1;
                        if self.popup_states.batch_size_selection.selected < BatchSizeSelection::count() - 1 {
                        }
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        if self.popup_states.batch_size_selection.selected < schemes.len() {
                            self.batch_size = schemes[self.popup_states.batch_size_selection.selected].to_words() as usize;
                        }
                        if self.time_mode {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), self.batch_size);
                        } else if self.word_mode {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), usize::min(self.batch_size, self.word_number));
                        }
                        self.is_correct = vec![0; self.reference.chars().count()];
                        self.pressed_vec.clear();
                        self.pos1 = 0;
                        self.words_done = 0;
                        self.popup_states.batch_size_selection.open = false;
                        self.save_config();
                        return Ok(());
                    }
                    _ => return Ok(()),
                }
            }

            if self.popup_states.top_words_selection.open {
                let schemes = TopWordsSelection::all();
                match key_event.code {
                    KeyCode::Esc => {
                        self.popup_states.top_words_selection.open = false;
                        return Ok(());
                    }
                    KeyCode::Up => {
                        self.popup_states.top_words_selection.selected -= 1;
                        if self.popup_states.top_words_selection.selected > 0 {
                        }
                        return Ok(());
                    }
                    KeyCode::Down => {
                        self.popup_states.top_words_selection.selected += 1;
                        if self.popup_states.top_words_selection.selected < TopWordsSelection::count() - 1 {
                        }
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        if self.popup_states.top_words_selection.selected < schemes.len() {
                            self.top_words = schemes[self.popup_states.top_words_selection.selected].to_words() as usize;
                        }
                        if self.time_mode {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), self.batch_size);
                        } else if self.word_mode {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), usize::min(self.batch_size, self.word_number));
                        }
                        self.is_correct = vec![0; self.reference.chars().count()];
                        self.pressed_vec.clear();
                        self.pos1 = 0;
                        self.words_done = 0;
                        self.popup_states.batch_size_selection.open = false;
                        self.save_config();
                        return Ok(());
                    }
                    _ => return Ok(()),
                }
            }

            if self.popup_states.settings.open {
                match key_event.code {
                    KeyCode::Esc => {
                        self.popup_states.settings.open = false;
                        return Ok(());
                    }
                    KeyCode::Up => {
                        if self.popup_states.settings.selected > 0 {
                            self.popup_states.settings.selected -= 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Down => {
                        if self.popup_states.settings.selected < Settings::all().len() - 1 {
                            self.popup_states.settings.selected += 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        if self.popup_states.settings.selected == 0 {
                            self.popup_states.color_scheme.open = true;
                            let schemes = ColorScheme::all();
                            self.popup_states.color_scheme.selected = schemes.iter().position(|&s| s == self.color_scheme).unwrap_or(0);
                        } else if self.popup_states.settings.selected == 1 {
                            self.popup_states.language.open = true;
                            self.popup_states.language.selected = match self.language {
                                Language::English => 0,
                                Language::Indonesian => 1,
                                Language::Italian=> 2,
                            };
                        } else if self.popup_states.settings.selected == 2 {
                            self.popup_states.batch_size_selection.open = true;
                        } else if self.popup_states.settings.selected == 3 {
                            self.popup_states.top_words_selection.open = true;
                        }
                    }
                    _ => return Ok(()),
                }
            }

            // Handle leaderboard if it's open
            if self.leaderboard.open {
                match key_event.code {
                    KeyCode::Esc => {
                        self.leaderboard.open = false;
                        return Ok(());
                    }
                    KeyCode::Up => {
                        if self.leaderboard.selected > 0 {
                            self.leaderboard.selected -= 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Down => {
                        if self.leaderboard.selected < self.leaderboard.entries.len().saturating_sub(1) {
                            self.leaderboard.selected += 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Tab => {
                        self.tab_pressed = Instant::now();
                        return Ok(());
                    }
                    KeyCode::Char('l') | KeyCode::Char('L') => {
                        if self.tab_pressed.elapsed() < Duration::from_secs(1) {
                            self.leaderboard.open = false;
                            self.tab_pressed = Instant::now() - Duration::from_secs(5);
                        }
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
                        if self.word_mode {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), usize::min(self.batch_size, self.word_number));
                        } else if self.time_mode {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), self.batch_size);
                        } else if self.quote {
                            self.reference = utils::get_random_quote();
                        } else if self.practice_mode {
                            self.reference = practice::create_words(TYPING_LEVELS[self.selected_level].1, 50);
                        } else if self.wiki_mode {
                            self.reference = utils::get_wiki_summary();
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
                        self.wiki_mode = false;
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
                                if self.menu_buttons_times.get("time").map_or(true, |&t| t.elapsed() <= Duration::from_millis(500)) {
                                    self.popup_states.time_selection.open = true;
                                }
                                self.time_mode = true;
                                self.word_mode = false;
                                self.quote = false;
                                self.practice_mode = false;
                                self.wiki_mode = false;
                                self.practice_mode = false;
                                if let Some(time) = self.menu_buttons_times.get_mut("time") {
                                    *time = Instant::now();
                                }
                            }
                            "words" => {
                                if self.menu_buttons_times.get("words").map_or(true, |&t| t.elapsed() <= Duration::from_millis(500)) {
                                    self.popup_states.word_number_selection.open = true;
                                }
                                self.time_mode = false;
                                self.word_mode = true;
                                self.wiki_mode = false;
                                self.quote = false;
                                self.practice_mode = false;
                                if let Some(time) = self.menu_buttons_times.get_mut("words") {
                                    *time = Instant::now();
                                }
                            }
                            "quote" => {
                                self.quote = true;
                                self.time_mode = false;
                                self.wiki_mode = false;
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
                            "wiki" => {
                                self.quote = false;
                                self.time_mode = false;
                                self.word_mode = false;
                                self.wiki_mode = true;
                                self.practice_mode = false;
                            }
                            "language" => {
                                self.popup_states.language.open = true;
                                self.popup_states.language.selected = match self.language {
                                    Language::English => 0,
                                    Language::Indonesian => 1,
                                    Language::Italian=> 2,
                                };
                            }
                            "theme" => {
                                self.popup_states.color_scheme.open = true;
                                let schemes = ColorScheme::all();
                                self.popup_states.color_scheme.selected = schemes.iter().position(|&s| s == self.color_scheme).unwrap_or(0);
                            }
                            "settings" => {
                                self.popup_states.settings.open = true;
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
                                self.word_number = 25;
                            }
                            "50" => {
                                self.word_number = 50;
                            }
                            "100" => {
                                self.word_number = 100;
                            }
                            _ => {}
                        }
                        if self.selected_config == "quote" {
                            self.reference = utils::get_random_quote();
                        } else if self.time_mode {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), self.batch_size);
                        } else if self.wiki_mode {
                            self.reference = utils::get_wiki_summary();
                        } else if !self.popup_states.settings.open {
                            self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), usize::min(self.batch_size, self.word_number));
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
                        self.leaderboard.open = !self.leaderboard.open;
                        self.tab_pressed = Instant::now() - Duration::from_secs(5);
                        // Reload entries when opening leaderboard
                        if self.leaderboard.open {
                            self.leaderboard.entries = crate::leaderboard::load_entries().unwrap_or_default();
                            self.leaderboard.selected = 0;
                        }
                        return Ok(());
                    }
                    
                    if self.practice_menu && ch == 'q' {
                        self.practice_menu = false;
                        self.practice_mode = false;
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
                        if (reference_chars.get(self.pos1) == Some(&' ') && !self.practice_mode) || (reference_chars.get(self.pos1) == Some(&' ') && self.is_correct[self.pos1] != -1 || self.pos1 == reference_chars.len()) {
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
                                //self.words_done += 1;
                            }
                        }
                        
                        // Only generate new reference if we haven't reached target word count yet
                        if self.time_mode || self.word_mode {
                            if self.time_mode {
                                self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), self.batch_size);
                            } else if self.word_mode {
                                self.reference = utils::get_reference(self.punctuation, self.numbers, &utils::read_first_n_words(self.top_words, self.language), usize::min(self.batch_size, self.word_number));
                            }
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
            wiki_mode: self.wiki_mode,
            batch_size: self.batch_size,
            test_time: self.test_time,
            selected_level: self.selected_level,
            language: self.language,
            color_scheme: self.color_scheme,
            word_number: self.word_number,
            top_words: self.top_words,
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
                crate::leaderboard::TestType::Word(self.word_number)
            } else if self.quote {
                crate::leaderboard::TestType::Quote
            } else if self.wiki_mode {
                crate::leaderboard::TestType::Wiki
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
                          else if self.wiki_mode { "wiki".to_string() }
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
            self.leaderboard.entries = crate::leaderboard::load_entries().unwrap_or_default();
        }
    }
}