use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Engine {
    Kvs,
    Sled,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)] // Read from Cargo.toml
#[clap(propagate_version = true)]
struct Args {
    /// addr: IP:port format
    #[clap(long, value_parser, default_value_t = String::from("127.0.0.1:4000"))]
    addr: String,

    /// Storage engine: 'kvs' or 'sled'
    #[clap(long, arg_enum, value_parser, default_value_t = Engine::Kvs)]
    engine: Engine,
}

fn main() {
    let args = Args::parse();

    println!("{}, {:?}", args.addr, args.engine);
}
