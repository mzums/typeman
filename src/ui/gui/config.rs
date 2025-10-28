use core::time;
use macroquad::prelude::*;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::color_scheme::ColorScheme;
use crate::language::Language;
use crate::ui::gui::main;
use crate::ui::gui::popup::{PopupContent, PopupStates};
use crate::{practice, utils};
use crate::config::AppConfig;

fn save_config(punctuation: bool, numbers: bool, time_mode: bool, word_mode: bool, quote: bool, test_time: f32, batch_size: usize, practice_mode: bool, wiki_mode: bool, language: Language, color_scheme: ColorScheme, word_number: usize, top_words: usize, selected_practice_level: Option<usize>) {
    let app_config = AppConfig {
        punctuation: punctuation,
        numbers: numbers,
        time_mode: time_mode,
        word_mode: word_mode,
        quote: quote,
        practice_mode: practice_mode,
        wiki_mode: wiki_mode,
        batch_size: batch_size,
        test_time: test_time,
        selected_level: selected_practice_level.unwrap_or(1),
        language: language,
        color_scheme: color_scheme,
        word_number: word_number,
        top_words: top_words,
    };

    let _ = app_config.save();
}

fn draw_toggle_button(
    x: f32,
    y: f32,
    btn_padding: f32,
    display_name: &str,
    font: &Option<Font>,
    is_active: bool,
    visible: bool,
    font_size: u16,
    selected: bool,
    color_scheme: &crate::color_scheme::ColorScheme,
) -> (bool, bool, f32) {
    if !visible {
        return (false, false, 0.0);
    }
    let padding = font_size as f32 * 0.5;

    let text_dims = measure_text(display_name, Some(font.as_ref().unwrap()), font_size, 1.0);
    let btn_width = text_dims.width + btn_padding * 2.0;
    let btn_height = measure_text("t", font.as_ref(), font_size, 1.0).height + padding * 2.0;

    let rect = Rect::new(x, y, btn_width, btn_height);
    let (mx, my) = mouse_position();
    let hovered = rect.contains(vec2(mx, my));
    let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);

    let mut text_color = if is_active {
        color_scheme.main_color()
    } else {
        color_scheme.ref_color()
    };
    let mut bg_color = Color::from_rgba(255, 0, 0, 0);
    if selected && is_active {
        text_color = macroquad::color::BLACK;
        bg_color = color_scheme.dimmer_main();
    } else if selected {
        text_color = macroquad::color::BLACK;
        bg_color = color_scheme.border_color();
    }

    let font_size: u16 = if display_name == "|" {
        (font_size as f32 * 1.5) as u16
    } else {
        font_size
    };

    let corner_radius: f32 = font_size as f32 / 3.0;
    let btn_x = x;
    utils::draw_rounded_rect(btn_x, y, btn_width, btn_height, corner_radius, bg_color);
    draw_text_ex(
        display_name,
        x + btn_padding,
        y + btn_height - padding,
        TextParams {
            font: font.as_ref(),
            font_size,
            font_scale: 1.0,
            color: text_color,
            ..Default::default()
        },
    );

    (clicked, hovered, btn_width)
}

pub fn update_game_state(
    reference: &str,
    pressed_vec: &mut Vec<char>,
    is_correct: &mut VecDeque<i32>,
    pos1: &mut usize,
    timer: &mut Duration,
    start_time: &mut Instant,
    game_started: &mut bool,
    game_over: &mut bool,
    test_time: f32,
    time_mode: bool,
    words_done: &mut usize,
    errors_this_second: &mut f64,
    practice_mode: &mut bool,
    practice_menu: bool,
) {
    if !*game_started
        && main::handle_input(
            reference,
            pressed_vec,
            is_correct,
            pos1,
            words_done,
            errors_this_second,
            &mut false,
            &mut vec![false; reference.chars().count()],
            *practice_mode,
            practice_menu,
        )
    {
        *game_started = true;
        *start_time = Instant::now();
    }

    if *game_started && !*game_over {
        *timer = start_time.elapsed();
        if (timer.as_secs_f32() >= test_time && time_mode) || *pos1 >= reference.chars().count() {
            *game_over = true;
        }
    }
}

pub fn reset_game_state(
    pressed_vec: &mut Vec<char>,
    is_correct: &mut VecDeque<i32>,
    pos1: &mut usize,
    timer: &mut Duration,
    start_time: &mut Instant,
    game_started: &mut bool,
    game_over: &mut bool,
    speed_per_second: &mut Vec<f64>,
    last_recorded_time: &mut Instant,
    words_done: &mut usize,
    errors_per_second: &mut Vec<f64>,
    saved_results: &mut bool,
    error_positions: &mut Vec<bool>,
) {
    *is_correct = VecDeque::from(vec![0; is_correct.len()]);
    pressed_vec.clear();
    *error_positions = vec![false; is_correct.len()];
    *pos1 = 0;
    *timer = Duration::new(0, 0);
    *start_time = Instant::now();
    *game_started = false;
    *game_over = false;
    *speed_per_second = vec![];
    *errors_per_second = vec![];
    *last_recorded_time = Instant::now();
    *words_done = 0;
    *saved_results = false;
}

