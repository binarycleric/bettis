use std::str;
use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
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




#[derive(Debug, PartialEq)]
enum RedisTypes {
    RedisBulkString,
    RedisSimpleString,
    RedisArray,
}

#[derive(Debug)]
struct RedisValue {
    rtype: RedisTypes,
    data: String, // Cause YOLO.
}

const REDIS_SEPARATOR: &'static str = "\r\n";

impl RedisValue {
    pub fn new(value_string: String) -> RedisValue {
        let type_token: char;

        match value_string.chars().next() {
            Some(value) => {
                type_token = value
            }
            None => {
                println!("invalid: {:?}", value_string);
                panic!("Invalid redis value.");
            }
        }

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

#[derive(Debug)]
enum RedisAvailableCommand {
    Select,
    Set,
    Get,
}

impl RedisAvailableCommand {
    fn from_string(command: String) -> Result<RedisAvailableCommand, &'static str> {
        if command == "select" {
            return Ok(RedisAvailableCommand::Select);
        }

        if command == "set" {
            return Ok(RedisAvailableCommand::Set);
        }

        if command == "get" {
            return Ok(RedisAvailableCommand::Get);
        }

        return Err("Invalid redis command");
    }
}

#[derive(Debug)]
struct RedisCommand {
    command: RedisAvailableCommand,
    value: RedisValue,
}

impl RedisCommand {
    fn build(command: RedisValue) -> RedisCommand {
        if command.rtype == RedisTypes::RedisArray {
            let command_array = command.to_array();
            let command_name = command_array[0].to_bulk_string();
            let command = RedisAvailableCommand::from_string(command_name);
            let value: RedisValue;

            match command {
                Ok(_) => {
                    let value_string = &command_array[1].data;
                    value = RedisValue::new(value_string.to_string());
                }
                Err(msg) => {
                    panic!(msg);
                }
            }

            return RedisCommand { command: command.unwrap(), value: value }
        }
        panic!("Improperly formed request.")
    }

    fn invoke(&self, stream: &mut TcpStream) -> Result<String, &'static str> {
        return Ok("+OK\r\n".to_string());
    }
}

fn process_command(stream: &mut TcpStream, data_table: &mut HashMap<String, RedisValue>) {
    let mut buffer = vec![0; 128];
    let payload_size = stream.read(&mut buffer).unwrap();

    if payload_size == 0 {
        return;
    }

    buffer.truncate(payload_size);
    let request_string = str::from_utf8(&buffer).unwrap().to_string();
    println!("{:?}", request_string);

    let redis_value = RedisValue::new(request_string);
    match RedisCommand::build(redis_value).invoke(stream) {
        Ok(response) => {
            stream.write(response.as_bytes()).unwrap();
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

    for (key, value) in data_table {
        println!("{:?} --- {:?}", key, value);
    }

    println!("\n\n");
}

fn main() {
    let mut data_table = HashMap::new();
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    println!("listening started, ready to accept");

    for stream in listener.incoming() {
        process_command(&mut stream.unwrap(), &mut data_table);
    }
}

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
