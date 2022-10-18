extern crate resp;

use crate::storage::Database;
use crate::command::Runnable;

#[derive(Debug)]
pub struct Select {
    _values: Vec<resp::Value>,
}

impl Select {
    pub fn new(values: Vec<resp::Value>) -> Self {
        Self { _values: values }
    }
}

impl Runnable for Select {
    fn invoke(&self, _database: &mut Database) -> Result<resp::Value, resp::Value> {
        Ok(Self::ok_response())
    }
}
