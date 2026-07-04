//! Cross‑platform command runner used by framework implementations

use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Verify that a binary exists in the system PATH.
// command_exists removed – not used

/// Return the full path to an executable if it exists in PATH.
fn find_program(bin: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths)
        .map(|dir| {
            #[cfg(windows)]
            {
                // Try .exe, .cmd, .bat, then bare name
                let exe = dir.join(format!("{}.exe", bin));
                if exe.is_file() {
                    return exe;
                }
                let cmd = dir.join(format!("{}.cmd", bin));
                if cmd.is_file() {
                    return cmd;
                }
                let bat = dir.join(format!("{}.bat", bin));
                if bat.is_file() {
                    return bat;
                }
                dir.join(bin)
            }
            #[cfg(not(windows))]
            {
                dir.join(bin)
            }
        })
        .find(|p| p.is_file())
}

// used for running commands
use crate::ui::error;
use crate::ui::step;
use crate::ui::success;

/// Execute a program with the given arguments.
///
/// * `program` – executable name (e.g. "bun", "npm").
/// * `args` – slice of argument strings.
/// Returns `Ok(())` on success, otherwise an error with context.
pub fn run(program: &str, args: &[String]) -> Result<()> {
    // Display the command that will run.
    step(&format!("Running: {} {}", program, args.join(" ")));

    let prog_path = find_program(program).ok_or_else(|| {
        anyhow::anyhow!(
            "Command `{}` not found in PATH. Install it or ensure it is on PATH",
            program
        )
    })?;
    let status = Command::new(prog_path)
        .args(args)
        .status()
        .with_context(|| format!("Failed to spawn `{}`", program))?;

    if status.success() {
        success("Command completed successfully");
        Ok(())
    } else {
        // Include exit code if available.
        let code = status
            .code()
            .map_or(String::from("unknown"), |c| c.to_string());
        error(&format!("Command exited with code {}", code));
        anyhow::bail!("Command `{}` failed with exit code {}", program, code);
    }
}
