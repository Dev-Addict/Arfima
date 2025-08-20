use std::fmt::Display;

use super::{precommand::Precommand, widgets::types::InputState};

#[derive(Debug)]
pub enum InputMode {
    Normal {
        precommand: Option<Precommand>,
    },
    Adding {
        state: InputState,
    },
    Renaming {
        original: String,
        state: InputState,
    },
    Removing {
        path: String,
        removing_selected: bool,
    },
    Opening {
        apps: Vec<String>,
        path: String,
        selected_index: usize,
    },
    Commanding {
        state: InputState,
        current_command: isize,
        return_state: Option<InputState>,
    },
    Help {
        selected_index: usize,
    },
}

impl InputMode {
    pub fn adding_default() -> Self {
        Self::Adding {
            state: InputState::default(),
        }
    }

    pub fn renaming_default() -> Self {
        Self::Renaming {
            original: String::default(),
            state: InputState::default(),
        }
    }

    pub fn removing_default() -> Self {
        Self::Removing {
            path: String::default(),
            removing_selected: false,
        }
    }

    pub fn help_default() -> Self {
        Self::Help {
            selected_index: usize::default(),
        }
    }
}

impl Display for InputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal { .. } => write!(f, "Normal"),
            Self::Adding { .. } => write!(f, "Adding"),
            Self::Renaming { .. } => write!(f, "Renaming"),
            Self::Removing { .. } => write!(f, "Removing"),
            Self::Opening { .. } => write!(f, "Opening"),
            Self::Commanding { .. } => write!(f, "Commanding"),
            Self::Help { .. } => write!(f, "Help"),
        }
    }
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Normal { precommand: None }
    }
}
