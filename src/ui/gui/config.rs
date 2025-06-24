use core::time;
use std::collections::VecDeque;
use macroquad::prelude::*;
use std::time::{Instant, Duration};

use crate::ui::gui::main;
use crate::utils;


fn draw_toggle_button(
    x: f32,
    y: f32,
    btn_padding: f32,
    label: &str,
    font: &Option<Font>,
    is_active: bool,
    inactive_color: Color,
    visible: bool,
) -> (bool, bool, f32) {
    if !visible {
        return (false, false, 0.0);
    }
    let font_size = 22;
    let padding = 10.0;
        
    let text_dims = measure_text(&label, Some(font.as_ref().unwrap()), font_size, 1.0);
    let btn_width = text_dims.width + padding * 2.0;
    let btn_height = text_dims.height + padding * 2.0;

    let rect = Rect::new(x, y, btn_width, btn_height);
    let (mx, my) = mouse_position();
    let hovered = rect.contains(vec2(mx, my));
    let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);

    let text_color = if is_active { main::MAIN_COLOR } else { inactive_color };
    let bg_color = Color::from_rgba(0, 0, 0, 0);
    
    draw_rectangle(x, y, btn_width, btn_height, bg_color);
    draw_text_ex(
        &label,
        x + padding,
        y + btn_height - padding,
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
    is_correct: &mut VecDeque<i8>,
    pos1: &mut usize,
    timer: &mut Duration,
    start_time: &mut Instant,
    game_started: &mut bool,
    game_over: &mut bool,
    test_time: f32,
) {
    if !*game_started && main::handle_input(reference, pressed_vec, is_correct, pos1) {
        *game_started = true;
        *start_time = Instant::now();
    }
    
    if *game_started && !*game_over {
        *timer = start_time.elapsed();
        if timer.as_secs_f32() >= test_time {
            *game_over = true;
        }
    }
}

fn reset_game_state(
    pressed_vec: &mut Vec<char>,
    is_correct: &mut VecDeque<i8>,
    pos1: &mut usize,
    timer: &mut Duration,
    start_time: &mut Instant,
    game_started: &mut bool,
    game_over: &mut bool,
) {
    pressed_vec.clear();
    is_correct.clear();
    *pos1 = 0;
    *timer = Duration::new(0, 0);
    *start_time = Instant::now();
    *game_started = false;
    *game_over = false;
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
    is_correct: &mut VecDeque<i8>,
    pos1: &mut usize,
    timer: &mut time::Duration,
    start_time: &mut Instant,
    game_started: &mut bool,
    game_over: &mut bool,
    reference: &mut String,
    test_time: &mut f32,
    batch_size: &mut usize,
    start_x: f32,
) -> bool {
    let inactive_color = Color::from_rgba(255, 255, 255, 80);
    let btn_y = 200.0;
    let btn_padding = 10.0;
    let divider = true;
    let mut total_width = 0.0;

    let mut button_states = vec![
        ("! punctuation", *punctuation, true),
        ("# numbers", *numbers, true),
        ("|", divider, true),
        ("time", *time_mode, true),
        ("words", *word_mode, true),
        ("quote", *quote, true),
        ("|", divider, true),
        ("15", test_time == &15.0, *time_mode),
        ("30", test_time == &30.0, *time_mode),
        ("60", test_time == &60.0, *time_mode),
        ("120", test_time == &120.0, *time_mode),
        ("25", *batch_size == 25, *word_mode),
        ("50", *batch_size == 50, *word_mode),
        ("100", *batch_size == 100, *word_mode),
    ];

    let mut any_button_hovered = false;

    for (label, state_val, visible) in button_states.iter_mut() {
        let x = start_x + total_width;
        let is_active = *state_val;
        let (clicked, hovered, btni_width) = draw_toggle_button(
            x, 
            btn_y,
            btn_padding,
            label,
            font, 
            is_active, 
            inactive_color,
            *visible,
        );
        total_width += btni_width;
        
        if hovered {
            any_button_hovered = true;
        }
        
        if clicked {
            match *label {
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
                },
                "words" => {
                    *word_mode = true;
                    *time_mode = false;
                    *quote = false;
                },
                "quote" => {
                    *quote = true;
                    *punctuation = false;
                    *numbers = false;
                    *time_mode = false;
                    *word_mode = false;
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
            if *quote {
                *reference = utils::get_random_quote();
                *punctuation = false;
                *numbers = false;
            } else {
                *reference = utils::get_reference(*punctuation, *numbers, word_list, *batch_size);
            }
            //*start_time = Instant::now();
            //*pos1 = 0;
            reset_game_state(pressed_vec, is_correct, pos1, timer, start_time, game_started, game_over);
        }
    }

    any_button_hovered
}