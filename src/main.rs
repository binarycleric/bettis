use std::str;
use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::FromStr;
use std::collections::HashMap;

// TODO: Redis protocol parser.
// https://redis.io/topics/protocol
// TODO: Need to keep reading from the stream until the connection is closed.
/*

    For Simple Strings the first byte of the reply is "+"
    For Errors the first byte of the reply is "-"
    For Integers the first byte of the reply is ":"
    For Bulk Strings the first byte of the reply is "$"
    For Arrays the first byte of the reply is "*"


            *2\r\n$6\r\nselect\r\n$2\r\n15\r\n
            [ select 15 ]



let command_string = str::from_utf8(&buffer).unwrap();
let mut command_list = command_string.split("\r\n");

println!("{:?}", command_list);

let command_size = command_list.next().unwrap();
println!("{:?}", command_size);



*/

pub mod commands {
    #[cfg(test)]
    mod tests {

        #[test]
        fn it_returns_basic_string_values() {
            assert_eq!(super::build_value(String::from("$1\r\n1\r\n")), "1");
            assert_eq!(super::build_value(String::from("$2\r\n15\r\n")), "15");
            assert_eq!(super::build_value(String::from("$3\r\n152\r\n")), "152");
        }

        #[test]
        fn it_ignores_garbage_values() {
            assert_eq!(super::build_value(String::from("$1\r\n153\r\n")), "1");
            assert_eq!(super::build_value(String::from("$2\r\n153\r\n")), "15");
        }
    }

    pub fn build_value(payload: String) -> String {
        let mut value_tokens = payload.split("\r\n");
        let mut type_info = value_tokens.next().unwrap().chars();

        // ["$2", "15"]

        match type_info.next().unwrap() {
            '$' => {
                let value_length = type_info.as_str().parse::<usize>().unwrap();
                let mut value = String::from(value_tokens.next().unwrap());

                value.truncate(value_length);

                return value;
            },
            _ => {
                // Do nothing.
                return String::from("999");
            }
        }
    }
}

fn build_command(command: String) {
    println!("{:?}", command);

    let mut command_args = command.split("\r\n");

    match command_args.next() {
        Some(v) => {
            println!("{:?}", v);
        }
        None => {
            // Nothing.
        }
    }
}

fn process_request(stream: &mut TcpStream, data_table: &mut HashMap<String, i32>) {
    let mut buffer = vec![0; 128];
    let payload_size = stream.read(&mut buffer).unwrap();

    buffer.truncate(payload_size);
    let command_string = str::from_utf8(&buffer).unwrap().to_string();

    build_command(command_string);

    let mut arguments = str::from_utf8(&buffer).unwrap().split_whitespace();
    let command = arguments.next().unwrap();

    match command {
        "GET" => {
            let identifier = arguments.next().unwrap();
            let value = data_table.get(identifier);

            match value {
                Some(v) => {
                    let rsp = format!("{}\n{}", identifier, v);
                    let _ = stream.write(rsp.as_bytes());
                }
                None => {
                    let rsp = format!("{}\n(null)", identifier);
                    let _ = stream.write(rsp.as_bytes());
                }
            }
        }
        "INCR" => {
            let identifier = arguments.next().unwrap();

            if data_table.contains_key(identifier) == false {
                data_table.insert(identifier.to_string(), 0);
            }
            if let Some(value) = data_table.get_mut(identifier) {
                *value = *value + 1;
            }
        }
        "SET" => {
            let identifier = arguments.next().unwrap();
            let raw_value = arguments.next().unwrap();
            let typed_value = i32::from_str(raw_value).unwrap();

            data_table.insert(String::from(identifier), typed_value);
        }
        "FLUSHDB" => {
            data_table.clear();
            println!("Keys have been flushed.");
        }
        _ => println!("Unknown command."),
    }

    println!("accepted incoming connection.");
    println!("Data table: ");

    for (key, value) in data_table {
        println!("{} --- {}", key, value);
    }

    println!("\n\n");

    stream.write("+OK\r\n".as_bytes()).unwrap();
}

fn main() {
    let mut data_table = HashMap::new();
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    println!("listening started, ready to accept");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        process_request(&mut stream, &mut data_table);
    }
}
