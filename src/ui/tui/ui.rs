use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use std::time::Duration;

use crate::ui::tui::app::{App, GameState};
use crate::ui::gui::results::smooth;

use std::fs::OpenOptions;
use std::io::Write;


const BORDER_COLOR: Color = Color::Rgb(100, 60, 0);
const REF_COLOR: Color = Color::Rgb(100, 100, 100);
const BG_COLOR: Color = Color::Black;
const MAIN_COLOR: Color = Color::Rgb(255, 155, 0);
const DIMMER_MAIN: Color = Color::Rgb(180, 100, 0);

fn render_instructions(frame: &mut Frame, area: Rect) {
    let text = Paragraph::new("  Press 'Esc' to exit.")
        .style(Style::default().fg(BORDER_COLOR).bg(BG_COLOR))
        .alignment(Alignment::Left);
    frame.render_widget(text, area);
}

pub fn render_app(frame: &mut Frame, app: &App, timer: Duration) {
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(frame.area());
    
    if app.game_state == GameState::Results {
        render_results(frame, chunks[0], app);
    }
    else {
        render_reference_frame(frame, chunks[0], &app, timer);
    }
    render_instructions(frame, chunks[1]);
}

fn render_results(frame: &mut Frame, area: Rect, app: &App) {
    frame.render_widget(
        Block::default().style(Style::default().bg(BG_COLOR)),
        area,
    );

    let wpm = (app.words_done as f32 / app.test_time) * 60.0;
    let wpm_line = Line::from(format!("WPM: {:.2}", wpm))
        .style(Style::default().fg(MAIN_COLOR).bg(BG_COLOR))
        .alignment(Alignment::Center);

    let correct_count = app.is_correct.iter().filter(|&&x| x == 2 || x == 1).count();
    let accuracy = if app.words_done > 0 {
        (correct_count as f32 / app.pressed_vec.len() as f32) * 100.0
    } else {
        0.0
    };
    let acc_line = Line::from(format!("Accuracy: {:.2}%", accuracy))
        .style(Style::default().fg(MAIN_COLOR).bg(BG_COLOR))
        .alignment(Alignment::Center);

    let data: Vec<(f64, f64)> = app.speed_per_second
        .iter()
        .enumerate()
        .map(|(i, &speed)| (i as f64 + 1.0, speed))
        .collect();

    let max_speed: f64 = app.speed_per_second.iter().fold(0.0_f64, |a, &b| a.max(b)).max(1.0);
    let max_time = data.len() as f64;

    let bar_dataset = Dataset::default()
        .graph_type(GraphType::Bar)
        .style(Style::default().fg(DIMMER_MAIN))
        .marker(symbols::Marker::Block)
        .data(&data);

    let chart = Chart::new(vec![bar_dataset])
        .bg(BG_COLOR)
        .x_axis(
            Axis::default()
                .title("Time (s)")
                .style(Style::default().fg(Color::White))
                .bounds([0.0, max_time])
                .labels(
                    (0..=max_time as usize)
                        .step_by(std::cmp::max(1, max_time as usize / 5))
                        .map(|i| Span::from(i.to_string()))
                        .collect::<Vec<Span>>(),
                ),
        )
        .y_axis(
            Axis::default()
                .title("CPM").labels_alignment(Alignment::Left)
                .style(Style::default().fg(Color::White))
                .bounds([0.0, max_speed * 1.1])
                .labels(vec![
                    Span::from("0"),
                    Span::from(format!("{:.0}", max_speed / 2.0)),
                    Span::from(format!("{:.0}", max_speed)),
                ]),
        );

    let block = create_reference_block(5);

    let inner_area = block.inner(area);
    let chunks = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(12),
    ]).split(inner_area);

    let stats = Paragraph::new(vec![wpm_line, acc_line])
        .style(Style::default().bg(BG_COLOR))
        .alignment(Alignment::Center);

    frame.render_widget(block, area);
    frame.render_widget(stats, chunks[0]);
    frame.render_widget(chart, chunks[1]);

}

fn render_reference_frame(frame: &mut Frame, area: Rect, app: &App, timer: Duration) {
    let max_ref_width = calculate_max_ref_width(area);
    let ref_padding = calculate_ref_padding(area, max_ref_width);
    
    let instruction_line = create_instruction_line(area, ref_padding, app);
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

fn create_instruction_line(area: Rect, ref_padding: u16, app: &App) -> Line<'static> {
    let divider = true;
    let mut button_states = vec![
        ("! punctuation", app.punctuation, !app.quote),
        ("# numbers", app.numbers, !app.quote),
        ("|", divider, true),
        ("time", app.time_mode, true),
        ("words", app.word_mode, true),
        ("quote", app.quote, true),
        ("|", divider, true),
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
    
    for i in 0..app.is_correct.len()-1 {
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
                    lines.push(current_line.trim().to_string());
                    current_line.clear();
                }
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            }
            if !current_line.is_empty() {
                lines.push(current_line.trim().to_string());
            }
            /*for _ in 0..len {
                lines.push(String::new()); // One empty line
                lines.push(String::new()); // Another empty line

            }*/
            lines
        })
        .collect()
}