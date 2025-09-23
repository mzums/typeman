use macroquad::prelude::*;
use miniquad::window::set_mouse_cursor;
use miniquad::CursorIcon;
use std::collections::VecDeque;
use std::thread;
use std::time::{Instant, Duration};

use crate::ui::gui::config;
use crate::practice::{check_if_completed, TYPING_LEVELS};
use crate::utils;
use crate::color_scheme::ColorScheme;


pub fn display_practice_menu(
    font: Option<Font>,
    scroll_offset: &mut f32,
    emoji_font: Font,
    selected_level: &mut Option<usize>,
    practice_menu: &mut bool, 
    time_mode: &mut bool,
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
    error_positions: &mut Vec<bool>,
    color_scheme: &ColorScheme,
) -> Option<usize> {
    let font_size = if screen_width() > 3000.0 {
        20
    } else if screen_width() > 1900.0 {
        18
    } else {
        15
    };
    let tick_offset = if screen_width() > 1900.0 { 
        (screen_width() - measure_text(TYPING_LEVELS[20].0, font.as_ref(), font_size, 1.0).width) / 2.0 - 50.0
    } else { 
        60.0 
    };
    let mouse_pos = mouse_position();

    let (_, y_scroll) = mouse_wheel();
    *scroll_offset -= y_scroll * 60.0;

    let total_height = TYPING_LEVELS.len() as f32 * 60.0;
    let visible_height = screen_height() - 100.0;
    let max_scroll = f32::max((TYPING_LEVELS.len() + 5) as f32 * (20.0 + font_size as f32) - screen_height(), 0.0);

    *scroll_offset = scroll_offset.clamp(0.0, max_scroll);


    if *scroll_offset < 20.0 {
        draw_text_ex(
            "Select Typing Level",
            tick_offset + 20.0,
            screen_height() / 10.0,
            TextParams {
                font: font.as_ref(),
                font_size: if screen_height() > 2000.0 && screen_width() > 1900.0 {
                        40
                    } else if screen_height() > 1000.0 && screen_width() > 800.0{
                        30
                    } else {
                        25
                    },
                color: color_scheme.main_color_mq(),
                ..Default::default()
            },
        );
    }

    let start_index = 0;
    let end_index = TYPING_LEVELS.len();

    let mut any_hovered = false;
    let mut y: f32 = screen_height() / 10.0 + 2.0 * font_size as f32;
    for (i, (level_name, _)) in TYPING_LEVELS.iter().enumerate().take(end_index).skip(start_index) {
        let mut text = format!("{}. {}", i + 1, level_name);
        if i + 1 < 10 {
            text = format!("{}.  {}", i + 1, level_name);
        }
        let text_size = measure_text(&text, font.as_ref(), font_size, 1.0);
        let button_rect = Rect::new(
            tick_offset + 40.0,
            y - *scroll_offset,
            text_size.width + 2.0 * font_size as f32,
            font_size as f32 + 20.0,
        );

        if button_rect.contains(vec2(mouse_pos.0, mouse_pos.1)) {
            any_hovered = true;
            break;
        }
        y += 20.0 + font_size as f32;
    }
    let mut y: f32 = screen_height() / 10.0 + 2.0 * font_size as f32;

    for (i, _) in TYPING_LEVELS.iter().enumerate().take(end_index).skip(start_index) {
        let (level_name, _) = &TYPING_LEVELS[i];
        let mut text = format!("{}. {}", i + 1, level_name);
        if i + 1 < 10 {
            text = format!("{}.  {}", i + 1, level_name);
        }

        let text_size = measure_text(&text, font.as_ref(), font_size, 1.0);

        if y - *scroll_offset > screen_height() + 60.0 || y - *scroll_offset < 0.0 {
            y += 20.0 + font_size as f32;
            continue;
        }

        let button_rect = Rect::new(
            tick_offset + 40.0,
            y - *scroll_offset,
            text_size.width + 2.0 * font_size as f32,
            20.0 + font_size as f32,
        );

        let results_path = format!("practice_results/level_{}.txt", i + 1);
        let show_tick = check_if_completed(results_path.as_str());

        let is_hovered = if any_hovered {
            button_rect.contains(vec2(mouse_pos.0, mouse_pos.1))
        } else if let Some(selected) = selected_level {
            *selected == i
        } else {
            false
        };
        if is_hovered {
            *selected_level = Some(i);
        }

        set_mouse_cursor(if any_hovered {
            CursorIcon::Pointer
        } else {
            CursorIcon::Default
        });

        let is_clicked = is_hovered && any_hovered && is_mouse_button_pressed(MouseButton::Left);

        if is_clicked {
            *selected_level = Some(i);
            if let Some(level) = *selected_level {
                return Some(level);
            }
        }

        let text_color = if is_hovered {
            color_scheme.main_color_mq()
        } else {
            color_scheme.text_color_mq()
        };

        if show_tick {
            draw_text_ex(
                "✓",
                tick_offset,
                y + 1.8 * font_size as f32 - *scroll_offset,
                TextParams {
                    font: Some(&emoji_font),
                    font_size: 50,
                    color: Color::from_rgba(0, 255, 0, 255),
                    ..Default::default()
                },
            );
        }
        
        draw_text_ex(
            &text,
            60.0 + tick_offset,
            y + 1.2 * font_size as f32 - *scroll_offset,
            TextParams {
                font: font.as_ref(),
                font_size,
                color: text_color,
                ..Default::default()
            },
        );
        y += 20.0 + font_size as f32;
    }

    thread_local! {
        static DOWN_KEY_HELD_START: std::cell::RefCell<Option<Instant>> = const { std::cell::RefCell::new(None) };
        static LAST_DOWN_SCROLL: std::cell::RefCell<Option<Instant>> = const { std::cell::RefCell::new(None) };
        static UP_KEY_HELD_START: std::cell::RefCell<Option<Instant>> = const { std::cell::RefCell::new(None) };
        static LAST_UP_SCROLL: std::cell::RefCell<Option<Instant>> = const { std::cell::RefCell::new(None) };
    }

    let down_pressed = is_key_down(KeyCode::Down);
    let up_pressed = is_key_down(KeyCode::Up);

    DOWN_KEY_HELD_START.with(|down_start| {
        LAST_DOWN_SCROLL.with(|last_down| {
            UP_KEY_HELD_START.with(|up_start| {
                LAST_UP_SCROLL.with(|last_up| {
                    let mut down_key_held_start = *down_start.borrow();
                    let mut last_down_scroll = *last_down.borrow();
                    let mut up_key_held_start = *up_start.borrow();
                    let mut last_up_scroll = *last_up.borrow();

                    if down_pressed && !any_hovered {
                        let now = Instant::now();
                        if down_key_held_start.is_none() {
                            down_key_held_start = Some(now);
                            last_down_scroll = Some(now);
                            *scroll_offset = (*scroll_offset + 20.0 + font_size as f32).min(max_scroll);
                            *selected_level = if let Some(level) = *selected_level {
                                Some((level + 1).min(TYPING_LEVELS.len() - 1))
                            } else {
                                Some(0)
                            };
                        } else if let (Some(start), Some(last)) = (down_key_held_start, last_down_scroll) {
                            let held_duration = now.duration_since(start).as_millis();
                            let interval = if held_duration < 500 {
                                200
                            } else if held_duration < 1500 {
                                80
                            } else {
                                30
                            };
                            if now.duration_since(last).as_millis() >= interval {
                                *scroll_offset = (*scroll_offset + 20.0 + font_size as f32).min(max_scroll);
                                *selected_level = if let Some(level) = *selected_level {
                                    Some((level + 1).min(TYPING_LEVELS.len() - 1))
                                } else {
                                    Some(0)
                                };
                                last_down_scroll = Some(now);
                            }
                        }
                    } else {
                        down_key_held_start = None;
                        last_down_scroll = None;
                    }

                    if up_pressed && !any_hovered {
                        let now = Instant::now();
                        if up_key_held_start.is_none() {
                            up_key_held_start = Some(now);
                            last_up_scroll = Some(now);
                            *scroll_offset = (*scroll_offset - 20.0 - font_size as f32).max(0.0);
                            *selected_level = if let Some(level) = *selected_level {
                                Some((level as isize - 1).max(0) as usize)
                            } else {
                                Some(TYPING_LEVELS.len() - 1)
                            };
                        } else if let (Some(start), Some(last)) = (up_key_held_start, last_up_scroll) {
                            let held_duration = now.duration_since(start).as_millis();
                            let interval = if held_duration < 500 {
                                200
                            } else if held_duration < 1500 {
                                80
                            } else {
                                30
                            };
                            if now.duration_since(last).as_millis() >= interval {
                                *scroll_offset = (*scroll_offset - 20.0 - font_size as f32).max(0.0);
                                *selected_level = if let Some(level) = *selected_level {
                                    Some((level as isize - 1).max(0) as usize)
                                } else {
                                    Some(TYPING_LEVELS.len() - 1)
                                };
                                last_up_scroll = Some(now);
                            }
                        }
                    } else {
                        up_key_held_start = None;
                        last_up_scroll = None;
                    }

                    *down_start.borrow_mut() = down_key_held_start;
                    *last_down.borrow_mut() = last_down_scroll;
                    *up_start.borrow_mut() = up_key_held_start;
                    *last_up.borrow_mut() = last_up_scroll;
                });
            });
        });
    });

    if is_key_pressed(KeyCode::Enter) && !is_key_down(KeyCode::Tab) {
        if let Some(level) = *selected_level {
            return Some(level);
        }
    }
    if is_key_pressed(KeyCode::Q) {
        if *practice_menu {
            *practice_menu = false;
            *time_mode = true;
            config::reset_game_state(
                pressed_vec,
                is_correct,
                pos1,
                timer,
                start_time,
                game_started,
                game_over,
                speed_per_second,
                last_recorded_time,
                words_done,
                errors_per_second,
                saved_results,
                error_positions,
            );
            thread::sleep(Duration::from_millis(200));
            pressed_vec.clear();
            return None
        }
    } else {
        let _pressed = get_char_pressed();
    }

    if max_scroll > 0.0 {
        let scroll_area_height = screen_height() - 100.0;
        let thumb_height = f32::min(scroll_area_height * (visible_height / total_height), screen_height() - 200.0);
        let thumb_position = if max_scroll > 0.0 {
            100.0 + (*scroll_offset / max_scroll) * (scroll_area_height - thumb_height)
        } else {
            100.0
        };
        
        utils::draw_rounded_rect(
            screen_width() - 20.0,
            thumb_position,
            10.0,
            thumb_height,
            5.0,
            Color::from_rgba(180, 180, 180, 220),
        );
    }

    None
}