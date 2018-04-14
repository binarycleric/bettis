extern crate quarrie;

use std::str;
use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

struct Server<'a> {
    ipaddr: &'a str,
    port: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(ipaddr: &'a str, port: &'a str) -> Server<'a> {
        return Server {
            ipaddr: ipaddr,
            port: port,
        }
    }

    pub fn start(&self) {
        // TEMP
        let mut data_table = quarrie::DataTable::new(15);

        let connection_string = self.ipaddr.to_owned() + ":" + self.port;
        let listener = TcpListener::bind(connection_string);

        match listener {
            Ok(l) => {
                println!("listening started, ready to accept");

                for stream in l.incoming() {
                    process_command(&mut stream.unwrap(), &mut data_table);
                }
            },
            Err(err) => {
                println!("{:?}", err)
            }
        }
    }
}


fn process_command(stream: &mut TcpStream, data_table: &mut quarrie::DataTable) {
    let mut buffer = vec![0; 128];
    let payload_size = stream.read(&mut buffer).unwrap();

    if payload_size == 0 {
        return;
    }

    buffer.truncate(payload_size);
    let request_string = str::from_utf8(&buffer).unwrap().to_string();
    println!("{:?}", request_string);

    let redis_value = quarrie::DataValue::new(request_string);
    match quarrie::Command::build(redis_value).invoke(data_table) {
        Ok(response) => {
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            // TODO: Maybe figure out how to make this an iterator instead of
            // using recursion.
            process_command(stream, data_table);
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

fn main() {
    let server = Server::new("127.0.0.1", "6379");

    server.start();
}