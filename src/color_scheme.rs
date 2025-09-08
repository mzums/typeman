use ratatui::prelude::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme {
    Default,
    Dark,
    Light,
    Monochrome,
    Ocean,
    Forest,
}

impl ColorScheme {
    pub fn all() -> Vec<ColorScheme> {
        vec![
            ColorScheme::Default,
            ColorScheme::Dark,
            ColorScheme::Light,
            ColorScheme::Monochrome,
            ColorScheme::Ocean,
            ColorScheme::Forest,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ColorScheme::Default => "Default",
            ColorScheme::Dark => "Dark",
            ColorScheme::Light => "Light",
            ColorScheme::Monochrome => "Monochrome",
            ColorScheme::Ocean => "Ocean",
            ColorScheme::Forest => "Forest",
        }
    }

    pub fn border_color(&self) -> Color {
        match self {
            ColorScheme::Default => Color::Rgb(100, 60, 0),
            ColorScheme::Dark => Color::Rgb(60, 60, 60),
            ColorScheme::Light => Color::Rgb(200, 180, 160),
            ColorScheme::Monochrome => Color::White,
            ColorScheme::Ocean => Color::Rgb(0, 100, 150),
            ColorScheme::Forest => Color::Rgb(50, 100, 50),
        }
    }

    pub fn ref_color(&self) -> Color {
        match self {
            ColorScheme::Default => Color::Rgb(100, 100, 100),
            ColorScheme::Dark => Color::Rgb(80, 80, 80),
            ColorScheme::Light => Color::Rgb(120, 120, 120),
            ColorScheme::Monochrome => Color::Gray,
            ColorScheme::Ocean => Color::Rgb(100, 150, 200),
            ColorScheme::Forest => Color::Rgb(100, 150, 100),
        }
    }

    pub fn bg_color(&self) -> Color {
        match self {
            ColorScheme::Default => Color::Rgb(10, 10, 10),
            ColorScheme::Dark => Color::Black,
            ColorScheme::Light => Color::Rgb(250, 250, 250),
            ColorScheme::Monochrome => Color::Black,
            ColorScheme::Ocean => Color::Rgb(10, 30, 50),
            ColorScheme::Forest => Color::Rgb(20, 40, 20),
        }
    }

    pub fn main_color(&self) -> Color {
        match self {
            ColorScheme::Default => Color::Rgb(255, 155, 0),
            ColorScheme::Dark => Color::Rgb(180, 180, 180),
            ColorScheme::Light => Color::Rgb(80, 80, 80),
            ColorScheme::Monochrome => Color::White,
            ColorScheme::Ocean => Color::Rgb(100, 200, 255),
            ColorScheme::Forest => Color::Rgb(150, 255, 150),
        }
    }

    pub fn dimmer_main(&self) -> Color {
        match self {
            ColorScheme::Default => Color::Rgb(180, 100, 0),
            ColorScheme::Dark => Color::Rgb(120, 120, 120),
            ColorScheme::Light => Color::Rgb(60, 60, 60),
            ColorScheme::Monochrome => Color::Gray,
            ColorScheme::Ocean => Color::Rgb(60, 140, 200),
            ColorScheme::Forest => Color::Rgb(100, 180, 100),
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            ColorScheme::Default => Color::White,
            ColorScheme::Dark => Color::White,
            ColorScheme::Light => Color::Black,
            ColorScheme::Monochrome => Color::White,
            ColorScheme::Ocean => Color::Rgb(200, 230, 255),
            ColorScheme::Forest => Color::Rgb(200, 255, 200),
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme::Default
    }
}