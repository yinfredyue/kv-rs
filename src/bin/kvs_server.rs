use clap::{Parser, ValueEnum};
use kvs::{KvServer, KvStore, Result};
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber;

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
    #[clap(long, value_parser, default_value = "127.0.0.1:4000")]
    addr: SocketAddr,

    /// Storage engine: 'kvs' or 'sled'
    #[clap(long, arg_enum, value_parser, default_value_t = Engine::Kvs)]
    engine: Engine,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    info!("addr: {}, Engine: {:?}", args.addr, args.engine);

    match args.engine {
        Engine::Kvs => KvServer::serve(
            KvStore::open(std::env::current_dir()?.as_path())?,
            args.addr,
        ),
        Engine::Sled => todo!(),
    };

    Ok(())
}
