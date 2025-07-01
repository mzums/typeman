use std::io;
use crate ::ui::tui::app::App;
use crate::utils;


pub fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let word_list = utils::read_first_n_words(500);
    let reference = utils::get_reference(false, false, &word_list, 50);

    let app_result = app.run(&mut terminal, reference);
    ratatui::restore();
    app_result
}