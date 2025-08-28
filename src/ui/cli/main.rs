use crossterm::{
    cursor, queue,
    style::{Color, Print, SetForegroundColor, Attribute, SetAttribute},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use std::time::Instant;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::collections::VecDeque;
use crate::utils;
use crate::practice;


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

fn display_results(elapsed: f64, accuracy: f64, wpm: f64, raw: f64) {
    println!("\n\nTime: {:.0}s | Accuracy: {:.0}% | WPM: {:.0} | Raw WPM: {:.0}",
        elapsed,
        accuracy,
        wpm,
        raw
    );
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

pub fn type_loop(reference: &str, time_limit: Option<u64>, start_time: &mut Option<Instant>, practice: Option<usize>, is_correct: &mut VecDeque<i32>, mode: &str) -> i32 {
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

    let all_words = reference.split_whitespace().count();
    let mut words_done = 0;

    loop {
        if mode == "time" {
            update_timer(&mut stdout, timer_pos,*start_time, &mut last_update, width, position, time_limit);
        } else {
            update_word_count(&mut stdout, timer_pos, words_done, width, position, all_words);
        }

        let byte_opt = poll_input();
        if byte_opt.is_none() {
            if let Some(limit) = time_limit {
                if start_time.is_some() && start_time.unwrap().elapsed().as_secs() >= limit {
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
            continue;
        }

        let byte = byte_opt.unwrap();

        if handle_control_keys(byte, &mut stdout) {
            return 1;
        }

        handle_typing(
            byte,
            &mut user_input,
            &ref_chars,
            &mut position,
            &mut error_positions,
            &mut stdout,
            is_correct,
            practice.is_some(),
            &mut words_done,
            start_time
        );

        stdout.flush().unwrap();

        if position >= ref_chars.len() {
            break;
        }
    }
    if practice.is_some() && start_time.is_some() {
        let elapsed = start_time.unwrap().elapsed().as_secs_f64();
        let error_count = error_positions.iter().filter(|&&e| e).count();
        let accuracy = 100.0 - (error_count as f64 / reference.len() as f64 * 100.0);
        let wpm = (user_input.len() as f64 / 5.0) / (elapsed / 60.0);

        let term_width = crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
        let lines = (reference.len() + term_width - 1) / term_width;

        practice::save_results(
            elapsed,
            accuracy,
            wpm,
            practice.unwrap(),
        );

        queue!(
            stdout,
            cursor::MoveTo(0, (lines as u16) + 1)
        ).unwrap();

        if wpm >= practice::WPM_MIN {
            println!("\nLevel passed!\n")
        } else {
            println!("\nAchive WPM of 35 to pass this level.\n");
        }

        let prev_best_wpm = practice::get_prev_best_wpm(practice.unwrap());

        if prev_best_wpm < wpm as f64 {
            println!("\nNew highscore for this level!");
        }
    }
    show_final_results(reference, start_time.expect("No start time"), is_correct);

    0
}

fn update_timer(
    stdout: &mut std::io::Stdout,
    timer_pos: (u16, u16),
    start_time: Option<Instant>,
    last_update: &mut Instant,
    width: u16,
    position: usize,
    time_limit: Option<u64>,
) {
    if last_update.elapsed().as_millis() > 100 || start_time.is_none(){
        let elapsed_secs = if start_time.is_some() {
            start_time.unwrap().elapsed().as_secs()
        } else {
            0
        };
        let remaining = if let Some(limit) = time_limit {
            if elapsed_secs >= limit {
                0
            } else {
                limit - elapsed_secs
            }
        } else {
            elapsed_secs
        };

        let display_mins = remaining / 60;
        let display_secs = remaining % 60;

        queue!(
            stdout,
            cursor::MoveTo(timer_pos.0, timer_pos.1),
            Clear(ClearType::UntilNewLine),
            Print(format!("Time: {:02}:{:02}", display_mins, display_secs)),
            cursor::MoveTo(position as u16 % width, position as u16 / width + 2)
        )
        .unwrap();

        stdout.flush().unwrap();
        *last_update = Instant::now();
    }
}

fn update_word_count(
    stdout: &mut std::io::Stdout,
    pos: (u16, u16),
    words_done: usize,
    width: u16,
    position: usize,
    all_words: usize,
) {
    queue!(
        stdout,
        cursor::MoveTo(pos.0, pos.1),
        Clear(ClearType::UntilNewLine),
        Print(format!("{}\\{}", words_done, all_words)),
        cursor::MoveTo(position as u16 % width, position as u16 / width + 2)
    )
    .unwrap();

    stdout.flush().unwrap();
}

fn poll_input() -> Option<u8> {
    if event::poll(std::time::Duration::from_millis(10)).unwrap() {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read().unwrap() {
            match (code, modifiers) {
                (KeyCode::Char('c'), event::KeyModifiers::CONTROL) => Some(0x03), // Ctrl+C
                (KeyCode::Char('d'), event::KeyModifiers::CONTROL) => Some(0x04), // Ctrl+D
                (KeyCode::Char(c), _) => Some(c as u8),
                (KeyCode::Backspace, _) => Some(8),
                (KeyCode::Esc, _) => Some(0x1B),
                (KeyCode::Enter, _) => Some(b'\n'),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn handle_control_keys(byte: u8, stdout: &mut std::io::Stdout) -> bool {
    match byte {
        0x03 | 0x04 => {
            queue!(
                stdout,
                Clear(ClearType::All),
                cursor::MoveTo(0, 0)
            )
            .unwrap();
            true
        }
        0x1B => true, // ESC
        _ => false,
    }
}

fn handle_typing(
    byte: u8,
    user_input: &mut String,
    ref_chars: &[char],
    position: &mut usize,
    error_positions: &mut Vec<bool>,
    stdout: &mut std::io::Stdout,
    is_correct: &mut VecDeque<i32>,
    practice_mode: bool,
    words_done: &mut usize,
    start_time: &mut Option<Instant>
) {
    match byte {
        // backspace
        8 | 127 if *position > 0 => {
            is_correct[*position] = 0;
            *position -= 1;
            user_input.pop();
            if ref_chars.len() > *position + 1 && ref_chars[*position + 1] == ' ' {
                *words_done -= 1;
            }

            queue!(
                stdout,
                cursor::MoveLeft(1),
                SetAttribute(Attribute::Dim),
                Print(ref_chars[*position]),
                SetAttribute(Attribute::Reset),
                cursor::MoveLeft(1),
                SetForegroundColor(Color::Reset)
            )
            .unwrap();
        }
        _ if *position < ref_chars.len() => {
            let c = byte as char;
            let ref_char = ref_chars[*position];
            
            if ref_chars.len() > *position + 1 && ref_chars[*position + 1] == ' ' {
                *words_done += 1;
            }
            if c == ref_char {
                if start_time.is_none() {
                    *start_time = Some(Instant::now());
                }
                if error_positions[*position] {
                    is_correct[*position] = 1;
                    // Corrected an error: yellow
                    let char_display = if practice_mode && c == ' ' {
                        '_'
                    } else {
                        c
                    };
                    queue!(
                        stdout,
                        SetForegroundColor(Color::Yellow),
                        Print(char_display),
                        SetForegroundColor(Color::Reset)
                    )
                    .unwrap();
                } else {
                    // Correct on first try: green
                    is_correct[*position] = 2;
                    queue!(
                        stdout,
                        SetForegroundColor(Color::White),
                        Print(c),
                        SetForegroundColor(Color::Reset)
                    )
                    .unwrap();
                }
                user_input.push(c);
                *position += 1;
            } else {
                if start_time.is_none() {
                    *start_time = Some(Instant::now());
                }
                is_correct[*position] = -1;
                error_positions[*position] = true;
                if practice_mode {
                    return;
                }
                if ref_char == ' ' {
                    queue!(
                        stdout,
                        SetForegroundColor(Color::Red),
                        Print('_'),
                        SetForegroundColor(Color::Reset)
                    )
                .unwrap();
                } else {
                    queue!(
                        stdout,
                        SetForegroundColor(Color::Red),
                        Print(ref_char),
                        SetForegroundColor(Color::Reset)
                    )
                    .unwrap();
                }
                user_input.push(c);
                *position += 1;
            }
        }
        _ => {}
    }
}

fn show_final_results(
    reference: &str,
    start_time: Instant,
    is_correct: &VecDeque<i32>,
) {
    let (_corrected_words, correct_words, all_words) = utils::count_correct_words(&reference, &is_correct);
    let elapsed = start_time.elapsed().as_secs_f64();
    let wpm = correct_words as f64 / (elapsed / 60.0);
    let raw = all_words as f64 / (elapsed / 60.0);

    let term_width = crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
    let lines = (reference.len() + term_width - 1) / term_width;

    let mut stdout = stdout();
    queue!(
        stdout,
        cursor::MoveTo(0, (lines as u16) + 1)
    )
    .unwrap();
    stdout.flush().unwrap();
    
    let correct_count = is_correct.iter().filter(|&&v| v == 1 || v == 2).count();
    let all_pressed_count = is_correct.iter().filter(|&&v| v != 0).count();
    let accuracy = if correct_count > 0 {
        (correct_count as f64 / all_pressed_count as f64) * 100.0
    } else {
        0.0
    };
    display_results(elapsed, accuracy, wpm, raw);
    
    queue!(
        stdout,
        cursor::MoveToNextLine(1)
    ).unwrap();
    stdout.flush().unwrap();
}