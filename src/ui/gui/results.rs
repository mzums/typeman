use std::collections::VecDeque;
use macroquad::prelude::*;
use eframe::egui;
use egui::{Color32,  Area, pos2};
use egui_plot::{Line, Plot};

use crate::utils;
use crate::practice;

use std::fs::OpenOptions;
use std::io::Write;



fn calc_standard_deviation(values: &[f64], average_word_length: f64) -> f64 {
    let wpm_values: Vec<f64> = values.iter().map(|&cpm| cpm / average_word_length).collect();
    let mean = wpm_values.iter().sum::<f64>() / wpm_values.len() as f64;
    let variance = wpm_values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / wpm_values.len() as f64;
    variance.sqrt()
}

pub fn write_results(
    is_correct: &VecDeque<i32>,
    screen_width: f32,
    screen_height: f32,
    font: Option<&Font>,
    test_time: f32,
    speed_per_second: &Vec<f64>,
    average_word_length: f64,
    mode: &str,
    punctuation: bool,
    numbers: bool,
    errors_per_second: &Vec<f64>,
    reference: &String,
    practice_level: Option<usize>,
    saved_results: &mut bool,
) {
    let (no_corrected_words, correct_words, all_words) = utils::count_correct_words(&reference, &is_correct);
    let error_count1 = is_correct.iter().filter(|&&v| v == 1 || v == -1).count();
    let error_count2 = is_correct.iter().filter(|&&v| v == -1).count();
    let accuracy = if practice_level.is_some() {
        (100.0 - (error_count1 as f64 / (all_words as f64 * 6.0)) * 100.0).round()
    } else {
        (100.0 - (error_count2 as f64 / (all_words as f64 * 6.0)) * 100.0).round()
    };

    let wpm = if practice_level.is_some() {
        (no_corrected_words as f32 / (test_time / 60.0)).round()
    } else {
        (correct_words as f32 / (test_time / 60.0)).round()
    };
    let raw = all_words as f32 / (test_time / 60.0);
    
    let chart_width = f32::min(f32::max(0.8 * f32::min(screen_width, screen_height), 0.6 * screen_width), 1800.0);
    let chart_height: f32 = f32::min(chart_width / 5.0, 360.0);
    
    let fontsize_1 = (chart_height / 3.0) as u16;
    let fontsize_2 = (chart_height / 7.0) as u16;
    let fontsize_3 = (chart_height / 5.0) as u16;
    let fontsize_4 = (chart_height / 12.0) as u16;
    
    let text_size = {
        let a = measure_text(&format!("{:0}%", accuracy), font, fontsize_1, 1.0);
        let b = measure_text(&format!("{:0}", wpm), font, fontsize_1 , 1.0);
        if a.width > b.width { a } else { b }
    };

    let chart_x = (screen_width - chart_width + text_size.width + 20.0) / 2.0;
    let chart_y = (screen_height - chart_height) / 4.0;
    
    let text2_width = measure_text("consistency", font, 25, 1.0).width;
    let padding = (chart_width - 4.0 * text2_width) / 4.0;


    let avg_wpm = write_wpm(
        font,
        (screen_width - chart_width - text_size.width) / 2.0,
        (screen_height - chart_height) / 3.8,
        wpm,
        fontsize_1,
        fontsize_2,
    );
    write_acc(
        accuracy,
        font,
        (screen_width - chart_width - text_size.width) / 2.0,
        (screen_height - chart_height) / 3.8 + text_size.height * 2.0,
        fontsize_1,
        fontsize_2,
    );
    write_raw_wpm(
        raw,
        font,
        (screen_width - chart_width + 2.0 * text_size.width) / 2.0,
        chart_y + chart_height + fontsize_4 as f32 * 1.5,
        fontsize_3,
        fontsize_4,
    );
    write_consistency(
        speed_per_second,
        average_word_length,
        font,
        (screen_width - chart_width + 2.0 * text_size.width) / 2.0 + text2_width + padding,
        chart_y + chart_height + fontsize_4 as f32 * 1.5,
        avg_wpm,
        fontsize_3,
        fontsize_4,
    );
    write_time(
        test_time,
        font,
        (screen_width - chart_width + 2.0 * text_size.width) / 2.0 + (text2_width + padding) * 2.0,
        chart_y + chart_height + fontsize_4 as f32 * 1.5,
        fontsize_3,
        fontsize_4,
    );
    write_mode(
        font,
        (screen_width - chart_width + 2.0 * text_size.width) / 2.0 + (text2_width + padding) * 3.0,
        chart_y + chart_height + fontsize_4 as f32 * 1.5,
        mode,
        punctuation,
        numbers,
        fontsize_3,
        fontsize_4,
        practice_level
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
    
    if practice_level.is_some() {
        let practice_text = if wpm >= practice::WPM_MIN as f32 {
            "Congratulations! You passed this level.".to_string()
        } else {
            format!("You need at least {} WPM to pass this level.", practice::WPM_MIN)
        };
        let text_size = measure_text(&practice_text, font, 30, 1.0);

        draw_text_ex(practice_text.as_str(), (screen_width - text_size.width) / 2.0, chart_y + chart_height + 200.0, TextParams { font: font, font_size: 30, font_scale: 1.0, color: Color::from_rgba(255, 255, 255, 100), ..Default::default() });
        if practice::get_prev_best_wpm(practice_level.unwrap() + 1) < wpm as f64 {
            let new_highscore_text = "New highscore for this level!";
            let text_size = measure_text(new_highscore_text, font, 30, 1.0);
            draw_text_ex(new_highscore_text, (screen_width - text_size.width) / 2.0, chart_y + chart_height + 250.0, TextParams { font: font, font_size: 30, font_scale: 1.0, color: Color::from_rgba(255, 255, 255, 100), ..Default::default() });
        }
        if !*saved_results {
            *saved_results = true;
            practice::save_results(
                test_time as f64,
                accuracy as f64,
                wpm as f64,
                practice_level.unwrap() + 1,
            );
        }
    }
}

fn write_mode(
    font: Option<&Font>,
    x: f32,
    y: f32,
    mode: &str,
    punctuation: bool,
    numbers: bool,
    fontsize_3: u16,
    fontsize_4: u16,
    practice_level: Option<usize>,
) {
    let mut fontsize_3 = fontsize_3;
    let fontsize_4 = fontsize_4;
    let mode_text = format!("{mode}");
    let mut punct_pos: f32 = y;
    let mut number_pos: f32 = y;
    let mut mode_pos: f32 = y + fontsize_3 as f32;
    let level_pos = y + fontsize_3 as f32 * 1.25;
    draw_text_ex(
        "mode",
        x,
        y,
        TextParams {
            font,
            font_size: fontsize_4,
            color: Color::from_rgba(255, 255, 255, 80),
            ..Default::default()
        },
    );
    if punctuation && numbers{
        punct_pos = y + fontsize_3 as f32 * 1.2;
        number_pos = y + fontsize_3 as f32 * 1.65;
        fontsize_3 = (fontsize_3 as f32 * 0.7) as u16;
        mode_pos = y + fontsize_3 as f32;
    }
    else if punctuation || numbers {
        fontsize_3 = (fontsize_3 as f32 * 0.8) as u16;
        mode_pos = y + fontsize_3 as f32;
        if punctuation {
            punct_pos = y + fontsize_3 as f32 * 1.65;
        } else {
            number_pos = y + fontsize_3 as f32 * 1.65;
        }
    } else if practice_level.is_some() {
        fontsize_3 = (fontsize_3 as f32 * 0.65) as u16;
        mode_pos = y + fontsize_3 as f32 * 0.9;
    }

    draw_text_ex(
        &mode_text,
        x,
        mode_pos,
        TextParams {
            font,
            font_size: fontsize_3,
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
                font_size: fontsize_4,
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
                font_size: fontsize_4,
                color: Color::from_rgba(255, 155, 0, 255),
                ..Default::default()
            },
        );
    }
    if practice_level.is_some() && mode == "practice" {
        let level_text = format!("level {}", practice_level.unwrap() + 1);
        draw_text_ex(
            &level_text,
            x,
            level_pos,
            TextParams {
                font,
                font_size: fontsize_4,
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
    fontsize_3: u16,
    fontsize_4: u16,
) {
    let time_text = format!("{:.0}s", test_time);
    draw_text_ex(
        "time",
        x,
        y,
        TextParams {
            font,
            font_size: fontsize_4,
            color: Color::from_rgba(255, 255, 255, 80),
            ..Default::default()
        },
    );
    draw_text_ex(
        &time_text,
        x,
        y + fontsize_3 as f32,
        TextParams {
            font,
            font_size: fontsize_3,
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
    fontsize_3: u16,
    fontsize_4: u16,
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
            font_size: fontsize_4,
            color: Color::from_rgba(255, 255, 255, 80),
            ..Default::default()
        },
    );
    draw_text_ex(
        &consistency_text,
        x,
        y + fontsize_3 as f32,
        TextParams {
            font,
            font_size: fontsize_3,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
}

fn write_wpm(
    font: Option<&Font>,
    x: f32,
    y: f32,
    wpm: f32,
    fontsize_1: u16,
    fontsize_2: u16,
) -> f32 {
    let wpm_text = format!("{:.0}", wpm);
    draw_text_ex(
        "wpm",
        x,
        y,
        TextParams {
            font,
            font_size: fontsize_2,
            color: Color::from_rgba(255, 255, 255, 80),
            ..Default::default()
        },
    );
    draw_text_ex(
        &wpm_text,
        x,
        y + fontsize_1 as f32,
        TextParams {
            font,
            font_size: fontsize_1,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
    return wpm;
}

fn write_acc(accuracy: f64, font: Option<&Font>, x: f32, y: f32, fontsize_1: u16, fontsize_2: u16) {
    let acc_text = format!("{:.0}%", accuracy);
    draw_text_ex(
        "acc",
        x,
        y,
        TextParams {
            font,
            font_size: fontsize_2,
            color: Color::from_rgba(255, 255, 255, 80),
            ..Default::default()
        },
    );
    draw_text_ex(
        &acc_text,
        x,
        y + fontsize_1 as f32,
        TextParams {
            font,
            font_size: fontsize_1,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
}

fn write_raw_wpm(raw_wpm: f32, font: Option<&Font>, x: f32, y: f32, fontsize_3: u16, fontsize_4: u16) {
    let raw_text = format!("{:.0}", raw_wpm);
    draw_text_ex(
        "raw wpm",
        x,
        y,
        TextParams {
            font,
            font_size: fontsize_4,
            color: Color::from_rgba(255, 255, 255, 80),
            ..Default::default()
        },
    );
    draw_text_ex(
        &raw_text,
        x,
        y + fontsize_3 as f32,
        TextParams {
            font,
            font_size: fontsize_3,
            color: Color::from_rgba(255, 155, 0, 255),
            ..Default::default()
        },
    );
}

fn smooth(values: &[f64], window: usize, average_word_length: f64) -> Vec<f64> {
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
    smoothed[0] = smoothed[1];

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

                        let mut last_drawn_pixel_x: Option<f32> = None;
                        for (i, &val) in errors.iter().enumerate() {
                            if val > 0.0 {
                                if let Some(&[x, _]) = points.get(i) {
                                    let screen_pos = plot_ui.screen_from_plot(egui_plot::PlotPoint::new(x, 0.0));
                                    let current_pixel_x = screen_pos.x;

                                    let should_draw = match last_drawn_pixel_x {
                                        None => true,
                                        Some(last_x) => (current_pixel_x - last_x).abs() >= 10.0,
                                    };

                                    if should_draw {
                                        let size = if val <= 1.0 {
                                            5.0
                                        } else if val <= 2.0 {
                                            8.0
                                        } else if val <= 3.0 {
                                            11.0
                                        } else {
                                            15.0
                                        };
                                        
                                        let cross_y = size;
                                        
                                        let cross = egui_plot::Points::new(
                                            "Error",
                                            vec![[x, cross_y]]
                                        )
                                        .color(Color32::from_rgb(255, 50, 50))
                                        .radius(size as f32)
                                        .shape(egui_plot::MarkerShape::Cross);
                                        
                                        plot_ui.points(cross);
                                        last_drawn_pixel_x = Some(current_pixel_x);
                                    }
                                }
                            }
                        }
                    });
            });
    });
}