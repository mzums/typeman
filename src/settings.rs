use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Settings {
    ColorScheme,
    Language,
}

impl Settings {
    pub fn all() -> &'static [Settings] {
        &[
            Settings::ColorScheme,
            Settings::Language,
        ]
    }

    pub fn count() -> usize {
        Self::all().len()
    }
}

impl Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Settings::ColorScheme => write!(f, "Color Scheme"),
            Settings::Language => write!(f, "Language"),
        }
    }
}
