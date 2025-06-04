use std::{fmt::Display, io};

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    MissingName,
    MissingPath,
    MissingEntryType,
    Io(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingName => write!(f, "name is missing"),
            Self::MissingPath => write!(f, "path is missing"),
            Self::MissingEntryType => write!(f, "entry_type is missing"),
            Self::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
