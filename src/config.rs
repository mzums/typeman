use crate::color_scheme::ColorScheme;
use crate::language::Language;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub punctuation: bool,
    pub numbers: bool,
    pub time_mode: bool,
    pub word_mode: bool,
    pub quote: bool,
    pub wiki_mode: bool,
    pub practice_mode: bool,
    pub batch_size: usize,
    pub test_time: f32,
    pub selected_level: usize,
    pub language: Language,
    pub color_scheme: ColorScheme,
    pub word_number: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            punctuation: false,
            numbers: false,
            time_mode: true,
            word_mode: false,
            quote: false,
            practice_mode: false,
            wiki_mode: false,
            batch_size: 50,
            test_time: 30.0,
            selected_level: 0,
            language: Language::default(),
            color_scheme: ColorScheme::default(),
            word_number: 50,
        }
    }
}

impl AppConfig {
    fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map_err(|_| "Unable to find home directory")?;

        let config_dir = PathBuf::from(home).join(".config").join("typeman");
        fs::create_dir_all(&config_dir)?;
        Ok(config_dir)
    }

    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(Self::get_config_dir()?.join("config.json"))
    }

    pub fn load() -> Self {
        match Self::get_config_path() {
            Ok(path) => {
                if path.exists() {
                    match fs::read_to_string(&path) {
                        Ok(content) => match serde_json::from_str::<AppConfig>(&content) {
                            Ok(config) => config,
                            Err(_) => Self::default(),
                        },
                        Err(_) => Self::default(),
                    }
                } else {
                    Self::default()
                }
            }
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }
}
