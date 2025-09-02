use anyhow::Result;
use clap::Parser;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::MultiSelect;
use std::path::PathBuf;

// Use library crate exports
use gitclean::{
    calculate_sizes, format_size, gather_gitignores, remove_item, scan_ignored_files,
    DEFAULT_IGNORES,
};

#[derive(Parser, Debug)]
#[command(
    name = "gitclean",
    version,
    about = "List and remove files and directories ignored by .gitignore."
)]
struct Cli {
    /// Project root path. Use '.' for the current directory (pwd)
    path: PathBuf,
    /// Extra ignore patterns (comma-separated), e.g.: ".env*,.config*"
    #[arg(short = 'i', long = "ignores")]
    ignores: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = if cli.path.as_os_str() == "." {
        std::env::current_dir()?
    } else {
        cli.path.canonicalize().unwrap_or(cli.path)
    };

    // Normalize extra patterns passed via CLI
    let extra_patterns: Vec<String> = cli
        .ignores
        .as_deref()
        .map(|s| {
            s.split(',')
                .map(|p| p.trim().to_string())
                .filter(|p| !p.is_empty())
                .collect()
        })
        .unwrap_or_else(Vec::new);

    // --- Header log -------------------------------------------------------
    let version = env!("CARGO_PKG_VERSION");
    println!(
        "{}",
        style(format!("gitclean v{} — root: {}", version, root.display())).bold()
    );
    println!(
        "{}",
        style(format!(
            "Using {} default ignore pattern(s), {} extra pattern(s)",
            DEFAULT_IGNORES.len(),
            extra_patterns.len()
        ))
        .dim()
    );
    println!("");

    // 1. Load all .gitignore files
    let spinner1 = ProgressBar::new_spinner();
    spinner1.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["|", "/", "-", "\\"])
            .template("{spinner} {msg}")
            .unwrap(),
    );
    spinner1.enable_steady_tick(std::time::Duration::from_millis(200));
    spinner1.set_message(format!(
        "Scanning for versioned projects in {}",
        root.display()
    ));

    let gitignore_map = gather_gitignores(&root, &spinner1, &extra_patterns)?;
    spinner1.finish_with_message(format!(
        "✓ Found {} .gitignore file(s)",
        gitignore_map.len()
    ));

    if gitignore_map.is_empty() {
        println!("{}", style("No .gitignore files found.").red());
        return Ok(());
    }

    // 2. Scan ignored files
    let spinner2 = ProgressBar::new_spinner();
    spinner2.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["|", "/", "-", "\\"])
            .template("{spinner} {msg}")
            .unwrap(),
    );
    spinner2.enable_steady_tick(std::time::Duration::from_millis(200));
    spinner2.set_message("Scanning ignored files...");

    let ignored_items = scan_ignored_files(&root, &gitignore_map, &spinner2)?;
    // quick summary counts
    let (mut ignored_dirs, mut ignored_files) = (0usize, 0usize);
    for p in &ignored_items {
        match std::fs::metadata(p) {
            Ok(md) => {
                if md.is_dir() {
                    ignored_dirs += 1
                } else if md.is_file() {
                    ignored_files += 1
                }
            }
            Err(_) => {}
        }
    }
    spinner2.finish_with_message(format!(
        "✓ Found {} ignored item(s) ({} dir(s), {} file(s))",
        ignored_items.len(), ignored_dirs, ignored_files
    ));

    if ignored_items.is_empty() {
        println!(
            "{}",
            style("No ignored files or directories found.").green()
        );
        return Ok(());
    }

    // 3. Compute sizes and sort
    let spinner3 = ProgressBar::new_spinner();
    spinner3.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["|", "/", "-", "\\"])
            .template("{spinner} {msg}")
            .unwrap(),
    );
    spinner3.enable_steady_tick(std::time::Duration::from_millis(200));
    spinner3.set_message("Calculating sizes...");

    let mut items_with_size = calculate_sizes(ignored_items, &spinner3, &root)?;
    items_with_size.sort_by(|a, b| b.size.cmp(&a.size));
    spinner3.finish_with_message("✓ Sizes calculated");

    println!(
        "{}",
        style(format!(
            "⚠️  {} ignored items found ('secrets' preserved).",
            items_with_size.len()
        ))
        .yellow()
    );

    // 4. Selection interface
    let choices: Vec<String> = items_with_size
        .iter()
        .map(|item| {
            let relative_path = item
                .path
                .strip_prefix(&root)
                .unwrap_or(&item.path)
                .to_string_lossy();
            format!("{:<50} {}", relative_path, format_size(item.size))
        })
        .collect();

    let selected = MultiSelect::new("Select items to delete:", choices.clone())
        .with_help_message("Use arrows to navigate, space to toggle, enter to confirm")
        .prompt()?;

    if selected.is_empty() {
        println!(
            "{}",
            style("No items selected. Nothing was removed.").yellow()
        );
        return Ok(());
    }

    // 5. Remove selected items
    let mut removed_count = 0;
    let mut total_freed_space = 0u64;

    for selected_item in &selected {
        // Find index of the selected item
        if let Some(index) = choices.iter().position(|choice| choice == selected_item) {
            let item = &items_with_size[index];
            match remove_item(&item.path, &root) {
                Ok(_) => {
                    println!(
                        "{} {} ({})",
                        style("✓").green(),
                        item.path
                            .strip_prefix(&root)
                            .unwrap_or(&item.path)
                            .to_string_lossy(),
                        format_size(item.size)
                    );
                    removed_count += 1;
                    total_freed_space += item.size;
                }
                Err(e) => {
                    println!(
                        "{} {} {}",
                        style("✗").red(),
                        item.path
                            .strip_prefix(&root)
                            .unwrap_or(&item.path)
                            .to_string_lossy(),
                        e
                    );
                }
            }
        }
    }

    println!(
        "{}",
        style(format!(
            "🏁  {} item(s) removed. 💾 {} freed.",
            removed_count,
            format_size(total_freed_space)
        ))
        .green()
    );
    Ok(())
}
