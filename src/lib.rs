#![allow(clippy::all)]

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
        pub mod practice;
        pub mod results;
        pub mod popup;
    }

    #[cfg(feature = "tui")]
    pub mod tui {
        pub mod app;
        pub mod r#mod;
        pub mod ui;
    }
}

#[cfg(feature = "tui")]
pub mod color_scheme;
pub mod language;
pub mod practice;
pub mod utils;
pub mod config;
pub mod leaderboard;
pub mod button_states;
pub mod custom_colors;

// Re-export types needed by modules
#[derive(Parser)]
#[command(
    name = "typeman",
    about = "Welcome to the typeman!",
    version = "1.0"
)]
pub struct Cli {
    #[arg(short = 'c', long = "custom", value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub custom_file: Option<PathBuf>,

    #[arg(short = 'q', long = "quote")]
    pub random_quote: bool,

    #[arg(short = 'p', long = "punctuation")]
    pub punctuation: bool,

    #[arg(short = 'd', long = "digits")]
    pub digits: bool,

    #[arg(short = 't', long = "time", value_name = "SECONDS")]
    pub time_limit: Option<Option<u64>>,

    #[arg(short = 'n', long = "top_words", value_name = "WORDS")]
    pub top_words: Option<usize>,

    #[arg(short = 'w', long = "word_number", value_name = "WORDS", num_args = 0..=1)]
    pub word_number: Option<Option<usize>>,

    #[arg(short = 'l', long = "level")]
    pub level: Option<Option<usize>>,

    #[arg(long = "gui")]
    pub gui: bool,

    #[arg(long = "tui")]
    pub tui: bool,

    #[arg(long = "cli")]
    pub cli: bool,

    #[arg(long = "lang", value_name = "LANGUAGE")]
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Quote {
    pub author: String,
    pub text: String,
}