use clap::{Parser, ValueHint};
use std::{path::PathBuf};
use serde::Deserialize;
use rand::prelude::IndexedRandom;
use rand::Rng;
use rand::prelude::SliceRandom;
mod game;
mod modes;
mod practice;

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
  - Use -n to specify the number of top words to use (default is 500, max is 1000)

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
}

#[derive(Debug, Deserialize)]
struct Quote {
    author: String,
    text: String,
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
        modes::custom_text(&path)
    } else if args.random_quote {
        println!("Starting random quote test");
        modes::quotes();
    } else if args.level.is_some() {
        practice::practice(&args);
    } else if args.word_number.is_some() &&  !args.time_limit.is_some() {
        modes::word_mode(&args);
    }
    else {
        modes::time_mode(&args);
    }
}
