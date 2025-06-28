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

fn calc_standard_deviation(values: &[f64], average_word_length: f64) -> f64 {
    let wpm_values: Vec<f64> = values.iter().map(|&cpm| cpm / average_word_length).collect();
    let mean = wpm_values.iter().sum::<f64>() / wpm_values.len() as f64;
    let variance = wpm_values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / wpm_values.len() as f64;
    variance.sqrt()
}

pub fn write_results(
    is_correct: &VecDeque<i32>,
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
    let avg_wpm = write_wpm(
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
    write_consistency(
        speed_per_second,
        average_word_length,
        font,
        font_size,
        screen_width / 2.0 - 100.0,
        screen_height / 2.0 + 250.0,
        avg_wpm,
    );
    let mut speed2: Vec<f64> = speed_per_second.clone();
    speed2.push(*speed_per_second.last().unwrap_or(&0.0));
    let smoothed_speeds = smooth(&speed2, 3, average_word_length);

    let chart_points: Vec<[f64; 2]> = smoothed_speeds
        .iter()
        .enumerate()
        .map(|(i, &cpm)| [i as f64, cpm])
        .collect();

    draw_chart(&chart_points);
    egui_macroquad::draw();
    
}

fn write_consistency(
    speed_per_second: &Vec<f64>,
    average_word_length: f64,
    font: Option<&Font>,
    font_size: f32,
    x: f32,
    y: f32,
    avg_wpm: f32,
) {
    let standard_deviation = calc_standard_deviation(speed_per_second, average_word_length);
    let consistency = if avg_wpm > 0.0 {
        100.0 - (standard_deviation / avg_wpm as f64 * 100.0).round()
    } else {
        0.0
    };

    let consistency_text = format!("Consistency: {consistency}%");
    draw_text_ex(
        &consistency_text,
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

fn write_wpm(
    reference: &str,
    typed_chars: usize,
    font: Option<&Font>,
    test_time: f32,
    font_size: f32,
    x: f32,
    y: f32,
) -> f32 {
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
    return wpm;
}

fn write_acc(is_correct: &VecDeque<i32>, typed_chars: usize, font: Option<&Font>, font_size: f32, x: f32, y: f32) {
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

fn write_err_rate(is_correct: &VecDeque<i32>, typed_chars: usize, font: Option<&Font>, font_size: f32, x: f32, y: f32) {
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

/*fn draw_chart(points: &[[f64; 2]]) {
    egui_macroquad::ui(|ctx| {
        let screen_width = macroquad::window::screen_width();
        let chart_width = 1300.0;
        let chart_height = 300.0;
        let chart_x = (screen_width - chart_width) / 2.0;
        let chart_y = (screen_height() - chart_height) / 4.0;

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
                    .view_aspect(5.0)
                    .x_axis_label("Time (s)")
                    .y_axis_label("Speed (WPM)")
                    .show(&mut child_ui, |plot_ui| {
                        let line = Line::new("Performance", points.to_vec())
                            .color(Color32::from_rgb(255, 155, 0))
                            .highlight(true)
                            .name("Performance");
                        plot_ui.line(line);
                    });
            });
    });
}*/


fn draw_chart(points: &[[f64; 2]]) {
    egui_macroquad::ui(|ctx| {
        let screen_width = macroquad::window::screen_width();
        let chart_width = 1300.0;
        let chart_height = 500.0;
        let chart_x = (screen_width - chart_width) / 2.0;
        let chart_y = (screen_height() - chart_height) / 4.0;

        Area::new("chart_area".into())
            .fixed_pos(pos2(chart_x, chart_y))
            .show(ctx, |ui| {
                let size = egui::Vec2::new(chart_width, chart_height);
                let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());
                ui.painter().rect_filled(rect, 5.0, Color32::from_rgb(20, 17, 15));

                let mut child_ui = ui.child_ui(rect, *ui.layout(), None);

                // Custom grid spacer for x-axis (every 2 seconds)
                let grid_spacer = |input: egui_plot::GridInput| -> Vec<egui_plot::GridMark> {
                    let min = input.bounds.0;
                    let max = input.bounds.1;
                    let step = 20.0; // Grid every 2 seconds
                    let mut marks = Vec::new();
                    
                    // Start from the first multiple of step >= min
                    let mut current = (min / step).ceil() * step;
                    while current <= max {
                        marks.push(egui_plot::GridMark {
                            value: current,
                            step_size: step,
                        });
                        current += step;
                    }
                    marks
                };

                Plot::new("performance_plot")
                    .include_y(0.0)
                    .show_background(false)
                    .show_axes([true, true])
                    .show_grid(true)
                    .view_aspect(5.0)
                    .x_axis_label("Time (s)")
                    .y_axis_label("Speed (WPM)")
                    .y_grid_spacer(grid_spacer) // Apply custom grid spacing
                    .show(&mut child_ui, |plot_ui| {
                        let line = Line::new("Performance", points.to_vec())
                            .color(Color32::from_rgb(255, 155, 0))
                            .highlight(true)
                            .name("Performance");
                        plot_ui.line(line);
                    });
            });
    });
}
     