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

trait Command<T> where Self: Sized  {
    fn new(values: Vec<resp::Value>) -> Self;
    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value>;

    fn hash_key(values: &Vec<resp::Value>) -> String {
        if let resp::Value::Bulk(ref hash_key) = values[1] {
            return hash_key.clone()
        }
        panic!("Shouldn't get here")
    }

    fn single_value(values: &Vec<resp::Value>) -> resp::Value {
        values[2].clone()
    }

    fn ok_response() -> resp::Value {
        resp::Value::String("OK".to_string())
    }

    fn error_response() -> resp::Value {
        resp::Value::Error("1".to_string())
    }
}


