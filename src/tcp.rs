extern crate resp;

use std::str;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Write;
use std::io::BufReader;

use storage::Database;
use command::run as run_command;

pub struct Listener<'a> {
    ipaddr: &'a str,
    port: &'a str,
}

impl<'a> Listener<'a> {
    pub fn new(ipaddr: &'a str, port: &'a str) -> Self {
        Self {
            ipaddr: ipaddr,
            port: port,
        }
    }

    pub fn start(&self, data_table: &mut Database) {
        match TcpListener::bind(self.connection_string()) {
            Ok(listener) => self.handle_incoming(listener, data_table),
            Err(err) => println!("{:?}", err),
        }
    }

    fn connection_string(&self) -> String {
        format!("{}:{}", self.ipaddr, self.port)
    }

    fn handle_incoming(&self, listener: TcpListener, data_table: &mut Database) {
        info!("listening started, ready to accept");

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut request = Request::new(&mut stream);

            request.run(data_table);
            debug!("data_table --> {:#?}", data_table);
        }
    }
}

struct Request<'tcp> {
    stream: &'tcp TcpStream,
}

impl<'tcp> Request<'tcp> {
    pub fn new(stream: &'tcp mut TcpStream) -> Self {
        Self { stream: stream }
    }

    pub fn run(&mut self, data_table: &mut Database) {
        loop {
            let mut buffer = vec![0; 128];
            let payload_size = self.stream.read(&mut buffer).unwrap();

            if payload_size == 0 {
                return;
            }

            let request = &buffer[0..payload_size];
            let reader = BufReader::new(request);

            match run_command(reader, data_table) {
                Ok(response) => self.write_response(response),
                Err(error) => {
                    println!("{:?}", error);
                    self.write_response(error);
                }
            }
        }
    }

    fn write_response(&mut self, response: resp::Value) {
        self.stream.write(&response.encode()).unwrap();
        self.stream.flush().unwrap();
    }
}
