use macroquad::prelude::*;

use crate::color_scheme::ColorScheme;
use crate::language::Language;
use crate::utils;

pub enum PopupContent {
    Language,
    ColorScheme,
}

pub struct PopupState {
    pub visible: bool,
    pub selected: usize,
    //ignore_next_enter: bool,
}

pub struct PopupStates {
    pub language: PopupState,
    pub color_scheme: PopupState,
}

pub trait PopupData {
    fn title(&self) -> &'static str;
    fn items(&self) -> Vec<String>;
    fn selected_index<'a>(&self, popup_states: &'a PopupStates) -> &'a usize;
}

impl PopupData for PopupContent {
    fn title(&self) -> &'static str {
        match self {
            PopupContent::Language => "Select Language",
            PopupContent::ColorScheme => "Select Color Scheme",
        }
    }

    fn items(&self) -> Vec<String> {
        match self {
            PopupContent::Language => Language::all().iter().map(|x| x.to_string()).collect(),
            PopupContent::ColorScheme => ColorScheme::all().iter().map(|x| x.name().to_string()).collect(),
        }
    }

    fn selected_index<'a>(&self, popup_states: &'a PopupStates) -> &'a usize {
        match self {
            PopupContent::Language => &popup_states.language.selected,
            PopupContent::ColorScheme => &popup_states.color_scheme.selected,
        }
    }
}

impl PopupState {
    pub fn new() -> Self {
        Self {
            visible: false,
            selected: 0,
            //ignore_next_enter: false,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
        //self.ignore_next_enter = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn draw(
        &mut self,
        font: &Option<Font>,
        theme: &mut ColorScheme,
        content: PopupContent,
    ) -> Option<PopupState> {
        if !self.visible {
            return None;
        }

        /*if self.ignore_next_enter {
            if is_key_pressed(KeyCode::Enter) {
                return None;
            }
            self.ignore_next_enter = false;
        }*/

        let screen_w = screen_width();
        let screen_h = screen_height();

        let popup_w = screen_w * 0.3;
        let popup_h = screen_h * 0.3;
        let x = (screen_w - popup_w) / 2.0;
        let y = (screen_h - popup_h) / 2.0;

        let bg_color = theme.bg_color();
        let main_color = theme.main_color();
        let ref_color = theme.ref_color();
        let border_color = theme.border_color();

        utils::draw_rounded_rect(x, y, popup_w, popup_h, 20.0, bg_color);
        utils::draw_rounded_rect_lines(x, y, popup_w, popup_h, 20.0, 5.0, border_color);

        let (title, items): (&str, Vec<String>) = match content {
            PopupContent::Language => (
                "Select Language",
                Language::all().iter().map(|l| l.to_string()).collect(),
            ),
            PopupContent::ColorScheme => (
                "Select Theme",
                ColorScheme::all()
                    .iter()
                    .map(|c| c.name().to_string())
                    .collect(),
            ),
        };

        let title_size = measure_text(title, font.as_ref(), 24, 1.0);
        draw_text_ex(
            title,
            x + (popup_w - title_size.width) / 2.0,
            y + 50.0,
            TextParams {
                font: font.as_ref(),
                font_size: 24,
                font_scale: 1.0,
                color: ref_color,
                ..Default::default()
            },
        );

        let item_h = 30.0;
        for (i, item) in items.iter().enumerate() {
            let item_y = y + 90.0 + i as f32 * item_h;
            let rect = Rect::new(x + 20.0, item_y - 20.0, popup_w - 40.0, item_h);

            if i == self.selected {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, main_color);
                draw_text_ex(
                    item,
                    rect.x + 10.0,
                    rect.y + rect.h - 8.0,
                    TextParams {
                        font: font.as_ref(),
                        font_size: 20,
                        font_scale: 1.0,
                        color: bg_color,
                        ..Default::default()
                    },
                );
            } else {
                draw_text_ex(
                    item,
                    rect.x + 10.0,
                    rect.y + rect.h - 8.0,
                    TextParams {
                        font: font.as_ref(),
                        font_size: 20,
                        font_scale: 1.0,
                        color: ref_color,
                        ..Default::default()
                    },
                );
            }
        }

        if is_key_pressed(KeyCode::Up) && self.selected > 0 {
            self.selected -= 1;
        }
        if is_key_pressed(KeyCode::Down) && self.selected + 1 < items.len() {
            self.selected += 1;
        }

        /*if is_key_pressed(KeyCode::Enter) {
            *popup_recently_closed = true;
            self.hide();

            match content {
                PopupContent::Language => {
                    let choice = Language::all()[self.selected];
                    *language = choice;
                    return Some(PopupState::Language(choice));
                }
                PopupContent::ColorScheme => {
                    let choice = ColorScheme::all()[self.selected];
                    *theme = choice;
                    return Some(PopupResult::ColorScheme(choice));
                }
            }
        }*/

        None
    }
}
