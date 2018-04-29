extern crate resp;

use storage::Database;
use command::Runnable;

pub struct Get {
    values: Vec<resp::Value>,
}

impl Get {
    pub fn new(values: Vec<resp::Value>) -> Self {
        Self {
            values: values
        }
    }
}

impl Runnable for Get {
    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        match database.get(&Self::hash_key(&self.values)) {
            Some(value) => Ok(value.clone()),
            None => Err(Self::error_response()),
        }
    }
}
