use ratatui::{prelude::*, widgets::*};

pub fn draw(frame: &mut Frame, app: &crate::ui::tui::app::App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());
    
    let header = Paragraph::new("Typing Test TUI")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    
    let content = Paragraph::new(format!(
        "Counter: {}\n\nPress 'c' to increase counter\nPress 'r' to reset\nPress 'q' to quit",
        app.counter
    ))
    .block(Block::default().borders(Borders::ALL).title("Instructions"))
    .alignment(Alignment::Center);
    
    let footer = Paragraph::new("a footer")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    
    frame.render_widget(header, chunks[0]);
    frame.render_widget(content, chunks[1]);
    frame.render_widget(footer, chunks[2]);
}