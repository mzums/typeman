use macroquad::prelude::*;

use crate::ui::gui::config;
use crate::practice::TYPING_LEVELS;


pub fn display_practice_menu(font: Option<Font>, scroll_offset: &mut f32, emoji_font: Font, selected_level: &mut Option<usize>) -> Option<usize> {
    let mouse_pos = mouse_position();

    let (_, y_scroll) = mouse_wheel();
    *scroll_offset -= y_scroll * 60.0;

    let total_height = TYPING_LEVELS.len() as f32 * 60.0;
    let visible_height = screen_height() - 100.0;
    let max_scroll = (total_height - visible_height).max(0.0) + 120.0;
    
    *scroll_offset = scroll_offset.clamp(0.0, max_scroll);

    draw_text_ex(
        "Select Typing Level",
        50.0,
        100.0,
        TextParams {
            font: font.as_ref(),
            font_size: 40,
            color: Color::from_rgba(255, 150, 0, 255),
            ..Default::default()
        },
    );

    let start_index = 0;
    let end_index = TYPING_LEVELS.len();

    let mut any_hovered = false;
    for i in start_index..end_index {
        let y = 100.0 + i as f32 * 60.0 - *scroll_offset;
        let text = if i + 1 < 10 {
            format!("{}.  {}", i + 1, TYPING_LEVELS[i].0)
        } else {
            format!("{}. {}", i + 1, TYPING_LEVELS[i].0)
        };
        let text_size = measure_text(&text, font.as_ref(), 36, 1.0);
        let button_rect = Rect::new(
            50.0,
            y - text_size.height / 2.0 + 100.0,
            text_size.width + 40.0,
            text_size.height + 20.0,
        );
        if button_rect.contains(vec2(mouse_pos.0, mouse_pos.1)) {
            any_hovered = true;
            break;
        }
    }

    for i in start_index..end_index {
        let (level_name, _) = &TYPING_LEVELS[i];
        let y = 100.0 + i as f32 * 60.0 - *scroll_offset;

        if y < 100.0 - 60.0 || y > screen_height() + 60.0 {
            continue;
        }

        let mut text = format!("{}. {}", i + 1, level_name);
        if i + 1 < 10 {
            text = format!("{}.  {}", i + 1, level_name);
        }

        let text_size = measure_text(&text, font.as_ref(), 36, 1.0);
        let button_rect = Rect::new(
            50.0,
            y - text_size.height / 2.0 + 100.0,
            text_size.width + 40.0,
            text_size.height + 20.0,
        );

        let mut show_tick = false;
        let results_path = format!("practice_results/level_{}.txt", i + 1);
        if let Ok(contents) = std::fs::read_to_string(&results_path) {
            for line in contents.lines() {
                if line.starts_with("WPM:") {
                    if let Some(wpm_str) = line.strip_prefix("WPM:").map(str::trim) {
                        if let Ok(wpm) = wpm_str.parse::<f32>() {
                            if wpm >= 35.0 {
                                show_tick = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        let is_hovered = if any_hovered {
            button_rect.contains(vec2(mouse_pos.0, mouse_pos.1))
        } else if let Some(selected) = selected_level {
            *selected == i
        } else {
            false
        };

        let is_clicked = is_hovered && is_mouse_button_pressed(MouseButton::Left);

        let text_color = if is_hovered {
            Color::from_rgba(255, 150, 0, 255)
        } else {
            Color::from_rgba(200, 200, 200, 230)
        };

        let tick_offset = 40.0;
        if show_tick {
            draw_text_ex(
                "✓",
                tick_offset + 20.0,
                button_rect.y + button_rect.h / 2.0 + 30.0,
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
            80.0 + tick_offset,
            button_rect.y + button_rect.h / 2.0 + 20.0,
            TextParams {
                font: font.as_ref(),
                font_size: 20,
                color: text_color,
                ..Default::default()
            },
        );

        if is_clicked {
            *selected_level = Some(i);
        }
    }
    
    if is_key_down(KeyCode::Down) {
        *scroll_offset = (*scroll_offset + 50.0).min(max_scroll);
        *selected_level = if let Some(level) = *selected_level {
            Some((level + 1).min(TYPING_LEVELS.len() - 1))
        } else {
            Some(0)
        };
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    if is_key_down(KeyCode::Up) {
        *scroll_offset = (*scroll_offset - 50.0).max(0.0);
        *selected_level = if let Some(level) = *selected_level {
            Some((level as isize - 1).max(0) as usize)
        } else {
            Some(TYPING_LEVELS.len() - 1)
        };
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    if is_key_pressed(KeyCode::Enter) {
        if let Some(level) = *selected_level {
            return Some(level);
        }
    }

    if max_scroll > 0.0 {
        let scroll_area_height = screen_height() - 100.0;
        let thumb_height = f32::min(scroll_area_height * (visible_height / total_height), screen_height() - 200.0);
        let thumb_position = if max_scroll > 0.0 {
            100.0 + (*scroll_offset / max_scroll) * (scroll_area_height - thumb_height)
        } else {
            100.0
        };
        
        config::draw_rounded_rect(
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