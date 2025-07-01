use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crate::ui::tui::app::App;

const BORDER_COLOR: Color = Color::Rgb(100, 60, 0);
const REF_COLOR: Color = Color::Rgb(80, 80, 80);
const BG_COLOR: Color = Color::Black;

fn render_instructions(frame: &mut Frame, area: Rect) {
    let text = Paragraph::new("\nPress 'q' to exit.")
        .style(Style::default().fg(BORDER_COLOR).bg(BG_COLOR));
    frame.render_widget(text, area);
}

pub fn render_app(frame: &mut Frame, _app: &App, reference: &String) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_content(frame, chunks[0], reference);
    render_instructions(frame, chunks[1]);
}

fn render_content(frame: &mut Frame, area: Rect, reference: &String) {
    let empty_line = Line::from("");
    let instructions = Line::from("! punctuation  # numbers | time  words  quote | 15 30 60 120")
        .style(style::Style::default().fg(REF_COLOR).bg(BG_COLOR));

    let horizontal_line = Line::from("─".repeat(area.width.saturating_sub(15) as usize).fg(BORDER_COLOR).bg(BG_COLOR));

    let colors: Vec<Color> = reference
        .chars()
        .enumerate()
        .map(|(i, _)| if i % 2 == 0 { Color::Yellow } else { Color::Green })
        .collect();

    let colored_spans: Vec<Span> = reference
        .chars()
        .zip(colors.iter().cloned().chain(std::iter::repeat(REF_COLOR)))
        .map(|(c, color)| Span::styled(c.to_string(), Style::default().fg(color).bg(BG_COLOR)))
        .collect();

    let colored_reference: Vec<Line> = colored_spans
        .chunks(area.width as usize)
        .map(|chunk| Line::from(chunk.to_vec()))
        .collect();

    let content = {
        let mut content = vec![
            empty_line.clone(),
            instructions,
            horizontal_line,
        ];
        content.push(empty_line.clone());
        content.extend(colored_reference);
        content
    };


    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BORDER_COLOR).bg(BG_COLOR))
        .style(Style::default().bg(BG_COLOR))
        .title(Line::from(vec![
            " Type".fg(Color::Rgb(255, 155, 0)).bg(BG_COLOR),
            "Man ".fg(Color::White).bg(BG_COLOR),
        ]))
        .padding(Padding {
            left: 5,
            right: 0,
            top: 0,
            bottom: 0,
        })
        .title_alignment(Alignment::Left);

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left)
        .style(Style::default().bg(BG_COLOR));

    frame.render_widget(paragraph, area);
}