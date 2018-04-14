use std::collections::HashMap;

#[derive(Debug)]
pub struct DataTable {
    database_id: u32,
    data_map: HashMap<String, String>,
}

impl DataTable {
    pub fn new(database_id: u32) -> DataTable {
        return DataTable {
            database_id: database_id,
            data_map: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_parses_bulk_string() {
        let request = "$6\r\nselect\r\n".to_string();
        let redis_value =  super::DataValue::new(request);

        assert_eq!("select".to_string(), redis_value.to_bulk_string());
    }

    #[test]
    fn it_parses_simple_string() {
        let request = "+Ok\r\n".to_string();
        let redis_value =  super::DataValue::new(request);

        assert_eq!("Ok".to_string(), redis_value.to_simple_string());
    }

    #[test]
    fn it_parses_array() {
        let request = "*2\r\n$6\r\nselect\r\n$2\r\n15\r\n".to_string();
        let redis_value = super::DataValue::new(request);
        let array = redis_value.to_array();

        assert_eq!(2, array.len());
        assert_eq!("select", array[0].to_bulk_string());
        assert_eq!("15", array[1].to_bulk_string());
    }
}

const REDIS_SEPARATOR: &'static str = "\r\n";

#[derive(Debug, PartialEq)]
enum ValidTypes {
    BulkString,
    SimpleString,
    Array,
}

#[derive(Debug)]
pub struct DataValue {
    rtype: ValidTypes,
    data: String, // Cause YOLO.
}

impl DataValue {
    pub fn new(value_string: String) -> DataValue {
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
                return DataValue {
                    rtype: ValidTypes::SimpleString,
                    data: value_string,
                }
            },
            '$' => {
                return DataValue {
                    rtype: ValidTypes::BulkString,
                    data: value_string,
                }
            }
            '*' => {
                return DataValue {
                    rtype: ValidTypes::Array,
                    data: value_string,
                }
            }
            _ => {
                panic!("Invalid type.")
            }
        }
    }

    // TODO: Memory usage is what?
    pub fn to_array(&self) -> Vec<DataValue> {
        let mut bundle = self.data.split(REDIS_SEPARATOR);
        let (rtype, size) = bundle.next().unwrap().split_at(1);
        let size = size.parse::<usize>().unwrap();
        let mut redis_array = Vec::with_capacity(size);

        // TODO handle size for bulk string.
        if rtype.chars().next().unwrap() != '*' {
            panic!("Not a bulk string");
        }

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

                        redis_array.push(DataValue::new(value_string))
                    } else {
                        let data = bundle.next();
                        let value_string = vec![type_token, data.unwrap()].join(REDIS_SEPARATOR);

                        redis_array.push(DataValue::new(value_string))
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

#[derive(Debug)]
enum AvailableCommands {
    Select,
    Set,
    Get,
}

impl AvailableCommands {
    // TODO: Automate this so I don't have to copy+paste a bunch of stuff.
    fn from_string(command: String) -> Result<AvailableCommands, &'static str> {
        if command == "select" {
            return Ok(AvailableCommands::Select);
        }

        if command == "set" {
            return Ok(AvailableCommands::Set);
        }

        if command == "get" {
            return Ok(AvailableCommands::Get);
        }

        return Err("Invalid redis command");
    }
}

#[derive(Debug)]
pub struct Command {
    command: AvailableCommands,
    value: DataValue,
}

impl Command {
    pub fn build(redis_value: DataValue) -> Command {
        if redis_value.rtype == ValidTypes::Array {
            let command_array = redis_value.to_array();
            let command_name = command_array[0].to_bulk_string();
            let command = AvailableCommands::from_string(command_name);
            let value: DataValue;

            match command {
                Ok(_) => {
                    // ARGH! Only getting the first value.
                    // let value_string = &command_array[1].data;
                    value = DataValue::new(redis_value.data.to_string());
                }
                Err(msg) => {
                    panic!(msg);
                }
            }

            return Command { command: command.unwrap(), value: value }
        }
        panic!("Improperly formed request.")
    }

    pub fn invoke(&self, data_table: &mut DataTable) -> Result<&'static str, &'static str> {
        match self.command {
            AvailableCommands::Select => {
                println!("Invoke select...");
                println!("{:?}", self.value);
            }
            AvailableCommands::Set => {
                let array = self.value.to_array();
                let key = &array[1];
                let value = &array[2];

                data_table.data_map.insert(
                    key.to_bulk_string().to_string(),
                    value.to_bulk_string().to_string()
                );

                println!("Invoke set...");
                println!("{:?}", array);
                println!("{:?} -- {:?}", key, value);
            }
            AvailableCommands::Get => {
                let array = self.value.to_array();
                let key = &array[1];
                let value = data_table.data_map.get(&key.to_bulk_string());

                println!("Invoke get ...");
                println!("{:?}", array);
                println!("{:?} -- {:?}", key, value);

                // TODO: Figure out types and stuff.
                return Ok("$2\r\n23\r\n");
            }
        }

        return Ok("+OK\r\n");
    }
}



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


