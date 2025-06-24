use core::time;
use std::collections::VecDeque;
use macroquad::prelude::*;
use miniquad::window::set_mouse_cursor;
use miniquad::CursorIcon;
use std::time::Instant;

use crate::utils;
use crate::ui::gui::results;
use crate::ui::gui::config;


pub const MAIN_COLOR: Color = Color::from_rgba(255, 155, 0, 255);

fn write_title(font: Option<Font>, font_size: f32, x: f32, y: f32) {
    let (type_text, man_text) = ("Type", "Man");
    let type_width = measure_text(type_text, font.as_ref(), font_size as u16, 1.0).width;
    
    for (text, color, dx) in [
        (type_text, MAIN_COLOR, 0.0),
        (man_text, Color::from_rgba(255, 255, 255, 220), type_width),
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
    
pub fn create_lines(reference: &str, font: Option<Font>, font_size: f32, max_width: f32, quote: bool, word_mode: bool) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    for word in reference.split_whitespace() {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };    
        let dims = measure_text(&test_line, font.as_ref(), font_size as u16, 1.0);
        if dims.width > max_width && !current_line.is_empty() {
            current_line += " ";
            lines.push(current_line.clone());
            if lines.len() >= 4 && !quote && !word_mode {
                return lines;
            }
            current_line = word.to_string();
        } else {
            current_line = test_line;
        }    
        if lines.len() >= 4 && !quote && !word_mode  {
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
    
async fn load_font_async(path: &str) -> Option<Font> {
    match load_ttf_font(path).await {
        Ok(f) => Some(f),
        Err(_) => None,
    }
}
    
pub fn handle_input(
    reference: &str,
    pressed_vec: &mut Vec<char>,
    is_correct: &mut VecDeque<i8>,
    pos1: &mut usize,
) -> bool {
    let pressed = get_char_pressed();
    if let Some(ch) = pressed {
        if ch == '\u{8}' {
            pressed_vec.pop();
            if *pos1 > 0 {
                *pos1 -= 1;
            }
        } else {
            let ref_char = reference.chars().nth(*pos1);
            if ref_char == Some(ch) && is_correct[*pos1] != -1 && is_correct[*pos1] != 1 {
                is_correct[*pos1] = 2; // Correct
            } else if ref_char == Some(ch) && is_correct[*pos1] == -1 {
                is_correct[*pos1] = 1; // Corrected
            } else {
                is_correct[*pos1] = -1; // Incorrect
            }
            *pos1 += 1;
            pressed_vec.push(ch);
        }
        return true;
    }
    false
}
    
fn draw_timer(font: Option<&Font>, font_size: f32, start_x: f32, start_y: f32, timer: time::Duration, test_time: f32) {
    let timer_str = format!("{:.0}", test_time - timer.as_secs_f32());
    draw_text_ex(
        &timer_str,
        start_x,
        start_y - screen_height() / 20.0,
        TextParams {
            font,
            font_size: font_size as u16,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
}
    
fn draw_reference_text(
    lines: &[String],
    pressed_vec: &[char],
    is_correct: &VecDeque<i8>,
    font: Option<&Font>,
    font_size: f32,
    start_x: f32,
    start_y: f32,
) {
    let mut pos = 0;
    let mut pos_y = 0.0;

    for line in lines.iter() {
        let mut pos_x = 0;
        for char in line.chars() {
            let mut curr_char = char;
            let color = if pos + 1 > pressed_vec.len() || is_correct[pos] == 0 {
                Color::from_rgba(255, 255, 255, 80)
            } else if is_correct.get(pos).is_some() && is_correct[pos] == 2 {
                Color::from_rgba(255, 255, 255, 200)
            } else if is_correct.get(pos).is_some() && is_correct[pos] == 1 {
                if char == ' ' {
                    curr_char = '_';
                }
                Color::from_rgba(255, 165, 0, 255)
            } else {
                if char == ' ' {
                    curr_char = '_';
                }
                Color::from_rgba(255, 50, 50, 180)
            };
            draw_text_ex(
                &curr_char.to_string(),
                pos_x as f32 + start_x,
                pos_y + start_y,
                TextParams {
                    font,
                    font_size: font_size as u16,
                    color,
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

pub async fn gui_main_async() {
    let mut punctuation = false;
    let mut numbers = false;
    let mut quote = false;
    let mut time_mode = true;
    let mut word_mode = false;

    let font = load_font_async("assets/font/RobotoMono-VariableFont_wght.ttf").await;
    let title_font = load_font_async("assets/font/static/RobotoMono-Medium.ttf").await;

    let top_words = 500;
    let word_list = utils::read_first_n_words(top_words as usize);
    let mut batch_size = 100;
    if screen_height() > screen_width() {
        batch_size = 50;
    }
    let mut reference = utils::get_reference(punctuation, false, &word_list, batch_size);

    let mut pressed_vec: Vec<char> = vec![];
    let mut is_correct: VecDeque<i8> = VecDeque::from(vec![0; reference.len()]);
    let mut pos1: usize = 0;
    let mut timer = time::Duration::from_secs(0);
    let mut start_time: Instant = Instant::now();
    let mut test_time = 15.0;
    let mut game_started = false;
    let mut game_over = false;

    let mut lines: Vec<String>;

    loop {
        clear_background(Color::from_rgba(20, 17, 15, 255));
        let max_width = f32::min(screen_width() * 0.9, 1700.0);
        let font_size = 40.0;
        lines = create_lines(&reference, font.clone(), font_size, max_width, quote, word_mode);
        
        let any_button_hovered = config::handle_settings_buttons(
            &font,
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
            screen_width() / 2.0 - max_width / 2.0,
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
        );


        if !game_started && handle_input(&reference, &mut pressed_vec, &mut is_correct, &mut pos1) {
            game_started = true;
            start_time = Instant::now();
        }
        
        if game_started && !game_over {
            timer = start_time.elapsed();
            if timer.as_secs_f32() >= test_time {
                game_over = true;
            }
        }

        if !game_over {
            
    
            let total_height = lines.len() as f32 * font_size * 1.2;
            let start_y = screen_height() / 2.0 - total_height / 2.0 + font_size;
            let start_x = screen_width() / 2.0 - max_width / 2.0;
    
            write_title(
                title_font.clone(),
                50.0,
                start_x,
                screen_height() / 15.0,
            );
    
            handle_input(&reference, &mut pressed_vec, &mut is_correct, &mut pos1);
    
            if time_mode {
                draw_timer(font.as_ref(), font_size, start_x, start_y, timer, test_time);
            }
    
            draw_reference_text(
                &lines,
                &pressed_vec,
                &is_correct,
                font.as_ref(),
                font_size,
                start_x,
                start_y,
            );    
    
            if is_key_down(KeyCode::Escape) {
                break;
            }
        }  
        else if game_over {
            results::write_results(
                &is_correct,
                &pressed_vec,
                screen_width(),
                screen_height(),
                &reference,
                font.as_ref(),
                test_time,
                40.0,
            );
        }
        next_frame().await;
    }
}