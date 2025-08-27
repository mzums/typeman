use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use std::time::Duration;
use std::collections::HashMap;

use ratatui::widgets::canvas::Canvas;
use crate::ui::tui::app::{App, GameState};
use crate::practice::TYPING_LEVELS;
use crate::practice;
use crate::utils;
use std::fs::OpenOptions;
use std::io::Write;


const BORDER_COLOR: Color = Color::Rgb(100, 60, 0);
const REF_COLOR: Color = Color::Rgb(100, 100, 100);
const BG_COLOR: Color = Color::Rgb(10, 10, 10);
const MAIN_COLOR: Color = Color::Rgb(255, 155, 0);
const DIMMER_MAIN: Color = Color::Rgb(180, 100, 0);

fn render_instructions(frame: &mut Frame, area: Rect, show: bool, practice_menu: bool) {
    let mut lines = Vec::new();
    if show {
        lines.push(Line::from("  \u{2191} - enter config, \u{2190}/\u{2192} - toggle config, ↵ - apply config"));
    } else if practice_menu {
        lines.push(Line::from("  ↑ or ↓ to navigate, ↵ to select"));
        lines.push(Line::from("  q - quit menu"));
    }
    if !practice_menu {
        lines.push(Line::from("  Tab + Enter - restart"));
    }
    lines.push(Line::from("  Esc - exit"));

    let text = Paragraph::new(lines)
        .style(Style::default().fg(BORDER_COLOR).bg(BG_COLOR))
        .alignment(Alignment::Left);
    frame.render_widget(text, area);
}

pub fn render_app(frame: &mut Frame, app: &App, timer: Duration) {
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            if app.game_state == GameState::Results {
                Constraint::Length(2)
            } else {
                Constraint::Length(3)
            },
        ])
        .split(frame.area());
    
    if app.game_state == GameState::Results {
        render_results(frame, chunks[0], app);
    } else if app.practice_menu {
        render_practice_menu(frame, chunks[0], &app);
    }
    else {
        render_reference_frame(frame, chunks[0], &app, timer);
    }
    render_instructions(frame, chunks[1], app.game_state != GameState::Results && !app.practice_menu, app.practice_menu);
}

fn render_practice_menu(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = Vec::new();

    let block = create_reference_block(3);
    let inner_area = block.inner(area);
    let chunks = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(0),
    ]).split(inner_area);

    let to_skip = if chunks[1].height <= app.selected_level as u16 {
        u16::min(chunks[1].height, app.selected_level as u16)
    } else {
        0
    };
    for level in TYPING_LEVELS.iter().enumerate().skip(to_skip as usize) {
        let mut fg_color = REF_COLOR;
        let mut bg_color = BG_COLOR;
        if app.selected_level == level.0 {
            fg_color = BG_COLOR;
            bg_color = Color::Rgb(150, 90, 0);
        }
        let line = if practice::check_if_completed(&format!("practice_results/level_{}.txt", level.0 + 1)) {
            Line::from(vec![
                Span::styled("✔ ", Style::default().fg(Color::Rgb(0, 255, 0)).bg(BG_COLOR)),
                if level.0 < 9 {
                    Span::styled(format!("  {}. {} ", level.0 + 1, level.1.0), Style::default().fg(fg_color).bg(bg_color))
                } else {
                    Span::styled(format!(" {}. {} ", level.0 + 1, level.1.0), Style::default().fg(fg_color).bg(bg_color))
                }
            ])
        } else {
            Line::from(vec![
                Span::styled("  ", Style::default().fg(Color::Rgb(0, 255, 0)).bg(BG_COLOR)),
                if level.0 < 9 {
                    Span::styled(format!("  {}. {} ", level.0 + 1, level.1.0), Style::default().fg(fg_color).bg(bg_color))
                } else {
                    Span::styled(format!(" {}. {} ", level.0 + 1, level.1.0), Style::default().fg(fg_color).bg(bg_color))
                }
            ])
        };
        lines.push(line);
    }

    let text = Paragraph::new(lines)
        .style(Style::default())
        .alignment(Alignment::Left);

    let title = Line::from("Select practice level")
        .style(Style::default().fg(MAIN_COLOR).bg(BG_COLOR))
        .alignment(Alignment::Center);

    frame.render_widget(block, area);
    frame.render_widget(title, chunks[0]);
    frame.render_widget(text, chunks[1]);
}

