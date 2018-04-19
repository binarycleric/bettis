use std::str;
use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

use storage::DataTable;
use parser::Parser;
use commands::Command;

pub struct Server<'a> {
    ipaddr: &'a str,
    port: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(ipaddr: &'a str, port: &'a str) -> Server<'a> {
        return Server {
            ipaddr: ipaddr,
            port: port,
        };
    }

    pub fn start(&self) {
        let connection_string = self.ipaddr.to_owned() + ":" + self.port;
        let listener = TcpListener::bind(connection_string);

        match listener {
            Ok(listener) => {
                println!("listening started, ready to accept");
                self.dispatch(listener);
            }
            Err(err) => println!("{:?}", err),
        }
    }

    fn dispatch(&self, listener: TcpListener) {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut request = RequestHandler::new(&mut stream);

            request.run();
        }
    }
}

struct RequestHandler<'tcp> {
    stream: &'tcp TcpStream,
}

impl<'tcp> RequestHandler<'tcp> {
    pub fn new(stream: &'tcp mut TcpStream) -> RequestHandler<'tcp> {
        RequestHandler {
            stream: stream,
        }
    }

    pub fn run(&mut self) {
        let mut buffer = vec![0; 128];
        let payload_size = self.stream.read(&mut buffer).unwrap();

        if payload_size == 0 {
            return;
        }

        let request = buffer[0..payload_size].to_vec();
        let request_string = str::from_utf8(&request).unwrap();
        let parser = Parser::new(&request_string);
        let redis_value = parser.to_data_type().unwrap();

        match Command::build(redis_value).invoke() {
            Ok(response) => {
                self.stream.write(response.as_bytes()).unwrap();
                self.stream.flush().unwrap();
                self.run();
            }
            Err(_) => {
                self.stream.write(":1\r\n".as_bytes()).unwrap();
            }
        }

        println!("accepted incoming connection.");
        println!("\n\n");
    }
}
