use std::str;
use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn process_request(stream: &mut TcpStream) {
    let mut buffer = [0; 128];

    let payload_size = stream.read(&mut buffer).unwrap();
    let request_string = str::from_utf8(&buffer).unwrap();

    let mut arguments = request_string.split_whitespace();

    match arguments.next().unwrap() {
        "GET" => {
            println!("GET command");
        },
        "SET" => {
            println!("SET command");            
        }
        _ => {
            println!("Unknown command.")
        }
    }

    println!("accepted incoming connection.");
    println!("Size: {}", payload_size);
    println!("Request: {}", request_string);
    println!("\n\n");

    stream.write(b"Done.\r\n").unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("listening started, ready to accept");

    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            process_request(&mut stream);
        });
    }
}