fn smooth(
    values: &[f64],
    average_word_length: f64,
    extra_columns: usize,
) -> Vec<f64> {
    let len = values.len();
    let mut smoothed = Vec::with_capacity((extra_columns + 1) * len);

    if len < 2 {
        for _ in 0..(len * extra_columns) {
            smoothed.push(values.get(0).copied().unwrap_or(0.0) / average_word_length);
        }
        return smoothed;
    }

    let get = |idx: isize| -> f64 {
        let i = idx.clamp(0, (len - 1) as isize) as usize;
        values[i] / average_word_length
    };

    for i in 0..len - 1 {
        let p0 = get(i as isize - 1);
        let p1 = get(i as isize);
        let p2 = get(i as isize + 1);
        let p3 = get(i as isize + 2);

        for j in 0..extra_columns {
            let t = j as f64 / extra_columns as f64;
            let t2 = t * t;
            let t3 = t2 * t;
            let interp = 0.5 * (
                2.0 * p1 +
                (p2 - p0) * t +
                (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2 +
                (3.0 * p1 - p0 - 3.0 * p2 + p3) * t3
            );
            smoothed.push(interp);
        }
    }
    let last = get((len - 1) as isize);
    for _ in 0..extra_columns {
        smoothed.push(last);
    }
    smoothed
}

fn calc_standard_deviation(values: &[f64], average_word_length: f64) -> f64 {
    let wpm_values: Vec<f64> = values.iter().map(|&cpm| cpm / average_word_length).collect();
    let mean = wpm_values.iter().sum::<f64>() / wpm_values.len() as f64;
    let variance = wpm_values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / wpm_values.len() as f64;
    variance.sqrt()
}

fn get_stats(app: &App) -> (Line<'static>, Line<'static>) {
    let wpm = (app.words_done as f32 / app.test_time) * 60.0;
    let wpm_str = format!("{}", wpm as i32);

    let accuracy = if app.words_done > 0 {
        (app.correct_count as f32 / app.pressed_vec.len() as f32) * 100.0
    } else {
        0.0
    };
    let acc_str = format!("{}%", accuracy.round());

    let (_, _, all_words) = utils::count_correct_words(&app.reference, &app.is_correct.iter().cloned().collect());
    let raw = all_words as f32 / (app.test_time / 60.0);

    let raw_str = format!("{}%", raw.round());

    let standard_deviation = calc_standard_deviation(&app.speed_per_second, 6.0);
    let consistency = if wpm > 0.0 {
        (100.0 - (standard_deviation / wpm as f64 * 100.0).round()).max(0.0)
    } else {
        0.0
    };
    let consistency_str = format!("{consistency}%");

    let time_str = format!("{:.0}s", app.timer.as_secs_f32());

    let mut mode_str = if app.time_mode {
        "time".to_string()
    } else if app.word_mode {
        "words".to_string()
    } else if app.quote {
        "quote".to_string()
    } else {
        "practice".to_string()
    };
    if app.punctuation {
        mode_str += " !";
    }
    if app.numbers {
        mode_str += " #";
    }

    let label_style = Style::default().fg(REF_COLOR).bg(BG_COLOR);
    let value_style = Style::default().fg(MAIN_COLOR).bg(BG_COLOR);
    let space_style = Style::default().bg(BG_COLOR);

    let col_widths = [3, 4, 4, 4, 4, 8];

    let labels = ["wpm", "acc", "err", "cons", "time", "mode"];
    let values = [
        format!("{:<3}", wpm_str),
        format!("{:<4}", acc_str),
        format!("{:<4}", raw_str),
        format!("{:<4}", consistency_str),
        format!("{:<4}", time_str),
        format!("{:<8}", mode_str),
    ];

    let mut label_spans = vec![Span::styled("    ", space_style)];
    let mut value_spans = vec![Span::styled("    ", space_style)];
    if app.punctuation && app.numbers {
        label_spans = vec![Span::styled("  ", space_style)];
        value_spans = vec![Span::styled("  ", space_style)];
    } else if app.punctuation || app.numbers {
        label_spans = vec![Span::styled("    ", space_style)];
        value_spans = vec![Span::styled("    ", space_style)];
    }
    label_spans.extend(
        labels.iter().zip(col_widths.iter()).enumerate().flat_map(|(i, (label, width))| {
            let mut spans = Vec::new();
            if i > 0 {
                spans.push(Span::styled("  ", space_style));
            }
            spans.push(Span::styled(format!("{:<width$}", label, width = *width), label_style));
            spans
        })
    );

    value_spans.extend(
        values.iter().zip(col_widths.iter()).enumerate().flat_map(|(i, (val, width))| {
            let mut spans = Vec::new();
            if i > 0 {
                spans.push(Span::styled("  ", space_style));
            }
            spans.push(Span::styled(format!("{:<width$}", val, width = *width), value_style));
            spans
        })
    );

    (
        Line::from(label_spans).alignment(Alignment::Center),
        Line::from(value_spans).alignment(Alignment::Center),
    )
}

fn get_chart(smoothed_speeds: &[f64], app: &App) -> Chart<'static> {
    let data: Vec<(f64, f64)> = smoothed_speeds
        .iter()
        .enumerate()
        .map(|(i, &speed)| (i as f64 + 1.0, speed))
        .collect();

    let data: &'static [(f64, f64)] = Box::leak(data.into_boxed_slice());

    let max_speed: f64 = f64::max(70.0, app.speed_per_second.iter().fold(0.0_f64, |a, &b| a.max(b)).max(1.0) / 6.0 + 30.0);
    let max_time = app.timer.as_secs_f32().ceil() as f64;

    let bar_dataset = Dataset::default()
        .graph_type(GraphType::Bar)
        .style(Style::default().fg(Color::Rgb(150, 80, 0)).bg(BG_COLOR))
        .marker(symbols::Marker::HalfBlock)
        .data(data);
    
    let chart = Chart::new(vec![bar_dataset])
    .block(Block::default().style(Style::default().bg(BG_COLOR)))
        .bg(BG_COLOR)
        .style(Style::default().bg(BG_COLOR))
        .x_axis(
            Axis::default()
                .style(Style::default().fg(REF_COLOR))
                .bounds([0.0, smoothed_speeds.len() as f64])
                .labels(
                    (0..=max_time as usize)
                        .step_by(5)
                        .map(|i| Span::styled(format!("{i}s"), Style::default().fg(REF_COLOR)))
                        .collect::<Vec<Span>>(),
                ),
        )
        .y_axis(
            Axis::default()
                .title("wpm")
                .labels_alignment(ratatui::layout::Alignment::Left)
                .style(Style::default().fg(REF_COLOR))
                .bounds([0.0, max_speed * 1.1])
                .labels(vec![
                    Span::from("0").style(Style::default().fg(REF_COLOR)),
                    Span::from(format!("{:.0}", max_speed / 2.0)).style(Style::default().fg(REF_COLOR)),
                    Span::from(format!("{:.0}", max_speed)).style(Style::default().fg(REF_COLOR)),
                ]),
        );
    return chart;
}

