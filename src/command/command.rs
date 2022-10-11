extern crate resp;

use std::io::BufReader;
use self::resp::{Decoder, Value};

use super::Runnable;
use super::commands::Available as AvailableCommand;

use storage::Database;

#[derive(Debug)]
pub struct Command {
    command_name: AvailableCommand,
    values: Vec<resp::Value>,
}

impl Command {
    pub fn new(incoming: resp::Value) -> Self {
        let (command_name, values) = Self::extract_values(incoming);

        Self {
            command_name: command_name,
            values: values,
        }
    }

    fn extract_values(incoming: resp::Value) -> (AvailableCommand, Vec<resp::Value>) {
        if let Value::Array(ref array) = incoming {
            let command_name: AvailableCommand;
            let values: Vec<resp::Value>;

            if let Value::Bulk(ref cname) = array[0] {
                command_name = AvailableCommand::from_str(String::from(cname));
            } else {
                panic!("This shouldn't happen");
            }

            values = array.get(1..).unwrap().clone().to_vec();

            return (command_name, values);
        }
        panic!("This shouldn't happen");
    }

    pub fn run(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        let values = self.values.clone().to_vec();

        match self.command_name {
            AvailableCommand::Select => {
                let command = super::commands::Select::new(values);
                command.invoke(database)
            }
            AvailableCommand::Set => {
                let command = super::commands::Set::new(values);
                command.invoke(database)
            }
            AvailableCommand::Get => {
                let command = super::commands::Get::new(values);
                command.invoke(database)
            }
            AvailableCommand::Del => {
                let command = super::commands::Del::new(values);
                command.invoke(database)
            }
            AvailableCommand::Incr => {
                let command = super::commands::Incr::new(values);
                command.invoke(database)
            }
            AvailableCommand::Decr => {
                let command = super::commands::Decr::new(values);
                command.invoke(database)
            }
            AvailableCommand::Expire => {
                let command = super::commands::Expire::new(values);
                command.invoke(database)
            }
        }
    }
}
