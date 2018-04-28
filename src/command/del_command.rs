extern crate resp;

use storage::Database;
use command::Command;

pub struct DelCommand {
    key: resp::Value,
}

impl DelCommand {
    pub fn new(key: resp::Value) -> Self {
        Self { key: key }
    }
}

impl Command for DelCommand {
    fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke Del...");
        debug!("KEY --> {:?}", self.key);

        if let resp::Value::Bulk(ref key) = self.key {
            data_table.del(&key);
            Ok(self.ok_response())
        } else {
            Err(self.error_response())
        }
    }
}
