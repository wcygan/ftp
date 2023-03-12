use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    let mut socket = TcpStream::connect("127.0.0.1:6655").await?;
    socket.write_all(b"hello").await?;
    let mut buf = [0; 1024];
    let n = socket.read(&mut buf).await?;
    println!("recv: {}", String::from_utf8_lossy(&buf[0..n]));
    Ok(())
}