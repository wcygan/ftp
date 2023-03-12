use std::fmt::Debug;
use std::io::Cursor;
use anyhow::Result;
use bytes::BytesMut;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::{TcpStream, ToSocketAddrs};

#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    /// Dial the given address and return a connection
    pub async fn dial<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self::new(stream))
    }

    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    /// Write a serializable value into the stream
    pub async fn write<T: Serialize>(&mut self, value: &T) -> Result<()> {
        let buf = bincode::serialize(value)?;
        self.stream.write_all(&buf).await?;
        self.stream.flush().await?;
        Ok(())
    }

    /// Reads from the socket until a complete message is received, or an error occurs
    pub async fn read<T: DeserializeOwned>(&mut self) -> Result<Option<T>> {
        loop {
            if let Some(frame) = self.parse()? {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err(anyhow::anyhow!("connection reset by peer"))
                };
            }
        }
    }

    /// Attempts to deserialize a T from the internal buffer.
    fn parse<T: DeserializeOwned>(&mut self) -> Result<Option<T>> {
        let mut buf = Cursor::new(&self.buffer[..]);
        match bincode::deserialize_from(&mut buf) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }

    pub async fn send_bytes_from_file(&mut self, filename: &str) -> Result<()> {
        let mut file = tokio::fs::File::open(filename).await?;
        loop {
            let n = file.read_buf(&mut self.buffer).await?;
            if n == 0 {
                return Ok(());
            }
            self.stream.write_all(&self.buffer).await?;
            self.stream.flush().await?;
        }
    }

    // TODO: figure out why a bunch of null bytes are being written to the file
    pub async fn read_bytes_to_file(&mut self, filename: &str) -> Result<()> {
        let mut file = tokio::fs::File::create(filename).await?;
        loop {
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                return if self.buffer.is_empty() {
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("connection reset by peer"))
                };
            }
            file.write_all(&self.buffer).await?;
        }
    }
}