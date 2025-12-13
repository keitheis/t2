use std::path::Path;

/// Print "FIGHT!" message before command execution
pub fn print_fight() {
    println!("FIGHT!");
}

/// Print "CONTINUE?" message after command completion
pub fn print_continue() {
    println!("CONTINUE?");
}

/// Print "GAMEOVER" message when exiting (Ctrl+C)
pub fn print_gameover() {
    println!("GAMEOVER");
}

/// Print the watching message at startup
pub fn print_watching(count: usize, paths: &str) {
    println!("r2 is watching about {} files:", count);
    println!("{}", paths);
}

/// Print a message when a file changes
pub fn print_file_changed(path: &Path) {
    if let Some(path_str) = path.to_str() {
        println!("{} changed", path_str);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_print_functions_exist() {
        // These tests just ensure the functions can be called without panic
        // In a real scenario, we'd capture stdout to verify output
        print_fight();
        print_continue();
        print_gameover();
        print_watching(5, "test paths");
        print_file_changed(&PathBuf::from("/tmp/test.txt"));
    }
}
