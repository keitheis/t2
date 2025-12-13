use std::path::Path;

/// File extensions to ignore during file watching
const IGNORE_EXTENSIONS: &[&str] = &[
    "pyc", "swp", "swo", "bmp", "jpg", "jpeg", "png", "gif", "svg", "psd", "xcf", "pxm",
];

/// Check if a file path should be ignored based on its extension
pub fn should_ignore(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| IGNORE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_ignore_pyc() {
        assert!(should_ignore(Path::new("test.pyc")));
        assert!(should_ignore(Path::new("/path/to/file.pyc")));
    }

    #[test]
    fn test_ignore_swap_files() {
        assert!(should_ignore(Path::new(".test.swp")));
        assert!(should_ignore(Path::new("file.swo")));
    }

    #[test]
    fn test_ignore_images() {
        assert!(should_ignore(Path::new("image.png")));
        assert!(should_ignore(Path::new("photo.jpg")));
        assert!(should_ignore(Path::new("picture.jpeg")));
        assert!(should_ignore(Path::new("icon.svg")));
        assert!(should_ignore(Path::new("design.psd")));
    }

    #[test]
    fn test_case_insensitive() {
        assert!(should_ignore(Path::new("IMAGE.PNG")));
        assert!(should_ignore(Path::new("Photo.JPG")));
        assert!(should_ignore(Path::new("File.PYC")));
    }

    #[test]
    fn test_do_not_ignore_source_files() {
        assert!(!should_ignore(Path::new("test.py")));
        assert!(!should_ignore(Path::new("main.rs")));
        assert!(!should_ignore(Path::new("README.md")));
        assert!(!should_ignore(Path::new("Cargo.toml")));
    }

    #[test]
    fn test_do_not_ignore_similar_extensions() {
        assert!(!should_ignore(Path::new("script.py")));
        assert!(!should_ignore(Path::new("file.sw")));
    }
}
