use ratatui::{prelude::*, widgets::*};
use crate::ui::tui::app::App;


pub fn draw(frame: &mut Frame, app: &crate::ui::tui::app::App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());
    
    let header = Paragraph::new("TypeMan")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);

    let colored_hello = Line::from(vec![
        Span::styled("h", Style::default().fg(Color::Red)),
        Span::styled("e", Style::default().fg(Color::Green)),
        Span::styled("l", Style::default().fg(Color::Yellow)),
        Span::styled("l", Style::default().fg(Color::Blue)),
        Span::styled("o", Style::default().fg(Color::Magenta)),
    ]);
    let content = Paragraph::new(Text::from(colored_hello))
        .block(Block::default().borders(Borders::ALL).title("Instructions"))
        .alignment(Alignment::Center);
    
    let footer = Paragraph::new("a footer")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    
    frame.render_widget(header, chunks[0]);
    frame.render_widget(content, chunks[1]);
    frame.render_widget(footer, chunks[2]);
}
