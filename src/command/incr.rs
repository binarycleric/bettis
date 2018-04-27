extern crate resp;

use storage::Database;
use command::Command;

pub struct IncrCommand {
    key: resp::Value,
}

impl IncrCommand {
    pub fn new(key: resp::Value) -> Self {
        Self { key: key }
    }
}

impl Command for IncrCommand {
    fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke Incr...");
        debug!("KEY --> {:?}", self.key);

        if let resp::Value::Bulk(ref key) = self.key {
            data_table.incr(&key)
        } else {
            Err(super::error_response())
        }
    }
}
