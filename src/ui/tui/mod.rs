use std::io;
use crate ::ui::tui::app::App;


pub fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}