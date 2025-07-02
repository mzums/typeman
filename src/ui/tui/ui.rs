use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use std::time::Duration;

use crate::ui::tui::app::App;

use std::fs::OpenOptions;
use std::io::Write;


const BORDER_COLOR: Color = Color::Rgb(100, 60, 0);
const REF_COLOR: Color = Color::Rgb(100, 100, 100);
const BG_COLOR: Color = Color::Black;
const MAIN_COLOR: Color = Color::Rgb(255, 155, 0);

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
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_reference_frame(frame, chunks[0], &app, timer);
    render_instructions(frame, chunks[1]);
}

fn render_reference_frame(frame: &mut Frame, area: Rect, app: &App, timer: Duration) {
    let max_ref_width = calculate_max_ref_width(area);
    let ref_padding = calculate_ref_padding(area, max_ref_width);
    
    let instruction_line = create_instruction_line(area, ref_padding);
    let horizontal_line = create_horizontal_line(area);
    let timer_line = create_timer(timer, app.test_time);
    let colored_lines = create_colored_lines(app, max_ref_width);
    let empty_space = calculate_vertical_padding(area, colored_lines.len());

    let content = assemble_content(
        instruction_line, 
        horizontal_line,
        timer_line,
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

fn create_timer(timer: Duration, test_time: Duration) -> Line<'static> {
    let seconds = test_time.as_secs().saturating_sub(timer.as_secs());
    let formatted_time = format!("{:?}", seconds);
    
    Line::from(formatted_time)
        .style(Style::default().fg(MAIN_COLOR).bg(BG_COLOR))
        .alignment(Alignment::Left)
}

fn create_instruction_line(area: Rect, ref_padding: u16) -> Line<'static> {
    let instruction_text = "! punctuation  # numbers | time  words  quote | 15 30 60 120";
    let available_width = area.width.saturating_sub(ref_padding * 2) as usize;
    let padding = (available_width.saturating_sub(instruction_text.len())) / 2;
    let padded_instruction = format!("{:width$}{}", "", instruction_text, width = padding);
    
    Line::from(padded_instruction)
        .style(style::Style::default().fg(REF_COLOR).bg(BG_COLOR))
}

fn create_horizontal_line(area: Rect) -> Line<'static> {
    Line::from("─".repeat(area.width.saturating_sub(15) as usize)
        .fg(BORDER_COLOR)
        .bg(BG_COLOR))
}

fn create_colored_lines<'a>(app: &App, max_ref_width: usize) -> Vec<Line<'a>> {
    let mut colors: Vec<Color> = vec![REF_COLOR; app.reference.chars().count()];
    
    for i in 0..app.is_correct.len()-1 {
        if app.is_correct[i] == 0 || i >= app.pos1{
            colors[i] = REF_COLOR;
        } else if app.is_correct[i] == 2 {
            colors[i] = Color::White;
        } else if app.is_correct[i] == 1 {
            colors[i] = MAIN_COLOR;
        } else if app.is_correct[i] == -1 {
            colors[i] = Color::Rgb(255, 0, 0);
        } else {
            colors[i] = REF_COLOR;
        }
    }

    let split = split_lines(&app.reference, max_ref_width);

    let mut char_index = 0;
    split.into_iter()
        .map(|line| {
            let spans: Vec<Span<'a>> = line
                .chars()
                .map(|c| {
                    let color = colors.get(char_index).cloned().unwrap_or(REF_COLOR);
                    char_index += 1;
                    Span::styled(c.to_string(), Style::default().fg(color).bg(BG_COLOR))
                })
                .collect();
            Line::from(spans)
        })
        .collect()
}

fn calculate_vertical_padding(area: Rect, content_lines: usize) -> usize {
    let empty_space = (area.height as u16).saturating_sub(3) as usize / 2;
    empty_space.saturating_sub(content_lines)
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
            lines
        })
        .collect()
}