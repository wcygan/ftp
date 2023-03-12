/// The signals that can be sent between the client and server
pub enum Signal {
    /// Write a file to the server
    Write {
        /// The name of the file to write to
        file: String,
    },
    /// Read a file from the server
    Read {
        /// The name of the file to read from
        file: String,
    },
    /// Acknowledge a signal
    ///
    /// This is useful for the server to acknowledge a write signal,
    /// allowing the client to begin sending the file contents.
    Ack,
}