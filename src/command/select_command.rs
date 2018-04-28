extern crate resp;

use storage::Database;
use command::Command;

pub struct SelectCommand {
    database: resp::Value,
}

impl SelectCommand {
    pub fn new(database: resp::Value) -> Self {
        Self { database: database }
    }
}

impl Command for SelectCommand {
    fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke select...");
        debug!("VALUE --> {:?}", self.database);

        // data_table.set(&self.key, self.value);
        Ok(self.ok_response())
    }
}
