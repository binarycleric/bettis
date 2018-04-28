extern crate resp;

use storage::Database;
use command::Command;

pub struct GetCommand {
    values: Vec<resp::Value>,
}

impl Command<GetCommand> for GetCommand {
    fn new(values: Vec<resp::Value>) -> Self {
        Self {
            values: values
        }
    }

    fn get_values(&self) -> Vec<resp::Value> {
        self.values.clone()
    }

    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke Get...");
        debug!("KEY --> {:?}", self.hash_key());

        match database.get(&self.hash_key()) {
            Some(value) => Ok(value.clone()),
            None => Err(self.error_response()),
        }
    }
}
