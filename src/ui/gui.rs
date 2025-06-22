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
            lines.push(current_line);
            current_line = word.to_string();
        } else {
            current_line = test_line;
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    loop {
        clear_background(Color::from_rgba(26, 23, 20, 255));

        let total_height = lines.len() as f32 * font_size * 1.2;
        let start_y = screen_height() / 2.0 - total_height / 2.0 + font_size;
        write_title(title_font.clone(), 50.0);

        for (i, line) in lines.iter().enumerate() {
            draw_text_ex(
                line,
                screen_width() / 20.0,
                start_y + i as f32 * font_size * 1.5,
                TextParams {
                    font: font.as_ref(),
                    font_size: font_size as u16,
                    color: Color::from_rgba(255, 255, 255, 100),
                    ..Default::default()
                },
            );
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}


