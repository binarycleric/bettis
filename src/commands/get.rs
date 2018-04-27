extern crate resp;

use storage::Database;

pub struct GetCommand {
    key: resp::Value,
}

impl GetCommand {
    pub fn new(key: resp::Value) -> Self {
        Self { key: key }
    }

    pub fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value> {
        println!("Invoke Get...");
        println!("KEY --> {:?}", self.key);

        if let resp::Value::Bulk(ref key) = self.key {
            match data_table.get(&key) {
                Some(value) => Ok(value.clone()),
                None => Err(super::error_response()),
            }
        } else {
            Err(super::error_response())
        }
    }
}
