use std::path::PathBuf;

use super::{Error, InputMode, precommand::Precommand};

pub enum AppEvent {
    UpdatePrecommand(Option<Precommand>),
    SetError(Option<Error>),
    UpdateInputMode(InputMode),
    Open { path: PathBuf, new: bool },
    Reset,
}
