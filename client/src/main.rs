use anyhow::Result;
use clap::Parser;
use tokio::io::{AsyncReadExt};
use tokio::net::TcpStream;
use common::signals::Signal;
use common::signals::Signal::Upload;
use common::transmission::Connection;
use crate::args::Args;

mod args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let f = tokio::fs::File::open(args.file.as_str()).await?;

    let upload = Upload {
        filename: args.file.clone(),
        size: f.metadata().await?.len(),
    };

    let mut conn = Connection::dial("127.0.0.1:6655").await?;

    conn.write(&upload).await?;

    let reply = conn.read::<Signal>().await?;

    match reply {
        Some(Signal::Ack) => {
            println!("Upload request acknowledged");

            conn.send_bytes_from_file(args.file.as_str()).await?;
        }
        _ => {
            println!("Upload failed");
        }
    }

    Ok(())
}

