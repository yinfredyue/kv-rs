use serde::Deserialize;
use serde_json::de::IoRead;

use crate::message::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::Result;
use serde_json::Deserializer;
use std::io::{BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpStream};

///
pub struct KvClient {
    writer: BufWriter<TcpStream>,
    deserializer: Deserializer<IoRead<BufReader<TcpStream>>>,
}

impl KvClient {
    ///
    pub fn new(addr: SocketAddr) -> Result<Self> {
        let tcp_reader = TcpStream::connect(addr)?;
        let tcp_writer = tcp_reader.try_clone()?; // Use try_clone()
        let writer = BufWriter::new(tcp_reader);
        let deserializer = Deserializer::from_reader(BufReader::new(tcp_writer));
        Ok(KvClient {
            writer,
            deserializer,
        })
    }

    ///
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        serde_json::to_writer(&mut self.writer, &Request::Get { key })?;
        self.writer.flush()?;
        // Cannot use serde_json::from_reader. It looks for EOF.
        let resp = GetResponse::deserialize(&mut self.deserializer)?;
        match resp {
            GetResponse::Ok(res) => Result::Ok(res),
            GetResponse::Err(err) => Result::Err(err),
        }
    }

    ///
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Set { key, value })?;
        self.writer.flush()?;
        let resp = SetResponse::deserialize(&mut self.deserializer)?;
        match resp {
            SetResponse::Ok => Result::Ok(()),
            SetResponse::Err(err) => Result::Err(err),
        }
    }

    ///
    pub fn remove(&mut self, key: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Remove { key })?;
        self.writer.flush()?;
        let resp = RemoveResponse::deserialize(&mut self.deserializer)?;
        match resp {
            RemoveResponse::Ok => Result::Ok(()),
            RemoveResponse::Err(err) => Result::Err(err),
        }
    }
}
