extern crate resp;

use std::str;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Write;
use std::io::BufReader;
use std::thread;

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
        match TcpListener::bind(self.connection_string()) {
            Ok(listener) => {
                info!("listening started, ready to accept");

                // This isn't great because it can spawn endless threads.
                // Eventually make this into a threadpool.
                for stream in listener.incoming() {
                    thread::spawn(move || {
                        debug!("stream --> {:#?}", stream);

                        let mut database = Database::new();
                        let mut stream = stream.unwrap();

                        Self::handle_stream(&mut stream, &mut database);
                    });
                }
            },
            Err(err) => println!("{:?}", err),
        }
    }

    fn connection_string(&self) -> String {
        format!("{}:{}", self.ipaddr, self.port)
    }

    fn handle_stream(stream: &mut TcpStream, database: &mut Database) {
        loop {
            let mut buffer = vec![0; 128];
            let payload_size = stream.read(&mut buffer).unwrap();

            if payload_size == 0 {
                break;
            }

            let request = &buffer[0..payload_size];
            let reader = BufReader::new(request);
            let mut decoder = Decoder::new(reader);
            let values = decoder.decode().unwrap();
            let command = Command::new(values);

            debug!("command --> {:?}", command);

            match command.run(database) {
                Ok(response) => {
                    stream.write(&response.encode()).unwrap();
                    stream.flush().unwrap();
                },
                Err(error) => {
                    error!("error --> {:?}", error);
                    stream.write(&error.encode()).unwrap();
                    stream.flush().unwrap();
                }
            }
        }
    }
}
