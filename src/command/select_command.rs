extern crate resp;

use storage::Database;
use command::Command;

pub struct SelectCommand {
    values: Vec<resp::Value>,
}

impl Command<SelectCommand> for SelectCommand {
    fn new(values: Vec<resp::Value>) -> Self {
        Self {
            values: values
        }
    }

    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke select...");
        // data_table.set(&self.key, self.value);
        Ok(Self::ok_response())
    }
}
