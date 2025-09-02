use anyhow::Result;
use indicatif::ProgressBar;
use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::types::ItemWithSize;

pub fn calculate_sizes(
    paths: Vec<PathBuf>,
    spinner: &ProgressBar,
    root: &Path,
) -> Result<Vec<ItemWithSize>> {
    let items: Vec<ItemWithSize> = paths
        .into_par_iter()
        .map(|path| {
            let rel = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
            spinner.set_message(format!("Calculating sizes in {}", rel.display()));

            let size = calculate_path_size_parallel(&path);
            ItemWithSize { path, size }
        })
        .collect();

    Ok(items)
}

pub fn calculate_path_size_parallel(path: &Path) -> u64 {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_file() {
                metadata.len()
            } else if metadata.is_dir() {
                WalkDir::new(path)
                    .into_iter()
                    .par_bridge()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file())
                    .map(|e| fs::metadata(e.path()).map(|m| m.len()).unwrap_or(0))
                    .sum()
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}
