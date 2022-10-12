extern crate resp;
extern crate threadpool;

use std::str;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Write;
use std::io::BufReader;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use self::threadpool::ThreadPool;
use self::resp::{Decoder, Value};

use storage::Database;
use command::Command;

pub struct Server {
    ipaddr: String,
    port: String,
}

impl Server {
    pub fn new(ipaddr: String, port: String) -> Self {
        Self {
            ipaddr: ipaddr,
            port: port
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.ipaddr, self.port));

        match listener {
            Ok(listener) => self.handle_incoming_connections(listener),
            Err(err) => println!("{:?}", err),
        }
    }

    fn handle_incoming_connections(&self, listener: TcpListener) {
        info!("listening started, ready to accept");

        // Using a small threadpool for testing purposes.
        let pool = ThreadPool::new(8);

        for stream in listener.incoming() {
            pool.execute(move|| {
                let mut database = Database::new();
                let mut stream = stream.unwrap();
                let mut request = Request { stream: &mut stream };

                request.run(&mut database);
            });
        }
    }
}

struct Request<'a> {
    stream: &'a mut TcpStream,
}

impl<'a> Request<'a> {
    pub fn run(&mut self, database: &mut Database) {
        loop {
            let mut buffer = vec![0; 128];
            let peeked = self.stream.peek(&mut buffer).unwrap();

            println!("peeked --> {:?}", peeked);

            if peeked == 0 {
                break;
            }

            let payload_size = self.stream.read(&mut buffer).unwrap();
            let request = &buffer[0..payload_size];
            let reader = BufReader::new(request);
            let mut decoder = Decoder::new(reader);
            let values = decoder.decode().unwrap();
            let command = Command::new(values);

            debug!("command --> {:?}", command);

            match command.run(database) {
                Ok(response) => {
                    self.stream.write(&response.encode()).unwrap();
                    self.stream.flush().unwrap();
                },
                Err(error) => {
                    error!("error --> {:?}", error);
                    self.stream.write(&error.encode()).unwrap();
                    self.stream.flush().unwrap();
                }
            }
        }
    }
}