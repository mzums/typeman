use std::collections::VecDeque;
use macroquad::prelude::*;
use eframe::egui;
use egui::{Color32,  Area, pos2};
use egui_plot::{Line, Plot};


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
    font: Option<&Font>,
    test_time: f32,
    speed_per_second: &Vec<f64>,
    average_word_length: f64,
    words_done: usize,
    mode: &str,
    punctuation: bool,
    numbers: bool,
    errors_per_second: &Vec<f64>,
) {
    let correct_count = is_correct.iter().filter(|&&x| x == 2 || x == 1).count();
    let accuracy = if pressed_vec.len() > 0 {
        (correct_count as f32 / pressed_vec.len() as f32 * 100.0).round()
    } else {
        0.0
    };

    let wpm = (words_done as f32 * 60.0 / test_time) as i32;

    let text_size = {
        let a = measure_text(&format!("{}%", accuracy), font, 60, 1.0);
        let b = measure_text(&format!("{wpm}"), font, 60, 1.0);
        if a.width > b.width { a } else { b }
    };

    let chart_width = f32::min(0.7 * screen_width, 1800.0);
    let chart_height: f32 = f32::min(chart_width / 5.0, 360.0);

    let chart_x = (screen_width - chart_width + 1.5 * text_size.width) / 2.0;
    let chart_y = (screen_height - chart_height) / 4.0;

    let text2_width = measure_text("consistency", font, 25, 1.0).width;
    let padding = (chart_width - 4.0 * text2_width) / 3.0;

    let avg_wpm = write_wpm(
        font,
        test_time,
        (screen_width - chart_width - 2.0 * text_size.width) / 2.0,
        (screen_height - chart_height) / 3.8,
        words_done,
    );
    write_acc(
        &is_correct,
        pressed_vec.len(),
        font,
        (screen_width - chart_width - 2.0 * text_size.width) / 2.0,
        (screen_height - chart_height) / 3.8 + text_size.height * 3.0,
    );
    write_err_rate(
        &is_correct,
        pressed_vec.len(),
        font,
        (screen_width - chart_width + 2.0 * text_size.width) / 2.0,
        chart_y + chart_height + 20.0,
    );
    write_consistency(
        speed_per_second,
        average_word_length,
        font,
        (screen_width - chart_width + 2.0 * text_size.width) / 2.0 + text2_width + padding,
        chart_y + chart_height + 20.0,
        avg_wpm,
    );
    write_time(
        test_time,
        font,
        (screen_width - chart_width + 2.0 * text_size.width) / 2.0 + (text2_width + padding) * 2.0,
        chart_y + chart_height + 20.0,
    );
    write_mode(
        font,
        (screen_width - chart_width + 2.0 * text_size.width) / 2.0 + (text2_width + padding) * 3.0,
        chart_y + chart_height + 20.0,
        mode,
        punctuation,
        numbers,
    );

    let mut speed2: Vec<f64> = speed_per_second.clone();
    speed2.push(*speed_per_second.last().unwrap_or(&0.0));
    let smoothed_speeds = smooth(&speed2, 2, average_word_length);

    let chart_points: Vec<[f64; 2]> = smoothed_speeds
        .iter()
        .enumerate()
        .map(|(i, &cpm)| [i as f64, cpm])
        .collect();

    draw_chart(&chart_points, chart_width, chart_height, chart_x, chart_y, errors_per_second);
    egui_macroquad::draw();
    
}

