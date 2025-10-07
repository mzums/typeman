#[cfg(feature = "cli")]
use std::path::PathBuf;

#[cfg(feature = "gui")]
use macroquad::prelude::*;

use ::rand::Rng;
use ::rand::prelude::IndexedRandom;
use ::rand::prelude::SliceRandom;

use std::collections::VecDeque;

use crate::{Quote, language::Language};

const QUOTES: &str = include_str!("../assets/quotes.json");

pub fn read_first_n_words(n: usize, language: Language) -> Vec<String> {
    language.get_words(n)
}

#[cfg(feature = "cli")]
pub fn validate_custom_file(path: &PathBuf) -> Result<(), String> {
    if path.exists() && path.is_file() {
        Ok(())
    } else {
        Err(format!("Custom file does not exist or is not a file: {:?}", path))
    }
}

#[cfg(feature = "gui")]
pub fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, radius: f32, color: Color) {
    draw_rectangle(x + radius, y, w - 2.0 * radius, h, color);
    draw_rectangle(x, y + radius, w, h - 2.0 * radius, color);

    draw_circle(x + radius, y + radius, radius, color);
    draw_circle(x + w - radius, y + radius, radius, color);
    draw_circle(x + radius, y + h - radius, radius, color);
    draw_circle(x + w - radius, y + h - radius, radius, color);
}

#[cfg(feature = "gui")]
pub fn draw_rounded_rect_lines(x: f32, y: f32, w: f32, h: f32, radius: f32, width: f32, color: Color) {
    draw_line(x + radius, y, x + w - radius, y, width, color);
    draw_line(x + radius, y + h, x + w - radius, y + h, width, color);
    draw_line(x, y + radius, x, y + h - radius, width, color);
    draw_line(x + w, y + radius, x + w, y + h - radius, width, color);

    let segments = 16;
    let theta = std::f32::consts::PI / 2.0 / segments as f32;

    for i in 0..segments {
        let angle1 = i as f32 * theta;
        let angle2 = (i + 1) as f32 * theta;

        // Top-left corner
        draw_line(
            x + radius - radius * angle1.cos(),
            y + radius - radius * angle1.sin(),
            x + radius - radius * angle2.cos(),
            y + radius - radius * angle2.sin(),
            width,
            color,
        );

        // Top-right corner
        draw_line(
            x + w - radius + radius * angle1.cos(),
            y + radius - radius * angle1.sin(),
            x + w - radius + radius * angle2.cos(),
            y + radius - radius * angle2.sin(),
            width,
            color,
        );

        // Bottom-left corner
        draw_line(
            x + radius - radius * angle1.cos(),
            y + h - radius + radius * angle1.sin(),
            x + radius - radius * angle2.cos(),
            y + h - radius + radius * angle2.sin(),
            width,
            color,
        );

        // Bottom-right corner
        draw_line(
            x + w - radius + radius * angle1.cos(),
            y + h - radius + radius * angle1.sin(),
            x + w - radius + radius * angle2.cos(),
            y + h - radius + radius * angle2.sin(),
            width,
            color,
        );
    }
}

pub fn get_reference(punctuation: bool, digits: bool, word_list: &[String], batch_size: usize) -> String {
    let mut items = Vec::new();
    let mut rng = ::rand::rng();

    // Calculate how many digits to include (if enabled)
    let num_digits = if digits {
        let max_digits = batch_size.min(batch_size / 3).max(1);
        rng.random_range((batch_size / 6).max(1)..=max_digits)
    } else {
        0
    };

    // Calculate how many words we need (remaining slots after digits)
    let num_words = batch_size - num_digits;

    // Generate words
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

    // Generate digits
    for _ in 0..num_digits {
        let choice = rng.random_range(0..4);
        let number = if choice == 0 {
            rng.random_range(1..10000).to_string()
        } else if choice == 1 {
            format!("{:04}", rng.random_range(0..10000))
        } else {
            rng.random_range(1..100).to_string()
        };
        items.push(number);
    }

    // Shuffle all items and join with spaces
    items.shuffle(&mut rng);
    
    // Clean each item to remove any embedded whitespace that could cause extra words
    let cleaned_items: Vec<String> = items.into_iter()
        .map(|item| item.replace(|c: char| c.is_whitespace(), ""))
        .filter(|item| !item.is_empty())
        .collect();
    
    let result = cleaned_items.join(" ");

    // Verify the word count matches batch_size
    debug_assert_eq!(
        result.split_whitespace().count(), 
        batch_size,
        "Generated text word count doesn't match batch_size"
    );

    result
}

