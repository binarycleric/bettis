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

    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke Decr...");
        debug!("KEY --> {:?}", Self::hash_key(&self.values));

        database.decr(&Self::hash_key(&self.values))
    }
}
