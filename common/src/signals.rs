use serde::{Deserialize, Serialize};

/// The signals that can be sent between the client and server
#[derive(Debug, Serialize, Deserialize)]
pub enum Signal {
    /// Upload a file to the server
    Upload {
        /// The name of the file to write to
        filename: String,
    },
    /// Download a file from the server
    Download {
        /// The name of the file to read from
        filename: String,
    },
    /// Acknowledge a signal
    ///
    /// This is useful for the server to acknowledge a write signal,
    /// allowing the client to begin sending the file contents.
    Ack,
}
