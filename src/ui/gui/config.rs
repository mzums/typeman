use core::time;
use std::collections::VecDeque;
use macroquad::prelude::*;
use std::time::{Instant, Duration};

use crate::ui::gui::main;
use crate::utils;


fn draw_toggle_button(
    x: f32,
    y: f32,
    label: &str,
    font: &Option<Font>,
    is_active: bool,
    inactive_color: Color,
) -> (bool, bool) {
    let font_size = 22;
    let padding = 20.0;
        
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

    (clicked, hovered)
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

pub fn handle_settings_buttons(
    font: &Option<Font>,
    word_list: &[String],
    batch_size: usize,
    punctuation: &mut bool,
    numbers: &mut bool,
    pressed_vec: &mut Vec<char>,
    is_correct: &mut VecDeque<i8>,
    pos1: &mut usize,
    timer: &mut time::Duration,
    start_time: &mut Instant,
    game_started: &mut bool,
    game_over: &mut bool,
    reference: &mut String,
    test_time: f32
) -> bool {
    let inactive_color = Color::from_rgba(255, 255, 255, 80);
    let btn_y = 200.0;
    let btn_x1 = 50.0;
    let btn_font_size = 22;
    let btn_padding = 20.0;

    let btn1_label = "! punctuation";
    let btn1_width = measure_text(
            btn1_label,
            font.as_ref(),
            btn_font_size,
            1.0,
        ).width + btn_padding * 2.0;

    let mut button_states = vec![
        ("! punctuation", *punctuation),
        ("# numbers", *numbers),
    ];

    let mut any_button_hovered = false;

    for (i, (label, state_val)) in button_states.iter_mut().enumerate() {
        let x = btn_x1 + i as f32 * (btn1_width + 20.0);
        let is_active = *state_val;
        let (clicked, hovered) = draw_toggle_button(
            x, 
            btn_y, 
            label, 
            font, 
            is_active, 
            inactive_color,
        );
        
        if hovered {
            any_button_hovered = true;
        }
        
        if clicked {
            match *label {
                "! punctuation" => *punctuation = !*punctuation,
                "# numbers" => *numbers = !*numbers,
                _ => {}
            }
            *reference = utils::get_reference(*punctuation, *numbers, word_list, batch_size);
            update_game_state(
                &reference,
                pressed_vec,
                is_correct,
                pos1,
                timer,
                start_time,
                game_started,
                game_over,
                test_time,
            );
        }
    }

    any_button_hovered
}