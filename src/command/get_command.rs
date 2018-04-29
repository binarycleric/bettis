extern crate resp;

use storage::Database;
use command::Runnable;

pub struct GetCommand {
    values: Vec<resp::Value>,
}

impl GetCommand {
    pub fn new(values: Vec<resp::Value>) -> Self {
        Self {
            values: values
        }
    }
}

impl Runnable for GetCommand {
    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        match database.get(&Self::hash_key(&self.values)) {
            Some(value) => Ok(value.clone()),
            None => Err(Self::error_response()),
        }
    }
}
