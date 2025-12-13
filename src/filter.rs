use regex::Regex;
use std::path::Path;

/// Filter for ignoring certain file types during file watching
pub struct IgnoreFilter {
    pattern: Regex,
}

impl IgnoreFilter {
    /// Create a new IgnoreFilter with default ignored extensions
    ///
    /// Ignores: .pyc, .swp, .swo, .bmp, .jpg, .jpeg, .png, .gif, .svg, .psd, .xcf, .pxm
    /// This matches the Python p2 implementation
    pub fn new() -> Self {
        let extensions = vec![
            "pyc", "swp", "swo",
            "bmp", "jpg", "jpeg",
            "png", "gif", "svg",
            "psd", "xcf", "pxm",
        ];

        // Create case-insensitive regex pattern
        let pattern_str = format!(r"(?i).*\.({})$", extensions.join("|"));
        let pattern = Regex::new(&pattern_str).expect("Invalid regex pattern");

        Self { pattern }
    }

    /// Check if a file path should be ignored
    pub fn should_ignore(&self, path: &Path) -> bool {
        path.to_str()
            .map(|s| self.pattern.is_match(s))
            .unwrap_or(false)
    }
}

impl Default for IgnoreFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_ignore_pyc() {
        let filter = IgnoreFilter::new();
        assert!(filter.should_ignore(Path::new("test.pyc")));
        assert!(filter.should_ignore(Path::new("/path/to/file.pyc")));
    }

    #[test]
    fn test_ignore_swap_files() {
        let filter = IgnoreFilter::new();
        assert!(filter.should_ignore(Path::new(".test.swp")));
        assert!(filter.should_ignore(Path::new("file.swo")));
    }

    #[test]
    fn test_ignore_images() {
        let filter = IgnoreFilter::new();
        assert!(filter.should_ignore(Path::new("image.png")));
        assert!(filter.should_ignore(Path::new("photo.jpg")));
        assert!(filter.should_ignore(Path::new("picture.jpeg")));
        assert!(filter.should_ignore(Path::new("icon.svg")));
        assert!(filter.should_ignore(Path::new("design.psd")));
    }

    #[test]
    fn test_case_insensitive() {
        let filter = IgnoreFilter::new();
        assert!(filter.should_ignore(Path::new("IMAGE.PNG")));
        assert!(filter.should_ignore(Path::new("Photo.JPG")));
        assert!(filter.should_ignore(Path::new("File.PYC")));
    }

    #[test]
    fn test_do_not_ignore_source_files() {
        let filter = IgnoreFilter::new();
        assert!(!filter.should_ignore(Path::new("test.py")));
        assert!(!filter.should_ignore(Path::new("main.rs")));
        assert!(!filter.should_ignore(Path::new("README.md")));
        assert!(!filter.should_ignore(Path::new("Cargo.toml")));
    }

    #[test]
    fn test_do_not_ignore_similar_extensions() {
        let filter = IgnoreFilter::new();
        // .py is not .pyc
        assert!(!filter.should_ignore(Path::new("script.py")));
        // .sw is not .swp or .swo
        assert!(!filter.should_ignore(Path::new("file.sw")));
    }
}
