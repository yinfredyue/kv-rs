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
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Implicit of the test case: use current directory.
    let dir = env::current_dir();
    let mut store = KvStore::open(dir?.as_path())?;

    match &cli.command {
        Commands::Get { key } => {
            match store.get(key.to_owned())? {
                None => println!("Key not found"),
                Some(v) => println!("{}", v),
            };
            Ok(())
        }
        Commands::Set { key, value } => store.set(key.to_owned(), value.to_owned()),
        Commands::Rm { key } => {
            let res = store.remove(key.to_owned());
            if let Err(KvStoreError::RemoveNonexistingKey) = res {
                println!("Key not found");
                process::exit(1);
            }
            Ok(())
        }
    }
}
