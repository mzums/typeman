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
    queue!(
        stdout,
        cursor::MoveTo(0, 0)
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