use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;
use rand::Rng;
use rand::prelude::IndexedRandom;
use rand::prelude::SliceRandom;


pub fn read_first_n_words(n: usize) -> Vec<String> {
    let file = File::open("assets/common_eng_words.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .take(n)
        .filter_map(Result::ok)
        .collect()
}

pub fn validate_custom_file(path: &PathBuf) -> Result<(), String> {
    if path.exists() && path.is_file() {
        Ok(())
    } else {
        Err(format!("Custom file does not exist or is not a file: {:?}", path))
    }
}

pub fn get_reference(punctuation: bool, digits: bool, word_list: &[String], batch_size: usize) -> String {
    let mut items = Vec::new();
    let mut rng = rand::rng();

    let num_digits = if digits {
        let max_digits = batch_size.min(batch_size / 3).max(1);
        rng.random_range((batch_size / 6).max(1)..=max_digits)
    } else {
        0
    };

    let num_words = batch_size - num_digits;

    for _ in 0..num_words {
        let mut word = word_list.choose(&mut rng).unwrap().clone();
        if punctuation {
            let punctuations = [".", ",", "!", "?", ";", ":"];
            if rng.random_bool(0.2) {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    word = format!("{}{}", first.to_uppercase(), chars.as_str());
                }
            }
            if rng.random_bool(0.2) {
                word.push_str(punctuations.choose(&mut rng).unwrap());
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

    items.shuffle(&mut rng);

    items.join(" ").replace('\n', " ")
}