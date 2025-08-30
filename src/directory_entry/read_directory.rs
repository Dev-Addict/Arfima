use std::{fs, path::Path};

use super::{DirectoryEntry, Result};

pub fn read_directory<P: AsRef<Path>>(path: P) -> Result<Vec<DirectoryEntry>> {
    let mut entries = fs::read_dir(path)?
        .filter_map(|entry| (&entry.ok()?.path()).try_into().ok())
        .collect::<Vec<_>>();

    entries.sort();

    Ok(entries)
}
