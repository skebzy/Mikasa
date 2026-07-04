//! Validation helpers for Mikasa CLI

use anyhow::{Result, bail};

/// Validate a project name.
///
/// * Must not be empty.
/// * Must not contain whitespace.
/// * Only alphanumeric, hyphens and underscores are allowed.
#[allow(dead_code)]
pub fn validate_project_name(name: &str) -> Result<()> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        bail!("Project name cannot be empty");
    }
    if trimmed.chars().any(|c| c.is_whitespace()) {
        bail!("Project name must not contain spaces");
    }
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        bail!("Project name may only contain letters, numbers, '_' or '-' characters");
    }
    Ok(())
}
