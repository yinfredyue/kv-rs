use clap::{Parser, ValueEnum};
use kvs::{KvServer, KvStore, Result, SledKvsStore};
use std::net::SocketAddr;
use tracing::info;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Engine {
    kvs,
    sled,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)] // Read from Cargo.toml
#[clap(propagate_version = true)]
struct Args {
    /// addr: IP:port format
    #[clap(long, value_parser, default_value = "127.0.0.1:4000")]
    addr: SocketAddr,

    /// Storage engine: 'kvs' or 'sled'
    #[clap(long, arg_enum, value_parser, default_value_t = Engine::kvs)]
    engine: Engine,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let args = Args::parse();

    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    info!("addr: {}, Engine: {:?}", args.addr, args.engine);

    let dir = std::env::current_dir()?;
    match args.engine {
        Engine::kvs => KvServer::serve(KvStore::open(dir.as_path())?, args.addr),
        Engine::sled => KvServer::serve(SledKvsStore::open(dir.as_path())?, args.addr),
    }?;

    Ok(())
}
