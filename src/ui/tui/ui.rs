use ratatui::{
    prelude::*,
    widgets::*,
    symbols::border,
    Frame,
};
use crate ::ui::tui::app::App;

pub fn render_app(frame: &mut Frame, app: &App) {
    let vertical_layout = Layout::vertical([
        Constraint::Percentage(20),
        Constraint::Percentage(80),
    ]);
    let [title_area, gauge_area] = vertical_layout.areas(frame.area());

    let gauge_areas: Vec<Rect> = Layout::vertical(
        vec![Constraint::Length(3); app.progress_bars.len()]
    ).split(gauge_area).to_vec();
    
    render_title(frame, title_area);
    for i in 0..app.progress_bars.len() {
        render_progress_bar(frame, gauge_areas[i], app, i);
    }
        
}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Line::from("Process overview").bold(),
        area,
    );
}

fn render_progress_bar(frame: &mut Frame, area: Rect, app: &App, index: usize) {
    let instructions = Line::from(vec![
        "Change color".into(),
        "<c>".blue().bold(),
        "Exit".into(),
        "<q>".blue().bold(),
    ]);

    let mut border_color = Color::White;
    if index == app.selected {
        border_color = Color::Yellow;
    }

    let block = Block::bordered()
        .title("Background processes")
        .title_bottom(instructions)
        .style(Style::default().fg(border_color))
        .border_set(border::THICK);

    let progress_bar = Gauge::default()
        .gauge_style(Style::default().fg(app.progress_bars[index].color))
        .label(format!("Progress: {:.2}%", app.progress_bars[index].progress * 100.0))
        .block(block)
        .ratio(app.progress_bars[index].progress);

    frame.render_widget(
        progress_bar,
        Rect::new(
            area.x,
            area.y,
            area.width,
            3,
        ),
    );
}