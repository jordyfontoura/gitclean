# gitclean

A fast Rust CLI to scan and clean files and directories ignored by .gitignore across your project. It detects ignored items, shows sizes, and lets you interactively delete them.

## Features

- Recursively collects .gitignore rules (supports nested .gitignore)
- Extra ignore patterns via CLI (`-i ".env*,.config*"`)
- Preserves common secret/config files by default (negated patterns)
- Parallel size computation (WalkDir + Rayon)
- TUI multiselect (inquire) with sizes, sorted descending
- Informative spinners and logs (indicatif + console)

## Install

Build from source:

```bash
cargo install --path .
```

Or build a release binary:

```bash
cargo build --release
./target/release/gitclean .
```

## Usage

```bash
gitclean <PATH> [--ignores|-i "pattern1,pattern2,..."]
```

Examples:

```bash
# Current directory
gitclean .

# Custom root path
gitclean /path/to/project

# Add extra ignore patterns (comma-separated)
gitclean . -i ".env*,.config*,*.log"
```

## Logs & UX

- Startup header: shows version, root, default and extra pattern counts
- Spinners for: loading .gitignore, scanning ignored items, computing sizes
- Summary after scanning: total ignored (dirs/files)
- Deletion logs include size per item

## Library usage

Core logic is exposed as a library for reuse:

```rust
use gitclean::{gather_gitignores, scan_ignored_files, calculate_sizes, format_size};
use indicatif::ProgressBar;
use std::path::Path;

let root = Path::new(".").canonicalize().unwrap();
let spinner = ProgressBar::hidden();
let extra: Vec<String> = vec![];
let map = gather_gitignores(&root, &spinner, &extra)?;
let ignored = scan_ignored_files(&root, &map, &spinner)?;
let items = calculate_sizes(ignored, &spinner, &root)?;
println!("{} items", items.len());
```

## Module layout

- `src/lib.rs`: public re-exports
- `src/patterns.rs`: default ignore patterns
- `src/types.rs`: shared types (`ItemWithSize`)
- `src/ignore.rs`: .gitignore loading (`gather_gitignores`)
- `src/scan.rs`: scan logic (`scan_ignored_files`, `is_path_ignored`)
- `src/size.rs`: size computation (`calculate_sizes`)
- `src/fsops.rs`: file ops (`remove_item`)
- `src/util.rs`: helpers (`format_size`)
- `src/main.rs`: thin CLI using the library

## Development

```bash
# Run tests
cargo test

# Build release
cargo build --release
```

## Contributing

Contributions are welcome! Please read `CONTRIBUTING.md` and follow the `CODE_OF_CONDUCT.md`.

## License

MIT © 2025 Jordy Fontoura
