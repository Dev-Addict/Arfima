use super::FileError;

pub type FileResult<T> = Result<T, FileError>;
