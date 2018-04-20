use types::DataType;
use storage::DataTable;

const SELECT_COMMAND: &'static str = "select";
const SET_COMMAND: &'static str = "set";
const GET_COMMAND: &'static str = "get";

#[derive(Debug, PartialEq)]
enum Available {
    Select,
    Set,
    Get,
}

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
pub struct Command {
    value: Vec<DataType>,
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

impl Command {
    // TODO: This method is a total mess and I don't trust it at all.
    // Needs a pretty serious refactor.
    pub fn new(redis_value: DataType) -> Command {
        match redis_value {
            DataType::Array(array) => {
                Command { value: array.to_vec() }
            }
            _ => panic!("Improperly formed request."),
        }
    }

    fn get_command(&self) -> Available {
        if let DataType::BulkString(ref value) = self.value[0] {
            return Available::from_str(value).unwrap();
        }
        panic!("Invalid command");
    }

    pub fn invoke(&self, data_table: &mut DataTable) -> Result<&'static str, &'static str> {
        match self.get_command() {
            Available::Select => {
                println!("Invoke select...");
                println!("VALUE --> {:?}", self.value);

                Ok("+OK\r\n")
            }
            Available::Set => {
                println!("Invoke set...");
                println!("VALUE --> {:?}", self.value);

                if let DataType::BulkString(dk) = self.value[1].clone() {
                    data_table.set(&dk, self.value[2].clone());
                }

                Ok("+OK\r\n")
            }
            Available::Get => {
                println!("Invoke get ...");
                println!("VALUE --> {:?}", self.value);

                // TODO: Figure out types and stuff.
                Ok("$2\r\n23\r\n")
            }
        }
    }
}
