extern crate resp;

use storage::Database;

pub struct SetCommand {
    key: resp::Value,
    value: resp::Value,
}

impl SetCommand {
    pub fn new(key: resp::Value, value: resp::Value) -> Self {
        Self {
            key: key,
            value: value,
        }
    }

    pub fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value> {
        println!("Invoke set...");
        println!("KEY --> {:?}", self.key);
        println!("VALUE --> {:?}", self.value);

        if let resp::Value::Bulk(ref key) = self.key {
            data_table.set(&key, self.value.clone());
            Ok(super::ok_response())
        } else {
            Err(super::error_response())
        }
    }
}
