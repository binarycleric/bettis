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

    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        match database.get(&Self::hash_key(&self.values)) {
            Some(value) => Ok(value.clone()),
            None => Err(Self::error_response()),
        }
    }
}
