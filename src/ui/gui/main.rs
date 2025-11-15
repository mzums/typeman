use core::time;
use macroquad::prelude::*;
use miniquad::CursorIcon;
use miniquad::window::set_mouse_cursor;
use std::collections::VecDeque;
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;

use crate::color_scheme::ColorScheme;
use crate::config::AppConfig;
use crate::practice::{self, TYPING_LEVELS};
use crate::ui::gui::config::{self, reset_game_state};
use crate::ui::gui::popup::{PopupStates, PopupState};
use crate::ui::gui::practice as gui_practice;
use crate::ui::gui::results;
use crate::utils;


pub const MAIN_COLOR: macroquad::color::Color =
    macroquad::color::Color::from_rgba(255, 155, 0, 255);

const ROBOTO_MONO: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/RobotoMono-VariableFont_wght.ttf"
));

const DEJAVU: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/DejaVuSansCondensed.ttf"
));

pub async fn gui_main_async() {
    let mut app_config = AppConfig::load();

    let mut punctuation = app_config.punctuation;
    let mut numbers = app_config.numbers;
    let mut quote = app_config.quote;
    let mut time_mode = app_config.time_mode;
    let mut word_mode = app_config.word_mode;
    let mut language = app_config.language;
    let mut practice_mode = app_config.practice_mode;
    let mut wiki_mode = app_config.wiki_mode;

    if !time_mode && !word_mode && !quote && !practice_mode && !wiki_mode {
        time_mode = true;
    }

    let font = load_ttf_font_from_bytes(ROBOTO_MONO).unwrap();
    let title_font = load_ttf_font_from_bytes(ROBOTO_MONO).unwrap();
    let emoji_font = load_ttf_font_from_bytes(DEJAVU).unwrap();

    let mut top_words = app_config.top_words;
    let word_list = utils::read_first_n_words(top_words as usize, language);
    let mut batch_size = app_config.batch_size;
    let mut word_number = app_config.word_number;

    let updated_word_list = utils::read_first_n_words(500, language);
    let mut selected_practice_level: Option<usize> = Some(app_config.selected_level);

    let mut reference = if practice_mode {
        practice::create_words(
            TYPING_LEVELS[selected_practice_level.unwrap_or(0)].1,
            50,
        )
    } else if quote {
        utils::get_random_quote()
    } else if wiki_mode {
        utils::get_wiki_summary()
    } else {
        utils::get_reference(punctuation, false, &updated_word_list, batch_size)
    };

    let mut pressed_vec: Vec<char> = vec![];
    let mut is_correct: VecDeque<i32> = VecDeque::from(vec![0; reference.len()]);
    let mut pos1: usize = 0;
    let mut timer = time::Duration::from_secs(0);
    let mut start_time: Instant = Instant::now();
    let mut test_time = app_config.test_time as f32;
    let mut game_started = false;
    let mut game_over = false;

    let mut speed_per_second: Vec<f64> = vec![];
    let mut errors_per_second: Vec<f64> = vec![];
    let mut errors_this_second: f64 = 0.0;
    let mut char_number = 0;
    let mut error_positions = vec![false; reference.len()];

    let mut lines: Vec<String>;
    let mut last_recorded_time = Instant::now();

    let mut words_done = 0;

    let mut config_opened = false;
    let mut selected_config: String = "time".to_string();

    let mut practice_menu = false;
    let mut scroll_offset: f32 = 0.0;
    let mut saved_results = false;

    let mut color_scheme = app_config.color_scheme;

    let mut menu_buttons_times: HashMap<String, Instant> = HashMap::from([
        ("settings".to_string(), Instant::now() - Duration::from_secs(5)),
        ("time".to_string(), Instant::now() - Duration::from_secs(5)),
        ("words".to_string(), Instant::now() - Duration::from_secs(5)),
        ("quote".to_string(), Instant::now() - Duration::from_secs(5)),
        ("practice".to_string(), Instant::now() - Duration::from_secs(5)),
        ("punctuation".to_string(), Instant::now() - Duration::from_secs(5)),
        ("numbers".to_string(), Instant::now() - Duration::from_secs(5)),
        ("wiki".to_string(), Instant::now() - Duration::from_secs(5)),
    ]);

    let mut popup_states: PopupStates = PopupStates {
        language: PopupState { visible: false, selected: 0 },
        color_scheme: PopupState { visible: false, selected: 0 },
        time_selection: PopupState { visible: false, selected: 0 },
        word_number_selection: PopupState { visible: false, selected: 0 },
        settings: PopupState { visible: false, selected: 0 },
        batch_size_selection: PopupState { visible: false, selected: 0 },
        top_words_selection: PopupState { visible: false, selected: 0 },
    };

    let words: Vec<&str> = reference.split_whitespace().collect();
    let average_word_length: f64 = if !words.is_empty() {
        words.iter().map(|w| w.len()).sum::<usize>() as f64 / words.len() as f64 + 1.0
    } else {
        5.0
    };

    loop {
        clear_background(color_scheme.bg_color());
        let mut max_width = f32::min(
            if screen_height() > screen_width() {
                screen_width() * 0.9
            } else {
                screen_width() * 0.7
            },
            1600.0,
        );
        if screen_width() < 1300.0 || screen_height() < 900.0 {
            max_width = 0.85 * screen_width();
        }
        let font_size = if screen_height() > 2000.0 || screen_width() > 3800.0 {
            40.0
        } else if screen_width() > 800.0 {
            (40.0 - (3840.0 / screen_width()) * 5.0).round()
        } else {
            20.0
        };
        let line_h = measure_text("Gy", Some(&font.clone()), font_size as u16, 1.0).height * 1.6;
        let char_w = measure_text("G", Some(&font.clone()), font_size as u16, 1.0)
            .width
            .floor();
        lines = create_lines(
            &mut reference,
            Some(font.clone()),
            font_size,
            max_width,
            quote,
            word_mode,
            wiki_mode,
        );

        let mut chars_in_line: Vec<i32> = vec![];
        for line in &lines {
            chars_in_line.push(line.chars().count() as i32);
        }

        if !game_started {
            last_recorded_time = Instant::now();
            timer = time::Duration::from_secs(0);
            start_time = Instant::now();
            pos1 = 0;
        }

        if !game_over && !practice_menu {
            let total_height = lines.len() as f32 * font_size * 1.2;
            let start_y = screen_height() / 2.0 - total_height / 2.0 + font_size;
            let start_x = screen_width() / 2.0 - max_width / 2.0 + 20.0;
            let title_y = screen_height() / 7.5;

            draw_reference_text(
                &lines,
                &pressed_vec,
                &is_correct,
                Some(&font.clone()),
                font_size,
                start_x,
                start_y,
                popup_states.language.visible,
                &color_scheme,
            );

            let any_button_hovered = config::handle_settings_buttons(
                &Option::Some(font.clone()),
                &emoji_font,
                &word_list,
                &mut punctuation,
                &mut numbers,
                &mut quote,
                &mut time_mode,
                &mut word_mode,
                &mut pressed_vec,
                &mut is_correct,
                &mut pos1,
                &mut timer,
                &mut start_time,
                &mut game_started,
                &mut game_over,
                &mut reference,
                &mut test_time,
                &mut batch_size,
                start_x,
                &mut speed_per_second,
                &mut last_recorded_time,
                &mut words_done,
                &mut errors_per_second,
                u16::max((font_size / 1.2) as u16, 15),
                &mut config_opened,
                &mut selected_config,
                &mut practice_menu,
                &mut selected_practice_level,
                &mut practice_mode,
                &mut saved_results,
                &mut error_positions,
                &mut language,
                &mut color_scheme,
                &mut wiki_mode,
                &mut menu_buttons_times,
                &mut popup_states,
                &mut top_words,
                &mut word_number,
           );

            set_mouse_cursor(if any_button_hovered {
                CursorIcon::Pointer
            } else {
                CursorIcon::Default
            });

            config::update_game_state(
                &reference,
                &mut pressed_vec,
                &mut is_correct,
                &mut pos1,
                &mut timer,
                &mut start_time,
                &mut game_started,
                &mut game_over,
                test_time,
                time_mode,
                &mut words_done,
                &mut errors_this_second,
                &mut practice_mode,
                practice_menu,
                wiki_mode,
                quote,
                word_number,
            );

            if !game_started
                && handle_input(
                    &reference,
                    &mut pressed_vec,
                    &mut is_correct,
                    &mut pos1,
                    &mut words_done,
                    &mut errors_this_second,
                    &mut config_opened,
                    &mut error_positions,
                    practice_mode,
                    practice_menu,
                    game_over,
                )
            {
                game_started = true;
            }

            if (game_started || words_done == word_number) && !game_over {
                timer = start_time.elapsed();
                if (timer.as_secs_f32() >= test_time && time_mode) || (pos1 >= reference.chars().count() && (wiki_mode || quote)) || (words_done >= word_number && !wiki_mode && !quote)
                {
                    game_over = true;
                }
            }

            write_title(
                Some(title_font.clone()),
                if screen_height() > 1000.0 && screen_width() > 800.0 {
                    50.0
                } else {
                    30.0
                },
                start_x,
                title_y,
                color_scheme,
            );

            if !game_over {
                handle_input(
                    &reference,
                    &mut pressed_vec,
                    &mut is_correct,
                    &mut pos1,
                    &mut words_done,
                    &mut errors_this_second,
                    &mut config_opened,
                    &mut error_positions,
                    practice_mode,
                    practice_menu,
                    game_over,
                );
            }

            if time_mode {
                draw_timer(
                    Some(&font.clone()),
                    font_size,
                    start_x,
                    start_y,
                    timer,
                    test_time,
                    &color_scheme,
                );
            } else if word_mode {
                draw_word_count(
                    Some(&font.clone()),
                    font_size,
                    start_x,
                    start_y,
                    &mut words_done,
                    word_number,
                    &color_scheme,
                );
            } else if practice_mode {
                draw_word_count(
                    Some(&font.clone()),
                    font_size,
                    start_x,
                    start_y,
                    &mut words_done,
                    50,
                    &color_scheme,
                );
            } else if quote || wiki_mode{
                draw_word_count(
                    Some(&font.clone()),
                    font_size,
                    start_x,
                    start_y,
                    &mut words_done,
                    reference.split_whitespace().count(),
                    &color_scheme,
                );
            }

            let (calc_pos_x, calc_pos_y) = calc_pos(&chars_in_line, pos1);
            if !game_started {
                let blink_interval = 0.5;
                let show_cursor = ((get_time() / blink_interval) as i32) % 2 == 0;
                if show_cursor && !game_over && !config_opened {
                    draw_cursor(
                        calc_pos_x,
                        calc_pos_y,
                        start_x,
                        start_y,
                        line_h,
                        char_w,
                        &color_scheme,
                    );
                }
            } else {
                draw_cursor(
                    calc_pos_x,
                    calc_pos_y,
                    start_x,
                    start_y,
                    line_h,
                    char_w,
                    &color_scheme,
                );
            }

            let now = Instant::now();
            let time_since_last = now.duration_since(last_recorded_time);

            if time_since_last >= Duration::from_secs(1) {
                let total_typed = pressed_vec.len();
                let chars_in_this_second = total_typed.saturating_sub(char_number);
                let cpm = chars_in_this_second as f64 * 60.0;

                speed_per_second.push(cpm);

                char_number = total_typed;

                errors_per_second.push(errors_this_second);
                errors_this_second = 0.0;
                last_recorded_time += Duration::from_secs(1);
            }
        } else if game_over {
            handle_input(
                &reference,
                &mut pressed_vec,
                &mut is_correct,
                &mut pos1,
                &mut words_done,
                &mut errors_this_second,
                &mut config_opened,
                &mut error_positions,
                practice_mode,
                practice_menu,
                game_over,
            );
            let mode = if time_mode {
                "time".to_string()
            } else if word_mode {
                "word".to_string()
            } else if quote {
                "quote".to_string()
            } else if wiki_mode {
                "wiki".to_string()
            } else {
                "practice".to_string()
            };

            let practice_level = if !practice_mode {
                None
            } else {
                selected_practice_level
            };

            results::write_results(
                &is_correct,
                screen_width(),
                screen_height(),
                Some(&title_font.clone()),
                timer.as_secs_f32(),
                &speed_per_second,
                average_word_length,
                &mode,
                punctuation,
                numbers,
                &errors_per_second,
                &reference,
                practice_level,
                &mut saved_results,
                &color_scheme,
            );
        } else if practice_menu {
            let level = gui_practice::display_practice_menu(
                Some(font.clone()),
                &mut scroll_offset,
                emoji_font.clone(),
                &mut selected_practice_level,
                &mut practice_menu,
                &mut time_mode,
                &mut pressed_vec,
                &mut is_correct,
                &mut pos1,
                &mut timer,
                &mut start_time,
                &mut game_started,
                &mut game_over,
                &mut speed_per_second,
                &mut last_recorded_time,
                &mut words_done,
                &mut errors_per_second,
                &mut saved_results,
                &mut error_positions,
                &color_scheme,
            );
            if level.is_some() {
                config::reset_game_state(
                    &mut pressed_vec,
                    &mut is_correct,
                    &mut pos1,
                    &mut timer,
                    &mut start_time,
                    &mut game_started,
                    &mut game_over,
                    &mut speed_per_second,
                    &mut last_recorded_time,
                    &mut words_done,
                    &mut errors_per_second,
                    &mut saved_results,
                    &mut error_positions,
                );
                reference = practice::create_words(TYPING_LEVELS[level.unwrap()].1, 50);
                is_correct = VecDeque::from(vec![0; reference.len()]);
                error_positions = vec![false; is_correct.len()];
                practice_mode = true;
                wiki_mode = false;
                time_mode = false;
                word_mode = false;
                quote = false;
                practice_menu = false;
                config_opened = false;
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            if practice_menu {
                practice_menu = false;
                practice_mode = false;
                game_over = false;

                reset_game_state(
                    &mut pressed_vec,
                    &mut is_correct,
                    &mut pos1,
                    &mut timer,
                    &mut start_time,
                    &mut game_started,
                    &mut game_over,
                    &mut speed_per_second,
                    &mut last_recorded_time,
                    &mut words_done,
                    &mut errors_per_second,
                    &mut saved_results,
                    &mut error_positions,
                );
                reset_game_state(
                    &mut pressed_vec,
                    &mut is_correct,
                    &mut pos1,
                    &mut timer,
                    &mut start_time,
                    &mut game_started,
                    &mut game_over,
                    &mut speed_per_second,
                    &mut last_recorded_time,
                    &mut words_done,
                    &mut errors_per_second,
                    &mut saved_results,
                    &mut error_positions,
                );
            } else if popup_states.language.visible {
                popup_states.language.visible = false;
                config_opened = false;
            } else if popup_states.color_scheme.visible {
                popup_states.color_scheme.visible = false;
                config_opened = false;
            } else if popup_states.time_selection.visible {
                popup_states.time_selection.visible = false;
                config_opened = false;
            } else if popup_states.word_number_selection.visible {
                popup_states.word_number_selection.visible = false;
                config_opened = false;
            } else if popup_states.batch_size_selection.visible {
                popup_states.batch_size_selection.visible = false;
                config_opened = false;
            } else if popup_states.top_words_selection.visible {
                popup_states.top_words_selection.visible = false;
                config_opened = false;
            } else if popup_states.settings.visible {
                popup_states.settings.visible = false;
                config_opened = false;
            } else {
                app_config = AppConfig {
                    punctuation: punctuation,
                    numbers: numbers,
                    time_mode: time_mode,
                    word_mode: word_mode,
                    quote: quote,
                    practice_mode: practice_mode,
                    wiki_mode: wiki_mode,
                    batch_size: batch_size,
                    test_time: test_time,
                    selected_level: selected_practice_level.unwrap_or(0),
                    language: language,
                    color_scheme: color_scheme,
                    word_number: word_number,
                    top_words: top_words,
                };
                let _ = app_config.save();

                break;
            }
        }

        if is_key_down(KeyCode::Tab) && is_key_down(KeyCode::Enter) && !practice_menu {
            config::reset_game_state(
                &mut pressed_vec,
                &mut is_correct,
                &mut pos1,
                &mut timer,
                &mut start_time,
                &mut game_started,
                &mut game_over,
                &mut speed_per_second,
                &mut last_recorded_time,
                &mut words_done,
                &mut errors_per_second,
                &mut saved_results,
                &mut error_positions,
            );
            if practice_mode {
                reference = practice::create_words(
                    TYPING_LEVELS[selected_practice_level.unwrap_or(0)].1,
                    50,
                );
            } else if quote {
                reference = utils::get_random_quote();
            } else if wiki_mode {
                reference = utils::get_wiki_summary();
            } else {
                let updated_word_list = utils::read_first_n_words(500, language);
                reference =
                    utils::get_reference(punctuation, false, &updated_word_list, batch_size);
            }
            is_correct = VecDeque::from(vec![0; reference.len()]);
            error_positions = vec![false; is_correct.len()];
            thread::sleep(time::Duration::from_millis(80));
        }

        if pos1 >= reference.chars().count() && (time_mode || word_mode) && !game_over {
            words_done += 1;
            reference = utils::get_reference(
                punctuation,
                numbers,
                &utils::read_first_n_words(500, language),
                batch_size,
            );
            is_correct = VecDeque::from(vec![0; reference.len()]);
            error_positions = vec![false; is_correct.len()];
            pos1 = 0;
        }

        draw_shortcut_info(
            Some(&font.clone()),
            f32::max(font_size / 1.7, 11.0),
            screen_width() / 2.0 - max_width / 2.0,
            screen_height() - screen_height() / 7.5,
            emoji_font.clone(),
            practice_menu,
            game_over,
            practice_mode,
            &color_scheme,
        );
        next_frame().await;
    }
}

fn write_title(font: Option<Font>, font_size: f32, x: f32, y: f32, color_scheme: ColorScheme) {
    let (type_text, man_text) = ("Type", "Man");
    let type_width = measure_text(type_text, font.as_ref(), font_size as u16, 1.0).width;

    let type_color = if color_scheme == ColorScheme::Light {
        color_scheme.dimmer_main()
    } else {
        color_scheme.main_color()
    };
    let man_color = if color_scheme == ColorScheme::Light {
        color_scheme.border_color()
    } else {
        Color::from_rgba(255, 255, 255, 220)
    };

    for (text, color, dx) in [
        (type_text, type_color, 0.0),
        (man_text, man_color, type_width),
    ] {
        draw_text_ex(
            text,
            x + dx,
            y,
            TextParams {
                font: font.as_ref(),
                font_size: font_size as u16,
                color,
                ..Default::default()
            },
        );
    }
}

fn draw_shortcut_info(
    font: Option<&Font>,
    font_size: f32,
    x: f32,
    y: f32,
    emoji_font: Font,
    practice_menu: bool,
    game_over: bool,
    practice_mode: bool,
    color_scheme: &ColorScheme,
) {
    let mut x = if practice_menu { 200.0 } else { x };
    let mut next_y = y;
    let lines = if practice_menu {
        let text_w = measure_text(
            "↑ or ↓ to navigate to config, ← → to change settings, ↵ - apply config (or click)",
            font,
            font_size as u16,
            1.0,
        )
        .width;
        x = screen_width() - text_w - 70.0;

        vec![
            "↑ or ↓ to navigate, ↵ to select (or click)",
            "+ - double Enter to view more options"
        ]
    } else if practice_mode {
        vec![
            "↑ or ↓ to navigate to config, ← → to change settings, ↵ - apply config (or click)",
            "+ - double Enter to view more options",
            "Tab + Enter - reset",
        ]
    } else if game_over {
        x /= 2.0;
        vec!["Tab + Enter - reset"]
    } else {
        vec![
            "↑ or ↓ to navigate to config, ← → to change settings, ↵ - apply config (or click)",
            "+ - double Enter to view more options",
            "Tab + Enter - reset",
        ]
    };

    fn is_emoji_char(c: char) -> bool {
        matches!(c, '↑' | '↓' | '←' | '→' | '↵')
    }

    for line in lines.iter() {
        let mut curr_x = x;
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.peek() {
            if is_emoji_char(*c) {
                let mut emoji_str = String::new();
                while let Some(&ec) = chars.peek() {
                    if is_emoji_char(ec) {
                        emoji_str.push(ec);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let dims = measure_text(&emoji_str, Some(&emoji_font), font_size as u16, 1.0);
                draw_text_ex(
                    &emoji_str,
                    curr_x,
                    next_y,
                    TextParams {
                        font: Some(&emoji_font),
                        font_size: font_size as u16,
                        color: color_scheme.ref_color(),
                        ..Default::default()
                    },
                );
                curr_x += dims.width;
            } else {
                let mut normal_str = String::new();
                while let Some(&nc) = chars.peek() {
                    if !is_emoji_char(nc) {
                        normal_str.push(nc);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let dims = measure_text(&normal_str, font, font_size as u16, 1.0);
                draw_text_ex(
                    &normal_str,
                    curr_x,
                    next_y,
                    TextParams {
                        font,
                        font_size: font_size as u16,
                        color: color_scheme.ref_color(),
                        ..Default::default()
                    },
                );
                curr_x += dims.width;
            }
        }
        next_y += font_size * 1.5;
    }
    draw_text_ex(
        "Esc - quit",
        x,
        next_y,
        TextParams {
            font,
            font_size: font_size as u16,
            color: color_scheme.ref_color(),
            ..Default::default()
        },
    );
}

pub fn create_lines(
    reference: &mut String,
    font: Option<Font>,
    font_size: f32,
    max_width: f32,
    quote: bool,
    word_mode: bool,
    wiki_mode: bool,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut no_words = 0;
    let words: Vec<&str> = reference.split_whitespace().collect();
    for word in words.iter() {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        let dims = measure_text(&test_line, font.as_ref(), font_size as u16, 1.0);
        if dims.width > max_width && !current_line.is_empty() {
            current_line += " ";
            lines.push(current_line.clone());
            if lines.len() >= 5 && !quote && !word_mode && !wiki_mode {
                if no_words < words.len() {
                    let char_indices = reference.char_indices();
                    let mut end_idx = 0;
                    let mut word_count = 0;
                    for (idx, c) in char_indices {
                        if c.is_whitespace() {
                            word_count += 1;
                            if word_count == no_words {
                                end_idx = idx;
                                break;
                            }
                        }
                    }
                    if end_idx == 0 {
                        end_idx = reference.len();
                    }
                    reference.truncate(end_idx);
                }
                return lines;
            }
            current_line = word.to_string();
        } else {
            current_line = test_line;
        }
        no_words += 1;
        if lines.len() >= 5 && !quote && !word_mode && !wiki_mode {
            break;
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    if let Some(last) = lines.last_mut() {
        *last = last.trim_end().to_string();
    }
    lines
}

pub fn handle_input(
    reference: &str,
    pressed_vec: &mut Vec<char>,
    is_correct: &mut VecDeque<i32>,
    pos1: &mut usize,
    words_done: &mut usize,
    errors_this_second: &mut f64,
    config_opened: &mut bool,
    error_positions: &mut Vec<bool>,
    practice_mode: bool,
    practice_menu: bool,
    game_over: bool,
) -> bool {
    let pressed = get_char_pressed();
    if let Some(ch) = pressed {
        if game_over {
            return true;
        }
        if ch == '\u{f700}'
            || ch == '\u{f701}'
            || ch == '\u{f702}'
            || ch == '\u{f703}'
            || ch == '\u{f704}'
            || ch == '\u{f705}'
            || ch == '\u{1b}' // esc
        {
            // Arrow keys
            return false;
        }
        *config_opened = false;
        if ch == '\t' || ch == '\n' || ch == '\r' {
            return false;
        }
        if ch == '\u{8}' {
            // Backspace
            if !pressed_vec.is_empty() && reference.chars().nth(*pos1) == Some(' ') {
                *words_done -= 1;
            }
            pressed_vec.pop();
            if *pos1 > 0 {
                *pos1 -= 1;
            }
        } else if ch == '\u{7f}' {
            // Delete
            return false;
        } else {
            if ch == 'q' && practice_menu {
                return false;
            }
            let ref_char: Option<char> = reference.chars().nth(*pos1);
            if is_correct.len() > *pos1
                && ref_char == Some(ch)
                && is_correct[*pos1] != -1
                && is_correct[*pos1] != 1
            {
                is_correct[*pos1] = 2; // Correct
            } else if is_correct.len() > *pos1
                && ref_char == Some(ch)
                && (is_correct[*pos1] == -1 || is_correct[*pos1] == 1)
            {
                is_correct[*pos1] = 1; // Corrected
            } else if is_correct.len() > *pos1 {
                is_correct[*pos1] = -1; // Incorrect
                error_positions[*pos1] = true;
                *errors_this_second += 1.0;
            }
            if practice_mode && is_correct.len() > *pos1 && is_correct[*pos1] == -1 {
                return true;
            }
            *pos1 += 1;
            pressed_vec.push(ch);
            if reference.chars().nth(*pos1) == Some(' ') {
                *words_done += 1;
            }
        }
        return true;
    }
    false
}

fn draw_timer(
    font: Option<&Font>,
    font_size: f32,
    start_x: f32,
    start_y: f32,
    timer: time::Duration,
    test_time: f32,
    color_scheme: &ColorScheme,
) {
    let timer_str = format!("{:.0}", test_time - timer.as_secs_f32());
    draw_text_ex(
        &timer_str,
        start_x,
        start_y - 2.0 * font_size,
        TextParams {
            font,
            font_size: font_size as u16,
            color: color_scheme.main_color(),
            ..Default::default()
        },
    );
}

fn draw_word_count(
    font: Option<&Font>,
    font_size: f32,
    start_x: f32,
    start_y: f32,
    words_done: &mut usize,
    total_words: usize,
    color_scheme: &ColorScheme,
) {
    let timer_str = format!("{}/{}", words_done, total_words);
    draw_text_ex(
        &timer_str,
        start_x,
        start_y - screen_height() / 20.0,
        TextParams {
            font,
            font_size: font_size as u16,
            color: color_scheme.main_color(),
            ..Default::default()
        },
    );
}

fn draw_reference_text(
    lines: &[String],
    pressed_vec: &[char],
    is_correct: &VecDeque<i32>,
    font: Option<&Font>,
    font_size: f32,
    start_x: f32,
    start_y: f32,
    _lang_popup_open: bool,
    color_scheme: &ColorScheme,
) {
    let mut pos = 0;
    let mut pos_y = 0.0;

    for line in lines.iter() {
        let mut pos_x = 0;
        for char in line.chars() {
            let mut curr_char = char;
            let color = if pos + 1 > pressed_vec.len() || is_correct[pos] == 0 {
                color_scheme.ref_color()
            } else if is_correct.get(pos).is_some() && is_correct[pos] == 2 {
                color_scheme.text_color()
            } else if is_correct.get(pos).is_some() && is_correct[pos] == 1 {
                if char == ' ' {
                    curr_char = '_';
                }
                color_scheme.corrected_color()
            } else {
                if char == ' ' {
                    curr_char = '_';
                }
                color_scheme.incorrect_color()
            };
            draw_text_ex(
                &curr_char.to_string(),
                pos_x as f32 + start_x,
                pos_y + start_y,
                TextParams {
                    font,
                    font_size: font_size as u16,
                    color,
                    font_scale: 1.0,
                    ..Default::default()
                },
            );
            let type_width = measure_text(&char.to_string(), font, font_size as u16, 1.0).width;
            pos_x += type_width as usize;
            pos += 1;
        }
        let type_height = measure_text("Gy", font, font_size as u16, 1.0).height;
        pos_y += type_height * 1.6;
    }
}

fn draw_cursor(
    cursor_x: usize,
    cursor_y: usize,
    start_x: f32,
    start_y: f32,
    line_h: f32,
    char_w: f32,
    color_scheme: &ColorScheme,
) {
    let cursor_x = start_x + cursor_x as f32 * char_w;
    let cursor_y = start_y + cursor_y as f32 * line_h;
    draw_line(
        cursor_x,
        cursor_y - line_h * 0.7,
        cursor_x,
        cursor_y + line_h * 0.3,
        2.0,
        color_scheme.main_color(),
    );
}

fn calc_pos(chars_in_line: &[i32], pos1: usize) -> (usize, usize) {
    let mut total = 0;
    for (i, &count) in chars_in_line.iter().enumerate() {
        if pos1 < total + count as usize {
            return (pos1 - total, i);
        }
        total += count as usize;
    }
    if let Some((i, &count)) = chars_in_line.iter().enumerate().next_back() {
        return (count as usize, i);
    }
    (0, 0)
}