pub fn handle_settings_buttons(
    font: &Option<Font>,
    _word_list: &[String],
    punctuation: &mut bool,
    numbers: &mut bool,
    quote: &mut bool,
    time_mode: &mut bool,
    word_mode: &mut bool,
    pressed_vec: &mut Vec<char>,
    is_correct: &mut VecDeque<i32>,
    pos1: &mut usize,
    timer: &mut time::Duration,
    start_time: &mut Instant,
    game_started: &mut bool,
    game_over: &mut bool,
    reference: &mut String,
    test_time: &mut f32,
    batch_size: &mut usize,
    start_x: f32,
    speed_per_second: &mut Vec<f64>,
    last_recorded_time: &mut Instant,
    words_done: &mut usize,
    errors_per_second: &mut Vec<f64>,
    font_size: u16,
    config_opened: &mut bool,
    selected_config: &mut String,
    practice_menu: &mut bool,
    selected_practice_level: &mut Option<usize>,
    practice_mode: &mut bool,
    saved_results: &mut bool,
    error_positions: &mut Vec<bool>,
    language: &mut Language,
    color_scheme: &mut crate::color_scheme::ColorScheme,
    wiki_mode: &mut bool,
    menu_buttons_times: &mut std::collections::HashMap<String, Instant>,
    popup_states: &mut PopupStates,
    top_words: usize,
) -> bool {

    let btn_y = screen_height() / 5.0;
    let btn_padding = if screen_width() > 800.0 {
        font_size as f32 * 0.7
    } else {
        font_size as f32 * 0.4
    };
    let divider = true;
    let mut total_width = 0.0;

    let mut button_states = vec![
        (
            "punctuation",
            if screen_width() > screen_height() && screen_width() > 1500.0 {
                "! punctuation"
            } else {
                "! punct"
            },
            *punctuation,
            !*quote && !*practice_mode,
        ),
        (
            "numbers",
            if screen_width() > screen_height() && screen_width() > 1500.0 {
                "# numbers"
            } else {
                "# num"
            },
            *numbers,
            !*quote && !*practice_mode,
        ),
        ("|", "|", divider, true),
        (
            "language",
            if screen_width() > screen_height() && screen_width() > 1500.0 {
                "language"
            } else {
                "lang"
            },
            popup_states.language.visible,
            true,
        ),
        ("theme", "theme", popup_states.color_scheme.visible, true),
        ("|", "|", divider, true),
        ("time", "time", *time_mode, true),
        ("words", "words", *word_mode, true),
        ("quote", "quote", *quote, true),
        ("practice", "practice", *practice_mode, true),
        (
            "wikipedia",
            if screen_width() > screen_height() && screen_width() > 1500.0 {
                "wikipedia"
            } else {
                "wiki"
            },
            *wiki_mode,
            true,
        ),
    ];

    if is_key_pressed(KeyCode::Up) {
        if !popup_states.language.visible && !popup_states.color_scheme.visible && !popup_states.time_selection.visible && !popup_states.word_number_selection.visible {
            *config_opened = true;
        }
    } else if is_key_pressed(KeyCode::Down) {
        if !popup_states.language.visible && !popup_states.color_scheme.visible && !popup_states.time_selection.visible && !popup_states.word_number_selection.visible {
            *config_opened = false;
        }
    } else if is_key_pressed(KeyCode::Left) {
        if !*config_opened {
            return false;
        }

        for (i, (label, _display_name, _state_val, visible)) in button_states.iter().enumerate() {
            if *visible && *selected_config == *label {
                let mut j = if i == 0 {
                    button_states.len() - 1
                } else {
                    i - 1
                };

                while j != i {
                    if button_states[j].3 && button_states[j].0 != "|" {
                        *selected_config = button_states[j].0.to_string();
                        break;
                    }
                    j = if j == 0 {
                        button_states.len() - 1
                    } else {
                        j - 1
                    };
                }
                break;
            }
        }
    } else if is_key_pressed(KeyCode::Right) {
        if !*config_opened {
            return false;
        }

        for (i, (label, _display_name, _state_val, visible)) in button_states.iter().enumerate() {
            if *visible && *selected_config == *label {
                let mut next = if i == button_states.len() - 1 {
                    0
                } else {
                    i + 1
                };

                while next != i {
                    if button_states[next].3 && button_states[next].0 != "|" {
                        *selected_config = button_states[next].0.to_string();
                        break;
                    }
                    next = if next == button_states.len() - 1 {
                        0
                    } else {
                        next + 1
                    };
                }
                break;
            }
        }
    } else if is_key_pressed(KeyCode::Enter)
        && *config_opened
        {

            println!("{}", batch_size);
       
        if popup_states.language.visible {
            *language = match popup_states.language.selected {
                0 => Language::English,
                1 => Language::Indonesian,
                2 => Language::Italian,
                _ => Language::English,
            };
            popup_states.language.visible = false;
            popup_states.language.hide();
            if *word_mode || *time_mode {
                if *time_mode {
                    *reference = utils::get_reference(*punctuation, *numbers, &utils::read_first_n_words(top_words, *language), *batch_size);
                } else if *word_mode {
                    *reference = utils::get_reference(*punctuation, *numbers, &utils::read_first_n_words(top_words, *language), usize::min(*batch_size, *batch_size));
                }
                *is_correct = VecDeque::from(vec![0; reference.chars().count()]);
                *error_positions = vec![false; reference.chars().count()];
                pressed_vec.clear();
                *pos1 = 0;
                *words_done = 0;
            }
            return false;
        } else if popup_states.color_scheme.visible {
            *color_scheme = match popup_states.color_scheme.selected {
                0 => ColorScheme::Default,
                1 => ColorScheme::Dark,
                2 => ColorScheme::Light,
                3 => ColorScheme::Monochrome,
                4 => ColorScheme::Ocean,
                5 => ColorScheme::OceanDark,
                6 => ColorScheme::Forest,
                7 => ColorScheme::ForestDark,
                8 => ColorScheme::Pink,
                _ => ColorScheme::Default,
            };
            popup_states.color_scheme.visible = false;
            popup_states.color_scheme.hide();
            return false;
        } else if popup_states.time_selection.visible {
            *test_time = match popup_states.time_selection.selected {
                0 => 15.0,
                1 => 30.0,
                2 => 60.0,
                3 => 120.0,
                _ => 30.0,
            };
            *reference = utils::get_reference(*punctuation, *numbers, &utils::read_first_n_words(top_words, *language), *batch_size);
            popup_states.time_selection.visible = false;
            popup_states.time_selection.hide();
            return false;
        } else if popup_states.word_number_selection.visible {
            *batch_size = match popup_states.word_number_selection.selected {
                0 => 25,
                1 => 50,
                2 => 100,
                3 => 200,
                4 => 500,
                _ => 50,
            };
            *reference = utils::get_reference(*punctuation, *numbers, &utils::read_first_n_words(top_words, *language), *batch_size);
            popup_states.word_number_selection.visible = false;
            popup_states.word_number_selection.hide();
            return false;
        }

        update_config(
            &selected_config,
            punctuation,
            numbers,
            time_mode,
            word_mode,
            quote,
            practice_menu,
            selected_practice_level,
            practice_mode,
            language,
            popup_states,
            wiki_mode,
        );

        save_config(*punctuation, *numbers, *time_mode, *word_mode, *quote, *test_time, *batch_size, *practice_mode, *wiki_mode, *language, *color_scheme, *batch_size, top_words, *selected_practice_level);

        if *quote {
            *reference = utils::get_random_quote();
        } else if *practice_mode {
            *reference = practice::create_words(
                practice::TYPING_LEVELS[selected_practice_level.unwrap_or(0)].1,
                *batch_size,
            );
            if let Some(time) = menu_buttons_times.get_mut("practice") {
                *time = Instant::now();
            }
        } else if *wiki_mode {
            *reference = utils::get_wiki_summary();
            if let Some(time) = menu_buttons_times.get_mut("wiki") {
                *time = Instant::now();
            }
        } else if *selected_config != "language" && *selected_config != "theme" {
            let updated_word_list = utils::read_first_n_words(500, *language);
            *reference =
                utils::get_reference(*punctuation, *numbers, &updated_word_list, *batch_size);
        }
        if *selected_config == "time" {
            if menu_buttons_times.get("time").map_or(true, |&t| t.elapsed() <= Duration::from_millis(500)) {
                popup_states.time_selection.visible = true;
            }
            if let Some(time) = menu_buttons_times.get_mut("time") {
                *time = Instant::now();
            }
        } else if *selected_config == "words" {
            if menu_buttons_times.get("words").map_or(true, |&t| t.elapsed() <= Duration::from_millis(500)) {
                popup_states.word_number_selection.visible = true;
            }
            if let Some(time) = menu_buttons_times.get_mut("words") {
                *time = Instant::now();
            }
        }
        if selected_config != "language" && selected_config != "theme" {
            *is_correct = VecDeque::from(vec![0; reference.len()]);
            *error_positions = vec![false; is_correct.len()];
            reset_game_state(
                pressed_vec,
                is_correct,
                pos1,
                timer,
                start_time,
                game_started,
                game_over,
                speed_per_second,
                last_recorded_time,
                words_done,
                errors_per_second,
                saved_results,
                &mut vec![false; reference.chars().count()],
            );
        }
    }

    let mut any_button_hovered = false;

    for (label, display_name, state_val, visible) in button_states.iter_mut() {
        let x = start_x + total_width;
        let is_active = *state_val;

        let (clicked, hovered, btni_width) = draw_toggle_button(
            x,
            btn_y,
            btn_padding,
            display_name,
            font,
            is_active,
            *visible,
            font_size,
            selected_config == label && *config_opened,
            color_scheme,
        );
        total_width += btni_width;

        if hovered && *label != "|" {
            any_button_hovered = true;
        }

        if clicked && *label != "|" && *label != "language" {
            update_config(
                label,
                punctuation,
                numbers,
                time_mode,
                word_mode,
                quote,
                practice_menu,
                selected_practice_level,
                practice_mode,
                language,
                popup_states,
                wiki_mode,
            );
            if *quote {
                *reference = utils::get_random_quote();
                *is_correct = VecDeque::from(vec![0; reference.chars().count()]);
                *error_positions = vec![false; is_correct.len()];
                *punctuation = false;
                *numbers = false;
                reset_game_state(
                    pressed_vec,
                    is_correct,
                    pos1,
                    timer,
                    start_time,
                    game_started,
                    game_over,
                    speed_per_second,
                    last_recorded_time,
                    words_done,
                    errors_per_second,
                    saved_results,
                    error_positions,
                );
            } else if *practice_menu {
                *practice_menu = true;
            } else {
                let updated_word_list = utils::read_first_n_words(500, *language);
                *reference = utils::get_reference(*punctuation, *numbers, &updated_word_list, *batch_size);
                *is_correct = VecDeque::from(vec![0; reference.chars().count()]);
                *error_positions = vec![false; is_correct.len()];
                reset_game_state(
                    pressed_vec,
                    is_correct,
                    pos1,
                    timer,
                    start_time,
                    game_started,
                    game_over,
                    speed_per_second,
                    last_recorded_time,
                    words_done,
                    errors_per_second,
                    saved_results,
                    error_positions,
                );
            }
        }
    }
    if popup_states.language.visible {
        popup_states.language.draw(font, color_scheme, PopupContent::Language);
    } else if popup_states.color_scheme.visible {
        popup_states.color_scheme.draw(font, color_scheme, PopupContent::ColorScheme);
    } else if popup_states.time_selection.visible {
        popup_states.time_selection.draw(font, color_scheme, PopupContent::TimeSelection);
    } else if popup_states.word_number_selection.visible {
        popup_states.word_number_selection.draw(font, color_scheme, PopupContent::WordNumberSelection);
    }

    any_button_hovered
}

