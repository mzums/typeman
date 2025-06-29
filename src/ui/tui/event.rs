use crate::ui::tui::app::{App, CurrentScreen, CurrentlyEditing};
use crossterm::event::KeyEvent;
use crossterm::event::KeyCode;

pub enum Event {
    Key(KeyEvent),
}

pub fn handle_key_event(key: KeyEvent, app: &mut App) -> Option<bool> {
    match app.current_screen {
        CurrentScreen::Main => match key.code {
            KeyCode::Char('e') => {
                app.current_screen = CurrentScreen::Editing;
                app.currently_editing = Some(CurrentlyEditing::Key);
                None
            }
            KeyCode::Char('q') => {
                app.current_screen = CurrentScreen::Exiting;
                None
            }
            _ => None,
        },
        CurrentScreen::Exiting => match key.code {
            KeyCode::Char('y') => Some(true),
            KeyCode::Char('n') | KeyCode::Char('q') => Some(false),
            _ => None,
        },
        CurrentScreen::Editing => match key.code {
            KeyCode::Enter => {
                if let Some(editing) = &app.currently_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            app.currently_editing = Some(CurrentlyEditing::Value);
                        }
                        CurrentlyEditing::Value => {
                            app.save_key_value();
                            app.current_screen = CurrentScreen::Main;
                        }
                    }
                }
                None
            }
            KeyCode::Backspace => {
                if let Some(editing) = &app.currently_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            app.key_input.pop();
                        }
                        CurrentlyEditing::Value => {
                            app.value_input.pop();
                        }
                    }
                }
                None
            }
            KeyCode::Esc => {
                app.current_screen = CurrentScreen::Main;
                app.currently_editing = None;
                None
            }
            KeyCode::Tab => {
                app.toggle_editing();
                None
            }
            KeyCode::Char(value) => {
                if let Some(editing) = &app.currently_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            app.key_input.push(value);
                        }
                        CurrentlyEditing::Value => {
                            app.value_input.push(value);
                        }
                    }
                }
                None
            }
            _ => None,
        },
    }
}