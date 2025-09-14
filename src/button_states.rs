use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ButtonState {
    pub label: String,
    pub display_name: String,
    pub short_name: String,
    pub state_val: bool,
    pub visible: bool,
}

impl ButtonState {
    pub fn new(label: &str, display_name: &str, short_name: &str, state_val: bool, visible: bool) -> Self {
        Self {
            label: label.to_string(),
            display_name: display_name.to_string(),
            short_name: short_name.to_string(),
            state_val,
            visible,
        }
    }
}

pub struct ButtonStates {
    pub punctuation: ButtonState,
    pub numbers: ButtonState,
    pub divider1: ButtonState,
    pub language: ButtonState,
    pub theme: ButtonState,
    pub divider2: ButtonState,
    pub time: ButtonState,
    pub words: ButtonState,
    pub quote: ButtonState,
    pub practice: ButtonState,
    pub divider3: ButtonState,
    pub time_15: ButtonState,
    pub time_30: ButtonState,
    pub time_60: ButtonState,
    pub time_120: ButtonState,
    pub batch_25: ButtonState,
    pub batch_50: ButtonState,
    pub batch_100: ButtonState,
}

impl ButtonStates {
    pub fn with_args() -> Self {
        fn btn(label: &str, display_name: &str, short_name: &str) -> ButtonState {
            ButtonState {
                label: label.to_string(),
                display_name: display_name.to_string(),
                short_name: short_name.to_string(),
                state_val: false,
                visible: true,
            }
        }
        Self {
            punctuation: btn("punctuation", "punctuation", "punct"),
            numbers: btn("numbers", "numbers", "num"),
            divider1: btn("|", "|", "|"),
            language: btn("language", "language", "lang"),
            theme: btn("theme", "theme", "theme"),
            quote: btn("quote", "quote", "quote"),
            practice: btn("practice", "practice", "practice"),
            divider2: btn("|", "|", "|"),
            time: btn("time", "time", "time"),
            words: btn("words", "words", "words"),
            divider3: btn("|", "|", "|"),
            time_15: btn("15", "15", "15"),
            time_30: btn("30", "30", "30"),
            time_60: btn("60", "60", "60"),
            time_120: btn("120", "120", "120"),
            batch_25: btn("25", "25", "25"),
            batch_50: btn("50", "50", "50"),
            batch_100: btn("100", "100", "100"),
        }
    }
}

impl ButtonStates {
    pub fn new() -> Self {
        Self::with_args()
    }

    pub fn as_vec(&self) -> Vec<&ButtonState> {
        vec![
            &self.punctuation,
            &self.numbers,
            &self.divider1,
            &self.language,
            &self.theme,
            &self.divider2,
            &self.time,
            &self.words,
            &self.quote,
            &self.practice,
            &self.divider3,
            &self.time_15,
            &self.time_30,
            &self.time_60,
            &self.time_120,
            &self.batch_25,
            &self.batch_50,
            &self.batch_100,
        ]
    }
}
