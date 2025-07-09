use core::time;
use std::collections::VecDeque;
use macroquad::prelude::*;
use std::time::{Instant, Duration};

use crate::ui::gui::main;
use crate::{practice, utils};
use crate::practice::TYPING_LEVELS;


pub fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, radius: f32, color: Color) {
    draw_rectangle(x + radius, y, w - 2.0 * radius, h, color);
    draw_rectangle(x, y + radius, w, h - 2.0 * radius, color);

    draw_circle(x + radius, y + radius, radius, color);
    draw_circle(x + w - radius, y + radius, radius, color);
    draw_circle(x + radius, y + h - radius, radius, color);
    draw_circle(x + w - radius, y + h - radius, radius, color);
}

fn draw_toggle_button(
    x: f32,
    y: f32,
    btn_padding: f32,
    label: &str,
    font: &Option<Font>,
    is_active: bool,
    inactive_color: Color,
    visible: bool,
    font_size: u16,
    selected: bool,
) -> (bool, bool, f32) {
    if !visible {
        return (false, false, 0.0);
    }
    let padding = font_size as f32 * 0.5;
        
    let text_dims = measure_text(&label, Some(font.as_ref().unwrap()), font_size, 1.0);
    let btn_width = text_dims.width + btn_padding * 2.0;
    let btn_height = measure_text("t", font.as_ref(), font_size, 1.0).height + padding as f32 * 2.0;

    let rect = Rect::new(x, y, btn_width, btn_height);
    let (mx, my) = mouse_position();
    let hovered = rect.contains(vec2(mx, my));
    let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);

    let mut text_color = if is_active { main::MAIN_COLOR } else { inactive_color };
    let mut bg_color = Color::from_rgba(255, 0, 0, 0);
    if selected && is_active {
        text_color = macroquad::color::BLACK;
        bg_color = Color::from_rgba(150, 90, 0, 255);
    } else if selected {
        text_color = macroquad::color::BLACK;
        bg_color = Color::from_rgba(100, 60, 0, 255);
    }

    let font_size: u16 = if label == "|" {
        (font_size as f32 * 1.5) as u16
    } else {
        font_size
    };
    
    let corner_radius: f32 = font_size as f32 / 3.0;
    let mut btn_x = x;
    if screen_width() < 800.0 {
        btn_x += 4.0;
    }
    draw_rounded_rect(btn_x, y, btn_width, btn_height, corner_radius, bg_color);
    draw_text_ex(
        &label,
        x + btn_padding,
        y + btn_height - padding as f32,
        TextParams {
            font: font.as_ref(),
            font_size,
            font_scale: 1.0,
            color: text_color,
            ..Default::default()
        },
    );

    (clicked, hovered, btn_width + btn_padding * 2.0)
}

pub fn update_game_state(
    reference: &str,
    pressed_vec: &mut Vec<char>,
    is_correct: &mut VecDeque<i32>,
    pos1: &mut usize,
    timer: &mut Duration,
    start_time: &mut Instant,
    game_started: &mut bool,
    game_over: &mut bool,
    test_time: f32,
    time_mode: bool,
    words_done: &mut usize,
    errors_this_second: &mut f64,
) {
    if !*game_started && main::handle_input(reference, pressed_vec, is_correct, pos1, words_done, errors_this_second, &mut false, &mut vec![false; reference.chars().count()]) {
        *game_started = true;
        *start_time = Instant::now();
    }
    
    if *game_started && !*game_over {
        *timer = start_time.elapsed();
        if (timer.as_secs_f32() >= test_time && time_mode) || *pos1 >= reference.chars().count() {
            //*game_started = false;
            *game_over = true;
        }
    }
}

pub fn reset_game_state(
    pressed_vec: &mut Vec<char>,
    is_correct: &mut VecDeque<i32>,
    pos1: &mut usize,
    timer: &mut Duration,
    start_time: &mut Instant,
    game_started: &mut bool,
    game_over: &mut bool,
    speed_per_second: &mut Vec<f64>,
    last_recorded_time: &mut Instant,
    words_done: &mut usize,
    errors_per_second: &mut Vec<f64>,
    saved_results: &mut bool,
) {
    *is_correct = VecDeque::from(vec![0; is_correct.len()]);
    pressed_vec.clear();
    *pos1 = 0;
    *timer = Duration::new(0, 0);
    *start_time = Instant::now();
    *game_started = false;
    *game_over = false;
    *speed_per_second = vec![];
    *errors_per_second = vec![];
    *last_recorded_time = Instant::now();
    *words_done = 0;
    *saved_results = false;
}

