use serde::{Deserialize, Serialize};

use crate::custom_colors::MyColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorScheme {
    Default,
    Dark,
    Light,
    Monochrome,
    Ocean,
    OceanDark,
    Forest,
    ForestDark,
    Pink,
}

impl ColorScheme {
    pub fn all() -> Vec<ColorScheme> {
        vec![
            ColorScheme::Default,
            ColorScheme::Dark,
            ColorScheme::Light,
            ColorScheme::Monochrome,
            ColorScheme::Ocean,
            ColorScheme::OceanDark,
            ColorScheme::Forest,
            ColorScheme::ForestDark,
            ColorScheme::Pink,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ColorScheme::Default => "Default",
            ColorScheme::Dark => "Dark",
            ColorScheme::Light => "Light",
            ColorScheme::Monochrome => "Monochrome",
            ColorScheme::Ocean => "Ocean",
            ColorScheme::OceanDark => "Ocean Dark",
            ColorScheme::Forest => "Forest",
            ColorScheme::ForestDark => "Forest Dark",
            ColorScheme::Pink => "Pink",
        }
    }

    #[cfg(feature = "gui")]
    pub fn mq_to_color32(c: macroquad::color::Color) -> egui_macroquad::egui::Color32 {
        egui_macroquad::egui::Color32::from_rgb(
            (c.r * 255.0) as u8,
            (c.g * 255.0) as u8,
            (c.b * 255.0) as u8,
        )
    }

    pub fn border_color<C: From<MyColor>>(&self) -> C {
        match self {
            ColorScheme::Default => MyColor::new(100, 60, 0, 255),
            ColorScheme::Dark => MyColor::new(60, 60, 60, 255),
            ColorScheme::Light => MyColor::new(200, 180, 160, 255),
            ColorScheme::Monochrome => MyColor::new(200, 255, 255, 255),
            ColorScheme::Ocean => MyColor::new(0, 100, 150, 255),
            ColorScheme::OceanDark => MyColor::new(0, 50, 80, 255),
            ColorScheme::Forest => MyColor::new(50, 100, 50, 255),
            ColorScheme::ForestDark => MyColor::new(60, 120, 60, 255),
            ColorScheme::Pink => MyColor::new(100, 20, 70, 255),
        }
        .into()
    }

    pub fn ref_color<C: From<MyColor>>(&self) -> C {
        match self {
            ColorScheme::Default => MyColor::new(100, 100, 100, 255),
            ColorScheme::Dark => MyColor::new(80, 80, 80, 255),
            ColorScheme::Light => MyColor::new(120, 120, 120, 255),
            ColorScheme::Monochrome => MyColor::new(80, 80, 80, 255),
            ColorScheme::Ocean => MyColor::new(100, 150, 200, 255),
            ColorScheme::OceanDark => MyColor::new(70, 70, 80, 255),
            ColorScheme::Forest => MyColor::new(100, 150, 100, 255),
            ColorScheme::ForestDark => MyColor::new(70, 80, 70, 255),
            ColorScheme::Pink => MyColor::new(80, 70, 70, 255),
        }
        .into()
    }

    pub fn bg_color<C: From<MyColor>>(&self) -> C {
        match self {
            ColorScheme::Default => MyColor::new(10, 10, 10, 255),
            ColorScheme::Dark => MyColor::new(10, 10, 10, 255),
            ColorScheme::Light => MyColor::new(250, 250, 250, 255),
            ColorScheme::Monochrome => MyColor::new(0, 0, 0, 255),
            ColorScheme::Ocean => MyColor::new(10, 30, 50, 255),
            ColorScheme::OceanDark => MyColor::new(0, 5, 10, 255),
            ColorScheme::Forest => MyColor::new(20, 40, 20, 255),
            ColorScheme::ForestDark => MyColor::new(10, 10, 10, 255),
            ColorScheme::Pink => MyColor::new(7, 0, 2, 255),
        }
        .into()
    }

    pub fn main_color<C: From<MyColor>>(&self) -> C {
        match self {
            ColorScheme::Default => MyColor::new(255, 155, 0, 255),
            ColorScheme::Dark => MyColor::new(180, 180, 180, 255),
            ColorScheme::Light => MyColor::new(80, 80, 80, 255),
            ColorScheme::Monochrome => MyColor::new(200, 255, 255, 255),
            ColorScheme::Ocean => MyColor::new(100, 200, 255, 255),
            ColorScheme::OceanDark => MyColor::new(80, 180, 230, 255),
            ColorScheme::Forest => MyColor::new(150, 255, 150, 255),
            ColorScheme::ForestDark => MyColor::new(100, 200, 100, 255),
            ColorScheme::Pink => MyColor::new(255, 20, 147, 255),
        }
        .into()
    }

