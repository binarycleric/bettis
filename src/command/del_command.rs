extern crate resp;

use storage::Database;
use command::Command;

pub struct DelCommand {
    values: Vec<resp::Value>,
}

impl Command<DelCommand> for DelCommand {
    fn new(values: Vec<resp::Value>) -> Self {
        Self {
            values: values
        }
    }

    fn get_values(&self) -> Vec<resp::Value> {
        self.values.clone()
    }

    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke Del...");
        debug!("KEY --> {:?}", self.hash_key());

        database.del(&self.hash_key());
        Ok(self.ok_response())
    }
}
