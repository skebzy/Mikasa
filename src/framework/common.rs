//! Shared utilities for framework modules.

use crate::prompt::project::PackageManager;
use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

/// Detect which package managers are available on the system by looking for
/// their executable in the `PATH`. Returns a vector of the detected managers.
pub fn detect_package_managers() -> Result<Vec<PackageManager>> {
    let mut managers = Vec::new();
    // Helper to check existence of an executable.
    fn exists(bin: &str) -> bool {
        env::var_os("PATH")
            .and_then(|paths| {
                env::split_paths(&paths)
                    .map(|dir| dir.join(bin))
                    .find(|p| p.is_file())
            })
            .is_some()
    }

    if exists("bun") {
        managers.push(PackageManager::Bun);
    }
    if exists("pnpm") {
        managers.push(PackageManager::Pnpm);
    }
    if exists("npm") {
        managers.push(PackageManager::Npm);
    }

    if managers.is_empty() {
        // Nothing found – this is a user‑visible error.
        anyhow::bail!("No supported package manager (bun, pnpm, npm) was found on PATH");
    }
    Ok(managers)
}

/// Helper to build a command vector for the selected package manager.
pub fn manager_command(pm: &PackageManager) -> &'static str {
    pm.command()
}
