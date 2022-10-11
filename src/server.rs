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

              for stream in listener.incoming() {
                  debug!("stream --> {:#?}", stream);

                  let mut stream = stream.unwrap();
                  let mut request = Request::new(&mut stream);

                  request.run();
              }
            },
            Err(err) => println!("{:?}", err),
        }
    }
}

#[derive(Debug)]
struct Request<'tcp> {
    stream: &'tcp TcpStream,
}

impl<'tcp> Request<'tcp> {
    pub fn new(stream: &'tcp mut TcpStream) -> Self {
        Self { stream: stream }
    }

    pub fn run(&mut self) {
        let mut database = Database::new(12);

        loop {
            let mut buffer = vec![0; 128];
            let payload_size = self.stream.read(&mut buffer).unwrap();

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
              Ok(response) => self.write_response(response),
              Err(error) => {
                  println!("{:?}", error);
                  self.write_response(error);
              }
            }
        }

        debug!("database --> {:#?}", database);
    }

    fn write_response(&mut self, response: resp::Value) {
        self.stream.write(&response.encode()).unwrap();
        self.stream.flush().unwrap();
    }
}
