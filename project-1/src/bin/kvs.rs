use clap::{Parser, Subcommand};

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
    Set { key : String, value : String},
    Rm  { key : String }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get { key } => {
            eprintln!("unimplemented");
            todo!()
        },
        Commands::Set { key, value } => {
            eprintln!("unimplemented");
            todo!()
        },
        Commands::Rm { key } => {
            eprintln!("unimplemented");
            todo!()
        },
    }
}