#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum DirectoryEntryType {
    File {
        extension: Option<String>,
        size: u64,
    },
    Directory,
    Other,
}
