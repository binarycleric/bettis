extern crate resp;

mod set_command;
mod select_command;
mod get_command;
mod del_command;
mod incr_command;
mod decr_command;
mod runner;

use storage::Database;

pub use self::runner::run;

trait Command {
    fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value>;

    fn ok_response(&self) -> resp::Value {
        resp::Value::String("OK".to_string())
    }

    fn error_response(&self) -> resp::Value {
        resp::Value::Error("1".to_string())
    }
}


