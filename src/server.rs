use crate::message::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::Result;
use serde_json::Deserializer;
use std::io::Write;
use std::{
    io::{BufRead, BufReader, BufWriter},
    net::{SocketAddr, TcpListener, TcpStream},
};
use tracing::info;

use crate::KvsEngine;

///
pub struct KvServer {}

impl KvServer {
    ///
    pub fn serve(engine: impl KvsEngine, addr: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            Self::handle_connection(stream)?;
        }

        Ok(())
    }

    fn handle_connection(stream: TcpStream) -> Result<()> {
        let reader = BufReader::new(&stream);
        let req_reader = Deserializer::from_reader(& stream).into_iter::<Request>();
        let mut resp_writer = BufWriter::new(&stream);

        for req in req_reader {
            println!("{:?}", req);

            match req? {
                Request::Get { key } => serde_json::to_writer(
                    &mut resp_writer,
                    &GetResponse::Ok(Some(String::from("dummy"))),
                ),
                Request::Set { key, value } => {
                    serde_json::to_writer(&mut resp_writer, &SetResponse::Ok)
                }
                Request::Remove { key } => {
                    serde_json::to_writer(&mut resp_writer, &RemoveResponse::Ok)
                }
            }?;

            resp_writer.flush()?;

            println!("sent resp");
        }

        Ok(())
    }
}
