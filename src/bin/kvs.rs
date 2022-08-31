use clap::{Parser, Subcommand};
use kvs::Result;

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

    match &cli.command {
        Commands::Get { key: _ } => {
            eprintln!("unimplemented");
            Result::Ok(())
        }
        Commands::Set { key: _, value: _ } => {
            eprintln!("unimplemented");
            Result::Ok(())
        }
        Commands::Rm { key: _ } => {
            eprintln!("unimplemented");
            Result::Ok(())
        }
    }
}
