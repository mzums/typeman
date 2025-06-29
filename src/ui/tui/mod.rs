use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use std::io;
use std::error::Error;
use ratatui::prelude::{CrosstermBackend, Backend};
use ratatui::Terminal;
use ratatui::crossterm::event::DisableMouseCapture;
use ratatui::crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};

use crate::ui::tui::app::App;
use crate::ui::tui::event::{Event, handle_key_event};
use crate::ui::tui::ui::ui;
use crossterm::event::{read, KeyEventKind};


pub fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        
        let crossterm_event = read()?;
        let event = match crossterm_event {
            crossterm::event::Event::Key(key) => Event::Key(key),
            _ => continue,
        };

        let key = match event {
            Event::Key(key) => key,
        };
        if key.kind == KeyEventKind::Release {
            continue;
        }
        if let Some(exit_action) = handle_key_event(key, app) {
            return Ok(exit_action);
        }
    }
}