use std::{path::PathBuf};
use std::fs;
use std::time::Instant;
use std::fs::File;
use std::io::BufReader;
use rand::prelude::IndexedRandom;

use crate::game::*;
use crate::game;
use crate::Cli;
use crate::Quote;
use crate::gui;
use crate::utils;


pub fn gui_main() {
    macroquad::Window::new("Hello World", async { gui::gui_main_async().await });
}

pub fn word_mode(args: &Cli) {
    println!("Starting common words test with specified word number");
    if args.top_words.is_some() && args.top_words.unwrap() > 1000 {
        eprintln!("The maximum number of top words is 1000.");
        return;
    }
    if args.word_number.is_some() && args.word_number.unwrap().is_none() {
        eprintln!("Please specify a valid word number.");
        return;
    }
    if args.word_number.is_some() && args.word_number.unwrap().unwrap() < 1 {
        eprintln!("Word number must be at least 1.");
        return;
    }
    if args.word_number.is_some() && args.word_number.unwrap().unwrap() > 1000 {
        eprintln!("The maximum word number is 1000.");
        return;
    }
    println!("Starting common words test with specified word number");
    
    let top_words = args.top_words.unwrap_or(500);
    let word_number = match args.word_number {
        Some(Some(n)) => n,
        Some(None) => 50,
        None => 50,
    };

    let word_list = utils::read_first_n_words(top_words as usize);
    let mut rng = rand::rng();

    let reference = utils::get_reference(&args, &word_list, word_number as usize, &mut rng);
    let start_time = Instant::now();
    type_loop(&reference, None, start_time);
}

pub fn time_mode(args: &Cli) {
    println!("Starting random words test with time limit");
    let time_limit = args.time_limit.unwrap_or(Some(30)).unwrap_or(30);

    if time_limit == 0 {
        eprintln!("Time limit must be at least 1 second.");
        return;
    }
    if time_limit > 300 {
        eprintln!("The maximum time limit is 300 seconds.");
        return;
    }

    let top_words = args.top_words.unwrap_or(500);
    println!("Starting common words test with {} second time limit", time_limit);
    let word_list = utils::read_first_n_words(top_words as usize);
    let mut rng = rand::rng();
    let batch_size = 20;
    let start_time = Instant::now();
    
    'outer: while start_time.elapsed().as_secs() < time_limit {
        let reference = utils::get_reference(&args, &word_list, batch_size, &mut rng) + " ";
        
        let elapsed = start_time.elapsed().as_secs();
        let remaining_time = if time_limit > elapsed {
            Some(time_limit - elapsed)
        } else {
            Some(0)
        };

        if remaining_time <= Some(0) {
            break;
        }

        let res = game::type_loop(&reference, Some(time_limit), start_time);
        if res != 0 {
            println!("Test interrupted by user.");
            break;
        }

        if start_time.elapsed().as_secs() >= time_limit {
            break 'outer;
        }
    }
}

pub fn custom_text(path: &PathBuf) {
    println!("Starting custom text test with file: {:?}", path);
    utils::validate_custom_file(path).unwrap_or_else(|err| {
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
    game::type_loop(reference.as_str(), None, start_time);
}

pub fn quotes() {
    println!("Starting random quote test");
    let file = File::open("src/quotes.json").expect("Failed to open quotes file");
    let reader = BufReader::new(file);
    let quotes: Vec<Quote> = serde_json::from_reader(reader).expect("Failed to parse quotes");
    let mut rng = rand::rng();
    let random_quote = quotes.choose(&mut rng).expect("No quotes available");
    let reference = format!("\"{}\" - {}", random_quote.text, random_quote.author);
    let start_time = Instant::now();
    game::type_loop(&reference, None, start_time);
}