fn write_mode(
    font: Option<&Font>,
    x: f32,
    y: f32,
    mode: &str,
    punctuation: bool,
    numbers: bool,
) {
    let mode_text = format!("{mode}");
    let mut font_size = 50;
    let mut punct_pos: f32 = y;
    let mut number_pos: f32 = y;
    let mut mode_pos: f32 = y + 50.0;
    draw_text_ex(
        "mode",
        x,
        y,
        TextParams {
            font,
            font_size: 25.0 as u16,
            color: Color::from_rgba(255, 255, 255, 80),
            ..Default::default()
        },
    );
    if punctuation && numbers{
        punct_pos = y + 70.0;
        number_pos = y + 90.0;
        font_size = 20;
        mode_pos = y + 40.0;
    }
    else if punctuation || numbers {
        font_size = 20;
        mode_pos = y + 40.0;
        if punctuation {
            punct_pos = y + 70.0;
        } else {
            number_pos = y + 70.0;
        }
    }

    draw_text_ex(
        &mode_text,
        x,
        mode_pos,
        TextParams {
            font,
            font_size: font_size + 10,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
    if punctuation {
        draw_text_ex(
            "punctuation",
            x,
            punct_pos,
            TextParams {
                font,
                font_size: font_size,
                color: Color::from_rgba(255, 155, 0, 255),
                ..Default::default()
            },
        );
    }
    if numbers {
        draw_text_ex(
            "numbers",
            x,
            number_pos,
            TextParams {
                font,
                font_size: font_size,
                color: Color::from_rgba(255, 155, 0, 255),
                ..Default::default()
            },
        );
    }
}

fn write_time(
    test_time: f32,
    font: Option<&Font>,
    x: f32,
    y: f32,
) {
    let time_text = format!("{:.0}s", test_time);
    draw_text_ex(
        "time",
        x,
        y,
        TextParams {
            font,
            font_size: 25.0 as u16,
            color: Color::from_rgba(255, 255, 255, 80),
            ..Default::default()
        },
    );
    draw_text_ex(
        &time_text,
        x,
        y + 60.0,
        TextParams {
            font,
            font_size: 60,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
}

fn write_consistency(
    speed_per_second: &Vec<f64>,
    average_word_length: f64,
    font: Option<&Font>,
    x: f32,
    y: f32,
    avg_wpm: f32,
) {
    let standard_deviation = calc_standard_deviation(speed_per_second, average_word_length);
    let consistency = if avg_wpm > 0.0 {
        (100.0 - (standard_deviation / avg_wpm as f64 * 100.0).round()).max(0.0)
    } else {
        0.0
    };

    let consistency_text = format!("{consistency}%");
    draw_text_ex(
        "consistency",
        x,
        y,
        TextParams {
            font,
            font_size: 25.0 as u16,
            color: Color::from_rgba(255, 255, 255, 80),
            ..Default::default()
        },
    );
    draw_text_ex(
        &consistency_text,
        x,
        y + 60.0,
        TextParams {
            font,
            font_size: 60,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
}

fn write_wpm(
    font: Option<&Font>,
    test_time: f32,
    x: f32,
    y: f32,
    words_done: usize,
) -> f32 {
    let wpm = words_done as f32 * 60.0 / test_time;
    let wpm_text = format!("{:.0}", wpm);
    draw_text_ex(
        "wpm",
        x,
        y,
        TextParams {
            font,
            font_size: 40.0 as u16,
            color: Color::from_rgba(255, 255, 255, 80),
            ..Default::default()
        },
    );
    draw_text_ex(
        &wpm_text,
        x,
        y + 85.0,
        TextParams {
            font,
            font_size: 90,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
    return wpm;
}

fn write_acc(is_correct: &VecDeque<i32>, typed_chars: usize, font: Option<&Font>, x: f32, y: f32) {
    let correct_count = is_correct.iter().filter(|&&x| x == 2 || x == 1).count();
    let accuracy = if typed_chars > 0 {
        (correct_count as f32 / typed_chars as f32 * 100.0).round()
    } else {
        0.0
    };
    let acc_text = format!("{:.0}%", accuracy);
    draw_text_ex(
        "acc",
        x,
        y,
        TextParams {
            font,
            font_size: 40.0 as u16,
            color: Color::from_rgba(255, 255, 255, 80),
            ..Default::default()
        },
    );
    draw_text_ex(
        &acc_text,
        x,
        y + 85.0,
        TextParams {
            font,
            font_size: 90,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
}

fn write_err_rate(is_correct: &VecDeque<i32>, typed_chars: usize, font: Option<&Font>, x: f32, y: f32) {
    let error_count = is_correct.iter().filter(|&&x| x == -1 || x == 1).count();
    let error_rate = if typed_chars > 0 {
        (error_count as f32 / typed_chars as f32 * 100.0).round()
    } else {
        0.0
    };
    let err_text = format!("{:.0}%", error_rate);
    draw_text_ex(
        "err rate",
        x,
        y,
        TextParams {
            font,
            font_size: 25.0 as u16,
            color: Color::from_rgba(255, 255, 255, 80),
            ..Default::default()
        },
    );
    draw_text_ex(
        &err_text,
        x,
        y + 60.0,
        TextParams {
            font,
            font_size: 60,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
}

pub fn smooth(values: &[f64], window: usize, average_word_length: f64) -> Vec<f64> {
    let len = values.len();
    let mut smoothed = Vec::with_capacity(len+1);
    smoothed.push(0.0);

    for i in 0..len {
        let start = i.saturating_sub(window);
        let end = (i + window + 1).min(len);
        let slice = &values[start..end];

        let avg = slice.iter().sum::<f64>() / slice.len() as f64 / average_word_length;
        smoothed.push(avg);
    }

    smoothed
}

fn draw_chart(points: &[[f64; 2]], chart_width: f32, chart_height: f32, chart_x: f32, chart_y: f32, errors_per_second: &Vec<f64>) {
    let mut errors: Vec<f64> = Vec::new();
    errors.push(0.0);
    errors.extend(errors_per_second.iter().cloned());
    egui_macroquad::ui(|ctx| {
        Area::new("chart_area".into())
            .fixed_pos(pos2(chart_x, chart_y))
            .show(ctx, |ui| {
                let size = egui::Vec2::new(chart_width, chart_height);
                let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());
                ui.painter().rect_filled(rect, 5.0, Color32::from_rgb(20, 17, 15));

                let mut child_ui = ui.child_ui(rect, *ui.layout(), None);

                let grid_spacer = |input: egui_plot::GridInput| -> Vec<egui_plot::GridMark> {
                    let min = input.bounds.0;
                    let max = input.bounds.1;
                    let step = 20.0;
                    let mut marks = Vec::new();
                    
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

                let max_x = f64::max(points.len() as f64 - 1.0, 5.0);
                let mut max_y = 50.0;
                for point in points {
                    if point[1] > max_y {
                        max_y = point[1];
                    }
                }
                max_y += 10.0;

                Plot::new("performance_plot")
                    .include_y(0.0)
                    .show_background(false)
                    .show_axes([true, true])
                    .show_grid(true)
                    .view_aspect(5.0)
                    .x_axis_label("Time (s)")
                    .y_axis_label("Speed (WPM)")
                    .y_grid_spacer(grid_spacer)
                    .default_x_bounds(0.8, max_x)
                    .default_y_bounds(0.0, max_y)
                    .show(&mut child_ui, |plot_ui| {
                        let line = Line::new("Performance", points.to_vec())
                            .color(Color32::from_rgb(255, 155, 0))
                            .highlight(true)
                            .name("Performance");
                        plot_ui.line(line);

                        let error_points: Vec<([f64; 2], f32)> = errors
                            .iter()
                            .enumerate()
                            .filter_map(|(i, &val)| {
                                if val > 0.0 {
                                    Some(([i as f64, val + 5.0], 2.0 + (val as f32 * 1.5)))
                                } else {
                                    None
                                }
                            })
                            .collect();
                        for (point, radius) in error_points.iter() {
                            let num_errors = errors[point[0] as usize];
                            let dots = egui_plot::Points::new(format!("Errors at t={}", point[0]), vec![*point])
                                .color(Color32::RED)
                                .radius(*radius)
                                .shape(egui_plot::MarkerShape::Circle)
                                .name(format!("Errors: {}", num_errors as u32));
                            plot_ui.points(dots);
                        }
                    });
            });
    });
}