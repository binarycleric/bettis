use storage::{DataType, DataTable};

#[derive(Debug)]
enum Available {
    Select,
    Set,
    Get,
}

const SELECT_COMMAND: &'static str = "select";
const SET_COMMAND: &'static str = "set";
const GET_COMMAND: &'static str = "get";

impl Available {
    fn from_str<'a>(command: &'a str) -> Result<Available, &'static str> {
        match command {
            SELECT_COMMAND => Ok(Available::Select),
            SET_COMMAND => Ok(Available::Set),
            GET_COMMAND => Ok(Available::Get),
            _ => Err("Invalid redis command"),
        }
    }
}

#[derive(Debug)]
pub struct Command<'a> {
    command: Available,
    value: DataType<'a>,
}

impl<'a> Command<'a> {

    // TODO: This method is a total mess and I don't trust it at all.
    // Needs a pretty serious refactor.
    pub fn build(redis_value: DataType<'a>) -> Command {
        match redis_value {
            DataType::Array(array) => {
                if let DataType::BulkString(ref command_name) = array[0] {
                    let command = Available::from_str(command_name);

                    if let DataType::BulkString(ref value) = array[1] {
                        return Command {
                            command: command.unwrap(),
                            value: DataType::BulkString(value.to_string()),
                        }
                    } else {
                        panic!("Haven't figured this out yet");
                    }
                } else {
                    panic!("Haven't figured this out yet");
                }
            }
            _ => {
                panic!("Improperly formed request.")
            }
        }
    }

    pub fn invoke(&self, data_table: &mut DataTable) -> Result<&'static str, &'static str> {
        match self.command {
            Available::Select => {
                println!("Invoke select...");
                println!("--> {:?}", self.value);

                Ok("+OK\r\n")
            }
            Available::Set => {
                println!("Invoke set...");
                println!("--> {:?}", self.value);

                Ok("+OK\r\n")
            }
            Available::Get => {
                println!("Invoke get ...");
                println!("--> {:?}", self.value);

                // TODO: Figure out types and stuff.
                Ok("$2\r\n23\r\n")
            }
        }
    }
}
