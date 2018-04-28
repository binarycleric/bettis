extern crate resp;

use std::io::BufReader;
use self::resp::{Value, Decoder};

use super::Command;
use super::set_command::SetCommand;
use super::select_command::SelectCommand;
use super::get_command::GetCommand;
use super::del_command::DelCommand;
use super::incr_command::IncrCommand;
use super::decr_command::DecrCommand;

use storage::Database;

const SELECT_COMMAND: &'static str = "select";
const SET_COMMAND: &'static str = "set";
const GET_COMMAND: &'static str = "get";
const DEL_COMMAND: &'static str = "del";
const INCR_COMMAND: &'static str = "incr";
const DECR_COMMAND: &'static str = "decr";

enum AvailableCommand {
    Select,
    Set,
    Get,
    Incr,
    Decr,
    Del,
}

impl AvailableCommand {
    pub fn from_str<'a>(string: &'a str) -> Self {
        match string {
            SELECT_COMMAND => AvailableCommand::Select,
            SET_COMMAND => AvailableCommand::Set,
            GET_COMMAND => AvailableCommand::Get,
            DEL_COMMAND => AvailableCommand::Del,
            INCR_COMMAND => AvailableCommand::Incr,
            DECR_COMMAND => AvailableCommand::Decr,
            _ => panic!("Nooo"),
        }
    }
}

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
        let values = self.values.clone();

        match self.command_name {
            AvailableCommand::Select => {
                let command = SelectCommand::new(values);
                command.invoke(database)
            }
            AvailableCommand::Set => {
                let command = SetCommand::new(values);
                command.invoke(database)
            }
            AvailableCommand::Get => {
                let command = GetCommand::new(values);
                command.invoke(database)
            }
            AvailableCommand::Del => {
                let command = DelCommand::new(values);
                command.invoke(database)
            }
            AvailableCommand::Incr => {
                let command = IncrCommand::new(values);
                command.invoke(database)
            }
            AvailableCommand::Decr => {
                let command = DecrCommand::new(values);
                command.invoke(database)
            }
        }
    }
}