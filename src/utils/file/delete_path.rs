use std::{fs, sync::mpsc::Sender};

use crate::{
    app::{AppEvent, Result},
    directory_entry::{DirectoryEntry, DirectoryEntryType},
};

pub fn delete_path(
    entry: Option<&DirectoryEntry>,
    path: &str,
    event_tx: &Sender<AppEvent>,
) -> Result<()> {
    if let Some(entry) = entry {
        match entry.entry_type() {
            DirectoryEntryType::Directory => fs::remove_dir_all(path)?,
            _ => fs::remove_file(path)?,
        }
    }

    let _ = event_tx.send(AppEvent::Reset);

    Ok(())
}
