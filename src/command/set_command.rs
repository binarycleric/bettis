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

    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        let key = Self::hash_key(&self.values);
        let value = Self::single_value(&self.values);

        database.set(&key, value);
        Ok(Self::ok_response())
    }
}
