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
    let mut conn = Connection::dial(addr).await?;
    match req {
        Signal::Upload { filename } => {
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
        Signal::Download { filename } => {
            conn.write(&Signal::Download {
                filename: filename.clone(),
            })
            .await?;

            let reply = conn.read::<Signal>().await?;

            match reply {
                Some(Signal::Ack) => {
                    println!("Download request acknowledged");
                    conn.read_bytes_to_file(filename.as_str()).await?;
                }
                _ => {
                    println!("Download failed");
                }
            }
        }
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
