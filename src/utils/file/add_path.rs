use std::{fs, path::Path};

use crossbeam::channel::Sender;

use crate::app::{AppEvent, Result, widgets::types::InputState};

pub fn add_path(directory: &str, state: &InputState, event_tx: &Sender<AppEvent>) -> Result<()> {
    let new_path = Path::new(directory).join(state.buffer());

    if new_path.extension().is_some() {
        if let Some(parent) = new_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::File::create(&new_path)?;
    } else {
        fs::create_dir_all(&new_path)?;
    }

    let _ = event_tx.send(AppEvent::Reset);

    Ok(())
}
