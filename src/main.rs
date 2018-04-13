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




#[derive(Debug, PartialEq, Copy)]
enum RedisTypes {
    RedisBulkString,
    RedisSimpleString,
    RedisArray,
}

#[derive(Debug, Copy)]
struct RedisValue {
    rtype: RedisTypes,
    data: String, // Cause YOLO.
}

const REDIS_SEPARATOR: &'static str = "\r\n";

impl RedisValue {
    pub fn new(value_string: String) -> RedisValue {
        let type_token = value_string.chars().next().unwrap();

        match type_token {
            '+' => {
                return RedisValue {
                    rtype: RedisTypes::RedisSimpleString,
                    data: value_string,
                }
            },
            '$' => {
                return RedisValue {
                    rtype: RedisTypes::RedisBulkString,
                    data: value_string,
                }
            }
            '*' => {
                return RedisValue {
                    rtype: RedisTypes::RedisArray,
                    data: value_string,
                }
            }
            _ => {
                panic!("Invalid type.")
            }
        }
    }

    // TODO: Memory usage is what?
    pub fn to_array(&self) -> Vec<RedisValue> {
        let mut bundle = self.data.split(REDIS_SEPARATOR);
        let (rtype, size) = bundle.next().unwrap().split_at(1);
        let size = size.parse::<usize>().unwrap();
        let mut redis_array = Vec::with_capacity(size);

        loop {
            match bundle.next() {
                Some(type_token) => {
                    // Why do we need this?
                    // Figure out why it is adding an empty atring at the end.
                    // ["*2", "$6", "select", "$2", "15", ""]
                    if type_token.len() == 0 {
                        break;
                    }

                    if type_token.chars().next().unwrap() == '+' {
                        let value_string = type_token.to_string();

                        redis_array.push(RedisValue::new(value_string))
                    } else {
                        let data = bundle.next();
                        let value_string = vec![type_token, data.unwrap()].join(REDIS_SEPARATOR);

                        redis_array.push(RedisValue::new(value_string))
                    }
                }
                None => { break }
            }
        }

        return redis_array;
    }

    // TODO: Error handling.
    pub fn to_bulk_string(&self) -> String {
        let mut bundle = self.data.split(REDIS_SEPARATOR);
        let (rtype, size) = bundle.next().unwrap().split_at(1);
        let size = size.parse::<usize>().unwrap();
        let (bulk_string, _) = bundle.next().unwrap().split_at(size);

        // TODO handle size for bulk string.
        if rtype.chars().next().unwrap() == '$' {
            return bulk_string.to_string();
        }
        panic!("Not a bulk string");
    }

    // TODO: Error handling.
    pub fn to_simple_string(&self) -> String {
        let (type_token, data) = self.data.split(REDIS_SEPARATOR).next().unwrap().split_at(1);
        if type_token.chars().next().unwrap() == '+' {
            return data.to_string();
        }
        panic!("Not a simple string");
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn it_parses_bulk_string() {
        let request = "$6\r\nselect\r\n".to_string();
        let redis_value =  super::RedisValue::new(request);

        assert_eq!("select".to_string(), redis_value.to_bulk_string());
    }

    #[test]
    fn it_parses_simple_string() {
        let request = "+Ok\r\n".to_string();
        let redis_value =  super::RedisValue::new(request);

        assert_eq!("Ok".to_string(), redis_value.to_simple_string());
    }

    #[test]
    fn it_parses_array() {
        let request = "*2\r\n$6\r\nselect\r\n$2\r\n15\r\n".to_string();
        let redis_value = super::RedisValue::new(request);
        let array = redis_value.to_array();

        assert_eq!(2, array.len());
        assert_eq!("select", array[0].to_bulk_string());
        assert_eq!("15", array[1].to_bulk_string());
    }
}







struct RedisCommand {
    command: String,
    value: RedisValue,
}



fn build_command(command: RedisValue) -> RedisCommand {
    if command.rtype == RedisTypes::RedisArray {
        let command_name = command.to_array()[0].to_bulk_string();
        if command_name == "select".to_string() {
            let database_id = command.to_array()[1].clone();

            return RedisCommand {
                command: "select".to_string(),
                value: database_id
            }
        }
    }
    panic!("Invalid redis command.")
}

fn process_request(stream: &mut TcpStream, data_table: &mut HashMap<String, i32>) {
    let mut buffer = vec![0; 128];
    let payload_size = stream.read(&mut buffer).unwrap();

    buffer.truncate(payload_size);

    let request_string = str::from_utf8(&buffer).unwrap().to_string();
    let redis_value = RedisValue::new(request_string);

    println!("{:?}", redis_value);

    build_command(redis_value);



/*
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
*/

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
