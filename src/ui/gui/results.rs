use std::collections::VecDeque;
use macroquad::prelude::*;
use eframe::egui;
use egui::{Color32,  Area, pos2};
use egui_plot::{Line, Plot};


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
    speed_per_second: &Vec<f64>,
    average_word_length: f64,
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

    let mut speed2: Vec<f64> = speed_per_second.clone();
    speed2.push(speed_per_second.last().unwrap_or(&0.0) * 60.0 / average_word_length);
    let smoothed_speeds = smooth(&speed2, 4, average_word_length);


    let chart_points: Vec<[f64; 2]> = smoothed_speeds
        .iter()
        .enumerate()
        .map(|(i, &cpm)| [i as f64, cpm])
        .collect();

    draw_chart(&chart_points);
    egui_macroquad::draw();
    
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

fn smooth(values: &[f64], window: usize, average_word_length: f64) -> Vec<f64> {
    let len = values.len();
    let mut smoothed = Vec::with_capacity(len);

    for i in 0..len {
        let start = i.saturating_sub(window);
        let end = (i + window + 1).min(len);
        let slice = &values[start..end];

        let avg = slice.iter().sum::<f64>() / slice.len() as f64 / average_word_length;
        smoothed.push(avg);
    }

    smoothed
}

fn draw_chart(points: &[[f64; 2]]) {
    egui_macroquad::ui(|ctx| {
        let screen_width = macroquad::window::screen_width();
        let chart_width = 1300.0;
        let chart_height = 500.0;
        let chart_x = (screen_width - chart_width) / 2.0;
        let chart_y = 30.0;

        Area::new("chart_area".into())
            .fixed_pos(pos2(chart_x, chart_y))
            .show(ctx, |ui| {
                let size = egui::Vec2::new(chart_width, chart_height);
                let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());
                ui.painter().rect_filled(rect, 5.0, Color32::from_rgb(20, 17, 15));

                let mut child_ui = ui.child_ui(rect, *ui.layout(), None);

                Plot::new("performance_plot")
                    .view_aspect(2.0)
                    .include_y(0.0)
                    .show_background(false)
                    .show_axes([true, true])
                    .show_grid(true)
                    .view_aspect(3.0)
                    .x_axis_label("Time (s)")
                    .y_axis_label("Speed (WPM)")
                    .show(&mut child_ui, |plot_ui| {
                        let line = Line::new("Performance", points.to_vec())
                            .color(Color32::from_rgb(255, 155, 0))
                            .highlight(true)
                            .name("Performance");
                        plot_ui.line(line);

                        let offset_points: Vec<[f64; 2]> = points
                            .iter()
                            .map(|[x, y]| [*x, y + 1.0])
                            .collect();
                        let line2 = Line::new("Offset", offset_points)
                            .color(Color32::from_rgb(0, 200, 255))
                            .highlight(true)
                            .name("Offset +10");
                        plot_ui.line(line2);
                    });
            });
    });
}