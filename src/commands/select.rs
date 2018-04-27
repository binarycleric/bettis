extern crate resp;

use storage::Database;

pub struct SelectCommand {
    database: resp::Value,
}

impl SelectCommand {

    pub fn new(database: resp::Value) -> Self {
         Self { database: database }
    }

    pub fn invoke(&self, data_table: &mut Database) -> Result<resp::Value, resp::Value> {
        println!("Invoke select...");
        println!("VALUE --> {:?}", self.database);

        // data_table.set(&self.key, self.value);
        Ok(super::ok_response())
    }
}
