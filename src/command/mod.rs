extern crate resp;

use std::io::BufReader;
use self::resp::{Decoder};

mod set_command;
mod select_command;
mod get_command;
mod del_command;
mod incr_command;
mod decr_command;
mod runner;

use storage::Database;
use self::runner::Runner;

trait Command {
    fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value>;

    fn ok_response(&self) -> resp::Value {
        resp::Value::String("OK".to_string())
    }

    fn error_response(&self) -> resp::Value {
        resp::Value::Error("1".to_string())
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