fn render_results(frame: &mut Frame, area: Rect, app: &App) {
    frame.render_widget(
        Block::default().style(Style::default().bg(BG_COLOR)),
        area,
    );

    let (wpm_line, acc_line) = get_stats(app);

    let columns_for_sec: HashMap<u32, usize> = [(5, 4), (15, 3), (30, 2), (60, 1)]
        .iter()
        .cloned()
        .collect();

    let test_time = app.timer.as_secs_f32().round() as u32;
    let mut extra_columns = columns_for_sec
        .keys()
        .filter(|&&k| k >= test_time)
        .min()
        .or_else(|| columns_for_sec.keys().max())
        .map(|k| columns_for_sec[k])
        .unwrap_or(1);

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("lposition.log") {
        let _ = writeln!(file, "area.width: {}", area.width);
    }
    
    if area.width < 70 {
        if test_time >= 5 {
            extra_columns = 1;
        }
    } else if area.width < 100 {
        if test_time >= 15 {
            extra_columns = 1;
        } else if test_time >= 5 {
            extra_columns = 2;
        }
    } else if area.width < 150 {
        if test_time >= 30 {
            extra_columns = 1;
        } else if test_time >= 15 {
            extra_columns = 2;
        } else if test_time >= 5 {
            extra_columns = 3;
        }
    }

    let mut errors_per_second: Vec<f32> = Vec::new();
    let mut speed_per_second: Vec<f64> = Vec::new();
    let mut prev = 0.0;

    if test_time >= 120 {
        let mut errs = app.errors_per_second.clone();
        for (i, err) in errs.iter_mut().enumerate() {
            if i % 2 == 0 {
                errors_per_second.push(*err);
                speed_per_second.push(app.speed_per_second[i]);
            }
            prev = *err;
        }
    } else {
        errors_per_second = app.errors_per_second.clone();
        speed_per_second = app.speed_per_second.clone();
    }

    for (i, err) in errors_per_second.iter_mut().enumerate() {
        let prev_val = prev;
        if i > 0 && *err > 0.0 && prev_val > 0.0 {
            *err = 0.0;
        }
        prev = *err;
    }
    errors_per_second[0] = 0.0;
    if errors_per_second.len() > 1 {
        errors_per_second[1] = 0.0;
    }

    let smoothed_speeds = smooth(
        &speed_per_second,
        6.0,
        extra_columns,
    );

    let chart = get_chart(&smoothed_speeds, app);

    let block = create_reference_block(5);

    let inner_area = block.inner(area);

    let chart_height = 12u16;
    let y_offset = if inner_area.height > chart_height {
        (inner_area.height - chart_height) / 2
    } else {
        0
    };
    let centered_area = Rect {
        x: inner_area.x,
        y: inner_area.y + y_offset,
        width: inner_area.width,
        height: chart_height.min(inner_area.height),
    };

    let chunks = Layout::vertical([
        Constraint::Length(9),
        Constraint::Length(1),
        Constraint::Length(3),
    ]).split(centered_area);

    let max_chart_width: u16 = 2 * smoothed_speeds.len() as u16 + 4;
    let chart_area = {
        let mut area = chunks[0];
        if area.width > max_chart_width {
            let padding = (area.width - max_chart_width) / 2;
            area.x += padding;
            area.width = max_chart_width;
        }
        area
    };

    let stats = Paragraph::new(vec![wpm_line, acc_line])
        .style(Style::default().bg(BG_COLOR))
        .alignment(Alignment::Center);

    let empty_line = Line::from("");

    frame.render_widget(block, area);
    frame.render_widget(
        Block::default().style(Style::default().bg(BG_COLOR)),
        chart_area,
    );
    frame.render_widget(chart, chart_area);

    let max_speed = f64::max(
        70.0,
        app.speed_per_second
            .iter()
            .fold(0.0_f64, |a, &b| a.max(b))
            .max(1.0) / 6.0
            + 30.0,
    );

    let canvas = Canvas::default()
        .block(Block::default().style(Style::default().bg(BG_COLOR)))
        .x_bounds([0.0, smoothed_speeds.len() as f64])
        .y_bounds([0.0, max_speed * 1.1])
        .background_color(BG_COLOR)
        .paint(|ctx| {
            for (i, err) in errors_per_second.iter().enumerate() {
                let cross: &str;
                if *err <= 1.0 {
                    cross = "\u{00D7}";
                } else if *err <= 2.0 {
                    cross = "\u{2715}";
                } else if *err <= 3.0 {
                    cross = "\u{2716}";
                } else {
                    cross = "\u{274C}";
                }
                if *err > 0.0 {
                    ctx.print(
                        (i as f64 + 0.8) * (smoothed_speeds.len() as f64 / test_time as f64),
                        1.0,
                        Span::styled(cross, Style::default().fg(Color::Red).bg(BG_COLOR)),
                    );
                }
            }
        });

    frame.render_widget(canvas, chart_area);

    frame.render_widget(empty_line, chunks[1]);
    frame.render_widget(stats, chunks[2]);
}

