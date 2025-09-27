pub struct MyColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl MyColor {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[cfg(feature = "gui")]
impl From<MyColor> for macroquad::color::Color {
    fn from(c: MyColor) -> Self {
        macroquad::color::Color {
            r: c.r as f32 / 255.0,
            g: c.g as f32 / 255.0,
            b: c.b as f32 / 255.0,
            a: c.a as f32 / 255.0,
        }
    }
}

#[cfg(any(feature = "cli", feature = "tui"))]
impl From<MyColor> for ratatui::style::Color {
    fn from(c: MyColor) -> Self {
        ratatui::style::Color::Rgb(c.r, c.g, c.b)
    }
}
