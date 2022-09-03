use serde::Deserialize;

use crate::message::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::Result;
use serde_json::Deserializer;
use std::io::{BufReader, BufWriter};
use std::{
    net::{SocketAddr, TcpStream},
};

///
pub struct KvClient {
    stream: TcpStream,
}

impl KvClient {
    ///
    pub fn new(addr: SocketAddr) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(KvClient { stream })
    }

    ///
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        serde_json::to_writer(&mut self.stream, &Request::Get { key })?;
        // Cannot use serde_json::from_reader. It looks for EOF.
        let resp = GetResponse::deserialize(&mut Deserializer::from_reader(BufReader::new(
            &mut self.stream,
        )))?;
        match resp {
            GetResponse::Ok(res) => Result::Ok(res),
            GetResponse::Err(err) => Result::Err(err),
        }
    }

    ///
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        serde_json::to_writer(&mut self.stream, &Request::Set { key, value })?;
        let resp = SetResponse::deserialize(&mut Deserializer::from_reader(BufReader::new(
            &mut self.stream,
        )))?;
        match resp {
            SetResponse::Ok => Result::Ok(()),
            SetResponse::Err(err) => Result::Err(err),
        }
    }

    ///
    pub fn remove(&mut self, key: String) -> Result<()> {
        serde_json::to_writer(&mut self.stream, &Request::Remove { key })?;
        let resp = RemoveResponse::deserialize(&mut Deserializer::from_reader(BufReader::new(
            &mut self.stream,
        )))?;
        match resp {
            RemoveResponse::Ok => Result::Ok(()),
            RemoveResponse::Err(err) => Result::Err(err),
        }
    }
}
