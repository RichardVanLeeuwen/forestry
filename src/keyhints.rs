use crate::app::{App, CurrentScreen};

#[derive(Debug, Clone, Copy)]
pub struct KeyHint {
    pub key: &'static str,
    pub description: &'static str,
}

impl KeyHint {
    pub const fn new(key: &'static str, description: &'static str) -> Self {
        Self { key, description }
    }
}

const MAIN_HINTS: &[KeyHint] = &[
    KeyHint::new("k/↑", "up"),
    KeyHint::new("j/↓", "down"),
    KeyHint::new("c/+", "create worktree"),
    KeyHint::new("d", "delete"),
    KeyHint::new("q/esc", "quit"),
];

const CREATING_HINTS: &[KeyHint] = &[
    KeyHint::new("↑/↓", "move select"),
    KeyHint::new("enter", "accept"),
    KeyHint::new("esc", "cancel"),
];

const DELETING_FORCE_HINTS: &[KeyHint] = &[
    KeyHint::new("y", "accept force delete"),
    KeyHint::new("n/enter/esc", "cancel"),
];

const DELETING_HINTS: &[KeyHint] = &[
    KeyHint::new("y/enter", "accept"),
    KeyHint::new("n/esc", "cancel"),
];

pub fn key_hints_for(app: &App) -> &'static [KeyHint] {
    match &app.current_screen {
        CurrentScreen::Main => MAIN_HINTS,
        CurrentScreen::Creating => CREATING_HINTS,
        CurrentScreen::Deleting => {
            if app.ask_force_delete {
                DELETING_FORCE_HINTS
            } else {
                DELETING_HINTS
            }
        }
    }
}