pub fn get_random_quote() -> String {
    let quotes: Vec<Quote> = serde_json::from_str(QUOTES).unwrap_or_default();

    let mut rng = ::rand::rng();

    let fallback = Quote {
        text: "Welcome to TypeMan!".to_string(),
        author: "mzums".to_string(),
    };

    let random_quote = quotes.choose(&mut rng).unwrap_or(&fallback);

    format!("\"{}\" - {}", random_quote.text, random_quote.author)
}

pub fn count_correct_words(reference: &str, is_correct: &VecDeque<i32>) -> (usize, usize, usize) {
    let mut correct_words = 0;
    let mut no_corrected_words = 0;
    let mut all_words = 0;
    let mut word_correct = true;
    let mut word_corrected = true;
    let mut char_idx = 0;

    for c in reference.chars() {
        if is_correct[char_idx] == 0 {
            break;
        }
        if c == ' ' {
            if word_correct && char_idx > 0 && (is_correct[char_idx] == 1 || is_correct[char_idx] == 2) {
                correct_words += 1;
            }
            if word_corrected && char_idx > 0 && is_correct[char_idx] == 2 {
                no_corrected_words += 1;
            }
            all_words += 1;
            word_correct = true;
            word_corrected = true;
        } else {
            if char_idx < is_correct.len() && is_correct[char_idx] <= 0 {
                word_correct = false;
            }
            if char_idx < is_correct.len() && is_correct[char_idx] == 1 || is_correct[char_idx] == -1 {
                word_corrected = false;
            }
        }
        char_idx += 1;
    }
    if !reference.ends_with(' ') && char_idx > 0 {
        if word_correct {
            correct_words += 1;
        }
        if word_corrected {
            no_corrected_words += 1;
        }
        all_words += 1;
    }
    (no_corrected_words, correct_words, all_words)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::Language;

    #[test]
    fn test_reference_text_structure() {
        let word_list = read_first_n_words(500, Language::English);
        
        // Test that reference text structure is correct for word completion logic
        for &batch_size in &[5, 10, 25] {
            let reference = get_reference(false, false, &word_list, batch_size);
            
            // Reference should not end with a space (important for completion logic)
            assert!(!reference.ends_with(' '), 
                "Reference should not end with space: '{}'", reference);
            
            // Reference should not start with a space
            assert!(!reference.starts_with(' '), 
                "Reference should not start with space: '{}'", reference);
            
            // Count words by splitting on whitespace should match batch_size
            let word_count = reference.split_whitespace().count();
            assert_eq!(word_count, batch_size,
                "Word count mismatch: expected {}, got {} for reference: '{}'", 
                batch_size, word_count, reference);
        }
    }

    #[test]
    fn test_word_count_generation() {
        let word_list = read_first_n_words(500, Language::English);
        
        // Test various batch sizes including the problematic 25
        let test_sizes = [1, 10, 25, 50, 100];
        let test_configs = [
            (false, false), // no punctuation, no digits
            (true, false),  // punctuation, no digits
            (false, true),  // no punctuation, digits
            (true, true),   // punctuation, digits
        ];

        for &batch_size in &test_sizes {
            for &(punctuation, digits) in &test_configs {
                // Run multiple times to catch random variations
                for iteration in 0..10 {
                    let reference = get_reference(punctuation, digits, &word_list, batch_size);
                    let actual_word_count = reference.split_whitespace().count();
                    
                    assert_eq!(
                        actual_word_count, 
                        batch_size,
                        "Word count mismatch: expected {}, got {} for batch_size={}, punctuation={}, digits={}, iteration={}. Reference: '{}'",
                        batch_size, actual_word_count, batch_size, punctuation, digits, iteration, reference
                    );
                    
                    // Additional check: ensure no double spaces or empty words
                    assert!(!reference.contains("  "), 
                        "Reference contains double spaces: '{}'", reference);
                    assert!(!reference.trim().is_empty(), 
                        "Reference is empty");
                }
            }
        }
    }
}
