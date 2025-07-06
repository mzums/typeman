use crate::Cli;
use rand::prelude::IndexedRandom;
use std::time::Instant;

use crate::ui::cli;


pub const TYPING_LEVELS: [(&str, &[char]); 32] = [
    ("new: f & j", &['f', 'j']),
    ("new: d & k", &['d', 'k']),
    ("repetition: f, j, d, k", &['f', 'j', 'd', 'k']),
    ("new: s & l", &['s', 'l']),
    ("repetition: home row 1", &['f', 'j', 'd', 'k', 's', 'l']),
    ("new: a & ;", &['a', ';']),
    ("repetition: home row 2", &['f', 'j', 'd', 'k', 's', 'l', 'a', ';']),
    ("new: g & h", &['g', 'h']),
    ("repetition: home row full", &['f', 'j', 'd', 'k', 's', 'l', 'a', ';', 'g', 'h']),
    ("new: r & u", &['r', 'u']),
    ("repetition: add r & u", &['f', 'j', 'd', 'k', 's', 'l', 'a', ';', 'g', 'h', 'r', 'u']),
    ("new: t & y", &['t', 'y']),
    ("repetition: left-right pairs", &['r', 'u', 't', 'y', 'g', 'h']),
    ("new: e & i", &['e', 'i']),
    ("repetition: stretch row 1", &['r', 'u', 't', 'y', 'e', 'i']),
    ("new: w & o", &['w', 'o']),
    ("new: q & p", &['q', 'p']),
    ("repetition: top row left-right", &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p']),
    ("new: v, b, n", &['v', 'b', 'n']),
    ("new: c & m", &['c', 'm']),
    ("repetition: bottom row left-right", &['z', 'x', 'c', 'v', 'b', 'n', 'm']),
    ("new: x & ,", &['x', ',']),
    ("new: z & . & /", &['z', '.', '/']),
    ("repetition: full lowercase", &[
        'a','b','c','d','e','f','g','h','i','j',
        'k','l','m','n','o','p','q','r','s','t',
        'u','v','w','x','y','z'
    ]),
    ("new: numbers row", &['1','2','3','4','5','6','7','8','9','0']),
    ("repetition: letters + numbers", &[
        'a','s','d','f','j','k','l',';', '1','2','3','4','5'
    ]),
    ("new: symbols row 1", &['-', '=', '[', ']', '\'', ';', ',', '.', '/', '`']),
    ("new: shifted: !@#$", &['!', '@', '#', '$']),
    ("new: shifted: %^&*()", &['%', '^', '&', '*', '(', ')']),
    ("repetition: shift practice", &['!', '@', '#', '$', '%', '^', '&', '*', '(', ')']),
    ("new: shifted: symbols", &['_', '+', '{', '}', ':', '"', '<', '>', '?', '~']),
    ("repetition: all punctuation", &[
        '.', ',', ':', ';', '!', '?', '\'', '"', '-', '_', '(', ')',
        '[', ']', '{', '}', '/', '\\', '`', '~'
    ]),
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
            chars.extend_from_slice(TYPING_LEVELS[curr_level as usize].1);
            curr_level -= 1;
        }
        let reference = create_words(&chars, args);
        let start_time = Instant::now();
        let res = cli::type_loop(&reference, None, start_time);
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