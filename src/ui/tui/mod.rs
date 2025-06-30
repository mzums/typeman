use std::io;
use ratatui::prelude::{Frame, Widget, Line, Layout, Constraint, Style, Stylize, Rect};
use ratatui::symbols::border;
use ratatui::widgets::{Gauge, Block};
use ratatui::DefaultTerminal;
use crossterm::event::{KeyEventKind, KeyCode};


pub fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App {exit: false, progress_bar_color: ratatui::prelude::Color::Green};
    let app_result = app.run(&mut terminal);
    ratatui::restore();

    app_result
}

pub struct App {
    exit: bool,
    progress_bar_color: ratatui::prelude::Color,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true;
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where Self: Sized,
    {
        let vertical_layout = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let [title_area, gauge_area] = vertical_layout.areas(area);

        Line::from("Process overview").bold().render(title_area, buf);

        let instructions = Line::from(vec![
            "Change color".into(),
            "<c>".into(),
            "Exit".into(),
            "<q>".into(),
        ]);

        let block = Block::bordered()
            .title("Background processese")
            .title_bottom(instructions)
            .border_set(border::THICK);

        let progress_bar = Gauge::default()
            .gauge_style(Style::default().fg(self.progress_bar_color))
            .label(format!("Progress: {:.2}%", 75.0))
            .block(block)
            .ratio(0.5);
            

        progress_bar.render(
            Rect {
                x: gauge_area.x,
                y: gauge_area.y,
                width: gauge_area.width,
                height: 3,
            },
            buf,
        );
    }
}