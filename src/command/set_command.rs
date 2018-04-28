extern crate resp;

use storage::Database;
use command::Command;

pub struct SetCommand {
    values: Vec<resp::Value>,
}

impl Command<SetCommand> for SetCommand {
    fn new(values: Vec<resp::Value>) -> Self {
        Self {
            values: values
        }
    }

    fn get_values(&self) -> Vec<resp::Value> {
        self.values.clone()
    }

    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke set...");

        database.set(&self.hash_key(), self.single_value());
        Ok(self.ok_response())
    }
}
