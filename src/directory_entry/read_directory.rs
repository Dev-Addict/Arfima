use std::{fs, path::Path};

use super::{DirectoryEntry, DirectoryEntryType, Result};

pub fn read_directory<P: AsRef<Path>>(path: P) -> Result<Vec<DirectoryEntry>> {
    let mut entries = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let metadata = entry.metadata().ok()?;

            let file_name = entry.file_name().to_string_lossy().to_string();

            let entry_builder = DirectoryEntry::builder()
                .name(file_name)
                .modified(metadata.modified().ok());

            if metadata.is_file() {
                let extension = path
                    .extension()
                    .map(|ext| ext.to_string_lossy().to_string());

                Some(
                    entry_builder
                        .path(path)
                        .entry_type(DirectoryEntryType::File {
                            extension,
                            size: metadata.len(),
                        })
                        .build()
                        .ok()?,
                )
            } else if metadata.is_dir() {
                Some(
                    entry_builder
                        .path(path)
                        .entry_type(DirectoryEntryType::Directory)
                        .build()
                        .ok()?,
                )
            } else {
                Some(
                    entry_builder
                        .path(path)
                        .entry_type(DirectoryEntryType::Other)
                        .build()
                        .ok()?,
                )
            }
        })
        .collect::<Vec<_>>();

    entries.sort();

    Ok(entries)
}