pub fn handle_settings_buttons(
    font: &Option<Font>,
    word_list: &[String],
    punctuation: &mut bool,
    numbers: &mut bool,
    quote: &mut bool,
    time_mode: &mut bool,
    word_mode: &mut bool,
    pressed_vec: &mut Vec<char>,
    is_correct: &mut VecDeque<i32>,
    pos1: &mut usize,
    timer: &mut time::Duration,
    start_time: &mut Instant,
    game_started: &mut bool,
    game_over: &mut bool,
    reference: &mut String,
    test_time: &mut f32,
    batch_size: &mut usize,
    start_x: f32,
    speed_per_second: &mut Vec<f64>,
    last_recorded_time: &mut Instant,
    words_done: &mut usize,
    errors_per_second: &mut Vec<f64>,
    font_size: u16,
    config_opened: &mut bool,
    selected_config: &mut String,
    practice_menu: &mut bool,
    selected_practice_level: &mut Option<usize>,
    practice_mode: &mut bool,
    saved_results: &mut bool,
) -> bool {
    let inactive_color = Color::from_rgba(255, 255, 255, 80);
    let btn_y = 200.0;
    let btn_padding = font_size as f32 * 0.5;
    let divider = true;
    let mut total_width = 0.0;

    let mut button_states = vec![
        ("! punctuation", *punctuation, !*quote && !*practice_mode),
        ("# numbers", *numbers, !*quote && !*practice_mode),
        ("|", divider, true),
        ("time", *time_mode, true),
        ("words", *word_mode, true),
        ("quote", *quote, true),
        ("practice", *practice_mode, true),
        ("|", divider, true),
        ("15", test_time == &15.0, *time_mode),
        ("30", test_time == &30.0, *time_mode),
        ("60", test_time == &60.0, *time_mode),
        ("120", test_time == &120.0, *time_mode),
        ("25", *batch_size == 25, *word_mode),
        ("50", *batch_size == 50, *word_mode),
        ("100", *batch_size == 100, *word_mode),
    ];

    if is_key_down(KeyCode::Up) {
        *config_opened = true;
    } else if is_key_down(KeyCode::Down) {
        *config_opened = false;
    } else if is_key_pressed(KeyCode::Left) {
        if !*config_opened {
            return false;
        }
        for (i, (label, _state_val, visible)) in button_states.iter().enumerate() {
            if *visible && *selected_config == *label {
            let mut j = if i == 0 {
                button_states.len() - 1
            } else {
                i - 1
            };

            while j != i {
                if button_states[j].2 && button_states[j].0 != "|" {
                *selected_config = button_states[j].0.to_string();
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
        } else if is_key_pressed(KeyCode::Right) {
        if !*config_opened {
            return false;
        }
        for (i, (label, _state_val, visible)) in button_states.iter().enumerate() {
            if *visible && *selected_config == *label {
            let mut next = if i == button_states.len() - 1 {
                0
            } else {
                i + 1
            };

            while next != i {
                if button_states[next].2 && button_states[next].0 != "|" {
                *selected_config = button_states[next].0.to_string();
                break;
                }
                next = if next == button_states.len() - 1 {
                0
                } else {
                next + 1
                };
            }
            break;
            }
        }
    } else if is_key_pressed(KeyCode::Enter) && *config_opened {
        update_config(&selected_config, punctuation, numbers, time_mode, word_mode, quote, test_time, batch_size, practice_menu, selected_practice_level, practice_mode);
        if *quote {
            *reference = utils::get_random_quote();
        } else if *practice_mode {
            *reference = practice::create_words(
                &practice::TYPING_LEVELS[selected_practice_level.unwrap_or(0)].1,
                *batch_size,
            );
        } else {
            *reference = utils::get_reference(*punctuation, *numbers, word_list, *batch_size);
        }
        *is_correct = VecDeque::from(vec![0; reference.len()]);
        reset_game_state(pressed_vec, is_correct, pos1, timer, start_time, game_started, game_over, speed_per_second, last_recorded_time, words_done, errors_per_second, saved_results);
    }

    let mut any_button_hovered = false;

    for (label, state_val, visible) in button_states.iter_mut() {
        let x = start_x + total_width;
        let is_active = *state_val;
        let mut y = btn_y;
        if *label == "! punctuation" || *label == "quote" || *label == "practice" {
            y -= (measure_text("p", font.as_ref(), font_size, 1.0).height - 
                   measure_text("n", font.as_ref(), font_size, 1.0).height) / 2.0;
        }

        let (clicked, hovered, btni_width) = draw_toggle_button(
            x, 
            y,
            btn_padding,
            label,
            font, 
            is_active, 
            inactive_color,
            *visible,
            font_size,
            selected_config == label && *config_opened,
        );
        total_width += btni_width;
        
        if hovered && *label != "|" {
            any_button_hovered = true;
        }
        
        if clicked && *label != "|"  && (!is_active || (*label == "! punctuation" || *label == "# numbers")) {
            
            update_config(label, punctuation, numbers, time_mode, word_mode, quote, test_time, batch_size, practice_menu, selected_practice_level, practice_mode);
            if *quote {
                *reference = utils::get_random_quote();
                *is_correct = VecDeque::from(vec![0; reference.chars().count()]);
                *punctuation = false;
                *numbers = false;
                reset_game_state(pressed_vec, is_correct, pos1, timer, start_time, game_started, game_over, speed_per_second, last_recorded_time, words_done, errors_per_second, saved_results);
            } else if *practice_menu {
                *practice_menu = true;
            } else {
                *reference = utils::get_reference(*punctuation, *numbers, word_list, *batch_size);
                *is_correct = VecDeque::from(vec![0; reference.chars().count()]);
                reset_game_state(pressed_vec, is_correct, pos1, timer, start_time, game_started, game_over, speed_per_second, last_recorded_time, words_done, errors_per_second, saved_results);
            }
        }
    }

    any_button_hovered
}

fn update_config(label: &str, punctuation: &mut bool, numbers: &mut bool, time_mode: &mut bool, word_mode: &mut bool, quote: &mut bool, test_time: &mut f32, batch_size: &mut usize, practice_menu: &mut bool, selected_practice_level: &mut Option<usize>, practice_mode: &mut bool) {
    match label {
        "! punctuation" => {
            *punctuation = !*punctuation;
            *quote = false;
        },
        "# numbers" => {
            *numbers = !*numbers;
            *quote = false;
        },
        "time" => {
            *time_mode = true;
            *word_mode = false;
            *quote = false;
            *practice_mode = false;
        },
        "words" => {
            *word_mode = true;
            *time_mode = false;
            *quote = false;
            *practice_mode = false;
        },
        "quote" => {
            *quote = true;
            *punctuation = false;
            *numbers = false;
            *time_mode = false;
            *word_mode = false;
            *practice_mode = false;
        },
        "practice" => {
            *quote = false;
            *punctuation = false;
            *numbers = false;
            *time_mode = false;
            *word_mode = false;
            *practice_menu = true;
            for i in 0..TYPING_LEVELS.len() {
                let results_path = format!("practice_results/level_{}.txt", i + 1);
                let mut done = false;
                if let Ok(contents) = std::fs::read_to_string(&results_path) {
                    for line in contents.lines() {
                        if line.starts_with("WPM:") {
                            if let Some(wpm_str) = line.strip_prefix("WPM:").map(str::trim) {
                                if let Ok(wpm) = wpm_str.parse::<f32>() {
                                    if wpm >= practice::WPM_MIN as f32 {
                                        done = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                if !done {
                    *selected_practice_level = Some(i);
                    break;
                }
            }
        },
        "15" => {
            *test_time = 15.0;
        },
        "30" => {
            *test_time = 30.0;
        },
        "60" => {
            *test_time = 60.0;
        },
        "120" => {
            *test_time = 120.0;
        },
        "25" => {
            *batch_size = 25;
        },
        "50" => {
            *batch_size = 50;
        },
        "100" => {
            *batch_size = 100;
        },
        _ => {}
    }
}