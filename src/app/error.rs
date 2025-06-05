use std::{fmt::Display, io};

use crate::directory_entry;

#[derive(Debug)]
pub enum Error {
    InvalidDirectoryPath(String),
    Io(io::Error),
    DirectoryEntry(directory_entry::Error),
    IncorrentInputMode,
    RenameBufferTypeMismatch,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDirectoryPath(path) => write!(
                f,
                "The directory path provided is not a directory: {}",
                path
            ),
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::DirectoryEntry(e) => write!(f, "DirectoryEntry error: {}", e),
            Self::IncorrentInputMode => write!(f, "Incorrect input mode"),
            Self::RenameBufferTypeMismatch => write!(f, "Rename buffer type mismatch"),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<directory_entry::Error> for Error {
    fn from(value: directory_entry::Error) -> Self {
        match value {
            directory_entry::Error::Io(e) => Self::Io(e),
            _ => Self::DirectoryEntry(value),
        }
    }
}
