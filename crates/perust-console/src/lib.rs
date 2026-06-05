//! # perust-console
//!
//! Console I/O and logging for the PeRust server.
//!
//! This crate provides:
//! - **Console**: Asynchronous stdin reader that sends lines through a channel
//! - **ConsoleSender**: Formatted, colored console output
//! - **PeRustLogger**: A `log` crate integration with colored, timestamped output

pub mod console;
pub mod console_sender;
pub mod logger;

pub use console::Console;
pub use console_sender::ConsoleSender;
pub use logger::PeRustLogger;
