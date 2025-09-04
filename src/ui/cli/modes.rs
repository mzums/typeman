use std::{path::PathBuf};
use std::fs;
use std::time::Instant;
use std::fs::File;
use std::io::BufReader;
use rand::prelude::IndexedRandom;
use std::collections::VecDeque;

use crate::ui::cli;
use crate::ui::gui::main as gui;
use crate::Cli;
use crate::Quote;
use crate::utils;
use crate::ui::tui::r#mod as tui_mod;
use crate ::practice;
use crate::language::Language;

fn get_language_from_args(args: &Cli) -> Language {
    args.language
        .as_ref()
        .and_then(|lang_str| Language::from_str(lang_str))
        .unwrap_or_default()
}


pub fn gui_main() {
    macroquad::Window::new("Hello World", async { gui::gui_main_async().await });
}

pub fn tui_main() {
    if let Err(e) = tui_mod::main() {
        eprintln!("TUI error: {}", e);
        std::process::exit(1);
    }
}

pub fn word_mode(args: &Cli) {
    println!("Starting common words test with specified word number");

    let punctuation = args.punctuation;
    let digits = args.digits;
    
    let top_words = args.top_words.unwrap_or(500);
    if top_words < 1 || top_words > 1000 {
        eprintln!("Top words must be between 1 and 1000.");
        return;
    }
    let word_number = match args.word_number {
        Some(Some(n)) => n,
        Some(None) => 50,
        None => 50,
    };
    if word_number < 1 || word_number > 1000 {
        eprintln!("Word number must be between 1 and 1000.");
        return;
    }

    let language = get_language_from_args(args);
    let word_list = utils::read_first_n_words(top_words as usize, language);

    let reference = utils::get_reference(punctuation, digits, &word_list, word_number as usize);
    let mut start_time: Option<Instant> = None;
    let mut is_correct: VecDeque<i32> = VecDeque::from(vec![0; reference.len()]);

    cli::main::type_loop(&reference, None, &mut start_time, None, &mut is_correct, "word");
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
    let language = get_language_from_args(args);
    let word_list = utils::read_first_n_words(top_words as usize, language);
    let batch_size = 20;
    let mut start_time: Option<Instant> = None;

    let punctuation = args.punctuation;
    let digits = args.digits;

    'outer: while start_time.is_none() || start_time.unwrap().elapsed().as_secs() < time_limit {
        let reference = utils::get_reference(punctuation, digits, &word_list, batch_size) + " ";
        let mut is_correct: VecDeque<i32> = VecDeque::from(vec![0; reference.len()]);
        
        if start_time.is_some() {
            let elapsed = start_time.unwrap().elapsed().as_secs();
            let remaining_time = if time_limit > elapsed {
                Some(time_limit - elapsed)
            } else {
                Some(0)
            };
    
            if remaining_time <= Some(0) {
                break;
            }
        }

        let res = cli::main::type_loop(&reference, Some(time_limit), &mut start_time, None, &mut is_correct, "time");
        if res != 0 {
            println!("Test interrupted by user.");
            break;
        }

        if start_time.unwrap().elapsed().as_secs() >= time_limit {
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
    let mut is_correct: VecDeque<i32> = VecDeque::from(vec![0; reference.len()]);
    let mut start_time: Option<Instant> = None;
    cli::main::type_loop(reference.as_str(), None, &mut start_time, None, &mut is_correct, "custom");
}

pub fn quotes() {
    println!("Starting random quote test");
    let file = File::open("assets/quotes.json").expect("Failed to open quotes file");
    let reader = BufReader::new(file);
    let quotes: Vec<Quote> = serde_json::from_reader(reader).expect("Failed to parse quotes");
    let mut rng = rand::rng();
    let random_quote = quotes.choose(&mut rng).expect("No quotes available");
    let reference = format!("\"{}\" - {}", random_quote.text, random_quote.author);
    let mut start_time: Option<Instant> = None;
    let mut is_correct: VecDeque<i32> = VecDeque::from(vec![0; reference.len()]);

    cli::main::type_loop(&reference, None, &mut start_time, None, &mut is_correct, "quote");
}

pub fn practice(args: &Cli) {
    let level = args.level.unwrap();
    if level.is_none() || level.unwrap() < 1 || level.unwrap() > practice::TYPING_LEVELS.len() {
        eprintln!("Please choose a level between 1 and {}.", practice::TYPING_LEVELS.len());
        for i in 0..practice::TYPING_LEVELS.len() {
            let results_path = format!("practice_results/level_{}.txt", i + 1);
            if practice::check_if_completed(results_path.as_str()) {
                println!("✔ Level {}: {}", i + 1, practice::TYPING_LEVELS[i].0);
            } else {
                println!("  Level {}: {}", i + 1, practice::TYPING_LEVELS[i].0);
            }
        }
        return;
    }
    
    let curr_level= level.unwrap() - 1;
    let chars = practice::TYPING_LEVELS[curr_level as usize].1;
    
    let reference = practice::create_words(&chars, args.word_number.unwrap_or(Some(50)).unwrap_or(50));
    let mut is_correct: VecDeque<i32> = VecDeque::from(vec![0; reference.len()]);
    let mut start_time: Option<Instant> = None;
    let res = cli::main::type_loop(&reference, None, &mut start_time, Some(curr_level), &mut is_correct, "practice");
    if res == 1 {
        println!("Exiting practice mode.");
        return;
    }
}