use clap::{Parser, ValueHint};
use std::{path::PathBuf};
use serde::Deserialize;

use crate ::ui::cli::modes;

mod ui {
    pub mod cli {
        pub mod main;
        pub mod modes;
    }
    pub mod gui {
        pub mod main;
        pub mod results;
        pub mod config;
        pub mod practice;
    }
    pub mod tui {
        pub mod app;
        pub mod ui;
        pub mod r#mod;
    }
}
mod practice;
mod utils;

#[derive(Parser)]
#[command(
    name = "type-test",
    about = "Welcome to the type test!",
    version = "1.0",
    after_long_help = "Run examples:
type-test -c ./text.txt
type-test -q
type-test -t=30 -n=500
type-test -w=50 -n=500
type-test -w=50 -n=500 -p -d
type_test --tui",
    long_about = "\n
Run 'type-test -c <path/to/your/file>' to test your typing on a specified text
Run 'type-test -q' to test your typing on a random quote
Run 'type-test -w=50 (-n=500 -p -d)' to test your typing on n most common English words, specify the -w for number of words (default is 50)
Run 'type-test (-t=30 -n=500 -p -d)' to test your typing on random words for t seconds; -t sets the time limit (default is 30 seconds)
Run 'type-test --tui' to start the terminal-based interface

Optional:
  - Use -p to include punctuation, -d to include digits
  - Use -n to specify the number of words to type (default is 50, max is 500)
  - Use -t to set a time limit for the test (default is 30 seconds, use 0 for no limit)
  - Use -n to specify the number of top words to use (default is 500, max is 1000)
  - Use --tui for terminal-based interface

Default behavior is to test typing on random words for 30 seconds with 500 most common English words.
    "
)]

struct Cli {
    #[arg(short = 'c', long = "custom", value_name = "FILE", value_hint = ValueHint::FilePath, conflicts_with_all = &["random_quote", "time_limit"], conflicts_with_all = &["top_words", "word_number"])]
    custom_file: Option<PathBuf>,

    #[arg(short = 'q', long = "quote", conflicts_with_all = &["custom_file", "time_limit", "top_words"])]
    random_quote: bool,

    #[arg(short = 'p', long = "punctuation", conflicts_with_all = &["custom_file", "random_quote"])]
    punctuation: bool,

    #[arg(short = 'd', long = "digits", conflicts_with_all = &["custom_file", "random_quote"])]
    digits: bool,

    #[arg(short = 't', long = "time", value_name = "SECONDS")]
    time_limit: Option<Option<u64>>,

    #[arg(short = 'n', long = "top_words", value_name = "WORDS")]
    top_words: Option<usize>,

    #[arg(short = 'w', long = "word_number", value_name = "WORDS", num_args = 0..=1)]
    word_number: Option<Option<usize>>,

    #[arg(short = 'l', long = "level", conflicts_with_all = &["custom_file", "random_quote", "time_limit", "top_words"])]
    level: Option<usize>,

    #[arg(long = "gui", conflicts_with_all = &["custom_file", "random_quote", "time_limit", "top_words", "word_number", "level"])]
    gui: bool,

    #[arg(long = "tui", conflicts_with_all = &["custom_file", "random_quote", "time_limit", "top_words", "word_number", "level", "gui"])]
    tui: bool,
}

#[derive(Debug, Deserialize)]
pub struct Quote {
    author: String,
    text: String,
}

fn main() {
    let args = Cli::parse();

    if args.gui {
        modes::gui_main();
        return;
    }
    if args.tui {
        modes::tui_main();
        return;
    }

    if let Some(path) = args.custom_file {
        modes::custom_text(&path)
    } else if args.random_quote {
        modes::quotes();
    } else if args.level.is_some() {
        modes::practice(&args);
    } else if args.word_number.is_some() &&  !args.time_limit.is_some() {
        modes::word_mode(&args);
    } else {
        modes::time_mode(&args);
    }
}
