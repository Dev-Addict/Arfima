use std::{env, fmt::Display, io};

#[derive(Debug)]
pub enum FileError {
    FileDoesNotExists,
    FailedToGetMimeType,
    FailedToGetAppsWithMimeType,
    Io(io::Error),
    CouldNotDetermineUTI,
    NoFileExtension,
    NoFileTypeFound,
    UnsupportedPlatform,
    NoAppsFound,
    FailedToOpenFile,
    VarError(env::VarError),
    NoExecLine,
    NoExecutableFound,
}

impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileDoesNotExists => write!(f, "File does not exists"),
            Self::FailedToGetMimeType => write!(f, "Failed to get mime type"),
            Self::FailedToGetAppsWithMimeType => write!(f, "Failed to get apps with mime type"),
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::CouldNotDetermineUTI => write!(f, "Could not determine UTI"),
            Self::NoFileExtension => write!(f, "No file extension"),
            Self::NoFileTypeFound => write!(f, "No file type found"),
            Self::UnsupportedPlatform => write!(f, "Unsupported platform"),
            Self::NoAppsFound => write!(f, "No apps found"),
            Self::FailedToOpenFile => write!(f, "Failed to open file"),
            Self::VarError(e) => write!(f, "Var error: {e}"),
            Self::NoExecLine => write!(f, "No exec line"),
            Self::NoExecutableFound => write!(f, "No executable found"),
        }
    }
}

impl std::error::Error for FileError {}

impl From<io::Error> for FileError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<env::VarError> for FileError {
    fn from(value: env::VarError) -> Self {
        Self::VarError(value)
    }
}
