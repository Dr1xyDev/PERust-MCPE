use std::io::Write;

use chrono::Local;
use log::{Level, LevelFilter, Metadata, Record};
use perust_utils::color::{format_color, ColorCode};

/// A simple logger implementation for PeRust that integrates with the `log` crate.
///
/// Output format: `[HH:MM:SS LEVEL] message` with level-dependent colors.
pub struct PeRustLogger {
    /// The minimum log level to output.
    level: LevelFilter,
}

impl PeRustLogger {
    /// Creates a new logger with the specified minimum log level.
    pub fn new(level: LevelFilter) -> Self {
        PeRustLogger { level }
    }

    /// Initializes the logger, setting it as the global logger.
    ///
    /// # Panics
    ///
    /// Panics if a logger has already been set.
    pub fn init(self) {
        let level = self.level;
        log::set_boxed_logger(Box::new(self))
            .expect("Failed to set logger: a logger is already initialized");
        log::set_max_level(level);
    }

    /// Returns the color associated with a log level.
    fn level_color(level: Level) -> ColorCode {
        match level {
            Level::Error => ColorCode::Red,
            Level::Warn => ColorCode::Yellow,
            Level::Info => ColorCode::Cyan,
            Level::Debug => ColorCode::Green,
            Level::Trace => ColorCode::BrightBlack,
        }
    }

    /// Formats a log level as a colored, right-aligned string.
    fn format_level(level: Level) -> String {
        let color = Self::level_color(level);
        let level_str = match level {
            Level::Error => "ERROR",
            Level::Warn => " WARN",
            Level::Info => " INFO",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        };
        format_color(level_str, color, None, true, false)
    }
}

impl log::Log for PeRustLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let timestamp = Local::now().format("%H:%M:%S");
        let level_str = Self::format_level(record.level());
        let target = record.target();

        // Only include target path if it's not the server binary
        let msg = if target.starts_with("perust_") || target == "server" {
            format!(
                "[{}] [{}] {}",
                timestamp,
                level_str,
                record.args()
            )
        } else {
            format!(
                "[{}] [{}] [{}] {}",
                timestamp,
                level_str,
                target,
                record.args()
            )
        };

        // Use stderr for errors, stdout for the rest
        if record.level() == Level::Error {
            eprintln!("{}", msg);
        } else {
            println!("{}", msg);
        }
    }

    fn flush(&self) {
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();
    }
}
