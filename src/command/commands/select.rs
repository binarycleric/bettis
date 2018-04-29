extern crate resp;

use storage::Database;
use command::Runnable;

#[derive(Debug)]
pub struct Select {
    values: Vec<resp::Value>,
}

impl Select {
    pub fn new(values: Vec<resp::Value>) -> Self {
        Self {
            values: values
        }
    }
}

impl Runnable for Select {
    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        // data_table.set(&self.key, self.value);
        Ok(Self::ok_response())
    }
}