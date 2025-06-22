use macroquad::prelude::*;

use crate::utils;


pub async fn gui_main_async() {
    loop {
        let font = match load_ttf_font("font/RobotoMono-VariableFont_wght.ttf").await {
            Ok(f) => Some(f),
            Err(_) => None,
        };
        clear_background(Color::from_rgba(25, 22, 20, 255));
        
        let text = "Hello World!";
        let font_size = 40.0;
        let text_dimensions = measure_text(
            text,
            font.as_ref(),
            font_size as u16,
            1.0,
        );
        let text_width = text_dimensions.width;
        let top_words = 500;
        let _word_list = utils::read_first_n_words(top_words as usize);
        
        draw_text_ex(
            text,
            screen_width() / 2.0 - text_width / 2.0,
            screen_height() / 2.0,
            TextParams {
                font: font.as_ref(),
                font_size: font_size as u16,
                color: WHITE,
                ..Default::default()
            },
        );
        
        if is_key_down(KeyCode::Escape) {
            break;
        }
        
        next_frame().await;
    }
}
