use clap::{arg_enum, Parser, ValueEnum};
use kvs::{
    thread_pool::{SharedQueueThreadPool, ThreadPool},
    KvServer, KvStore, KvStoreError, Result, SledKvsStore,
};
use std::{fs, net::SocketAddr, path::Path};
use tracing::info;

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
    enum Engine {
        kvs,
        sled,
    }
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

    // check engine
    match current_engine(dir.as_path())? {
        None => fs::write(dir.join("engine").as_path(), format!("{:?}", args.engine))?,
        Some(prev_engine) => {
            if prev_engine != args.engine {
                return Err(KvStoreError::WrongEngine);
            }
        }
    }

    let num_threads = (num_cpus::get() * 2) as u32;
    let thread_pool = SharedQueueThreadPool::new(num_threads)?;

    match args.engine {
        Engine::kvs => KvServer::serve(KvStore::open(dir.as_path())?, thread_pool, args.addr),
        Engine::sled => KvServer::serve(SledKvsStore::open(dir.as_path())?, thread_pool, args.addr),
    }?;

    Ok(())
}

fn current_engine(dir_path: &Path) -> Result<Option<Engine>> {
    let engine_file = dir_path.join("engine");

    if !engine_file.try_exists()? {
        return Ok(None);
    }

    match fs::read_to_string(engine_file)?.parse::<Engine>() {
        Ok(e) => Ok(Some(e)),
        Err(_) => Err(KvStoreError::WrongEngine),
    }
}
