extern crate resp;

use storage::Database;
use command::Runnable;

pub struct DelCommand {
    values: Vec<resp::Value>,
}

impl DelCommand {
    pub fn new(values: Vec<resp::Value>) -> Self {
        Self {
            values: values
        }
    }
}

impl Runnable for DelCommand {
    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        database.del(&Self::hash_key(&self.values));
        Ok(Self::ok_response())
    }
}
