use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    English,
    Indonesian,
    Italian,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::English => write!(f, "English"),
            Language::Indonesian => write!(f, "Indonesian"),
            Language::Italian => write!(f, "Italian"),
        }
    }
}

impl Language {
    #[cfg(feature = "cli")]
    pub fn from_str(s: &str) -> Option<Language> {
        match s.to_lowercase().as_str() {
            "english" | "en" => Some(Language::English),
            "indonesian" | "indonesia" | "id" | "indo" => Some(Language::Indonesian),
            "italian" | "ita" | "it" => Some(Language::Italian),
            _ => None,
        }
    }

    pub fn word_list(&self) -> &'static str {
        match self {
            Language::English => include_str!("../assets/common_eng_words.txt"),
            Language::Indonesian => include_str!("../assets/common_ind_words.txt"),
            Language::Italian=> include_str!("../assets/common_ita_words.txt"),
        }
    }

    pub fn get_words(&self, n: usize) -> Vec<String> {
        self.word_list()
            .lines()
            .take(n.min(1000))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_alphabetic() || c.is_whitespace()))
            .collect()
    }
    pub fn all() -> &'static [Language] {
        &[
            Language::English,
            Language::Indonesian,
            Language::Italian,
        ]
    }

    pub fn count() -> usize {
        Self::all().len()
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}
