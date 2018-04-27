extern crate resp;

use storage::Database;

pub struct DecrCommand {
    key: resp::Value,
}

impl DecrCommand {
    pub fn new(key: resp::Value) -> Self {
        Self { key: key }
    }

    pub fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value> {
        println!("Invoke Decr...");
        println!("KEY --> {:?}", self.key);

        if let resp::Value::Bulk(ref key) = self.key {
            data_table.decr(&key)
        } else {
            Err(super::error_response())
        }
    }
}