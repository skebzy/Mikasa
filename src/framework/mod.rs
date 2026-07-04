//! Framework abstraction and dispatcher

use crate::prompt::project::ProjectOptions;
use anyhow::Result;

pub mod next;
pub mod react;

/// The common trait all frameworks implement.
#[allow(dead_code)]
pub trait Framework {
    fn name(&self) -> &'static str;
    fn run(&self, opts: ProjectOptions) -> Result<()>;
}

/// Dispatch to the concrete framework implementation based on CLI enum.
pub fn dispatch(command: crate::cli::Commands, opts: ProjectOptions) -> Result<()> {
    match command {
        crate::cli::Commands::Next => {
            let fw = next::NextFramework;
            fw.run(opts)
        }
        crate::cli::Commands::React => {
            let fw = react::ReactFramework;
            fw.run(opts)
        }
    }
}