    pub fn dimmer_main<C: From<MyColor>>(&self) -> C {
        match self {
            ColorScheme::Default => MyColor::new(180, 100, 0, 255),
            ColorScheme::Dark => MyColor::new(120, 120, 120, 255),
            ColorScheme::Light => MyColor::new(60, 60, 60, 255),
            ColorScheme::Monochrome => MyColor::new(128, 128, 128, 255),
            ColorScheme::Ocean => MyColor::new(60, 140, 200, 255),
            ColorScheme::OceanDark => MyColor::new(50, 120, 180, 255),
            ColorScheme::Forest => MyColor::new(100, 180, 100, 255),
            ColorScheme::ForestDark => MyColor::new(150, 230, 100, 255),
            ColorScheme::Pink => MyColor::new(200, 10, 120, 255),
        }
        .into()
    }

    pub fn text_color<C: From<MyColor>>(&self) -> C {
        match self {
            ColorScheme::Default => MyColor::new(200, 200, 200, 255),
            ColorScheme::Dark => MyColor::new(200, 200, 200, 255),
            ColorScheme::Light => MyColor::new(0, 0, 0, 255),
            ColorScheme::Monochrome => MyColor::new(200, 200, 200, 255),
            ColorScheme::Ocean => MyColor::new(200, 230, 255, 255),
            ColorScheme::OceanDark => MyColor::new(180, 220, 255, 255),
            ColorScheme::Forest => MyColor::new(200, 255, 200, 255),
            ColorScheme::ForestDark => MyColor::new(180, 255, 180, 255),
            ColorScheme::Pink => MyColor::new(200, 200, 200, 255),
        }
        .into()
    }

    pub fn chart_color<C: From<MyColor>>(&self) -> C {
        match self {
            ColorScheme::Default => MyColor::new(150, 80, 0, 255),
            ColorScheme::Dark => MyColor::new(180, 180, 180, 255),
            ColorScheme::Light => MyColor::new(80, 80, 80, 255),
            ColorScheme::Monochrome => MyColor::new(200, 255, 255, 255),
            ColorScheme::Ocean => MyColor::new(100, 200, 255, 255),
            ColorScheme::OceanDark => MyColor::new(80, 180, 230, 255),
            ColorScheme::Forest => MyColor::new(150, 255, 150, 255),
            ColorScheme::ForestDark => MyColor::new(100, 200, 100, 255),
            ColorScheme::Pink => MyColor::new(100, 20, 70, 255),
        }
        .into()
    }

    pub fn correct_color<C: From<MyColor>>(&self) -> C {
        match self {
            ColorScheme::Default => MyColor::new(210, 200, 200, 255),
            ColorScheme::Dark => MyColor::new(200, 255, 255, 255),
            ColorScheme::Light => MyColor::new(150, 200, 150, 255),
            ColorScheme::Monochrome => MyColor::new(200, 255, 255, 255),
            ColorScheme::Ocean => MyColor::new(200, 255, 255, 255),
            ColorScheme::OceanDark => MyColor::new(200, 255, 255, 255),
            ColorScheme::Forest => MyColor::new(200, 255, 255, 255),
            ColorScheme::ForestDark => MyColor::new(200, 255, 255, 255),
            ColorScheme::Pink => MyColor::new(200, 255, 255, 255),
        }
        .into()
    }

    pub fn corrected_color<C: From<MyColor>>(&self) -> C {
        match self {
            ColorScheme::Default => MyColor::new(255, 155, 0, 255),
            ColorScheme::Dark => MyColor::new(100, 60, 0, 255),
            ColorScheme::Light => MyColor::new(150, 100, 0, 255),
            ColorScheme::Monochrome => MyColor::new(200, 50, 50, 255),
            ColorScheme::Ocean => MyColor::new(180, 100, 255, 255),
            ColorScheme::OceanDark => MyColor::new(180, 100, 255, 255),
            ColorScheme::Forest => MyColor::new(255, 100, 100, 255),
            ColorScheme::ForestDark => MyColor::new(180, 100, 0, 255),
            ColorScheme::Pink => MyColor::new(255, 100, 100, 255),
        }
        .into()
    }

    pub fn incorrect_color<C: From<MyColor>>(&self) -> C {
        match self {
            ColorScheme::Default => MyColor::new(200, 30, 30, 255),
            ColorScheme::Dark => MyColor::new(200, 30, 30, 255),
            ColorScheme::Light => MyColor::new(200, 30, 30, 255),
            ColorScheme::Monochrome => MyColor::new(200, 30, 30, 255),
            ColorScheme::Ocean => MyColor::new(255, 0, 200, 255),
            ColorScheme::OceanDark => MyColor::new(255, 0, 200, 255),
            ColorScheme::Forest => MyColor::new(200, 30, 30, 255),
            ColorScheme::ForestDark => MyColor::new(150, 30, 30, 255),
            ColorScheme::Pink => MyColor::new(255, 30, 30, 255),
        }
        .into()
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme::Default
    }
}
