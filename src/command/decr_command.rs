extern crate resp;

use storage::Database;
use command::Command;

pub struct DecrCommand {
    values: Vec<resp::Value>,
}

impl Command<DecrCommand> for DecrCommand {
    fn new(values: Vec<resp::Value>) -> Self {
        Self {
            values: values
        }
    }

    fn get_values(&self) -> Vec<resp::Value> {
        self.values.clone()
    }

    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke Decr...");
        debug!("KEY --> {:?}", self.hash_key());

        database.decr(&self.hash_key())
    }
}
