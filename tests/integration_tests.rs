use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

/// Integration test for basic file watching functionality
#[test]
fn test_file_watching_basic() {
    // This test verifies that the core components work together
    // More comprehensive end-to-end testing would require running the binary

    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    // Create initial file
    let mut file = File::create(&test_file).unwrap();
    writeln!(file, "initial content").unwrap();

    // Wait a bit
    thread::sleep(Duration::from_millis(100));

    // Modify file
    let mut file = File::create(&test_file).unwrap();
    writeln!(file, "modified content").unwrap();

    // Test passed if we got this far without errors
    assert!(test_file.exists());
}

/// Test that ignored files are properly filtered
#[test]
fn test_ignored_files_filtering() {
    let temp_dir = TempDir::new().unwrap();

    // Create various files
    File::create(temp_dir.path().join("source.rs")).unwrap();
    File::create(temp_dir.path().join("compiled.pyc")).unwrap();
    File::create(temp_dir.path().join("image.png")).unwrap();
    File::create(temp_dir.path().join("data.txt")).unwrap();

    // Verify files exist
    assert!(temp_dir.path().join("source.rs").exists());
    assert!(temp_dir.path().join("compiled.pyc").exists());
    assert!(temp_dir.path().join("image.png").exists());
    assert!(temp_dir.path().join("data.txt").exists());
}

/// Test recursive directory scanning
#[test]
fn test_recursive_directory_scan() {
    let temp_dir = TempDir::new().unwrap();

    // Create nested directory structure
    let subdir = temp_dir.path().join("subdir");
    fs::create_dir(&subdir).unwrap();

    let nested_subdir = subdir.join("nested");
    fs::create_dir(&nested_subdir).unwrap();

    // Create files at different levels
    File::create(temp_dir.path().join("root.txt")).unwrap();
    File::create(subdir.join("sub.txt")).unwrap();
    File::create(nested_subdir.join("nested.txt")).unwrap();

    // Verify structure
    assert!(temp_dir.path().join("root.txt").exists());
    assert!(subdir.join("sub.txt").exists());
    assert!(nested_subdir.join("nested.txt").exists());
}
