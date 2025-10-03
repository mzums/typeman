#![allow(clippy::all)]

#[cfg(not(any(feature = "cli", feature = "tui", feature = "gui")))]
compile_error!("At least one of 'cli', 'tui', or 'gui' must be enabled");

use clap::{Parser, ValueHint};
use serde::Deserialize;
use std::path::PathBuf;

pub mod ui {
    #[cfg(feature = "cli")]
    pub mod cli {
        pub mod main;
        pub mod modes;
    }

    #[cfg(feature = "gui")]
    pub mod gui {
        pub mod config;
        pub mod main;
        pub mod popup;
        pub mod practice;
        pub mod results;
    }

    #[cfg(feature = "tui")]
    pub mod tui {
        pub mod app;
        pub mod r#mod;
        pub mod ui;
    }
}

pub mod button_states;
pub mod color_scheme;
pub mod config;
pub mod custom_colors;
pub mod language;
pub mod leaderboard;
pub mod practice;
pub mod utils;
pub mod time_selection;

#[cfg(feature = "cli")]
use crate::ui::cli::modes;

#[cfg(feature = "tui")]
use crate::ui::tui::r#mod as tui_mod;

#[cfg(feature = "gui")]
use crate::ui::gui::main as gui;

#[derive(Parser)]
#[command(
    name = "typeman",
    about = "Welcome to the typeman!",
    version = "1.0",
    after_long_help = "Run examples:
typeman --cli -c ./text.txt
typeman --cli -q
typeman --cli -t=30 -n=500
typeman --cli -w=50 -n=500
typeman --cli -w=50 -n=500 -p -d
typeman --gui
typeman",
    long_about = "\n
Run 'typeman --cli -c <path/to/your/file>' to test your typing on a specified text
Run 'typeman --cli -q' to test your typing on a random quote
Run 'typeman --cli -w=50 (-n=500 -p -d)' to test your typing on n most common English words, specify the -w for number of words (default is 50)
Run 'typeman --cli (-t=30 -n=500 -p -d)' to test your typing on random words for t seconds; -t sets the time limit (default is 30 seconds)
Run 'typeman (--tui)' to start the terminal-based interface

Optional:
  - Use -p to include punctuation, -d to include digits
  - Use -n to specify the number of words to type (default is 50, max is 500)
  - Use -t to set a time limit for the test (default is 30 seconds, use 0 for no limit)
  - Use -n to specify the number of top words to use (default is 500, max is 1000)
  - Use --gui for terminal-based interface
  - Use --cli for terminal-based interface

Default behavior for cli is to test typing on random words for 30 seconds with 500 most common English words.
Default mode is tui.
    "
)]
pub struct Cli {
    #[arg(short = 'c', long = "custom", value_name = "FILE", value_hint = ValueHint::FilePath, conflicts_with_all = &["random_quote", "time_limit", "top_words", "word_number", "gui", "tui"])]
    custom_file: Option<PathBuf>,

    #[arg(short = 'q', long = "quote", conflicts_with_all = &["custom_file", "time_limit", "top_words", "gui", "tui"])]
    random_quote: bool,

    #[arg(short = 'p', long = "punctuation", conflicts_with_all = &["custom_file", "random_quote", "gui", "tui"])]
    punctuation: bool,

    #[arg(short = 'd', long = "digits", conflicts_with_all = &["custom_file", "random_quote", "gui", "tui"])]
    digits: bool,

    #[arg(short = 't', long = "time", value_name = "SECONDS", conflicts_with_all = &["gui", "tui"])]
    time_limit: Option<Option<u64>>,

    #[arg(short = 'n', long = "top_words", value_name = "WORDS", conflicts_with_all = &["gui", "tui"])]
    top_words: Option<usize>,

    #[arg(short = 'w', long = "word_number", value_name = "WORDS", num_args = 0..=1, conflicts_with_all = &["gui", "tui"])]
    word_number: Option<Option<usize>>,

    #[arg(short = 'l', long = "level", conflicts_with_all = &["custom_file", "random_quote", "time_limit", "top_words"], conflicts_with_all = &["gui", "tui"])]
    level: Option<Option<usize>>,

    #[arg(long = "gui", conflicts_with_all = &["custom_file", "random_quote", "time_limit", "top_words", "word_number", "level", "tui", "cli"])]
    gui: bool,

    #[arg(long = "tui", conflicts_with_all = &["custom_file", "random_quote", "time_limit", "top_words", "word_number", "level", "gui", "cli"])]
    tui: bool,

    #[arg(long = "cli", conflicts_with_all = &["tui", "gui"])]
    cli: bool,

    #[arg(
        long = "lang",
        value_name = "LANGUAGE",
        help = "Language for word lists (english, indonesian)"
    )]
    language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Quote {
    author: String,
    text: String,
}

#[cfg(feature = "gui")]
pub fn gui_main() {
    macroquad::Window::new("TypeMan", async { gui::gui_main_async().await });
}

#[cfg(feature = "tui")]
pub fn tui_main() {
    if let Err(e) = tui_mod::main() {
        eprintln!("TUI error: {}", e);
        std::process::exit(1);
    }
}

fn main() {
    let args = Cli::parse();

    if args.tui && !cfg!(feature = "tui") {
        eprintln!("TUI mode is not available in this build.");
        std::process::exit(1);
    }

    if args.gui && !cfg!(feature = "gui") {
        eprintln!("GUI mode is not available in this build.");
        std::process::exit(1);
    }

    if args.cli && !cfg!(feature = "cli") {
        eprintln!("CLI mode is not available in this build.");
        std::process::exit(1);
    }

    #[cfg(feature = "gui")]
    if args.gui {
        gui_main();
        return;
    }

    #[cfg(feature = "cli")]
    if args.cli {
        run_cli(&args);
        return;
    }

    #[cfg(feature = "tui")]
    {
        ui::tui::r#mod::main().unwrap();
        return;
    }
}

#[cfg(feature = "cli")]
fn run_cli(args: &Cli) {
    if let Some(path) = args.custom_file.as_ref() {
        modes::custom_text(path)
    } else if args.random_quote {
        modes::quotes();
    } else if args.level.is_some() {
        modes::practice(args);
    } else if args.word_number.is_some() && args.time_limit.is_none() {
        modes::word_mode(args);
    } else {
        modes::time_mode(args);
    }
}
