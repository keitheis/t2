use anyhow::Result;
use tokio::process::Command;

/// Execute a shell command asynchronously
///
/// Maps to Python's run_commands() function
/// Prints the command before executing it
/// Uses platform-specific shell (sh on Unix, cmd on Windows)
pub async fn execute_command(cmd: &str) -> Result<()> {
    // Print command before execution (like Python version)
    println!("{}", cmd);

    // Execute command via shell
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .status()
            .await?
    } else {
        Command::new("sh")
            .args(["-c", cmd])
            .status()
            .await?
    };

    // Print status if command failed (non-zero exit code)
    if !status.success() {
        eprintln!("Command exited with status: {}", status);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_simple_command() {
        // Test with a simple echo command
        let result = execute_command("echo 'Hello, r2!'").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_true_command() {
        // 'true' command always succeeds
        let result = execute_command("true").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_failing_command() {
        // 'false' command always fails, but shouldn't error out
        let result = execute_command("false").await;
        // The function should succeed (no panic) even if command fails
        assert!(result.is_ok());
    }
}
