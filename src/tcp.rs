use std::str;
use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

use storage::Database;

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
                self.dispatch(listener, data_table);
            }
            Err(err) => println!("{:?}", err),
        }
    }

    fn dispatch(&self, listener: TcpListener, data_table: &mut Database) {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut request = RequestHandler::new(&mut stream);

            request.run(data_table);
            println!("data_table --> {:#?}", data_table);
        }
    }
}

struct RequestHandler<'tcp> {
    stream: &'tcp TcpStream,
}

impl<'tcp> RequestHandler<'tcp> {
    pub fn new(stream: &'tcp mut TcpStream) -> Self {
        Self { stream: stream }
    }

    fn write_error(&mut self) {
        self.stream.write(":1\r\n".as_bytes()).unwrap();
    }

    fn write_response(&mut self, response: String) {
        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn run(&mut self, data_table: &mut Database) {
        let mut buffer = vec![0; 128];
        let payload_size = self.stream.read(&mut buffer).unwrap();

        if payload_size == 0 {
            return;
        }

        let request = buffer[0..payload_size].to_vec();
        let request_string = String::from_utf8(request).unwrap();

        // "*2\r\n$3\r\nget\r\n$9\r\nincr-test\r\n"

        println!("request --> {:#?}\n\n", request_string);
        // let command = Command::new(redis_value);

        self.write_response("+OK\r\n".to_string());
        self.run(data_table);

/*
        match command.invoke(data_table) {
            Ok(response) => {
                self.write_response(response);
                self.run(data_table);
            }
            Err(_) => {
                self.write_error();
            }
        }
*/
    }
}
