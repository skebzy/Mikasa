//! Post‑install automation utilities

use crate::prompt::project::ProjectOptions;
use crate::shell;
use crate::ui;
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Execute post‑setup steps: install dependencies, init git, create .gitignore, optional formatter.
pub fn run_postinstall(opts: &ProjectOptions) -> Result<()> {
    let project_dir = Path::new(&opts.project_name);

    // Helper to run a command.
    fn exec(cmd: &str, args: &[&str]) -> Result<()> {
        // Build a vector of arguments without re‑adding the command itself.
        let args_vec: Vec<String> = args.iter().map(|a| (*a).to_string()).collect();
        shell::run(cmd, &args_vec)?;
        Ok(())
    }

    // Install dependencies step removed – scaffolds already handle it.

    // 2. Initialise git repository.
    ui::step("Initialising git repository...");
    exec("git", &["init"])?;
    exec("git", &["add", "."])?;
    exec("git", &["commit", "-m", "Initial commit"])?;

    // 3. Create default .gitignore if missing.
    let gitignore = project_dir.join(".gitignore");
    if !gitignore.exists() {
        let content = "target/\nnode_modules/\n.DS_Store\n.idea/\n*.log\n";
        fs::write(&gitignore, content)?;
        ui::step("Created default .gitignore");
    }

    // 5. Run any custom scripts defined in a `mikasa.toml` config file.
    let config_path = project_dir.join("mikasa.toml");
    if config_path.exists() {
        ui::step("Running custom post‑install scripts from mikasa.toml");
        if let Ok(contents) = fs::read_to_string(&config_path) {
            // Very simple parser: each line is a quoted command string.
            let mut scripts = Vec::new();
            for line in contents.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with('"') && trimmed.ends_with('"') {
                    if let Some(stripped) =
                        trimmed.strip_prefix('"').and_then(|s| s.strip_suffix('"'))
                    {
                        scripts.push(stripped.to_string());
                    }
                }
            }
            for script in scripts {
                let parts: Vec<&str> = script.split_whitespace().collect();
                if !parts.is_empty() {
                    let cmd = parts[0];
                    let args = &parts[1..];
                    let _ = exec(cmd, args);
                }
            }
        }
    }
    Ok(())
}
