//! Prompt utilities for project creation.

use anyhow::Context;
use inquire::{Confirm, Select, Text};
use std::env;

/// Supported package managers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Bun,
    Npm,
    Pnpm,
}

impl PackageManager {
    /// Return the executable name used to invoke the manager.
    pub fn command(&self) -> &'static str {
        match self {
            PackageManager::Bun => "bun",
            PackageManager::Npm => "npm",
            PackageManager::Pnpm => "pnpm",
        }
    }
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command())
    }
}

/// Detect installed package managers by checking PATH
pub fn detect_package_managers() -> anyhow::Result<Vec<PackageManager>> {
    let mut managers = Vec::new();
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
        anyhow::bail!("No supported package manager (bun, pnpm, npm) was found on PATH");
    }
    Ok(managers)
}

/// Struct collecting all options needed to scaffold a project
#[derive(Debug, Clone)]
pub struct ProjectOptions {
    pub project_name: String,
    pub package_manager: PackageManager,
    pub typescript: bool,
    pub tailwind: bool,
    /// Run post‑install automation (install deps, git init, etc.)
    pub run_postinstall: bool,
}

/// Prompt for a non‑empty, valid project name
pub fn ask_project_name() -> anyhow::Result<String> {
    let name = Text::new("Project name:")
        .with_help_message("Alphanumeric, hyphens or underscores, no spaces")
        .prompt()
        .context("Failed to read project name")?;
    if name.trim().is_empty() {
        anyhow::bail!("Project name cannot be empty");
    }
    Ok(name)
}

/// Prompt for the package manager, showing only those that are installed.
pub fn ask_package_manager() -> anyhow::Result<PackageManager> {
    let available = detect_package_managers()?;
    let selection = Select::new("Package manager:", available.clone())
        .prompt()
        .context("Failed to select package manager")?;
    Ok(selection)
}

/// Prompt whether to use TypeScript
pub fn ask_typescript() -> anyhow::Result<bool> {
    Ok(Confirm::new("Use TypeScript?")
        .with_default(true)
        .prompt()
        .context("Failed to read TypeScript choice")?)
}

/// Prompt whether to add Tailwind CSS
pub fn ask_tailwind() -> anyhow::Result<bool> {
    Ok(Confirm::new("Add Tailwind CSS?")
        .with_default(true)
        .prompt()
        .context("Failed to read Tailwind choice")?)
}

/// Gather all options in one call – used by the CLI dispatcher
pub fn gather_options() -> anyhow::Result<ProjectOptions> {
    let project_name = ask_project_name()?;
    let package_manager = ask_package_manager()?;
    let typescript = ask_typescript()?;
    let tailwind = ask_tailwind()?;
    Ok(ProjectOptions {
        project_name,
        package_manager,
        typescript,
        tailwind,
        run_postinstall: true,
    })
}
