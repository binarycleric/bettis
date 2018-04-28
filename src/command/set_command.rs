extern crate resp;

use storage::Database;
use command::Command;

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
}

impl Command for SetCommand {
    fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke set...");
        debug!("KEY --> {:?}", self.key);
        debug!("VALUE --> {:?}", self.value);

        if let resp::Value::Bulk(ref key) = self.key {
            data_table.set(&key, self.value.clone());
            Ok(self.ok_response())
        } else {
            Err(self.error_response())
        }
    }
}
