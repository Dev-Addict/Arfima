use std::path::PathBuf;

use crate::directory_entry::DirectoryEntryType;

use super::{Error, InputMode, precommand::Precommand};

pub enum AppEvent {
    UpdatePrecommand(Option<Precommand>),
    SetError(Option<Error>),
    UpdateInputMode(InputMode),
    Open {
        path: PathBuf,
        new: bool,
        entry_type: DirectoryEntryType,
    },
    Reset,
}
