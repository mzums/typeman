use clap::{Parser, ValueHint};
use std::{path::PathBuf};
use std::fs;
use crossterm::{
    cursor, queue,
    style::{Color, Print, SetForegroundColor, Attribute, SetAttribute},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use std::time::Instant;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::prelude::IndexedRandom;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use rand::Rng;
use rand::prelude::SliceRandom;

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
type_test",
    long_about = "\n
Run 'type-test -c <path/to/your/file>' to test your typing on a specified text
Run 'type-test -q' to test your typing on a random quote
Run 'type-test -w=50 (-n=500 -p -d)' to test your typing on n most common English words, specify the -w for number of words (default is 50)
Run 'type-test (-t=30 -n=500 -p -d)' to test your typing on random words for t seconds; -t sets the time limit (default is 30 seconds)

Optional:
  - Use -p to include punctuation, -d to include digits
  - Use -n to specify the number of words to type (default is 50, max is 500)
  - Use -t to set a time limit for the test (default is 30 seconds, use 0 for no limit)
  - Use -n to specify the number of top words to use (default is 500, max is 500)

Default behavior is to test typing on random words for 30 seconds with 500 most common English words.
    "
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

    #[arg(short = 'q', long = "quote", conflicts_with_all = &["custom_file", "time_limit", "top_words"])]
    random_quote: bool,

    #[arg(short = 'p', long = "punctuation", conflicts_with_all = &["custom_file", "random_quote"])]
    punctuation: bool,

    #[arg(short = 'd', long = "digits", conflicts_with_all = &["custom_file", "random_quote"])]
    digits: bool,

    #[arg(short = 't', long = "time", value_name = "SECONDS")]
    time_limit: Option<Option<u64>>,

    #[arg(short = 'n', long = "top_words", value_name = "WORDS")]
    top_words: Option<i16>,

    #[arg(short = 'w', long = "word_number", value_name = "WORDS", num_args = 0..=1)]
    word_number: Option<Option<i32>>,
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

fn initial_display(reference: &str, timer_pos: (u16, u16)) {
    let mut stdout = stdout();

    queue!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 2),
        SetAttribute(Attribute::Dim),
        Print(reference),
        SetAttribute(Attribute::Reset),
        cursor::MoveTo(timer_pos.0, timer_pos.1),
        Print("Time: 00:00"),
        cursor::MoveTo(0, 2)
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

fn type_loop(reference: &str, time_limit: Option<u64>, start_time: Instant) -> i32 {
    let ref_chars: Vec<char> = reference.chars().collect();
    let mut stdout = stdout();
    let _raw_guard = RawModeGuard::new();

    let (width, _height) = crossterm::terminal::size().unwrap();
    let timer_pos = (width.saturating_sub(15), 0);

    initial_display(reference, timer_pos);

    let mut user_input = String::new();
    let mut position = 0;
    let mut error_positions = vec![false; ref_chars.len()];
    let mut last_update = Instant::now();

    loop {
        if last_update.elapsed().as_millis() > 100 {
            let elapsed = start_time.elapsed();
            let secs = elapsed.as_secs();
            let display_secs = secs % 60;
            let display_mins = secs / 60;
            
            queue!(
                stdout,
                cursor::MoveTo(timer_pos.0, timer_pos.1),
                Clear(ClearType::UntilNewLine),
                Print(format!("Time: {:02}:{:02}", display_mins, display_secs)),
                cursor::MoveTo(position as u16 % width, position as u16 / width+2)
            ).unwrap();
            stdout.flush().unwrap();
            last_update = Instant::now();
        }

        // non-blocking input
        let mut byte_opt = None;

        if event::poll(std::time::Duration::from_millis(10)).unwrap() {
            if let Event::Key(KeyEvent { code, modifiers , kind: _ , state:_ }) = event::read().unwrap() {
                match (code, modifiers) {
                    (KeyCode::Char('c'), event::KeyModifiers::CONTROL) => { // Ctrl+C
                        queue!(
                            stdout,
                            Clear(ClearType::All),
                            cursor::MoveTo(0, 0)
                        ).unwrap();
                        return 1
                    },
                    (KeyCode::Char('d'), event::KeyModifiers::CONTROL) => { // Ctrl+D
                        queue!(
                            stdout,
                            Clear(ClearType::All),
                            cursor::MoveTo(0, 0)
                        ).unwrap();
                        return 1
                    },
                    (KeyCode::Char(c), _) => byte_opt = Some(c as u8),
                    (KeyCode::Backspace, _) => byte_opt = Some(8),
                    (KeyCode::Esc, _) => break,
                    (KeyCode::Enter, _) => byte_opt = Some(b'\n'),
                    _ => {}
                }
            }
        }
        if byte_opt.is_none() {
            if let Some(limit) = time_limit {
                if start_time.elapsed().as_secs() >= limit {
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
            continue;
        }

        let byte = byte_opt.unwrap();

        match byte {
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

    return 0;
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
    let start_time = Instant::now();
    type_loop(reference.as_str(), None, start_time);
}

fn quotes() {
    let file = File::open("src/quotes.json").expect("Failed to open quotes file");
    let reader = BufReader::new(file);
    let quotes: Vec<Quote> = serde_json::from_reader(reader).expect("Failed to parse quotes");
    let mut rng = rand::rng();
    let random_quote = quotes.choose(&mut rng).expect("No quotes available");
    let reference = format!("\"{}\" - {}", random_quote.text, random_quote.author);
    let start_time = Instant::now();
    type_loop(&reference, None, start_time);
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

fn _practice() {
    const TYPING_LEVELS: [&[char]; 16] = [
        &['f', 'j'],
        &['d', 'k'],
        &['s', 'l'],
        &['a', ';'],
        &['g', 'h'],
        &['r', 'u'],
        &['t', 'y'],
        &['e', 'i'],
        &['w', 'o'],
        &['q', 'p'],
        &['v', 'b', 'n'],
        &['c', 'm'],
        &['x', ','],
        &['z', '.', '/'],
        &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
        &['-', '=', '[', ']', '\'', ';', ',', '.', '/', '`'],
    ];
}

fn get_reference(args: &Cli, word_list: &[String], batch_size: usize, rng: &mut impl Rng) -> String {
    let mut items = Vec::new();

    let num_digits = if args.digits {
        let max_digits = batch_size.min(batch_size / 3).max(1);
        rng.random_range((batch_size / 6).max(1)..=max_digits)
    } else {
        0
    };

    let num_words = batch_size - num_digits;

    for _ in 0..num_words {
        let mut word = word_list.choose(rng).unwrap().clone();
        if args.punctuation {
            let punctuations = [".", ",", "!", "?", ";", ":"];
            if rng.random_bool(0.2) {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    word = format!("{}{}", first.to_uppercase(), chars.as_str());
                }
            }
            if rng.random_bool(0.2) {
                word.push_str(punctuations.choose(rng).unwrap());
            }
        }
        items.push(word);
    }

    for _ in 0..num_digits {
        let choice = rng.random_range(0..4);
        let number;
        if choice == 0 {
            number = rng.random_range(1..10000).to_string();
        } else if choice == 1 {
            number = format!("{:04}", rng.random_range(0..10000));
        }
        else {
            number = rng.random_range(1..100).to_string();
        }
        items.push(number);
        continue;
    }

    items.shuffle(rng);

    items.join(" ").replace('\n', " ")
}

fn main() {
    let args = Cli::parse();

    if let Some(path) = args.custom_file {
        println!("Starting custom text test with file: {:?}", path);
        custom_text(&path)
    } else if args.random_quote {
        println!("Starting random quote test");
        quotes();
    } else if args.word_number.is_some() {
        let top_words = args.top_words.unwrap_or(500);
        let word_number = match args.word_number {
            Some(Some(n)) => n,
            Some(None) => 50,
            None => 50,
        };

        let word_list = read_first_n_words(top_words as usize);
        let mut rng = rand::rng();

        let reference = get_reference(&args, &word_list, word_number as usize, &mut rng);
        let start_time = Instant::now();
        type_loop(&reference, None, start_time);
    }
    else {
        let time_limit = args.time_limit.unwrap_or(Some(30)).unwrap_or(30);
        let top_words = args.top_words.unwrap_or(500);
        println!("Starting common words test with {} second time limit", time_limit);
        let word_list = read_first_n_words(top_words as usize);
        let mut rng = rand::rng();
        let batch_size = 20;
        let start_time = Instant::now();
        
        'outer: while start_time.elapsed().as_secs() < time_limit {
            let reference = get_reference(&args, &word_list, batch_size, &mut rng);
            
            let elapsed = start_time.elapsed().as_secs();
            let remaining_time = if time_limit > elapsed {
                Some(time_limit - elapsed)
            } else {
                Some(0)
            };

            if remaining_time <= Some(0) {
                break;
            }

            let res = type_loop(&reference, Some(time_limit), start_time);
            if res != 0 {
                println!("Test interrupted by user.");
                break;
            }

            if start_time.elapsed().as_secs() >= time_limit {
                break 'outer;
            }
        }
    }
}
