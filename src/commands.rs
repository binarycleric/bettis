use types::DataType;
use storage::Database;

const SELECT_COMMAND: &'static str = "select";
const SET_COMMAND: &'static str = "set";
const GET_COMMAND: &'static str = "get";
const INCR_COMMAND: &'static str = "incr";
const DECR_COMMAND: &'static str = "decr";
const DEL_COMMAND: &'static str = "del";

#[derive(Debug, PartialEq)]
enum Available {
    Select,
    Set,
    Get,
    Incr,
    Decr,
    Del,
}

impl Available {
    fn from_str<'a>(command: &'a str) -> Result<Available, &'static str> {
        match command {
            SELECT_COMMAND => Ok(Available::Select),
            SET_COMMAND => Ok(Available::Set),
            GET_COMMAND => Ok(Available::Get),
            INCR_COMMAND => Ok(Available::Incr),
            DECR_COMMAND => Ok(Available::Decr),
            DEL_COMMAND => Ok(Available::Del),
            _ => Err("Invalid redis command"),
        }
    }
}

#[derive(Debug)]
pub struct Command {
    value: Vec<DataType>,
}

impl Command {
    pub fn new(redis_value: DataType) -> Command {
        match redis_value {
            DataType::Array(array) => Command {
                value: array.to_vec(),
            },
            _ => panic!("Improperly formed request."),
        }
    }

    fn get_command(&self) -> Available {
        if let DataType::BulkString(ref value) = self.value[0] {
            return Available::from_str(value).unwrap();
        }
        panic!("Invalid command");
    }

    pub fn invoke(&self, data_table: &mut Database) -> Result<String, &'static str> {
        match self.get_command() {
            Available::Select => {
                println!("Invoke select...");
                println!("VALUE --> {:?}", self.value);

                Ok("+OK\r\n".to_string())
            }
            Available::Del => {
                println!("Invoke del...");
                println!("VALUE --> {:?}", self.value);

                match self.value[1].clone() {
                    DataType::BulkString(dk) => {
                        data_table.del(&dk);
                        Ok("+OK\r\n".to_string())
                    }
                    _ => {
                        println!("Something bad happened: {:?}", self.value);
                        Err("Something bad happened")
                    }
                }
            },
            Available::Decr => {
                println!("Invoke decr...");
                println!("VALUE --> {:?}", self.value);

                match self.value[1].clone() {
                    DataType::BulkString(dk) => {
                        data_table.decr(&dk);
                        Ok("+OK\r\n".to_string())
                    }
                    _ => {
                        println!("Something bad happened: {:?}", self.value);
                        Err("Something bad happened")
                    }
                }
            }
            Available::Incr => {
                println!("Invoke incr...");
                println!("VALUE --> {:?}", self.value);

                match self.value[1].clone() {
                    DataType::BulkString(dk) => {
                        data_table.incr(&dk);
                        Ok("+OK\r\n".to_string())
                    }
                    _ => {
                        println!("Something bad happened: {:?}", self.value);
                        Err("Something bad happened")
                    }
                }
            }
            Available::Set => {
                println!("Invoke set...");
                println!("VALUE --> {:?}", self.value);

                match self.value[1].clone() {
                    DataType::BulkString(dk) => {
                        data_table.set(&dk, self.value[2].clone());

                        Ok("+OK\r\n".to_string())
                    }
                    _ => {
                        println!("Something bad happened: {:?}", self.value);
                        Err("Something bad happened")
                    }
                }
            }
            Available::Get => {
                println!("Invoke get ...");
                println!("VALUE --> {:?}", self.value);

                if let DataType::BulkString(dk) = self.value[1].clone() {
                    match data_table.get(&dk) {
                        Some(value) => return Ok(value.to_redis_protocol()),
                        None => return Err("-MalformedValue\r\n"),
                    }
                }
                println!("Something bad happened: {:?}", self.value);
                Err("Something bad happened")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Available;

    #[test]
    fn it_parses_command_from_string() {
        assert_eq!(Ok(Available::Select), Available::from_str("select"));
        assert_eq!(Ok(Available::Set), Available::from_str("set"));
        assert_eq!(Ok(Available::Get), Available::from_str("get"));
    }
}