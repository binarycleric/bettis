mod set;

use types::QDataType;
use types::QType;
use storage::Database;

pub use self::set::SetCommand;

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


extern crate resp;

use std::io::prelude::*;
use std::io::BufReader;
use self::resp::{Decoder, Value};

#[derive(Debug)]
pub struct Command {
    values: Value,
}

impl Command {

    // "*2\r\n$3\r\nget\r\n$9\r\nincr-test\r\n"
    // println!("request --> {:#?}\n\n", request_string);
    pub fn build(reader: BufReader<&[u8]>) -> Self {
        let mut decoder = Decoder::new(reader);
        let values = decoder.decode().unwrap();

        Self { values: values }
    }

    pub fn invoke(&self, data_table: &mut Database) -> Result<String, String> {
        println!("{:?}", self.values);

        // Err(":1\r\n".to_string())
        Ok("+OK\r\n".to_string())
    }
}
