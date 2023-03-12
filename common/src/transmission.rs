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
}