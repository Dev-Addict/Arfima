use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    FailedToParseError(nom::Err<nom::error::Error<String>>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToParseError(e) => write!(f, "Failed to parse the command: {e}"),
        }
    }
}

impl std::error::Error for Error {}
