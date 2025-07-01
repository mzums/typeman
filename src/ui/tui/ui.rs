use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
};
use crate::ui::tui::app::App;

const BORDER_COLOR: Color = Color::Rgb(140, 80, 0);
const REF_COLOR: Color = Color::Rgb(100, 100, 100);

fn render_instructions(frame: &mut Frame, area: Rect) {
    let text = Paragraph::new("\nPress 'q' to exit.").style(Style::default().fg(BORDER_COLOR));
    frame.render_widget(text, area);
}

pub fn render_app(frame: &mut Frame, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_content(frame, chunks[0]);
    render_instructions(frame, chunks[1]);
}

fn render_content(frame: &mut Frame, area: Rect) {
    let empty_line = Line::from("");
    let instructions = Line::from("! punctuation  # numbers | time  words  quote | 15 30 60 120").style(style::Style::default().fg(REF_COLOR));

    let horizontal_line = Line::from("─".repeat(area.width.saturating_sub(15) as usize).fg(BORDER_COLOR));

    let content = vec![
        empty_line,
        instructions,
        horizontal_line,
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BORDER_COLOR))
        .title(Line::from(vec![
            " Type".fg(Color::Rgb(255, 155, 0)),
            "Man ".fg(Color::White),
        ]))
        .title_alignment(Alignment::Center);

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}
