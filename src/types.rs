use std::path::PathBuf;

#[derive(Debug)]
pub struct ItemWithSize {
    pub path: PathBuf,
    pub size: u64,
}