fn render_reference_frame(frame: &mut Frame, area: Rect, app: &App, timer: Duration) {
    let max_ref_width = calculate_max_ref_width(area);
    let ref_padding = calculate_ref_padding(area, max_ref_width);

    let instruction_line = create_config_line(app);
    let horizontal_line = create_horizontal_line(area);
    let time_words = if app.time_mode {
        create_timer(timer, app.test_time)
    } else {
        create_words_count(app.batch_size, app.words_done)
    };
    let colored_lines = create_colored_lines(app, max_ref_width);
    let empty_space = calculate_vertical_padding(area, colored_lines.len());

    let content = assemble_content(
        instruction_line,
        horizontal_line,
        time_words,
        colored_lines,
        empty_space
        
    );

    let block = create_reference_block(ref_padding);
    let paragraph = Paragraph::new(content)
        .block(block)
        .style(Style::default().bg(BG_COLOR));

    frame.render_widget(paragraph, area);
}

fn calculate_max_ref_width(area: Rect) -> usize {
    usize::min(area.width as usize - 15, 150)
}

fn calculate_ref_padding(area: Rect, max_ref_width: usize) -> u16 {
    if area.width > max_ref_width as u16 {
        (area.width - max_ref_width as u16) / 2
    } else {
        7
    }
}

