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

    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke Del...");
        debug!("KEY --> {:?}", Self::hash_key(&self.values));

        database.del(&Self::hash_key(&self.values));
        Ok(Self::ok_response())
    }
}
