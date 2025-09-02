use anyhow::Result;
use ignore::gitignore::Gitignore;
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn scan_ignored_files(
    root: &Path,
    gitignore_map: &HashMap<PathBuf, Gitignore>,
    spinner: &ProgressBar,
) -> Result<Vec<PathBuf>> {
    let mut ignored_items = Vec::new();
    scan_directory_recursive(root, root, gitignore_map, &mut ignored_items, spinner)?;
    Ok(ignored_items)
}

fn scan_directory_recursive(
    dir: &Path,
    root: &Path,
    gitignore_map: &HashMap<PathBuf, Gitignore>,
    ignored_items: &mut Vec<PathBuf>,
    spinner: &ProgressBar,
) -> Result<()> {
    spinner.set_message(format!(
        "Scanning {}",
        dir.strip_prefix(root).unwrap_or(dir).display()
    ));

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Ok(()),
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name == ".git" {
            continue;
        }

        if is_path_ignored(&path, gitignore_map, root) {
            ignored_items.push(path);
            continue;
        }

        if path.is_dir() {
            scan_directory_recursive(&path, root, gitignore_map, ignored_items, spinner)?;
        }
    }

    Ok(())
}

pub fn is_path_ignored(path: &Path, gitignore_map: &HashMap<PathBuf, Gitignore>, root: &Path) -> bool {
    let mut current_dir = path.parent();

    while let Some(dir) = current_dir {
        if !dir.starts_with(root) {
            break;
        }

        if let Some(gitignore) = gitignore_map.get(dir) {
            if let Ok(relative_path) = path.strip_prefix(dir) {
                if gitignore.matched(relative_path, path.is_dir()).is_ignore() {
                    return true;
                }
            }
        }

        current_dir = dir.parent();
    }

    false
}
