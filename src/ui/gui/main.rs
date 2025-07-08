use core::time;
use std::collections::VecDeque;
use macroquad::prelude::*;
use miniquad::window::set_mouse_cursor;
use miniquad::CursorIcon;
use std::time::{Duration, Instant};
use std::thread;

use crate::utils;
use crate::ui::gui::results;
use crate::ui::gui::config;
use crate::ui::gui::practice as gui_practice;
use crate::practice::{self, TYPING_LEVELS};


pub const MAIN_COLOR: macroquad::color::Color = macroquad::color::Color::from_rgba(255, 155, 0, 255);

fn write_title(font: Option<Font>, font_size: f32, x: f32, y: f32) {
    let (type_text, man_text) = ("Type", "Man");
    let type_width = measure_text(type_text, font.as_ref(), font_size as u16, 1.0).width;

    for (text, color, dx) in [
        (type_text, MAIN_COLOR, 0.0),
        (man_text, macroquad::color::Color::from_rgba(255, 255, 255, 220), type_width),
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
) {
    let mut x = x;
    let mut next_y = y;
    let lines = if practice_menu {
        x = screen_width() - measure_text("↑ or ↓ to navigate, ↵ to select (or click)", font, font_size as u16, 1.0).width - 70.0;
        vec![
            "↑ or ↓ to navigate, ↵ to select (or click)",
            "Tab + Enter - quit menu",
        ]
    } else if game_over {
        vec!["Tab + Enter - reset"]
    } else {
        vec![
            "↑ to navigate to config, ← → to change settings (or click)",
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
                        color: Color::from_rgba(255, 255, 255, 80),
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
                        color: Color::from_rgba(255, 255, 255, 80),
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
            color: Color::from_rgba(255, 255, 255, 80),
            ..Default::default()
        },
    );
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
    is_correct: &mut VecDeque<i32>,
    pos1: &mut usize,
    words_done: &mut usize,
    errors_this_second: &mut f64,
    config_opened: &mut bool,
) -> bool {
    let pressed = get_char_pressed();
    if let Some(ch) = pressed {
        *config_opened = false;
        if ch == '\t' || ch == '\n' || ch == '\r' {
            return false;
        }
        if ch == '\u{8}' {
            if !pressed_vec.is_empty() && reference.chars().nth(*pos1) == Some(' ') {
                *words_done -= 1;
            }
            pressed_vec.pop();
            if *pos1 > 0 {
                *pos1 -= 1;
            }
        } else {
            let ref_char: Option<char> = reference.chars().nth(*pos1);
            if is_correct.len() > *pos1 && ref_char == Some(ch) && is_correct[*pos1] != -1 && is_correct[*pos1] != 1 {
                is_correct[*pos1] = 2; // Correct
            } else if ref_char == Some(ch) && is_correct[*pos1] == -1 {
                is_correct[*pos1] = 1; // Corrected
            } else {
                is_correct[*pos1] = -1; // Incorrect
                *errors_this_second += 1.0;
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
    
fn draw_timer(font: Option<&Font>, font_size: f32, start_x: f32, start_y: f32, timer: time::Duration, test_time: f32) {
    let timer_str = format!("{:.0}", test_time - timer.as_secs_f32());
    draw_text_ex(
        &timer_str,
        start_x,
        start_y - 2.0 * font_size,
        TextParams {
            font,
            font_size: font_size as u16,
            color: macroquad::color::Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
}

fn draw_word_count(font: Option<&Font>, font_size: f32, start_x: f32, start_y: f32, words_done: &mut usize, total_words: usize) {
    let timer_str = format!("{}/{}", words_done, total_words);
    draw_text_ex(
        &timer_str,
        start_x,
        start_y - screen_height() / 20.0,
        TextParams {
            font,
            font_size: font_size as u16,
            color: macroquad::color::Color::from_rgba(255, 155, 0, 255),
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
) {
    let mut pos = 0;
    let mut pos_y = 0.0;

    for line in lines.iter() {
        let mut pos_x = 0;
        for char in line.chars() {
            let mut curr_char = char;
            let color = if pos + 1 > pressed_vec.len() || is_correct[pos] == 0 {
                macroquad::color::Color::from_rgba(255, 255, 255, 80)
            } else if is_correct.get(pos).is_some() && is_correct[pos] == 2 {
                macroquad::color::Color::from_rgba(255, 255, 255, 200)
            } else if is_correct.get(pos).is_some() && is_correct[pos] == 1 {
                if char == ' ' {
                    curr_char = '_';
                }
                macroquad::color::Color::from_rgba(255, 165, 0, 255)
            } else {
                if char == ' ' {
                    curr_char = '_';
                }
                macroquad::color::Color::from_rgba(255, 50, 50, 180)
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

fn draw_cursor(cursor_x: usize, cursor_y: usize, start_x: f32, start_y: f32, line_h: f32, char_w: f32) {
    let cursor_x = start_x + cursor_x as f32 * char_w;
    let cursor_y = start_y + cursor_y as f32 * line_h;
    draw_line(cursor_x, cursor_y - line_h * 0.7, cursor_x, cursor_y + line_h * 0.3, 2.0, macroquad::color::Color::from_rgba(255, 155, 0, 255));
}

fn calc_pos(chars_in_line: &[i32], pos1: usize) -> (usize, usize) {
    let mut total = 0;
    for (i, &count) in chars_in_line.iter().enumerate() {
        if pos1 < total + count as usize {
            return (pos1 - total, i);
        }
        total += count as usize;
    }
    if let Some((i, &count)) = chars_in_line.iter().enumerate().last() {
        return (count as usize, i);
    }
    (0, 0)
}

pub async fn gui_main_async() {
    let mut punctuation = false;
    let mut numbers = false;
    let mut quote = false;
    let mut time_mode = true;
    let mut word_mode = false;

    let font = load_font_async("assets/fonts/RobotoMono-VariableFont_wght.ttf").await;
    let title_font = load_font_async("assets/fonts/RobotoMono-VariableFont_wght.ttf").await;
    let emoji_font = load_font_async("assets/fonts/DejaVuSansCondensed.ttf").await;

    let top_words = 500;
    let word_list = utils::read_first_n_words(top_words as usize);
    let mut batch_size = 100;
    if screen_height() > screen_width() {
        batch_size = 50;
    }
    let mut reference = utils::get_reference(punctuation, false, &word_list, batch_size);

    let mut pressed_vec: Vec<char> = vec![];
    let mut is_correct: VecDeque<i32> = VecDeque::from(vec![0; reference.len()]);
    let mut pos1: usize = 0;
    let mut timer = time::Duration::from_secs(0);
    let mut start_time: Instant = Instant::now();
    let mut test_time = 5.0;
    let mut game_started = false;
    let mut game_over = false;

    let mut speed_per_second: Vec<f64> = vec![];
    let mut errors_per_second: Vec<f64> = vec![];
    let mut errors_this_second: f64 = 0.0;
    let mut char_number = 0;

    let mut lines: Vec<String>;
    let mut last_recorded_time = Instant::now();

    let mut words_done = 0;

    let mut config_opened = false;
    let mut selected_config: String = "time".to_string();

    let mut practice_menu = false;
    let mut practice_mode = false;
    let mut scroll_offset: f32 = 0.0;
    let mut selected_practice_level: Option<usize> = None;

    let words: Vec<&str> = reference.split_whitespace().collect();
    let average_word_length: f64 = if !words.is_empty() {
        words.iter().map(|w| w.len()).sum::<usize>() as f64 / words.len() as f64 + 1.0
    } else {
        5.0
    };
    
    loop {
        clear_background(macroquad::color::Color::from_rgba(20, 17, 15, 255));
        let max_width = f32::min(if screen_height() > screen_width() {screen_width() * 0.9} else {screen_width() * 0.7}, 1700.0);
        let font_size = if screen_height() > 2000.0 || screen_width() > 3800.0 {
            40.0
        } else {
            40.0 - (3840.0 / screen_width()) * 5.0
        };
        let line_h = measure_text("Gy", font.as_ref(), font_size as u16, 1.0).height * 1.6;
        let char_w = measure_text("G", font.as_ref(), font_size as u16, 1.0).width;
        lines = create_lines(&reference, font.clone(), font_size, max_width, quote, word_mode);

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
                &mut speed_per_second,
                &mut last_recorded_time,
                &mut words_done,
                &mut errors_per_second,
                u16::max((font_size / 1.5) as u16, 20),
                &mut config_opened,
                &mut selected_config,
                &mut practice_menu,
                &mut selected_practice_level,
                &mut practice_mode,
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
            );

            if !game_started && handle_input(&reference, &mut pressed_vec, &mut is_correct, &mut pos1, &mut words_done, &mut errors_this_second, &mut config_opened) {
                game_started = true;
            }
            
            if game_started && !game_over {
                timer = start_time.elapsed();
                if (timer.as_secs_f32() >= test_time - 0.2 && time_mode) || pos1 >= reference.chars().count() {
                    game_over = true;
                }
            }            
            
            let total_height = lines.len() as f32 * font_size * 1.2;
            let start_y = screen_height() / 2.0 - total_height / 2.0 + font_size;
            let start_x = screen_width() / 2.0 - max_width / 2.0;
            
            write_title(
                title_font.clone(),
                50.0,
                start_x,
                140.0,
            );
            
            handle_input(&reference, &mut pressed_vec, &mut is_correct, &mut pos1, &mut words_done, &mut errors_this_second, &mut config_opened);
            
            if time_mode {
                draw_timer(font.as_ref(), font_size, start_x, start_y, timer, test_time);
            } else if word_mode {
                draw_word_count(font.as_ref(), font_size, start_x, start_y, &mut words_done, batch_size);
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
            let (calc_pos_x, calc_pos_y) = calc_pos(&chars_in_line, pos1);
            if !game_started {
                let blink_interval = 0.5;
                let show_cursor = ((get_time() / blink_interval) as i32) % 2 == 0;
                if show_cursor && !game_over {
                    draw_cursor(calc_pos_x, calc_pos_y, start_x, start_y, line_h, char_w);
                }
            } else {
                draw_cursor(calc_pos_x, calc_pos_y, start_x, start_y, line_h, char_w);
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

        }  
        else if game_over {
            let mode = if time_mode {
                "time".to_string()
            } else if word_mode {
                "word".to_string()
            } else {
                "quote".to_string()
            };
            results::write_results(
                &is_correct,
                &pressed_vec,
                screen_width(),
                screen_height(),
                font.as_ref(),
                timer.as_secs_f32(),
                &speed_per_second,
                average_word_length,
                words_done,
                &mode,
                punctuation,
                numbers,
                &errors_per_second,
            );
        } else if practice_menu {
            let level = gui_practice::display_practice_menu(font.clone(), &mut scroll_offset, emoji_font.clone().unwrap(), &mut selected_practice_level);
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
                    &mut practice_menu,
                );
                practice_mode = true;
                reference = practice::create_words(TYPING_LEVELS[level.unwrap()].1, 50);
        }
        }
        if is_key_down(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::Tab) && is_key_down(KeyCode::Enter) {
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
                &mut practice_menu,
            );
            reference = utils::get_reference(punctuation, false, &word_list, batch_size);
            is_correct = VecDeque::from(vec![0; reference.len()]);
            thread::sleep(time::Duration::from_millis(80));
        }
        else {
            let _pressed = get_char_pressed();
        }

        draw_shortcut_info(font.as_ref(), f32::max(font_size / 1.7, 15.0), screen_width() / 2.0 - max_width / 2.0, screen_height() - 100.0, emoji_font.clone().unwrap(), practice_menu, game_over);

        next_frame().await;
    }
}
