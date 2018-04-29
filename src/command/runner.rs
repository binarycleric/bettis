extern crate resp;

use std::io::BufReader;
use self::resp::{Value, Decoder};

use super::Runnable;
use super::commands::Available as AvailableCommand;

use storage::Database;

pub fn run(reader: BufReader<&[u8]>, data_table: &mut Database) -> Result<resp::Value, resp::Value> {
    let mut decoder = Decoder::new(reader);
    let values = decoder.decode().unwrap();
    let runner = Runner::new(values);

    match runner {
        Ok(runner) => {
            runner.run(data_table)
        }
        Err(error) => {
            Err(resp::Value::Error(error.to_string()))
        }
    }
}

struct Runner {
    command_name: AvailableCommand,
    values: Vec<resp::Value>,
}

impl Runner {
    pub fn new(incoming: resp::Value) -> Result<Self, &'static str> {
        let command_name: AvailableCommand;
        let values: Vec<resp::Value>;

        if let Value::Array(ref array) = incoming {
            values = array.clone();

            if let Value::Bulk(ref cname) = array[0] {
                command_name = AvailableCommand::from_str(cname);

                Ok(Self {
                    command_name: command_name,
                    values: values,
                })
            } else {
                Err("Malformed request")
            }
        } else {
            Err("Malformed request")
        }
    }

    pub fn run(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        let values = self.values.get(1..).unwrap().clone().to_vec();
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
        }
    }
}