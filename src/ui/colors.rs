//! UI helper functions with colored output using `owo-colors`.

use owo_colors::OwoColorize;

/// Print a step/message before an action.
pub fn step(msg: &str) {
    println!("{} {}", "⏳".blue(), msg);
}

/// Print an informational message.
#[allow(dead_code)]
pub fn info(msg: &str) {
    println!("{} {}", "ℹ".blue(), msg);
}

/// Print a success message.
pub fn success(msg: &str) {
    println!("{} {}", "✔".green(), msg);
}

/// Print an error message to stderr.
pub fn error(msg: &str) {
    eprintln!("{} {}", "✖".red(), msg);
}
