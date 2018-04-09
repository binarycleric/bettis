use std::str;
use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::FromStr;
use std::collections::HashMap;

fn process_request(stream: &mut TcpStream, data_table: &mut HashMap<String, i32>) {
    let mut buffer = [0; 128];

    let payload_size = stream.read(&mut buffer).unwrap();
    let request_string = str::from_utf8(&buffer).unwrap();

    let mut arguments = request_string.split_whitespace();
    let command = arguments.next().unwrap();

    match command {
        "GET" => {
            let identifier = arguments.next().unwrap();
            let value = data_table.get(identifier);

            match value {
                Some(v) => {
                    let rsp = format!("{}\n{}", identifier, v);
                    let _ = stream.write(rsp.as_bytes());
                },
                None => {
                    let rsp = format!("{}\n(null)", identifier);
                    let _ = stream.write(rsp.as_bytes());
                }
            }

            println!("GET command");
        },
        "SET" => {
            let identifier = arguments.next().unwrap();
            let raw_value = arguments.next().unwrap();
            let typed_value = i32::from_str(raw_value).unwrap();

            data_table.insert(String::from(identifier), typed_value);

            println!("SET command");            
        }
        _ => {
            println!("Unknown command.")
        }
    }

    println!("accepted incoming connection.");
    println!("Size: {}", payload_size);
    println!("Request: {}", request_string);

    println!("Data table: ");

    for (key, value) in data_table {
        println!("{} --- {}", key, value);        
    }

    println!("\n\n");
}

fn main() {
    let mut data_table = HashMap::new();
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("listening started, ready to accept");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        process_request(&mut stream, &mut data_table);
    }
}
