mod directory_entry_builder;
mod directory_entry_type;
mod error;
mod read_directory;

use std::{path::PathBuf, time::SystemTime};

use chrono::{DateTime, Local};
pub use directory_entry_builder::DirectoryEntryBuilder;
pub use directory_entry_type::DirectoryEntryType;
pub use error::Error;
pub use read_directory::read_directory;

use crate::utils::get_icon;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub struct DirectoryEntry {
    name: String,
    path: PathBuf,
    modified: Option<SystemTime>,
    entry_type: DirectoryEntryType,
}

impl DirectoryEntry {
    pub fn builder() -> DirectoryEntryBuilder {
        DirectoryEntryBuilder::new()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }

    pub fn modified(&self) -> Option<&SystemTime> {
        self.modified.as_ref()
    }

    pub fn set_modified(&mut self, modified: Option<SystemTime>) {
        self.modified = modified;
    }

    pub fn entry_type(&self) -> &DirectoryEntryType {
        &self.entry_type
    }

    pub fn set_entry_type(&mut self, entry_type: DirectoryEntryType) {
        self.entry_type = entry_type;
    }

    pub fn icon(&self) -> (&str, Option<&str>) {
        match &self.entry_type {
            DirectoryEntryType::Directory => ("📁", None),
            DirectoryEntryType::File { extension, size: _ } => match &extension {
                Some(extension) => match get_icon(&extension) {
                    Some((icon, color)) => (icon, Some(color)),
                    None => ("📄", None),
                },
                None => ("📄", None),
            },
            DirectoryEntryType::Other => ("", None),
        }
    }

    pub fn formatted_size(&self) -> Option<String> {
        match &self.entry_type {
            DirectoryEntryType::File { extension: _, size } => {
                const UNITS: [&str; 5] = ["B", "kB", "MB", "GB", "TB"];
                let mut size = *size as f64;
                let mut unit = 0;

                while size >= 1024.0 && unit < UNITS.len() - 1 {
                    size /= 1024.0;
                    unit += 1;
                }

                if unit == 0 {
                    Some(format!("{} {}", size as usize, UNITS[unit]))
                } else {
                    Some(format!("{:.2} {}", size, UNITS[unit]))
                }
            }
            _ => None,
        }
    }

    pub fn formatted_modified(&self) -> Option<String> {
        self.modified.map(|modified| {
            let datetime: DateTime<Local> = modified.into();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }
}

impl PartialOrd for DirectoryEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DirectoryEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.entry_type == DirectoryEntryType::Directory {
            if other.entry_type == DirectoryEntryType::Directory {
                return self.name.cmp(&other.name);
            }

            return std::cmp::Ordering::Less;
        }

        if other.entry_type == DirectoryEntryType::Directory {
            return std::cmp::Ordering::Greater;
        }

        self.name.cmp(&other.name)
    }
}
