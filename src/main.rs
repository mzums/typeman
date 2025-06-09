use clap::{Parser, ValueHint};
use std::path::PathBuf;
use std::fs;
use crossterm::{
    cursor, queue,
    style::{Color, Print, SetForegroundColor, Attribute, SetAttribute},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write, stdin, Read};
use std::time::Instant;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::prelude::IndexedRandom;

#[derive(Parser)]
#[command(
    name = "type-test",
    about = "Welcome to the type test!",
    version = "1.0",
    after_long_help = "Run examples:
  type-test -t=30 -n=500
  type-test -c ./text.txt
  type-test -q
  type-test -w=50 -n=500",
    long_about = "\nRun 'type-test -t=30 -n=500' to test your typing on words picked from the top N most common English words (use -n to set N, default is 500); -t sets the time limit (default is 30 seconds)
    Run 'type-test -w=50 -n=500' to test your typing on n most common English words, specify the -w for number of words (default is 50)
    Run 'type-test -c <path/to/your/file>' to test your typing on a specified text
    Run 'type-test -q' to test your typing on a random quote"
)]

struct Cli {
    #[arg(
        short = 'c',
        long = "custom",
        value_name = "FILE",
        value_hint = ValueHint::FilePath,
        conflicts_with_all = &["random_quote", "time_limit"]
    )]
    custom_file: Option<PathBuf>,

    #[arg(short = 'q', long = "quote", conflicts_with_all = &["custom_file", "time_limit"])]
    random_quote: bool,

    #[arg(short = 't', long = "time", value_name = "SECONDS")]
    time_limit: Option<u64>,

    #[arg(short = 'n', long = "top_words", value_name = "WORDS", required_unless_present_any = &["time_limit", "word_number"])]
    top_words: Option<i16>,

    #[arg(short = 'w', long = "word_number", value_name = "WORDS")]
    word_number: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct Quote {
    author: String,
    text: String,
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

fn initial_display(reference: &str) {
    let mut stdout = stdout();

    queue!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        SetAttribute(Attribute::Bold),
        SetAttribute(Attribute::Dim),
        Print(reference),
        SetAttribute(Attribute::Reset),
        cursor::MoveTo(0, 0)
    ).unwrap();
    stdout.flush().unwrap();
}

fn display_results(elapsed: f64, accuracy: f64, wpm: f64) {
    println!("\n\nTime: {:.2}s | Accuracy: {:.1}% | WPM: {:.1}",
        elapsed,
        accuracy,
        wpm
    );
}

fn type_loop(reference: &str) {
    let ref_chars: Vec<char> = reference.chars().collect();

    let mut stdout = stdout();
    let mut stdin = stdin().bytes();
    let _raw_guard = RawModeGuard::new();

    initial_display(&reference);

    let mut user_input = String::new();
    let start_time = Instant::now();
    let mut position = 0;
    let mut error_positions = vec![false; ref_chars.len()];

    loop {
        let byte = match stdin.next() {
            Some(Ok(b)) => b,
            Some(Err(_)) | None => break,
        };

        match byte {
            // Ctrl+c or Ctrl+d
            3 | 4 => break,

            // backspace
            8 | 127 if position > 0 => {
            position -= 1;
                user_input.pop();

                queue!(
                    stdout,
                    cursor::MoveLeft(1),
                    SetAttribute(Attribute::Dim),
                    Print(ref_chars[position]),
                    SetAttribute(Attribute::Reset),
                    cursor::MoveLeft(1),
                    SetForegroundColor(Color::Reset)
                ).unwrap();
            }

            _ if position < ref_chars.len() => {
                let c = byte as char;
                let ref_char = ref_chars[position];

                if c == ref_char {
                    if error_positions[position] {
                    // Corrected an error: yellow
                    queue!(
                        stdout,
                        SetForegroundColor(Color::Yellow),
                        Print(c),
                        SetForegroundColor(Color::Reset)
                    ).unwrap();
                } else {
                    // Correct on first try: green
                    queue!(
                        stdout,
                        SetForegroundColor(Color::Green),
                        Print(c),
                        SetForegroundColor(Color::Reset)
                    ).unwrap();
                }
                    user_input.push(c);
                    position += 1;
                } else {
                    error_positions[position] = true;
                    queue!(
                    stdout,
                    SetForegroundColor(Color::Red),
                    Print(ref_char),
                    SetForegroundColor(Color::Reset)
                    ).unwrap();
                    user_input.push(c);
                    position += 1;
                }
            }

            _ => {}
        }

        stdout.flush().unwrap();

        if position >= ref_chars.len() {
            break;
        }
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    let error_count = error_positions.iter().filter(|&&e| e).count();
    let accuracy = 100.0 - (error_count as f64 / reference.len() as f64 * 100.0);
    let wpm = (user_input.len() as f64 / 5.0) / (elapsed / 60.0);

    let term_width = crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
    let lines = (reference.len() + term_width - 1) / term_width;

    queue!(
        stdout,
        cursor::MoveTo(0, (lines as u16) + 1)
    ).unwrap();
    stdout.flush().unwrap();
    display_results(elapsed, accuracy, wpm);
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
    type_loop(reference.as_str());
}

fn quotes() {
    let file = File::open("src/quotes.json").expect("Failed to open quotes file");
    let reader = BufReader::new(file);
    let quotes: Vec<Quote> = serde_json::from_reader(reader).expect("Failed to parse quotes");
    let mut rng = rand::rng();
    let random_quote = quotes.choose(&mut rng).expect("No quotes available");
    let reference = format!("\"{}\" - {}", random_quote.text, random_quote.author);
    type_loop(&reference);
}

fn read_first_n_words(n: usize) -> Vec<String> {
    let file = File::open("src/common_eng_words.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .take(n)
        .filter_map(Result::ok)
        .collect()
}

fn main() {
    let args = Cli::parse();

    if let Some(path) = args.custom_file {
        println!("Starting custom text test with file: {:?}", path);
        custom_text(&path)
    } else if args.random_quote {
        println!("Starting random quote test");
        quotes();
    } else {
        let time_limit = args.time_limit.unwrap_or(30);
        let words = args.top_words.unwrap_or(500);
        let word_number = args.word_number.unwrap_or(500);

        if time_limit > 0 {
            println!("Starting common words test with {} second time limit", time_limit);
        } else {
            println!("Starting common words test with 30s time limit");
        }
        let word_list = read_first_n_words(words as usize);
        let mut rng = rand::thread_rng();
        let sample_size = word_number as usize;

        let reference = (0..sample_size)
            .map(|_| word_list.choose(&mut rng).unwrap().clone())
            .collect::<Vec<_>>()
            .join(" ")
            .replace('\n', " ");
        type_loop(&reference);
        /*println!("Welcome to the type test!");
        println!("Run 'type-test -t=30 -n=500' to test your typing on words picked from the top N most common English words (use -n to set N, default is 500); -t sets the time limit (default is 30 seconds)");
        println!("Run 'type-test -w=50' to test your typing on 100 most common english words, specify the -w for number of words (default is 50)");
        println!("Run 'type-test -c <path/to/your/file>' to test your typing on a specified text");
        println!("Run 'type-test -q' to test your typing on a random quote");*/
    }
}