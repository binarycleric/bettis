extern crate resp;

use storage::Database;

pub struct IncrCommand {
    key: resp::Value,
}

impl IncrCommand {
    pub fn new(key: resp::Value) -> Self {
        Self { key: key }
    }

    pub fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value> {
        println!("Invoke Incr...");
        println!("KEY --> {:?}", self.key);

        if let resp::Value::Bulk(ref key) = self.key {
            data_table.incr(&key)
        } else {
            Err(super::error_response())
        }
    }
}