fn update_config(
    label: &str,
    punctuation: &mut bool,
    numbers: &mut bool,
    time_mode: &mut bool,
    word_mode: &mut bool,
    quote: &mut bool,
    practice_menu: &mut bool,
    selected_practice_level: &mut Option<usize>,
    practice_mode: &mut bool,
    language: &mut Language,
    popup_states: &mut PopupStates,
    wiki_mode: &mut bool,
) {
    match label {
        "punctuation" => {
            *punctuation = !*punctuation;
            *quote = false;
        }
        "numbers" => {
            *numbers = !*numbers;
            *quote = false;
        }
        "time" => {
            *time_mode = true;
            *word_mode = false;
            *quote = false;
            *practice_mode = false;
        }
        "words" => {
            *word_mode = true;
            *time_mode = false;
            *quote = false;
            *practice_mode = false;
        }
        "quote" => {
            *quote = true;
            *punctuation = false;
            *numbers = false;
            *time_mode = false;
            *word_mode = false;
            *practice_mode = false;
        }
        "practice" => {
            *quote = false;
            *punctuation = false;
            *numbers = false;
            *time_mode = false;
            *word_mode = false;
            *practice_menu = true;
            *selected_practice_level = Some(practice::get_first_not_done());
        }
        "wikipedia" => {
            *wiki_mode = true;
            *time_mode = false;
            *word_mode = false;
            *practice_mode = false;
            *quote = false;
        }
        "english" => {
            *language = Language::English;
        }
        "indonesian" => {
            *language = Language::Indonesian;
        }
        "language" => {
            popup_states.language.visible = true;
        }
        "theme" => {
            popup_states.color_scheme.show();
        }
        _ => {}
    }
}
