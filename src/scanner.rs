use crate::filter::IgnoreFilter;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Recursively scan a directory and collect all files
///
/// Maps to Python's scan_dir() function
pub fn scan_directory(path: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if !path.exists() {
        return Ok(files);
    }

    if path.is_file() {
        files.push(path.to_path_buf());
        return Ok(files);
    }

    // Recursively walk directory
    let entries = fs::read_dir(path)
        .with_context(|| format!("Failed to read directory: {}", path.display()))?;

    for entry in entries {
        let entry = entry.with_context(|| format!("Failed to read entry in {}", path.display()))?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            // Recursively scan subdirectories
            let mut subdir_files = scan_directory(&entry_path)?;
            files.append(&mut subdir_files);
        } else {
            files.push(entry_path);
        }
    }

    Ok(files)
}

/// Gather files from given paths, applying ignore filter
///
/// Maps to Python's gather_files() function
/// Supports both individual files and directories
pub fn gather_files(paths: &[String], filter: &IgnoreFilter) -> Result<Vec<PathBuf>> {
    let mut all_files = Vec::new();

    for path_str in paths {
        let path = PathBuf::from(path_str);

        if !path.exists() {
            eprintln!("Warning: Path does not exist: {}", path_str);
            continue;
        }

        let files = if path.is_dir() {
            scan_directory(&path)?
        } else {
            vec![path]
        };

        // Apply filter
        for file in files {
            if !filter.should_ignore(&file) {
                all_files.push(file);
            }
        }
    }

    Ok(all_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    #[test]
    fn test_scan_directory_empty() {
        let temp_dir = TempDir::new().unwrap();
        let files = scan_directory(temp_dir.path()).unwrap();
        assert_eq!(files.len(), 0);
    }

    #[test]
    fn test_scan_directory_with_files() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");

        File::create(&file1).unwrap();
        File::create(&file2).unwrap();

        let files = scan_directory(temp_dir.path()).unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_scan_directory_recursive() {
        let temp_dir = TempDir::new().unwrap();
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        let file1 = temp_dir.path().join("file1.txt");
        let file2 = subdir.join("file2.txt");

        File::create(&file1).unwrap();
        File::create(&file2).unwrap();

        let files = scan_directory(temp_dir.path()).unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_gather_files_with_filter() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("test.py");
        let file2 = temp_dir.path().join("test.pyc");
        let file3 = temp_dir.path().join("image.png");

        File::create(&file1).unwrap();
        File::create(&file2).unwrap();
        File::create(&file3).unwrap();

        let filter = IgnoreFilter::new();
        let paths = vec![temp_dir.path().to_str().unwrap().to_string()];
        let files = gather_files(&paths, &filter).unwrap();

        // Should only include test.py, not test.pyc or image.png
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("test.py"));
    }

    #[test]
    fn test_gather_files_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.path().join("single.txt");
        File::create(&file).unwrap();

        let filter = IgnoreFilter::new();
        let paths = vec![file.to_str().unwrap().to_string()];
        let files = gather_files(&paths, &filter).unwrap();

        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_gather_files_nonexistent_path() {
        let filter = IgnoreFilter::new();
        let paths = vec!["/nonexistent/path".to_string()];
        let files = gather_files(&paths, &filter).unwrap();

        // Should return empty, not error
        assert_eq!(files.len(), 0);
    }
}
