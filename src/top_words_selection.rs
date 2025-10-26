use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TopWordsSelection {
    Words25,
    Words50,
    Words100,
    Words200,
    Words500,
    Words1000,
}

impl TopWordsSelection {
    pub fn all() -> &'static [TopWordsSelection] {
        &[
            TopWordsSelection::Words25,
            TopWordsSelection::Words50,
            TopWordsSelection::Words100,
            TopWordsSelection::Words200,
            TopWordsSelection::Words500,
            TopWordsSelection::Words1000,
        ]
    }

    pub fn count() -> usize {
        Self::all().len()
    }

    pub fn to_words(&self) -> u64 {
        match self {
            TopWordsSelection::Words25 => 25,
            TopWordsSelection::Words50 => 50,
            TopWordsSelection::Words100 => 100,
            TopWordsSelection::Words200 => 200,
            TopWordsSelection::Words500 => 500,
            TopWordsSelection::Words1000 => 1000,
        }
    }
}

impl Default for TopWordsSelection {
    fn default() -> Self {
        TopWordsSelection::Words500
    }
}

impl Display for TopWordsSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopWordsSelection::Words25 => write!(f, "25"),
            TopWordsSelection::Words50 => write!(f, "50"),
            TopWordsSelection::Words100 => write!(f, "100"),
            TopWordsSelection::Words200 => write!(f, "200"),
            TopWordsSelection::Words500 => write!(f, "500"),
            TopWordsSelection::Words1000 => write!(f, "1000"),
        }
    }
}