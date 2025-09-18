use macroquad::prelude::*;

use crate::language::Language;
use crate::color_scheme::ColorScheme;
use crate::utils;


pub enum PopupContent {
    Language(Language),
    ColorScheme(ColorScheme),
}

pub struct Popup {
    _content: PopupContent,
    pub visible: bool,
    selected: usize,
    ignore_next_enter: bool,
}

impl Popup {
    pub fn new(content: PopupContent) -> Self {
        Self {
            _content: content,
            visible: false,
            selected: 0,
            ignore_next_enter: false,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
        self.ignore_next_enter = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn draw(&mut self, font: &Option<Font>, language: &mut Option<Language>, _theme: &mut Option<ColorScheme>) -> Option<Language> {
        if !self.visible {
            return None;
        }
        if self.ignore_next_enter {
            if is_key_pressed(KeyCode::Enter) {
                return None;
            }
            self.ignore_next_enter = false;
        }

        let screen_w = screen_width();
        let screen_h = screen_height();

        let popup_w = screen_w * 0.3;
        let popup_h = screen_h * 0.3;
        let x = (screen_w - popup_w) / 2.0;
        let y = (screen_h - popup_h) / 2.0;

        let bg_color = Color::from_rgba(10, 10, 10, 255);
        let main_color = Color::from_rgba(255, 155, 0, 255);
        let ref_color = Color::from_rgba(100, 60, 0, 255);
        let border_color = Color::from_rgba(100, 60, 0, 255);

        //draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.5));

        utils::draw_rounded_rect(x, y, popup_w, popup_h, 20.0, bg_color);

        utils::draw_rounded_rect_lines(x, y, popup_w, popup_h, 20.0, 5.0, border_color);

        let title = "Select Language";
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
        if language.is_some() {
            for (i, lang) in Language::all().iter().enumerate() {
                let item_y = y + 90.0 + i as f32 * item_h;
                let rect = Rect::new(x + 20.0, item_y - 20.0, popup_w - 40.0, item_h);
    
                if i == self.selected {
                    draw_rectangle(rect.x, rect.y, rect.w, rect.h, main_color);
                    draw_text_ex(
                        &lang.to_string(),
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
                        &lang.to_string(),
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
            if is_key_pressed(KeyCode::Down) && self.selected + 1 < Language::count() {
                self.selected += 1;
            }
        } else {
            let title = "Select Theme";
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

            for (i, _theme) in ColorScheme::all().iter().enumerate() {
                let item_y = y + 90.0 + i as f32 * item_h;
                let rect = Rect::new(x + 20.0, item_y - 20.0, popup_w - 40.0, item_h);
    
                if i == self.selected {
                    draw_rectangle(rect.x, rect.y, rect.w, rect.h, main_color);
                    draw_text_ex(
                        "selected",
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
                        "not selected",
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
            if is_key_pressed(KeyCode::Down) && self.selected + 1 < 5 {
                self.selected += 1;
            }
        }

        if is_key_pressed(KeyCode::Up) && self.selected > 0 {
            self.selected -= 1;
        }
        
        if is_key_pressed(KeyCode::Enter) {
            //*language = Language::all()[self.selected];
            self.hide();
            return *language;
        }

        None
    }
}