use rand::prelude::IndexedRandom;
use std::io::Write;
use std::path::Path;
use std::fs;


pub const WPM_MIN: f64 = 35.0;

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
    ("repetition: top row letters", &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p']),
    ("new: v, b, n", &['v', 'b', 'n']),
    ("new: c & m", &['c', 'm']),
    ("repetition: bottom row letters", &['z', 'x', 'c', 'v', 'b', 'n', 'm']),
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


pub fn create_words(chars: &[char], word_number: usize) -> String {
    let mut reference = String::new();
    for i in 0..word_number {
        let word_length = rand::random::<u16>() % 5 + 2;
        let word: String = (0..word_length)
            .map(|_| *chars.choose(&mut rand::rng()).unwrap())
            .collect();
        reference.push_str(&word);
        if i != word_number - 1 {
            reference.push(' ');
        }
    }
    return reference;
}


pub fn save_results(time: f64, accuracy: f64, wpm: f64, level:usize) {
    let results_dir = "practice_results";
    fs::create_dir_all(results_dir).ok();

    let filename = format!("{}/level_{:?}.txt", results_dir, level);
    let file_path = Path::new(&filename);

    let stats = format!(
        "Time: {:.2}s\nAccuracy: {:.1}%\nWPM: {:.1}\n---\n",
        time, accuracy, wpm
    );
    
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();
    file.write_all(stats.as_bytes()).unwrap();
}


pub fn get_prev_best_wpm(level: usize) -> f64 {
    let results_path = format!("practice_results/level_{}.txt", level);
    let contents = match fs::read_to_string(&results_path) {
        Ok(c) if !c.trim().is_empty() => c,
        _ => return 0.0,
    };
    let mut best_wpm = 0.0;
    for line in contents.lines() {
        if line.starts_with("WPM:") {
            if let Some(wpm_str) = line.strip_prefix("WPM:").map(str::trim) {
                if let Ok(wpm) = wpm_str.parse::<f64>() {
                    if wpm > best_wpm {
                        best_wpm = wpm;
                    }
                }
            }
        }
    }
    best_wpm
}

pub fn check_if_completed(results_path: &str) -> bool {
    if let Ok(contents) = std::fs::read_to_string(&results_path) {
        for line in contents.lines() {
            if line.starts_with("WPM:") {
                if let Some(wpm_str) = line.strip_prefix("WPM:").map(str::trim) {
                    if let Ok(wpm) = wpm_str.parse::<f32>() {
                        if wpm >= 35.0 {
                            return true;
                        }
                    }
                }
            }
        }
    }
    return false;
}

pub fn get_first_not_done() -> usize {
    for i in 0..TYPING_LEVELS.len() {
        let results_path = format!("practice_results/level_{}.txt", i + 1);
        let done = check_if_completed(results_path.as_str());
        if !done {
            return i;
        }
    }
    return 1;
}