fn create_timer(timer: Duration, test_time: f32) -> Line<'static> {
    let seconds = test_time - timer.as_secs() as f32;
    let formatted_time = format!("{:?}", seconds as i32);
    
    Line::from(formatted_time)
        .style(Style::default().fg(MAIN_COLOR).bg(BG_COLOR))
        .alignment(Alignment::Left)
}

fn create_words_count(all_words: usize, typed_words: usize) -> Line<'static> {
    let words_text = format!("{}/{}", typed_words, all_words);
    Line::from(words_text)
        .style(Style::default().fg(MAIN_COLOR).bg(BG_COLOR))
        .alignment(Alignment::Left)
}

fn create_config_line( app: &App) -> Line<'static> {
    let divider = true;
    let mut button_states = vec![
        ("! punctuation", app.punctuation, !app.quote && !app.practice_mode),
        ("# numbers", app.numbers, !app.quote && !app.practice_mode),
        ("|", divider, app.word_mode || app.time_mode),
        ("time", app.time_mode, true),
        ("words", app.word_mode, true),
        ("quote", app.quote, true),
        ("practice", app.practice_mode, true),
        ("|", divider, app.word_mode || app.time_mode),
        ("15", app.test_time == 15.0, app.time_mode),
        ("30", app.test_time == 30.0, app.time_mode),
        ("60", app.test_time == 60.0, app.time_mode),
        ("120", app.test_time == 120.0, app.time_mode),
        ("25", app.batch_size == 25, app.word_mode),
        ("50", app.batch_size == 50, app.word_mode),
        ("100", app.batch_size == 100, app.word_mode),
    ];

    let mut spans: Vec<Span<'static>> = vec![];

    let mut fg_colors = vec![REF_COLOR; button_states.len()];
    let mut bg_colors = vec![BG_COLOR; button_states.len()];
    for (i, (label, state_val, visible)) in button_states.iter_mut().enumerate() {
        if !*visible {
            continue;
        }
        if *state_val && app.selected_config == *label && app.config && *label != "|" {
            bg_colors[i] = DIMMER_MAIN;
            fg_colors[i] = BG_COLOR;
        } else if app.selected_config == *label && app.config && *label != "|" {
            bg_colors[i] = BORDER_COLOR;
            fg_colors[i] = BG_COLOR;
        } else if *state_val {
            fg_colors[i] = MAIN_COLOR;
        } else {
            fg_colors[i] = REF_COLOR;
        }
        spans.push(Span::styled(
            format!(" {} ", label),
            Style::default().fg(fg_colors[i]).bg(bg_colors[i]),
        ));
    }

    Line::from(spans).alignment(Alignment::Center)
}

