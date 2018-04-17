use storage::{DataType, DataTable};

#[derive(Debug)]
enum Available {
    Select,
    Set,
    Get,
}

impl Available {
    // TODO: Automate this so I don't have to copy+paste a bunch of stuff.
    fn from_string(command: String) -> Result<Available, &'static str> {
        if command == "select" {
            return Ok(Available::Select);
        }

        if command == "set" {
            return Ok(Available::Set);
        }

        if command == "get" {
            return Ok(Available::Get);
        }

        return Err("Invalid redis command");
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
                    let command = Available::from_string(command_name.to_string());

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

        println!("{:?}", self.command);
        return Ok("+OK\r\n");

/*
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
                    value.to_bulk_string().to_string(),
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
*/
    }
}
