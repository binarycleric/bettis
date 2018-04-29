extern crate resp;

use storage::Database;
use command::Runnable;

pub struct Del {
    values: Vec<resp::Value>,
}

impl Del {
    pub fn new(values: Vec<resp::Value>) -> Self {
        Self {
            values: values
        }
    }
}

impl Runnable for Del {
    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        database.del(&Self::hash_key(&self.values));
        Ok(Self::ok_response())
    }
}
