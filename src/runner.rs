use anyhow::{Result, bail};
use std::process::Command;

pub fn run(program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program).args(args).status()?;

    if !status.success() {
        bail!("Command failed");
    }

    Ok(())
}
