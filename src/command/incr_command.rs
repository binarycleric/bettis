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

    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke Incr...");
        debug!("KEY --> {:?}", Self::hash_key(&self.values));

        database.incr(&Self::hash_key(&self.values))
    }
}