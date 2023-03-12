use anyhow::Result;
use clap::Parser;
use tokio::io::{AsyncReadExt};
use tokio::net::TcpStream;
use crate::args::Args;

mod args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let _f = tokio::fs::File::open(args.file).await?;
    let mut socket = TcpStream::connect("127.0.0.1:6655").await?;
    let mut buf = [0; 1024];
    let n = socket.read(&mut buf).await?;
    println!("recv: {}", String::from_utf8_lossy(&buf[0..n]));
    Ok(())
}

