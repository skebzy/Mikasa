//! Entry point for Mikasa CLI

mod cli;
mod framework;
mod prompt;
mod shell;
mod ui;
mod utils;

use anyhow::{Context, Result};
use clap::Parser;

use framework::dispatch;
use prompt::project::gather_options;

use inquire::Select;
use std::env;

fn main() -> Result<()> {
    let cli = if env::args().len() == 1 {
        let choices = vec!["Next.js", "React (Vite)"];
        let selection = Select::new("Select a framework to scaffold", choices)
            .prompt()
            .context("Failed to select framework")?;
        let command = match selection {
            "Next.js" => cli::Commands::Next,
            "React (Vite)" => cli::Commands::React,
            _ => unreachable!(),
        };
        cli::Cli { command, no_postinstall: false }
    } else {
        cli::Cli::parse()
    };

    let mut options = gather_options()?;
    // Respect the CLI flag to disable post‑install automation
    options.run_postinstall = !cli.no_postinstall;

    dispatch(cli.command, options)
}
