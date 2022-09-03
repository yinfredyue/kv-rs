use clap::{Parser, Subcommand};
use kvs::{KvClient, KvStoreError, Result};
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)] // Read from Cargo.toml
#[clap(propagate_version = true)]
struct Args {
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
    let args = Args::parse();

    let mut cli = KvClient::new(args.addr)?;

    match &args.command {
        Commands::Get { key } => {
            let resp = cli.get(key.to_owned())?;
            println!("{:?}", resp);
        }
        Commands::Set { key, value } => {
            let resp = cli.set(key.to_owned(), value.to_owned())?;
            println!("{:?}", resp);
        }
        Commands::Rm { key } => {
            let resp = cli.remove(key.to_owned())?;
            println!("{:?}", resp);
        }
    }

    Ok(())
}
