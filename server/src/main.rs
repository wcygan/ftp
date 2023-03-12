use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use common::signals::Signal;
use common::signals::Signal::{Download, Upload};
use common::transmission::Connection;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6655").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut conn = Connection::new(socket);
            let action = conn.read::<Signal>().await?;

            match action {
                Some(Upload { filename, size }) => {
                    println!("Uploading {} bytes to {}", size, filename);
                    conn.write(&Signal::Ack).await?;
                }
                Some(Download { filename }) => {
                    println!("Downloading {}", filename);
                    conn.write(&Signal::Ack).await?;
                }
                None | _ => {
                    println!("Invalid signal");
                }
            }

            Ok::<_, anyhow::Error>(())
        });
    }
}