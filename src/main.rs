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
        pub struct RString(String);

        #[derive(Debug, PartialEq)]
        pub struct RInt(i32);

        #[derive(Debug, PartialEq)]
        pub struct RArray(Vec<Primitives>);

        #[derive(Debug, PartialEq)]
        pub enum Primitives {
            Array(RArray),
            String(RString),
            Int(RInt),
        }

        impl RString {
            // $6\r\nselect
            // TODO: Return result instead.
            fn from_vector(raw: Vec<String>) -> RString {
                let (_, size) = raw.first().unwrap().split_at(1);
                let size = size.parse::<usize>().unwrap();
                let mut value = raw.last().unwrap().to_string();

                value.truncate(size);

                return RString(value);
            }
        }

        enum TypeTokens {
            BulkString,
            Integer,
            Array,
        }

        impl TypeTokens {
            fn to_type_token(token: String) -> Option<TypeTokens> {
                match token.as_str() {
                    "$" => Some(TypeTokens::BulkString),
                    ":" => Some(TypeTokens::Integer),
                    "*" => Some(TypeTokens::Array),
                    _ => None,
                }
            }

            fn value(&self) -> char {
                match *self {
                    TypeTokens::BulkString => '$',
                    TypeTokens::Integer => ':',
                    TypeTokens::Array => '*',
                }
            }
        }

        struct Incoming {
            token: TypeTokens,
            size: usize,
            data: String,
        }

        impl Incoming {
            fn from_string(incoming: String) -> Incoming {
                let mut bundle = incoming.split("\r\n");
                let type_info = bundle.next().unwrap().to_string();
                let (rtype, size) = type_info.split_at(1);
                let data: Vec<String> = bundle.map(|s| s.to_string()).collect();

                return Incoming {
                    token: TypeTokens::to_type_token(rtype.to_string()).unwrap(),
                    size: size.parse::<usize>().unwrap(),
                    data: data.join("\r\n"),
                }
            }
        }


        pub fn build_redis_value(raw: String) -> Primitives {
            let mut bundle = raw.split("\r\n");
            let type_info = bundle.next().unwrap().to_string();
            let (rtype, _) = type_info.split_at(1);

            match rtype {
                "$" => {
                    let rstring_raw = bundle.next().unwrap().to_string();
                    let rstring_data = vec![type_info.clone(), rstring_raw];
                    let rstring = RString::from_vector(rstring_data);

                    return Primitives::String(rstring);
                }
                _ => {
                    return Primitives::String(RString("FAIL".to_string()));
                }
            }
        }


        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn it_builds_redis_string() {
                let expected = "select".to_string();
                let raw = vec!["$6".to_string(), expected.clone()];
                let rstring = super::RString::from_vector(raw);

                assert_eq!(rstring, RString(expected.clone()));
            }

            #[test]
            fn it_builds_redis_string_with_multi_digit_length() {
                let expected = "AAAAAAAAAAA".to_string();
                let raw = vec!["$11".to_string(), expected.clone()];
                let rstring = super::RString::from_vector(raw);

                assert_eq!(rstring, RString(expected.clone()));
            }

            #[test]
            fn it_builds_redis_string_discarding_extra_values() {
                let expected = "select_zzzzz".to_string();
                let raw = vec!["$16".to_string(), expected.clone()];
                let rstring = super::RString::from_vector(raw);

                assert_eq!(rstring, RString(expected.clone()));
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
