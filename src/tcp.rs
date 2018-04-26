use std::str;
use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

use storage::Database;
use commands::Command;

pub struct Listener<'a> {
    ipaddr: &'a str,
    port: &'a str,
}

impl<'a> Listener<'a> {
    pub fn new(ipaddr: &'a str, port: &'a str) -> Self {
        Self { ipaddr: ipaddr, port: port }
    }

    pub fn start(&self, data_table: &mut Database) {
        let connection_string = self.ipaddr.to_owned() + ":" + self.port;

        match TcpListener::bind(connection_string) {
            Ok(listener) => {
                println!("listening started, ready to accept");

                for stream in listener.incoming() {
                    let mut stream = stream.unwrap();
                    let mut request = Request::new(&mut stream);

                    request.run(data_table);
                    println!("data_table --> {:#?}", data_table);
                }
            }
            Err(err) => println!("{:?}", err),
        }
    }
}

struct CommandIterator<'s> {
    stream: &'s TcpStream,
}

impl<'s> Iterator for CommandIterator<'s> {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = vec![0; 128];
        let payload_size = self.stream.read(&mut buffer).unwrap();

        if payload_size == 0 {
            return None
        }

        let request = buffer[0..payload_size].to_vec();
        let request_string = String::from_utf8(request).unwrap();
        println!("request --> {:#?}\n\n", request_string);
        // "*2\r\n$3\r\nget\r\n$9\r\nincr-test\r\n"
        let command = Command::build(request_string);

        return Some(command);
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
        let command_iter = CommandIterator { stream: self.stream };

        for command in command_iter {
            match command.invoke(data_table) {
                Ok(response) => {
                    self.write_response(response);
                }
                Err(error) => {
                    self.write_response(error);
                }
            }
        }
    }

    fn write_response(&mut self, response: String) {
        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}
