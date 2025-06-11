use crate::game::type_loop;
use crate::Cli;
use rand::prelude::IndexedRandom;
use std::time::Instant;

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

fn create_words(chars: &[char], args: &Cli) -> String {
    let mut reference = String::new();
    let word_number = args.word_number.unwrap_or(Some(50)).unwrap_or(50);
    for _ in 0..word_number {
        let word_length = rand::random::<u16>() % 5 + 2;
        let word: String = (0..word_length)
            .map(|_| *chars.choose(&mut rand::rng()).unwrap())
            .collect();
        reference.push_str(&word);
        reference.push(' ');
    }
    return reference;
}
    
pub fn practice(args: &Cli) {
    let level = args.level.unwrap();
    if level < 1 || level > TYPING_LEVELS.len() {
        eprintln!("Invalid typing level. Please choose a level between 1 and {}.", TYPING_LEVELS.len());
        return;
    }

    let x = [1, 1, 2, 3, 3];
    let mut chars: Vec<char> = Vec::new();
    let mut curr_level: i16 = level as i16 - 1;

    for i in x {
        if curr_level < 0 {
            return;
        }
        for _ in 0..i {
            if curr_level < 0 {
                break;
            }
            chars.extend_from_slice(TYPING_LEVELS[curr_level as usize]);
            curr_level -= 1;
        }
        let reference = create_words(&chars, args);
        let start_time = Instant::now();
        let res = type_loop(&reference, None, start_time);
        if res == 1 {
            println!("Exiting practice mode.");
            return;
        }
        chars.clear();
        if curr_level < 0 {
            break;
        }
    }
}   