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
    fn get_values(&self) -> Vec<resp::Value>;

    fn hash_key(&self) -> String {
        if let resp::Value::Bulk(ref hash_key) = self.get_values()[1] {
            return hash_key.clone()
        }
        panic!("Shouldn't get here")
    }

    fn single_value(&self) -> resp::Value {
        self.get_values()[2].clone()
    }

    fn ok_response(&self) -> resp::Value {
        resp::Value::String("OK".to_string())
    }

    fn error_response(&self) -> resp::Value {
        resp::Value::Error("1".to_string())
    }
}


