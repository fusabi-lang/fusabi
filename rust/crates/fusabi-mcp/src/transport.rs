use anyhow::Result;
use serde_json::Value;
use std::io::{self, BufRead, BufReader, Write};
use tokio::sync::mpsc;
use tracing::{debug, error};

use crate::protocol::{JsonRpcMessage, JsonRpcResponse};

pub struct StdioTransport {
    tx: mpsc::UnboundedSender<JsonRpcMessage>,
}

impl StdioTransport {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<JsonRpcMessage>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, rx)
    }

    /// Start reading from stdin and sending messages through the channel
    pub fn start_reading(self) {
        std::thread::spawn(move || {
            let stdin = io::stdin();
            let reader = BufReader::new(stdin.lock());

            for line in reader.lines() {
                match line {
                    Ok(line) if !line.is_empty() => {
                        debug!("Received: {}", line);
                        match serde_json::from_str::<JsonRpcMessage>(&line) {
                            Ok(msg) => {
                                if self.tx.send(msg).is_err() {
                                    error!("Failed to send message, receiver dropped");
                                    break;
                                }
                            }
                            Err(e) => {
                                error!("Failed to parse JSON-RPC message: {}", e);
                            }
                        }
                    }
                    Ok(_) => {} // Empty line, skip
                    Err(e) => {
                        error!("Error reading from stdin: {}", e);
                        break;
                    }
                }
            }
            debug!("Stdin reader thread exiting");
        });
    }

    /// Write a response to stdout
    pub fn write_response(response: &JsonRpcResponse) -> Result<()> {
        let json = serde_json::to_string(response)?;
        let mut stdout = io::stdout();
        writeln!(stdout, "{}", json)?;
        stdout.flush()?;
        debug!("Sent: {}", json);
        Ok(())
    }

    /// Write any JSON value to stdout (for notifications)
    pub fn write_message(message: &Value) -> Result<()> {
        let json = serde_json::to_string(message)?;
        let mut stdout = io::stdout();
        writeln!(stdout, "{}", json)?;
        stdout.flush()?;
        debug!("Sent: {}", json);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_creation() {
        let (_transport, mut rx) = StdioTransport::new();
        assert!(rx.try_recv().is_err());
    }
}