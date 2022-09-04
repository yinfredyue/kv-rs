use crate::{
    message::{GetResponse, RemoveResponse, Request, SetResponse},
    KvsEngine, Result,
};
use serde_json::Deserializer;
use std::io::Write;
use std::{
    io::{BufReader, BufWriter},
    net::{SocketAddr, TcpListener, TcpStream},
};

///
pub struct KvServer<E: KvsEngine> {
    engine: E,
}

impl<E: KvsEngine> KvServer<E> {
    ///
    pub fn serve(engine: E, addr: SocketAddr) -> Result<()> {
        let mut server = KvServer { engine };

        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            server.handle_connection(stream)?;
        }

        Ok(())
    }

    fn handle_connection(&mut self, stream: TcpStream) -> Result<()> {
        let reader = BufReader::new(&stream);
        let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();
        let mut resp_writer = BufWriter::new(&stream);

        for req in req_reader {
            let req = req?;
            println!("req: {:?}", req);

            match req {
                Request::Get { key } => {
                    let resp = match self.engine.get(key) {
                        Ok(res) => GetResponse::Ok(res),
                        Err(err) => GetResponse::Err(err),
                    };
                    println!("resp: {:?}", resp);
                    serde_json::to_writer(&mut resp_writer, &resp)
                }
                Request::Set { key, value } => {
                    let resp = match self.engine.set(key, value) {
                        Ok(()) => SetResponse::Ok,
                        Err(err) => SetResponse::Err(err),
                    };
                    println!("resp: {:?}", resp);
                    serde_json::to_writer(&mut resp_writer, &resp)
                }
                Request::Remove { key } => {
                    let resp = match self.engine.remove(key) {
                        Ok(()) => RemoveResponse::Ok,
                        Err(err) => RemoveResponse::Err(err),
                    };
                    println!("resp: {:?}", resp);
                    serde_json::to_writer(&mut resp_writer, &resp)
                }
            }?;
            resp_writer.flush()?;
        }

        Ok(())
    }
}
