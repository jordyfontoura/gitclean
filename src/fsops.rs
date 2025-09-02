use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn remove_item(path: &Path, _root: &Path) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}
