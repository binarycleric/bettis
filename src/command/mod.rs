extern crate resp;

mod runner;
mod commands;

use storage::Database;

pub use self::runner::run;

trait Runnable {
    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value>;

    fn hash_key(values: &Vec<resp::Value>) -> String {
        match values[0] {
            resp::Value::Bulk(ref hash_key) => {
                hash_key.clone()
            }
            _ => panic!("Shouldn't get here"),
        }
    }

    fn single_value(values: &Vec<resp::Value>) -> resp::Value {
        values[1].clone()
    }

    fn ok_response() -> resp::Value {
        resp::Value::String("OK".to_string())
    }

    fn error_response() -> resp::Value {
        resp::Value::Error("1".to_string())
    }
}
