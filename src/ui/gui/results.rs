use std::collections::VecDeque;
use macroquad::prelude::*;


fn get_typed_words(reference: &str, typed_chars: usize) -> usize {
    let mut res = 0;
    for c in reference[..typed_chars].chars() {
        if c.is_whitespace() {
            res += 1;
        }
    }
    res
}

pub fn write_results(
    is_correct: &VecDeque<i8>,
    pressed_vec: &Vec<char>,
    screen_width: f32,
    screen_height: f32,
    reference: &str,
    font: Option<&Font>,
    test_time: f32,
    font_size: f32,
) {
    write_wpm(
        &reference,
        pressed_vec.len(),
        font,
        test_time,
        font_size,
        screen_width / 2.0 - 100.0,
        screen_height / 2.0 + 100.0,
    );
    write_acc(
        &is_correct,
        pressed_vec.len(),
        font,
        font_size,
        screen_width / 2.0 - 100.0,
        screen_height / 2.0 + 150.0,
    );
    write_err_rate(
        &is_correct,
        pressed_vec.len(),
        font,
        font_size,
        screen_width / 2.0 - 100.0,
        screen_height / 2.0 + 200.0,
    );
    
}

pub fn write_wpm(
    reference: &str,
    typed_chars: usize,
    font: Option<&Font>,
    test_time: f32,
    font_size: f32,
    x: f32,
    y: f32,
) {
    let typed_words = get_typed_words(reference, typed_chars);
    let wpm = typed_words as f32 * 60.0 / test_time;
    let wpm_text = format!("WPM: {:.0}", wpm);
    draw_text_ex(
        &wpm_text,
        x,
        y,
        TextParams {
            font,
            font_size: font_size as u16,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
}

pub fn write_acc(is_correct: &VecDeque<i8>, typed_chars: usize, font: Option<&Font>, font_size: f32, x: f32, y: f32) {
    let correct_count = is_correct.iter().filter(|&&x| x == 2 || x == 1).count();
    let accuracy = if typed_chars > 0 {
        (correct_count as f32 / typed_chars as f32 * 100.0).round()
    } else {
        0.0
    };
    let acc_text = format!("Accuracy: {:.0}%", accuracy);
    draw_text_ex(
        &acc_text,
        x,
        y,
        TextParams {
            font,
            font_size: font_size as u16,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
}

pub fn write_err_rate(is_correct: &VecDeque<i8>, typed_chars: usize, font: Option<&Font>, font_size: f32, x: f32, y: f32) {
    let error_count = is_correct.iter().filter(|&&x| x == -1 || x == 1).count();
    let error_rate = if typed_chars > 0 {
        (error_count as f32 / typed_chars as f32 * 100.0).round()
    } else {
        0.0
    };
    let acc_text = format!("Error rate: {:.0}%", error_rate);
    draw_text_ex(
        &acc_text,
        x,
        y,
        TextParams {
            font,
            font_size: font_size as u16,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
}