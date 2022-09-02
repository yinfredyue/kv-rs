use clap::{Parser, Subcommand};
use kvs::{KvStore, KvStoreError, Result};
use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)] // Read from Cargo.toml
#[clap(propagate_version = true)]
struct Cli {
    #[clap(
        long,
        global = true, // Global args can be provided anywhere in the command
        value_name = "IP-PORT",
        default_value = "127.0.0.1:4000"
    )]
    addr: SocketAddr,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut stream = TcpStream::connect(cli.addr)?;
    stream.write("hello!".as_bytes())?;

    match &cli.command {
        Commands::Get { key } => Ok(()),
        Commands::Set { key, value } => Ok(()),
        Commands::Rm { key } => Ok(()),
    }
}
