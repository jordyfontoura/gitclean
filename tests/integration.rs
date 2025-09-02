use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use indicatif::ProgressBar;
use tempfile;

use gitclean::{calculate_sizes, gather_gitignores, scan_ignored_files};

// Since the binary crate does not expose functions, we create minimal helpers
// by copying signatures. In a real project, move core logic into a lib crate.

fn write_file(path: &PathBuf, contents: &str) {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).unwrap();
    }
    let mut f = File::create(path).unwrap();
    f.write_all(contents.as_bytes()).unwrap();
}

#[test]
fn test_ignored_scan_and_sizes() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().to_path_buf();

    // Create nested structure
    let gi_root = root.join(".gitignore");
    write_file(&gi_root, "target\n*.log\n");

    // Ignored dir
    let target_dir = root.join("target");
    fs::create_dir_all(&target_dir).unwrap();
    write_file(&target_dir.join("a.bin"), &"x".repeat(1024));

    // Ignored file by pattern
    write_file(&root.join("error.log"), &"y".repeat(2048));

    // Not ignored
    write_file(&root.join("keep.txt"), "hello");

    // Build gitignore map
    let spinner = ProgressBar::hidden();
    let extra: Vec<String> = vec![];
    let map = gather_gitignores(&root, &spinner, &extra).unwrap();
    assert!(!map.is_empty());

    // Scan ignored
    let ignored = scan_ignored_files(&root, &map, &spinner).unwrap();
    // Should include target dir and error.log
    assert!(ignored.iter().any(|p| p.ends_with("target")));
    assert!(ignored.iter().any(|p| p.ends_with("error.log")));
    assert!(!ignored.iter().any(|p| p.ends_with("keep.txt")));

    // Sizes
    let items = calculate_sizes(ignored, &spinner, &root).unwrap();
    // Sum should be >= 3KB approximately
    let total: u64 = items.iter().map(|i| i.size).sum();
    assert!(total >= 3 * 1024);
}
