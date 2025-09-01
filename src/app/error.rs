use std::{fmt::Display, io};

use crate::{command, config, directory_entry, utils::file::FileError};

#[derive(Debug)]
pub enum Error {
    InvalidDirectoryPath(String),
    Io(io::Error),
    DirectoryEntry(directory_entry::Error),
    IncorrentInputMode,
    RenameBufferTypeMismatch,
    File(FileError),
    Command(command::Error),
    Config(config::Error),
    // Called something on a dummy that shouldn't
    NotADummy,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDirectoryPath(path) => {
                write!(f, "The directory path provided is not a directory: {path}")
            }
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::DirectoryEntry(e) => write!(f, "DirectoryEntry error: {e}"),
            Self::IncorrentInputMode => write!(f, "Incorrect input mode"),
            Self::RenameBufferTypeMismatch => write!(f, "Rename buffer type mismatch"),
            Self::File(e) => write!(f, "File error: {e}"),
            Self::Command(e) => write!(f, "Command parse error: {e}"),
            Self::Config(e) => write!(f, "Config error: {e}"),
            Self::NotADummy => write!(f, "Not a DUMMY!"),
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

impl From<FileError> for Error {
    fn from(value: FileError) -> Self {
        match value {
            FileError::Io(e) => Self::Io(e),
            _ => Self::File(value),
        }
    }
}

impl From<command::Error> for Error {
    fn from(value: command::Error) -> Self {
        Self::Command(value)
    }
}

impl From<config::Error> for Error {
    fn from(value: config::Error) -> Self {
        Self::Config(value)
    }
}
