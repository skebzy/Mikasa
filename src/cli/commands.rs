use clap::{Parser, Subcommand};

/// Top‑level CLI definition.
#[derive(Parser)]
#[command(
    name = "mikasa",
    version,
    about = "A simple project setup CLI",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    /// Disable automatic post‑install steps
    #[arg(long = "no-postinstall", action = clap::ArgAction::SetTrue)]
    pub no_postinstall: bool,
}

/// Sub‑commands for each framework.
#[derive(Subcommand)]
pub enum Commands {
    React,
    Next,
}
