use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BatchSizeSelection {
    Words10,
    Words25,
    Words50,
    Words100,
}

impl BatchSizeSelection {
    pub fn all() -> &'static [BatchSizeSelection] {
        &[
            BatchSizeSelection::Words10,
            BatchSizeSelection::Words25,
            BatchSizeSelection::Words50,
            BatchSizeSelection::Words100,
        ]
    }

    pub fn count() -> usize {
        Self::all().len()
    }

    pub fn to_words(&self) -> u64 {
        match self {
            BatchSizeSelection::Words10 => 10,
            BatchSizeSelection::Words25 => 25,
            BatchSizeSelection::Words50 => 50,
            BatchSizeSelection::Words100 => 100,
        }
    }
}

impl Default for BatchSizeSelection {
    fn default() -> Self {
        BatchSizeSelection::Words50
    }
}

impl Display for BatchSizeSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatchSizeSelection::Words10 => write!(f, "10"),
            BatchSizeSelection::Words25 => write!(f, "25"),
            BatchSizeSelection::Words50 => write!(f, "50"),
            BatchSizeSelection::Words100 => write!(f, "100"),
        }
    }
}