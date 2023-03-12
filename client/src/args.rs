#[derive(clap::Parser)]
pub struct Args {
    #[clap(short = 'f', long = "file")]
    pub file: String,
    #[clap(short = 'a', long = "address")]
    pub address: String,
    #[clap(short = 'p', long = "port")]
    pub port: u16,
}