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
            self.process_command(&mut stream, data_table);
        }
    }

    fn process_command(&self, stream: &mut TcpStream, data_table: &mut DataTable) {
        let mut buffer = [0; 128];
        let payload_size = stream.read(&mut buffer).unwrap();

        if payload_size == 0 {
            return;
        }

        let request = &buffer[0..payload_size];
        let request_string = str::from_utf8(&request).unwrap();
        let parser = Parser::new(&request_string);
        let redis_value = parser.to_data_type().unwrap();

        println!("{:?}", request_string);

        match Command::build(redis_value).invoke(data_table) {
            Ok(response) => {
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
                // TODO: Maybe figure out how to make this an iterator instead of
                // using recursion.
                self.process_command(stream, data_table);
            }
            Err(_) => {
                stream.write(":1\r\n".as_bytes()).unwrap();
            }
        }

        println!("accepted incoming connection.");
        println!("Data table: ");
        println!("{:?}", data_table);
        println!("\n\n");
    }
}
