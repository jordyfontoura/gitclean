# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog and this project adheres to Semantic Versioning.

## [Unreleased]
### Added
- Initial Rust CLI to scan and delete .gitignore-ignored files
- Parallel size computation (WalkDir + Rayon)
- TUI with multi-select (inquire)
- Spinners and informative logs (indicatif + console)
- Extra ignore patterns via `--ignores/-i`
- Default preserved patterns for secrets/configs
- Exposed core as a library (`gitclean` crate) with modules
- Integration test covering scan + sizes

### Changed
- Refactored into modules: patterns, ignore, scan, size, fsops, util
- `main.rs` is now a thin CLI using the library
- Improved logs and summaries
