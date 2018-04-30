extern crate resp;

use storage::Database;
use command::Runnable;

pub struct Set {
    values: Vec<resp::Value>,
}

impl Set {
    pub fn new(values: Vec<resp::Value>) -> Self {
        Self { values: values }
    }
}

impl Runnable for Set {
    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        let key = Self::hash_key(&self.values);
        let value = Self::single_value(&self.values);

        database.set(key, value);
        Ok(Self::ok_response())
    }
}
