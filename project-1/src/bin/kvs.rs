use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Get { key: String },
    Set { key : String, value : String},
    Rm  { key : String }
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
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