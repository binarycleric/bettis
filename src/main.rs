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




// TODO: Name this something else.
pub mod redis {
    pub mod types {
        #[derive(Debug, PartialEq)]
        pub enum Primitives {
            Array(Vec<Primitives>),
            String(String),
            Int(i64),
        }

        #[derive(Debug, PartialEq)]
        pub struct RedisType {
            type_token: TypeTokens,
        }

        pub fn build_primitive(incoming: String) -> Primitives {
            let rtype = RedisType::from_char(incoming.chars().next().unwrap());
            let primitive = rtype.build_primitive(incoming);

            return primitive;
        }

        impl RedisType {

            // Need to double check that bulk strings actually work since thet
            // can be up to 512mb in length and include "\r\n" characters.
            pub fn build_primitive(&self, incoming: String) -> Primitives {
                // Simple string. This code sucks.
                if self.type_token == TypeTokens::SimpleString {
                    let mut bundle = incoming.split("\r\n");
                    let (_, simple_string) = bundle.next().unwrap().split_at(1);

                    return Primitives::String(simple_string.to_string());
                } else {
                    let mut bundle = incoming.split("\r\n");
                    let type_info = bundle.next().unwrap().to_string();
                    let (rtype, size) = type_info.split_at(1);
                    let data: Vec<String> = bundle.map(|s| s.to_string()).collect();

                    return Primitives::String(data.join("\r\n").to_string());
                }
            }

            // TODO: Actual error handling.
            pub fn from_char(token: char) -> RedisType {
                let type_token = TypeTokens::from_string(token).unwrap();

                return RedisType {
                    type_token: type_token,
                }
            }
        }

        #[derive(Debug, PartialEq)]
        enum TypeTokens {
            SimpleString,
            BulkString,
            Integer,
            Array,
        }

        impl TypeTokens {
            fn from_string(token: char) -> Option<TypeTokens> {
                match token {
                    '+' => Some(TypeTokens::SimpleString),
                    '$' => Some(TypeTokens::BulkString),
                    ':' => Some(TypeTokens::Integer),
                    '*' => Some(TypeTokens::Array),
                    _ => None,
                }
            }
        }

        #[cfg(test)]
        mod tests {

            #[test]
            fn it_builds_incoming_from_simple_string() {
                let request = "+OK\r\n".to_string();
                let primitive = super::build_primitive(request);
                let expected_primitive = super::Primitives::String("OK".to_string());

                assert_eq!(expected_primitive, primitive);
            }

            #[test]
            fn it_builds_incoming_from_bulk_string() {
                let request = "$6\r\nselect".to_string();
                let primitive = super::build_primitive(request);
                let expected_primitive = super::Primitives::String("select".to_string());

                assert_eq!(expected_primitive, primitive);
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
