use crossterm::{
    cursor, queue,
    style::{Color, Print, SetForegroundColor, Attribute, SetAttribute},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use std::time::Instant;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::fs::{self, OpenOptions};
use std::path::Path;


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

fn display_results(elapsed: f64, accuracy: f64, wpm: f64) {
    println!("\n\nTime: {:.2}s | Accuracy: {:.1}% | WPM: {:.1}",
        elapsed,
        accuracy,
        wpm
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

pub fn type_loop(reference: &str, time_limit: Option<u64>, start_time: Instant, practice: Option<usize>) -> i32 {
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
        update_timer(&mut stdout, timer_pos, start_time, &mut last_update, width, position);

        let byte_opt = poll_input();
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
        );

        stdout.flush().unwrap();

        if position >= ref_chars.len() {
            break;
        }
    }
    if practice.is_some() {
        let elapsed = start_time.elapsed().as_secs_f64();
        let error_count = error_positions.iter().filter(|&&e| e).count();
        let accuracy = 100.0 - (error_count as f64 / reference.len() as f64 * 100.0);
        let wpm = (user_input.len() as f64 / 5.0) / (elapsed / 60.0);

        let results_dir = "practice_results";
        fs::create_dir_all(results_dir).ok();

        let filename = format!("{}/level_{:?}.txt", results_dir, practice.unwrap());

        let stats = format!(
            "Time: {:.2}s\nAccuracy: {:.1}%\nWPM: {:.1}\n---\n",
            elapsed, accuracy, wpm
        );

        let file_path = Path::new(&filename);

        let mut prev_best_wpm = None;
        if file_path.exists() {
            if let Ok(contents) = fs::read_to_string(file_path) {
            for line in contents.lines() {
                if line.starts_with("WPM:") {
                    if let Some(wpm_str) = line.split_whitespace().nth(1) {
                        if let Ok(val) = wpm_str.parse::<f64>() {
                            if prev_best_wpm.map_or(true, |best| val > best) {
                                prev_best_wpm = Some(val);
                            }
                        }
                    }
                }
            }
            }
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .unwrap();
        file.write_all(stats.as_bytes()).unwrap();

        if prev_best_wpm.map_or(true, |best| wpm > best) {
            println!("\nNew highscore for this level!");
        }
    }
    show_final_results(reference, &user_input, &error_positions, start_time);

    0
}

fn update_timer(
    stdout: &mut std::io::Stdout,
    timer_pos: (u16, u16),
    start_time: Instant,
    last_update: &mut Instant,
    width: u16,
    position: usize,
) {
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
            cursor::MoveTo(position as u16 % width, position as u16 / width + 2)
        )
        .unwrap();
        stdout.flush().unwrap();
        *last_update = Instant::now();
    }
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
) {
    match byte {
        // backspace
        8 | 127 if *position > 0 => {
            *position -= 1;
            user_input.pop();

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

            if c == ref_char {
                if error_positions[*position] {
                    // Corrected an error: yellow
                    queue!(
                        stdout,
                        SetForegroundColor(Color::Yellow),
                        Print(c),
                        SetForegroundColor(Color::Reset)
                    )
                    .unwrap();
                } else {
                    // Correct on first try: green
                    queue!(
                        stdout,
                        SetForegroundColor(Color::Green),
                        Print(c),
                        SetForegroundColor(Color::Reset)
                    )
                    .unwrap();
                }
                user_input.push(c);
                *position += 1;
            } else {
                error_positions[*position] = true;
                queue!(
                    stdout,
                    SetForegroundColor(Color::Red),
                    Print(ref_char),
                    SetForegroundColor(Color::Reset)
                )
                .unwrap();
                user_input.push(c);
                *position += 1;
            }
        }
        _ => {}
    }
}

fn show_final_results(
    reference: &str,
    user_input: &str,
    error_positions: &[bool],
    start_time: Instant,
) {
    let elapsed = start_time.elapsed().as_secs_f64();
    let error_count = error_positions.iter().filter(|&&e| e).count();
    let accuracy = 100.0 - (error_count as f64 / reference.len() as f64 * 100.0);
    let wpm = (user_input.len() as f64 / 5.0) / (elapsed / 60.0);

    let term_width = crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
    let lines = (reference.len() + term_width - 1) / term_width;

    let mut stdout = stdout();
    queue!(
        stdout,
        cursor::MoveTo(0, (lines as u16) + 1)
    )
    .unwrap();
    stdout.flush().unwrap();
    display_results(elapsed, accuracy, wpm);
}
