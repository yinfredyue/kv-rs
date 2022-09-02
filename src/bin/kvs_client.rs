use clap::{Parser, Subcommand};
use kvs::{KvStore, KvStoreError, Result};
use std::{env, process};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)] // Read from Cargo.toml
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get {
        key: String,

        #[clap(long, value_name = "IP-PORT")]
        addr: Option<String>,
    },
    Set {
        key: String,
        value: String,

        #[clap(long, value_name = "IP-PORT")]
        addr: Option<String>,
    },
    Rm {
        key: String,

        #[clap(long, value_name = "IP-PORT")]
        addr: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get { key, addr } => Ok(()),
        Commands::Set { key, value, addr } => Ok(()),
        Commands::Rm { key, addr } => Ok(()),
    }
}