fn create_horizontal_line(area: Rect) -> Line<'static> {
    Line::from("─".repeat(area.width.saturating_sub(15) as usize)
        .fg(BORDER_COLOR)
        .bg(BG_COLOR))
}

fn create_colored_lines<'a>(app: &App, max_ref_width: usize) -> Vec<Line<'a>> {
    let mut fg_colors: Vec<Color> = vec![REF_COLOR; app.reference.chars().count()];
    let mut bg_colors: Vec<Color> = vec![BG_COLOR; app.reference.chars().count()];

    for i in 0..app.is_correct.len() {
        if app.pos1 == i {
            fg_colors[i] = BG_COLOR;
            bg_colors[i] = MAIN_COLOR
        } else if app.is_correct[i] == 0 || i >= app.pos1{
            fg_colors[i] = REF_COLOR;
        } else if app.is_correct[i] == 2 {
            fg_colors[i] = Color::White;
        } else if app.is_correct[i] == 1 {
            fg_colors[i] = MAIN_COLOR;
        } else if app.is_correct[i] == -1 {
            fg_colors[i] = Color::Rgb(255, 0, 0);
        } else {
            fg_colors[i] = REF_COLOR;
        }
    }

    let split = split_lines(&app.reference, max_ref_width);
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("lposition.log") {
        let _ = writeln!(file, "{:?}|", split[0]);
    }

    let mut char_index = 0;
    split.into_iter()
        .map(|line| {
            let spans: Vec<Span<'a>> = line
                .chars()
                .map(|c| {
                    let fg_color = fg_colors.get(char_index).cloned().unwrap_or(REF_COLOR);
                    let bg_color = bg_colors.get(char_index).cloned().unwrap_or(BG_COLOR);
                    char_index += 1;
                    Span::styled(c.to_string(), Style::default().fg(fg_color).bg(bg_color))
                })
                .collect();
            Line::from(spans)
        })
        .collect()
}

fn calculate_vertical_padding(area: Rect, content_lines: usize) -> usize {
    let empty_space = (area.height as u16).saturating_sub(3) as usize / 2;
    empty_space.saturating_sub(content_lines / 2) - 3
}

fn assemble_content<'a>(
    instruction_line: Line<'a>,
    horizontal_line: Line<'a>,
    timer: Line<'a>,
    colored_lines: Vec<Line<'a>>,
    empty_space: usize
) -> Vec<Line<'a>> {
    let empty_line = Line::from("");
    let mut content = vec![
        empty_line.clone(),
        instruction_line,
        horizontal_line,
        empty_line.clone(),
    ];
    
    content.extend(vec![empty_line.clone(); empty_space]);
    content.push(timer);
    content.push(empty_line.clone());
    content.extend(colored_lines);
    content
}

fn create_reference_block(ref_padding: u16) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BORDER_COLOR).bg(BG_COLOR))
        .style(Style::default().bg(BG_COLOR))
        .title(Line::from(vec![
            " Type".fg(MAIN_COLOR).bg(BG_COLOR),
            "Man ".fg(Color::White).bg(BG_COLOR),
        ]))
        .padding(Padding {
            left: ref_padding,
            right: ref_padding.saturating_sub(2),
            top: 0,
            bottom: 0,
        })
        .title_alignment(Alignment::Left)
}

fn split_lines(text: &str, width: usize) -> Vec<String> {
    text.lines()
        .flat_map(|line| {
            let mut words = line.split_whitespace();
            
            let mut current_line = String::new();
            let mut lines = Vec::new();
            
            while let Some(word) = words.next() {
                if current_line.len() + word.len() + 1 > width {
                    lines.push(current_line.to_string());
                    current_line.clear();
                }
                current_line.push_str(&format!("{} ", word));
            }
            if !current_line.is_empty() {
                lines.push(current_line.to_string());
            }
            lines
        })
        .collect()
}
