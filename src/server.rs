extern crate resp;

use std::str;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Write;
use std::io::BufReader;

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
            port: port,
        }
    }

    pub fn start(&self) {
        let connection_string = format!("{}:{}", self.ipaddr, self.port);

        match TcpListener::bind(connection_string) {
            Ok(listener) => {
                info!("listening started, ready to accept");
                let mut database = Database::new();

                for stream in listener.incoming() {
                    debug!("stream --> {:#?}", stream);

                    let mut stream = stream.unwrap();

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

                        match command.run(&mut database) {
                            Ok(response) => {
                                stream.write(&response.encode()).unwrap();
                                stream.flush().unwrap();
                            },
                            Err(error) => {
                                println!("error --> {:?}", error);
                                stream.write(&error.encode()).unwrap();
                                stream.flush().unwrap();
                            }
                        }
                    }
                }
            },
            Err(err) => println!("{:?}", err),
        }
    }
}
