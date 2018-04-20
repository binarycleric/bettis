use std::str;
use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

use storage::DataTable;
use parser::Parser;
use types::DataType;
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

    pub fn start(&self, data_table: &mut DataTable) {
        let connection_string = self.ipaddr.to_owned() + ":" + self.port;
        let listener = TcpListener::bind(connection_string);

        match listener {
            Ok(listener) => {
                println!("listening started, ready to accept");
                self.dispatch(listener, data_table);
            }
            Err(err) => println!("{:?}", err),
        }
    }

    fn dispatch(&self, listener: TcpListener, data_table: &mut DataTable) {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut request = RequestHandler::new(&mut stream);

            request.run(data_table);

            println!("data_table --> {:?}", data_table);
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

    fn write_error(&mut self) {
        self.stream.write(":1\r\n".as_bytes()).unwrap();
    }

    fn write_response(&mut self, response: &'static str) {
        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    // TODO error handling for invalid types.
    fn parse_to_value(&self, request: Vec<u8>) -> DataType {
        let request_string = String::from_utf8(request).unwrap();
        let parser = Parser::new(request_string);
        let redis_value = parser.to_data_type().unwrap();

        return redis_value;
    }

    pub fn run(&mut self, data_table: &mut DataTable) {
        let mut buffer = vec![0; 128];
        let payload_size = self.stream.read(&mut buffer).unwrap();

        if payload_size == 0 {
            return;
        }

        let request = buffer[0..payload_size].to_vec();
        let redis_value = self.parse_to_value(request);
        let command = Command::new(redis_value);

        match command.invoke(data_table) {
            Ok(response) => {
                self.write_response(response);
                self.run(data_table);
            }
            Err(_) => {
                self.write_error();
            }
        }

        println!("accepted incoming connection.");
        println!("\n\n");
    }
}
