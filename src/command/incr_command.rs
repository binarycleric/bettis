extern crate resp;

use storage::Database;
use command::Command;

pub struct IncrCommand {
    values: Vec<resp::Value>,
}

impl Command<IncrCommand> for IncrCommand {
    fn new(values: Vec<resp::Value>) -> Self {
        Self {
            values: values
        }
    }

    fn get_values(&self) -> Vec<resp::Value> {
        self.values.clone()
    }

    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke Incr...");
        debug!("KEY --> {:?}", self.hash_key());

        database.incr(&self.hash_key())
    }
}
