extern crate resp;

use storage::Database;
use command::Command;

pub struct DecrCommand {
    key: resp::Value,
}

impl DecrCommand {
    pub fn new(key: resp::Value) -> Self {
        Self { key: key }
    }
}

impl Command for DecrCommand {
    fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke Decr...");
        debug!("KEY --> {:?}", self.key);

        if let resp::Value::Bulk(ref key) = self.key {
            data_table.decr(&key)
        } else {
            Err(super::error_response())
        }
    }
}
