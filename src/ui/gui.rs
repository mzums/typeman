use std::collections::VecDeque;

use macroquad::prelude::*;

use crate::utils;


fn write_title(font: Option<Font>, font_size: f32) {

    let (type_text, man_text) = ("Type", "Man");
    let x = 50.0;
    let y = 100.0;

    let type_width = measure_text(type_text, font.as_ref(), font_size as u16, 1.0).width;

    for (text, color, dx) in [
        (type_text, Color::from_rgba(255, 155, 0, 255), 0.0),
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

fn create_lines(reference: &str, font: Option<Font>, font_size: f32, max_width: f32) -> Vec<String> {
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
            lines.push(current_line);
            current_line = word.to_string();
        } else {
            current_line = test_line;
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

pub async fn gui_main_async() {
    let font = match load_ttf_font("assets/font/RobotoMono-VariableFont_wght.ttf").await {
        Ok(f) => Some(f),
        Err(_) => None,
    };
    let title_font = match load_ttf_font("assets/font/static/RobotoMono-Medium.ttf").await {
        Ok(f) => Some(f),
        Err(_) => None,
    };

    let top_words = 500;
    let word_list = utils::read_first_n_words(top_words as usize);
    let batch_size = 50;
    let font_size = 40.0;
    
    let reference = utils::get_reference(
        false,
        false,
        &word_list,
        batch_size,
    );

    let max_width = screen_width() * 0.9;

    let lines: Vec<String> = create_lines(&reference, font.clone(), font_size, max_width);

    let mut pressed_vec: Vec<char>  = vec![];
    let mut is_correct: VecDeque<i8> = VecDeque::from(vec![0; reference.len()]); // 2 - correct, 1 - corrected, 0 - not typed, -1 - incorrect
    let mut pos: usize;
    let mut pos1: usize = 0;
    
    loop {
        clear_background(Color::from_rgba(20, 17, 15, 255));
        
        let total_height = lines.len() as f32 * font_size * 1.2;
        let start_y = screen_height() / 2.0 - total_height / 2.0 + font_size;
        write_title(title_font.clone(), 50.0);
        
        let pressed = get_char_pressed();
        if pressed.is_some() {
            //println!("{:?}", reference);
            //println!("{:?} {:?} {:?}", pressed, reference.chars().nth(pos1 as usize), pos1);
            println!("Pressed: {:?}", pressed);
            if pressed == Some('\u{8}') {
                pressed_vec.pop();
                if pos1 > 0 {
                    pos1 -= 1;
                }
            } else {
                if pressed == reference.chars().nth(pos1 as usize) && is_correct[pos1] != -1 && is_correct[pos1] != 1 {
                    is_correct[pos1] = 2; // Correct
                } else if pressed == reference.chars().nth(pos1 as usize) && is_correct[pos1] == -1 {
                    is_correct[pos1] = 1; // Corrected
                } else {
                    is_correct[pos1] = -1; // Incorrect
                }
                //println!("is_correct: {:?}", is_correct);
                pos1 += 1;
                pressed_vec.push(pressed.unwrap());
            }
        }
        pos = 0;
        let mut type_height: f32;
        let mut type_width: f32;
        let mut pos_y = 0.0;
        //println!("pos1: {}", pos1);
        
        for line in lines.iter() {
            let mut pos_x = 0;
            for char in line.chars() {
                let mut curr_char = char;
                let color = if pos + 1 > pressed_vec.len() || is_correct[pos] == 0 {
                    Color::from_rgba(255, 255, 255, 80) // not typed
                } else if is_correct.get(pos).is_some() && is_correct[pos] == 2 {
                    Color::from_rgba(255, 255, 255, 200) // correct
                } else if is_correct.get(pos).is_some() && is_correct[pos] == 1 {
                    Color::from_rgba(255, 165, 0, 255) // corrected
                } else {
                    if char == ' ' {
                        curr_char = '_';
                    }
                    Color::from_rgba(255, 50, 50, 180) // incorrect
                };
                draw_text_ex(
                    &curr_char.to_string(),
                    pos_x as f32 + 50.0,
                    pos_y + start_y,
                    TextParams {
                        font: font.as_ref(),
                        font_size: font_size as u16,
                        color: color,
                        ..Default::default()
                    }
                );
                type_width = measure_text(&char.to_string(), font.as_ref(), font_size as u16, 1.0).width;
                pos_x += type_width as usize;
                pos += 1;
            }
            type_height = measure_text(line, font.as_ref(), font_size as u16, 1.0).height;
            pos_y += type_height * 1.6;
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}


