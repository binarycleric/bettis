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


/*

// TODO: Name this something else.
pub mod redis {
    pub mod types {

        #[derive(Debug, PartialEq)]
        struct RedisArray(Vec<Primitives>);

        #[derive(Debug, PartialEq)]
        struct RedisSimpleString(String);

        #[derive(Debug, PartialEq)]
        struct RedisBulkString(String);

        #[derive(Debug, PartialEq)]
        struct RedisInteger(i64);

        trait RedisPrimitive {}
        impl RedisPrimitive for RedisSimpleString {}
        impl RedisPrimitive for RedisBulkString {}

        impl ToString for RedisBulkString {
            fn to_string(&self) -> String {
                return "derp".to_string();
            }
        }

        impl ToString for RedisSimpleString {
            fn to_string(&self) -> String {
                return "derp".to_string();
            }
        }

        #[derive(Debug, PartialEq)]
        pub enum Primitives {
            Array(RedisArray),
            SimpleString(RedisSimpleString),
            BulkString(RedisBulkString),
            Int(RedisInteger),
        }

        #[derive(Debug, PartialEq)]
        pub struct RedisType {
            type_token: TypeTokens,
        }

        pub fn build_primitive(incoming: String) -> Box<RedisPrimitive> {
            let rtype = RedisType::from_char(incoming.chars().next().unwrap());
            let primitive = rtype.build_primitive(incoming);

            return primitive;
        }

        impl RedisType {

            // Need to double check that bulk strings actually work since thet
            // can be up to 512mb in length and include "\r\n" characters.
            pub fn build_primitive(&self, incoming: String) -> Box<RedisPrimitive> {
                // Simple string. This code sucks.
                if self.type_token == TypeTokens::SimpleString {
                    let mut bundle = incoming.split("\r\n");
                    let (_, simple_string) = bundle.next().unwrap().split_at(1);

                    return Box::new(RedisSimpleString(simple_string.to_string()));
                } else {
                    let mut bundle = incoming.split("\r\n");
                    let type_info = bundle.next().unwrap().to_string();
                    let (rtype, size) = type_info.split_at(1);
                    let data: Vec<String> = bundle.map(|s| s.to_string()).collect();

                    return Box::new(RedisBulkString(data.join("\r\n").to_string()));
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

                assert_eq!("OK".to_string(), primitive.borrow().to_string());
            }

            #[test]
            fn it_builds_incoming_from_bulk_string() {
                let request = "$6\r\nselect".to_string();
                let primitive = super::build_primitive(request);

                assert_eq!("select".to_string(), primitive.borrow().to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_builds_command() {
        let request = "$6\r\nselect".to_string();
        let redis_value = super::RedisValue::from_string(request);

        assert_eq!(false, true);
    }

    #[test]
    fn it_parses_redis_type() {
        let rtype = super::RedisTypes::parse_type('$');
        assert_eq!(rtype, Ok(super::RedisTypes::BulkString));

        let rtype = super::RedisTypes::parse_type('G');
        assert_eq!(rtype, Err("Invalid redis type"));
    }

    #[test]
    fn it_builds_redis_bulk_string() {
        let size = 6;
        let bulk_string = super::BulkString::build(size, "select".to_string());

        assert_eq!("select".to_string(), bulk_string.value);
    }

    #[test]
    fn it_truncates_excess_string_data() {
        let size = 6;
        let bulk_string = super::BulkString::build(size, "selectzzzzz".to_string());

        assert_eq!("select".to_string(), bulk_string.value);
    }
}

#[derive(Debug, PartialEq)]
enum RedisTypes {
    BulkString,
    SimpleString,
}

impl RedisTypes {
    fn parse_type(token: char) -> Result<RedisTypes, &'static str> {
        match token {
            '$' => Ok(RedisTypes::BulkString),
            _ => Err("Invalid redis type"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct BulkString {
    value: String,
}

impl BulkString {
    // Redis bulk strings have a set size.
    fn build(size: usize, raw: String) -> BulkString {
        let (bulk_string, _) = raw.split_at(size);

        return BulkString {
            value: bulk_string.to_string(),
        }
    }
}

struct SimpleString {
    value: String,
}

struct RedisValue<T> {
    rtype: RedisTypes,
    value: T,
}

impl RedisValue<SimpleString> {
    pub fn from_string(raw: String) -> RedisValue<SimpleString> {
        let (_, redis_value) = raw.split_at(1);

        return RedisValue {
            rtype: RedisTypes::SimpleString,
            value: SimpleString { value: redis_value.to_string() },
        }

    }
}

impl RedisValue<BulkString> {
    pub fn from_string(raw: String) -> RedisValue<BulkString> {
        let mut bundle = raw.split("\r\n");
        let (_, size) = bundle.next().unwrap().split_at(1);
        let size: usize = size.parse().unwrap();
        let string_data = bundle.next().unwrap();
        let redis_value = BulkString::build(size, string_data.to_string());

        return RedisValue {
            rtype: RedisTypes::BulkString,
            value: redis_value,
        }
    }
}

*/



pub mod rustis {

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_does_stuff() {
            let request = "$6\r\nselect".to_string();
            let value = super::Value::from_string(request);

            assert_eq!(true, false);
        }
    }

    enum SupportedType {
        SimpleString,
        BulkString,
    }

    struct Value<T> {
        rtype: SupportedType,
        data: String,
        typed_data: T,
    }

    impl Value<String> {
        fn from_string(incoming: String) -> Result<Value<String>, &'static str> {
            let type_token = incoming.chars().next();

            match type_token {
                Some('$') => {
                    let value = Value<String> {
                        rtype: SupportedType::SimpleString,
                        data: incoming,
                    };
                    return Ok(value);
                }
                _ => {
                    return Err("Sadface");
                }
            }
        }
    }
}



fn build_command(command: String) {
    println!("{:?}", command);

    match command.chars().next() {
        Some(token) => {
            match token {
                '$' => {
                    let mut bundle = command.split("\r\n");
                    let (_, size) = bundle.next().unwrap().split_at(1);

                    println!("{:?}", size);
                }
                _ => {

                }
            }
        },
        None => {
            // Bad!
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
