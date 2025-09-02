use anyhow::Result;
use console::style;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::patterns::DEFAULT_IGNORES;

pub fn gather_gitignores(
    root: &Path,
    spinner: &ProgressBar,
    extra_patterns: &[String],
) -> Result<HashMap<PathBuf, Gitignore>> {
    let mut gitignore_map = HashMap::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == ".gitignore")
    {
        let gitignore_path = entry.path();
        let gitignore_dir = gitignore_path.parent().unwrap().to_path_buf();

        spinner.set_message(format!(
            ".gitignore at {}",
            gitignore_dir
                .strip_prefix(root)
                .unwrap_or(&gitignore_dir)
                .display()
        ));

        let mut builder = GitignoreBuilder::new(&gitignore_dir);
        if let Some(err) = builder.add(gitignore_path) {
            eprintln!(
                "{} Error reading {}: {}",
                style("Error:").red(),
                gitignore_path.display(),
                err
            );
            continue;
        }

        for pat in DEFAULT_IGNORES {
            let negated = format!("!{}", pat);
            if let Err(err) = builder.add_line(None, &negated) {
                eprintln!(
                    "{} Error adding default pattern '{}': {}",
                    style("Warning:").yellow(),
                    pat,
                    err
                );
            }
        }

        for pat in extra_patterns {
            let negated = format!("!{}", pat);
            if let Err(err) = builder.add_line(None, &negated) {
                eprintln!(
                    "{} Error adding extra pattern '{}': {}",
                    style("Warning:").yellow(),
                    pat,
                    err
                );
            }
        }

        match builder.build() {
            Ok(gitignore) => {
                gitignore_map.insert(gitignore_dir, gitignore);
            }
            Err(e) => {
                eprintln!(
                    "{} Error building gitignore for {}: {}",
                    style("Error:").red(),
                    gitignore_path.display(),
                    e
                );
            }
        }
    }

    Ok(gitignore_map)
}
