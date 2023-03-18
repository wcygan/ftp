use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "client")]
#[command(about = "A client to interact with the ftp server", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub request: Request,
}

#[derive(clap::Parser, Debug)]
pub enum Request {
    Download {
        #[clap(short = 'f', long = "file")]
        file: String,
        #[clap(short = 'a', long = "address")]
        address: String,
        #[clap(short = 'p', long = "port")]
        port: u16,
    },
    Upload {
        #[clap(short = 'f', long = "file")]
        file: String,
        #[clap(short = 'a', long = "address")]
        address: String,
        #[clap(short = 'p', long = "port")]
        port: u16,
    },
}
