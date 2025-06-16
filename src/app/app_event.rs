use super::{Error, InputMode, precommand::Precommand};

pub enum AppEvent {
    UpdatePrecommand(Option<Precommand>),
    SetError(Option<Error>),
    UpdateInputMode(InputMode),
    Reset,
}
