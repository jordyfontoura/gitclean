<div align="center">

# 🧹 gitclean

### _Clean your Git projects like a pro_

**A blazingly fast Rust CLI that scans and removes files ignored by `.gitignore` — interactively and safely.**

[![Crates.io](https://img.shields.io/crates/v/gitclean.svg)](https://crates.io/crates/gitclean)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

[Installation](#-installation) •
[Features](#-features) •
[Usage](#-usage) •
[Examples](#-examples) •
[Library](#-library-usage)

</div>

---

## 🎯 Why gitclean?

Ever wondered how much disk space those `node_modules`, `target`, or `.next` folders are eating up across your projects? **gitclean** gives you **instant visibility** and **safe cleanup** of all ignored files and directories.

### ✨ What makes it special?

- **⚡ Blazingly Fast** – Written in Rust with parallel processing
- **🎯 Smart Detection** – Recursively respects all nested `.gitignore` files
- **🔒 Safe by Default** – Preserves secrets like `.env` files automatically
- **📊 Visual Insights** – See sizes before you delete
- **🎨 Beautiful TUI** – Interactive multiselect with sorted results
- **🛡️ Zero Risk** – Review and confirm before any deletion

---

## 🚀 Installation

Install directly from crates.io:

```bash
cargo install gitclean
```

That's it! Now you can use `gitclean` anywhere.

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| 🔄 **Recursive Scanning** | Collects rules from all `.gitignore` files in your project tree |
| 🎯 **Custom Patterns** | Add extra ignore patterns via CLI (`-i ".env*,.config*"`) |
| 🔐 **Smart Preservation** | Automatically protects common secret/config files |
| ⚡ **Parallel Processing** | Lightning-fast size computation with WalkDir + Rayon |
| 🎨 **Interactive TUI** | Beautiful multiselect interface with sizes, sorted descending |
| 📊 **Detailed Logging** | Informative spinners and progress indicators |

---

## 📖 Usage

```bash
gitclean <PATH> [OPTIONS]
```

### Options

```
<PATH>                    Project root directory to scan
-i, --ignores <PATTERNS>  Extra ignore patterns (comma-separated)
-h, --help                Print help information
-V, --version             Print version information
```

---

## 💡 Examples

### Clean current directory

```bash
gitclean .
```

**Output:**
```
🧹 gitclean v1.0.0
📂 Root: /home/user/projects/my-app
📋 Default patterns: 15
🎯 Extra patterns: 0

⠋ Loading .gitignore files...
✓ Found 3 .gitignore files

⠋ Scanning ignored items...
✓ Found 127 ignored items (45 dirs, 82 files)

⠋ Computing sizes...
✓ Sizes computed

┌──────────────────────────────────────────┐
│  Select items to delete (Space to toggle) │
└──────────────────────────────────────────┘

  [ ] node_modules/          1.2 GB
  [ ] target/                856 MB
  [ ] .next/                 234 MB
  [ ] dist/                  89 MB
  [ ] coverage/              12 MB
```

### Scan specific project

```bash
gitclean ~/projects/my-app
```

### Add custom ignore patterns

```bash
gitclean . -i ".env*,.config*,*.log"
```

This will additionally ignore:
- All files starting with `.env`
- All files starting with `.config`
- All `.log` files

---

## 🎨 Interactive Experience

gitclean provides a delightful CLI experience:

1. **🔄 Spinner animations** while scanning
2. **📊 Progress indicators** for long operations
3. **📋 Organized results** sorted by size (largest first)
4. **✅ Multi-select interface** to choose what to delete
5. **✓ Confirmation logs** showing freed space

---

## 📚 Library Usage

Use `gitclean` as a library in your own Rust projects:

```rust
use gitclean::{
    gather_gitignores,
    scan_ignored_files,
    calculate_sizes,
    format_size
};
use indicatif::ProgressBar;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let root = Path::new(".").canonicalize()?;
    let spinner = ProgressBar::hidden();
    let extra: Vec<String> = vec![];
    
    // Gather all .gitignore rules
    let map = gather_gitignores(&root, &spinner, &extra)?;
    
    // Scan for ignored files
    let ignored = scan_ignored_files(&root, &map, &spinner)?;
    
    // Calculate sizes
    let items = calculate_sizes(ignored, &spinner, &root)?;
    
    println!("Found {} ignored items", items.len());
    for item in items {
        println!("{}: {}", item.path.display(), format_size(item.size));
    }
    
    Ok(())
}
```

---

## 🏗️ Architecture

gitclean is built with a clean modular structure:

```
src/
├── lib.rs         → Public API exports
├── patterns.rs    → Default ignore patterns
├── types.rs       → Core types (ItemWithSize)
├── ignore.rs      → .gitignore parsing
├── scan.rs        → Scanning logic
├── size.rs        → Size computation
├── fsops.rs       → File operations
├── util.rs        → Utilities (format_size)
└── main.rs        → CLI interface
```

Each module is designed to be **reusable** and **testable**.

---

## 🛠️ Development

### Run tests

```bash
cargo test
```

### Build release

```bash
cargo build --release
./target/release/gitclean .
```

### Run locally

```bash
cargo run -- .
```

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. 🍴 Fork the repository
2. 🌿 Create a feature branch (`git checkout -b feature/amazing-feature`)
3. 💾 Commit your changes (`git commit -m 'Add amazing feature'`)
4. 📤 Push to the branch (`git push origin feature/amazing-feature`)
5. 🎉 Open a Pull Request

Please read [`CONTRIBUTING.md`](CONTRIBUTING.md) and follow the [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

---

## 📝 License

MIT © 2025 [Jordy Fontoura](https://github.com/jordyfontoura)

---

<div align="center">

**Made with ❤️ and 🦀 Rust**

[⬆ Back to top](#-gitclean)

</div>
