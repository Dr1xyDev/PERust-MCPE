use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// Console input reader.
///
/// Starts a background tokio task that reads lines from stdin and sends
/// them through an unbounded channel for consumption by the server.
pub struct Console {
    /// Shared flag indicating whether the console reader is running.
    running: Arc<AtomicBool>,
    /// Sender half of the channel (kept for reference; actual sending is in the reader task).
    sender: mpsc::UnboundedSender<String>,
    /// Receiver half of the channel for reading console input.
    receiver: mpsc::UnboundedReceiver<String>,
}

impl Console {
    /// Creates a new `Console` instance.
    ///
    /// The console is not started until [`Console::start`] is called.
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Console {
            running: Arc::new(AtomicBool::new(false)),
            sender,
            receiver,
        }
    }

    /// Starts the background stdin reader task.
    ///
    /// Returns a `JoinHandle` for the spawned tokio task. Lines typed in
    /// stdin will be sent through the internal channel and can be read
    /// with [`Console::readline`].
    pub fn start(&self) -> JoinHandle<()> {
        self.running.store(true, Ordering::SeqCst);

        let running = self.running.clone();
        let sender = self.sender.clone();

        let handle = tokio::spawn(async move {
            let stdin = tokio::io::stdin();
            let mut reader = BufReader::new(stdin);
            let mut line = String::new();

            while running.load(Ordering::SeqCst) {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        // EOF reached
                        running.store(false, Ordering::SeqCst);
                        break;
                    }
                    Ok(_) => {
                        let trimmed = line.trim_end().to_string();
                        if !trimmed.is_empty() {
                            if sender.send(trimmed).is_err() {
                                // Receiver dropped
                                running.store(false, Ordering::SeqCst);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Console read error: {}", e);
                        running.store(false, Ordering::SeqCst);
                        break;
                    }
                }
            }
        });

        handle
    }

    /// Stops the console reader.
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Attempts to read a line from the console input channel.
    ///
    /// Returns `None` if no input is available.
    pub fn readline(&mut self) -> Option<String> {
        self.receiver.try_recv().ok()
    }

    /// Returns `true` if the console reader is still running.
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}
