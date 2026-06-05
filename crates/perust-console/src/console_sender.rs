use perust_utils::color::{format_color, ColorCode};

/// Sends formatted, colored output to the console (stdout).
///
/// Each method formats the message with an appropriate color and prefix,
/// then prints it to stdout.
pub struct ConsoleSender;

impl ConsoleSender {
    /// Creates a new `ConsoleSender`.
    pub fn new() -> Self {
        ConsoleSender
    }

    /// Sends a plain message to the console (no prefix, white text).
    pub fn send_message(&self, message: &str) {
        println!("{}", message);
    }

    /// Sends an info-level message to the console (cyan prefix).
    pub fn send_info(&self, message: &str) {
        let prefix = format_color("INFO", ColorCode::Cyan, None, true, false);
        println!("[{}] {}", prefix, message);
    }

    /// Sends a warning-level message to the console (yellow prefix).
    pub fn send_warning(&self, message: &str) {
        let prefix = format_color("WARN", ColorCode::Yellow, None, true, false);
        println!("[{}] {}", prefix, message);
    }

    /// Sends an error-level message to the console (red prefix).
    pub fn send_error(&self, message: &str) {
        let prefix = format_color("ERROR", ColorCode::Red, None, true, false);
        println!("[{}] {}", prefix, message);
    }
}

impl Default for ConsoleSender {
    fn default() -> Self {
        Self::new()
    }
}
