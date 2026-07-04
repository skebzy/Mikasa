//! UI helper functions with colored output using `owo-colors`.

use owo_colors::OwoColorize;

/// Print a step/message before an action.
pub fn step(msg: &str) {
    println!("{}", msg);
}

/// Print an informational message.
#[allow(dead_code)]
pub fn info(msg: &str) {
    println!("{}", msg);
}

/// Print a success message.
pub fn success(msg: &str) {
    println!("{}", msg);
}

/// Print an error message to stderr.
pub fn error(msg: &str) {
    eprintln!("{}", msg);
}
