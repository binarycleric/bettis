extern crate resp;

use storage::Database;

pub struct DelCommand {
    key: resp::Value,
}

impl DelCommand {
    pub fn new(key: resp::Value) -> Self {
        Self { key: key }
    }

    pub fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value> {
        println!("Invoke Del...");
        println!("KEY --> {:?}", self.key);

        if let resp::Value::Bulk(ref key) = self.key {
            data_table.del(&key);
            Ok(super::ok_response())
        } else {
            Err(super::error_response())
        }
    }
}
