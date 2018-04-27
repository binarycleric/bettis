extern crate resp;

use std::io::BufReader;
use self::resp::{Decoder, Value};

mod set;
mod select;
mod get;
mod del;
mod incr;
mod decr;

use storage::Database;

use self::set::SetCommand;
use self::select::SelectCommand;
use self::get::GetCommand;
use self::del::DelCommand;
use self::incr::IncrCommand;
use self::decr::DecrCommand;

const SELECT_COMMAND: &'static str = "select";
const SET_COMMAND: &'static str = "set";
const GET_COMMAND: &'static str = "get";
const INCR_COMMAND: &'static str = "incr";
const DECR_COMMAND: &'static str = "decr";
const DEL_COMMAND: &'static str = "del";

pub fn ok_response() -> resp::Value {
    resp::Value::String("OK".to_string())
}

pub fn error_response() -> resp::Value {
    resp::Value::Error("1".to_string())
}

#[derive(Debug)]
pub struct Command {
    values: Value,
}

impl Command {
    pub fn build(reader: BufReader<&[u8]>) -> Self {
        let mut decoder = Decoder::new(reader);
        let values = decoder.decode().unwrap();
        Self { values: values }
    }

    pub fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value> {
        if let Value::Array(ref array) = self.values {
            if let Value::Bulk(ref command_name) = array[0] {
                match command_name.as_str() {
                    SELECT_COMMAND => {
                        let database_id = array[1].clone();
                        let command = SelectCommand::new(database_id);
                        command.invoke(data_table)
                    }
                    SET_COMMAND => {
                        let set_key = array[1].clone();
                        let set_value = array[2].clone();
                        let command = SetCommand::new(set_key, set_value);
                        command.invoke(data_table)
                    }
                    GET_COMMAND => {
                        let set_key = array[1].clone();
                        let command = GetCommand::new(set_key);
                        command.invoke(data_table)
                    }
                    DEL_COMMAND => {
                        let set_key = array[1].clone();
                        let command = DelCommand::new(set_key);
                        command.invoke(data_table)
                    }
                    INCR_COMMAND => {
                        let set_key = array[1].clone();
                        let command = IncrCommand::new(set_key);
                        command.invoke(data_table)
                    }
                    DECR_COMMAND => {
                        let set_key = array[1].clone();
                        let command = DecrCommand::new(set_key);
                        command.invoke(data_table)
                    }
                    x => {
                        println!("Nope -- {:?}", x);
                        Err(resp::Value::Error("1".to_string()))
                    }
                }
            } else {
                Err(resp::Value::Error("1".to_string()))
            }
        } else {
            Err(resp::Value::Error("1".to_string()))
        }
    }
}
