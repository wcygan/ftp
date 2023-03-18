use anyhow::Result;
use common::signals::Signal;
use common::signals::Signal::{Download, Upload};
use common::transmission::Connection;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = bind("0.0.0.0:6655").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle(stream).await?;
            Ok::<_, anyhow::Error>(())
        });
    }
}

async fn handle(stream: TcpStream) -> Result<()> {
    let mut conn = Connection::new(stream);
    let action = conn.read::<Signal>().await?;

    match action {
        Some(Upload { filename }) => {
            conn.write(&Signal::Ack).await?;
            conn.read_bytes_to_file(filename.as_str()).await?;
        }
        Some(Download { filename }) => {
            conn.write(&Signal::Ack).await?;
            conn.send_bytes_from_file(filename.as_str()).await?;
        }
        None => {
            println!("Invalid signal");
        }
        _ => {
            println!("{:?} not handled", action)
        }
    }

    Ok(())
}

async fn bind<A: ToSocketAddrs>(addr: A) -> Result<TcpListener> {
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on: {}", listener.local_addr()?);
    Ok(listener)
}
