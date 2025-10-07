use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WordNumberSelection {
    Words25,
    Words50,
    Words100,
    Words200,
    Words500,
}

impl WordNumberSelection {
    pub fn all() -> &'static [WordNumberSelection] {
        &[
            WordNumberSelection::Words25,
            WordNumberSelection::Words50,
            WordNumberSelection::Words100,
            WordNumberSelection::Words200,
            WordNumberSelection::Words500,
        ]
    }

    pub fn count() -> usize {
        Self::all().len()
    }

    pub fn to_words(&self) -> u64 {
        match self {
            WordNumberSelection::Words25 => 25,
            WordNumberSelection::Words50 => 50,
            WordNumberSelection::Words100 => 100,
            WordNumberSelection::Words200 => 200,
            WordNumberSelection::Words500 => 500,
        }
    }
}

impl Default for WordNumberSelection {
    fn default() -> Self {
        WordNumberSelection::Words50
    }
}

impl Display for WordNumberSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WordNumberSelection::Words25 => write!(f, "25"),
            WordNumberSelection::Words50 => write!(f, "50"),
            WordNumberSelection::Words100 => write!(f, "100"),
            WordNumberSelection::Words200 => write!(f, "200"),
            WordNumberSelection::Words500 => write!(f, "500"),
        }
    }
}