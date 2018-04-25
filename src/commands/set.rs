use types::QType;
use storage::Database;

pub struct SetCommand {
    key: String,
    value: QType,
}

impl SetCommand {
    pub fn invoke(&self, data_table: &mut Database) -> Result<String, &'static str> {
        println!("Invoke set...");
        println!("VALUE --> {:?}", self.value);

        // data_table.set(&self.key, self.value);

        Ok("+OK\r\n".to_string())
    }
}
