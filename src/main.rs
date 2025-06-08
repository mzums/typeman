use clap::{Parser, ValueHint};
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
#[command(
    name = "type-test",
    about = "Welcome to the type test!",
    version = "1.0",
    after_long_help = "Run examples:
  type-test -r -t=30
  type-test -c ./text.txt
  type-test -q"
)]
struct Cli {
    #[arg(short = 'r', long = "random", conflicts_with_all = &["custom_file", "random_quote"])]
    common_words: bool,

    #[arg(
        short = 'c',
        long = "custom",
        value_name = "FILE",
        value_hint = ValueHint::FilePath,
        conflicts_with_all = &["common_words", "random_quote", "time_limit"]
    )]
    custom_file: Option<PathBuf>,

    #[arg(short = 'q', long = "quote", conflicts_with_all = &["common_words", "custom_file", "time_limit"])]
    random_quote: bool,

    #[arg(short = 't', long = "time", value_name = "SECONDS", requires = "common_words")]
    time_limit: Option<u64>,
}


fn custom_text(path: &PathBuf) {
    validate_custom_file(path).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    let reference = match fs::read_to_string(path) {
        Ok(content) => content.replace('\n', " "),
        Err(_) => {
            eprintln!("Error reading file");
            return;
        }
    };

    let _raw_guard = RawModeGuard::new();
    println!("{reference}");
}

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> Self {
        crossterm::terminal::enable_raw_mode().unwrap();
        RawModeGuard
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}

fn validate_custom_file(path: &PathBuf) -> Result<(), String> {
    if path.exists() && path.is_file() {
        Ok(())
    } else {
        Err(format!("Custom file does not exist or is not a file: {:?}", path))
    }
}


fn main() {
    let args = Cli::parse();

    if args.common_words {
        let time_limit = args.time_limit.unwrap_or(0);
        
        if time_limit > 0 {
            println!("Starting common words test with {} second time limit", time_limit);
        } else {
            println!("Starting common words test with 30s time limit");
        }
    } else if let Some(path) = args.custom_file {
        println!("Starting custom text test with file: {:?}", path);
        custom_text(&path)
    } else if args.random_quote {
        println!("Starting random quote test");
    } else {
        println!("Welcome to the type test!");
        println!("Run 'type-test -r -t=30' to test your typing on 100 most common english words, specify the -t flag for time, default is 30");
        println!("Run 'type-test -c <path/to/your/file>' to test your typing on a specified text");
        println!("Run 'type-test -q' to test your typing on a random quote");
    }
}