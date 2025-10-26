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
    pub settings: ButtonState,
    pub divider0: ButtonState,
    pub punctuation: ButtonState,
    pub numbers: ButtonState,
    pub divider1: ButtonState,
    pub time: ButtonState,
    pub words: ButtonState,
    pub quote: ButtonState,
    pub practice: ButtonState,
    pub wiki_mode: ButtonState,
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
            settings: btn("settings", "settings", "⚙"),
            divider0: btn("|", "|", "|"),
            punctuation: btn("punctuation", "punctuation", "punct"),
            numbers: btn("numbers", "numbers", "num"),
            divider1: btn("|", "|", "|"),
            quote: btn("quote", "quote", "quote"),
            practice: btn("practice", "practice", "practice"),
            wiki_mode: btn("wiki", "wikipedia", "wiki"),
            time: btn("time", "time", "time"),
            words: btn("words", "words", "words"),
        }
    }
}

impl ButtonStates {
    pub fn new() -> Self {
        Self::with_args()
    }

    pub fn as_vec(&self) -> Vec<&ButtonState> {
        vec![
            &self.settings,
            &self.divider0,
            &self.punctuation,
            &self.numbers,
            &self.divider1,
            &self.time,
            &self.words,
            &self.quote,
            &self.practice,
            &self.wiki_mode,
        ]
    }
}
