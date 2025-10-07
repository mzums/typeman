use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeSelection {
    Seconds15,
    Seconds30,
    Seconds60,
    Seconds120,
    Minutes5,
    Minutes10,
}

impl TimeSelection {
    pub fn all() -> &'static [TimeSelection] {
        &[
            TimeSelection::Seconds15,
            TimeSelection::Seconds30,
            TimeSelection::Seconds60,
            TimeSelection::Seconds120,
            TimeSelection::Minutes5,
            TimeSelection::Minutes10,
        ]
    }

    pub fn to_seconds(&self) -> u64 {
        match self {
            TimeSelection::Seconds15 => 15,
            TimeSelection::Seconds30 => 30,
            TimeSelection::Seconds60 => 60,
            TimeSelection::Seconds120 => 120,
            TimeSelection::Minutes5 => 300,
            TimeSelection::Minutes10 => 600,
        }
    }

    pub fn count() -> usize {
        Self::all().len()
    }
}

impl Default for TimeSelection {
    fn default() -> Self {
        TimeSelection::Seconds30
    }
}

impl Display for TimeSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeSelection::Seconds15 => write!(f, "15s"),
            TimeSelection::Seconds30 => write!(f, "30s"),
            TimeSelection::Seconds60 => write!(f, "60s"),
            TimeSelection::Seconds120 => write!(f, "120s"),
            TimeSelection::Minutes5 => write!(f, "5min"),
            TimeSelection::Minutes10 => write!(f, "10min"),
        }
    }
}