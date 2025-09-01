use std::{fs, path::Path};

use crossbeam::channel::Sender;

use crate::app::{AppEvent, Result, widgets::types::InputState};

pub fn rename_path(
    directory: &str,
    state: &InputState,
    original: &str,
    event_tx: &Sender<AppEvent>,
) -> Result<()> {
    let new_path = Path::new(directory).join(state.buffer());
    let original_path = Path::new(directory).join(original);

    if let Some(parent) = new_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::rename(original_path, new_path)?;

    let _ = event_tx.send(AppEvent::Reset);

    Ok(())
}
