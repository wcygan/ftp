use crate::args::Request;
use anyhow::Result;
use clap::{arg, Parser};
use common::signals::Signal;
use common::transmission::Connection;
use tokio::fs::File;
use tokio::net::ToSocketAddrs;

mod args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Cli::parse();
    let (addr, req) = get(args);

    match req {
        Signal::Upload { filename } => {
            let f = File::open(filename.as_str()).await?;

            let mut conn = Connection::dial("127.0.0.1:6655").await?;

            conn.write(&Signal::Upload {
                filename: filename.clone(),
            })
            .await?;

            let reply = conn.read::<Signal>().await?;

            match reply {
                Some(Signal::Ack) => {
                    println!("Upload request acknowledged");

                    conn.send_bytes_from_file(filename.as_str()).await?;
                }
                _ => {
                    println!("Upload failed");
                }
            }
        }
        Signal::Download { filename } => {}
        _ => {}
    };

    Ok(())
}

fn get(cli: args::Cli) -> (String, Signal) {
    match cli.request {
        Request::Upload {
            file,
            address,
            port,
        } => (
            socket_addr(address.as_str(), port),
            Signal::Upload { filename: file },
        ),
        Request::Download {
            file,
            address,
            port,
        } => (
            socket_addr(address.as_str(), port),
            Signal::Download { filename: file },
        ),
    }
}

fn socket_addr(address: &str, port: u16) -> String {
    format!("{address}:{port}")
}
