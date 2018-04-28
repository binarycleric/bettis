extern crate resp;

use storage::Database;
use command::Command;

pub struct IncrCommand {
    values: Vec<resp::Value>,
}

impl Command<IncrCommand> for IncrCommand {
    fn new(values: Vec<resp::Value>) -> Self {
        Self {
            values: values
        }
    }

    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke Incr...");
        debug!("KEY --> {:?}", Self::hash_key(&self.values));

        database.incr(&Self::hash_key(&self.values))
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, IncrCommand, Database};
    use super::resp::Value;

    #[test]
    fn it_incroments_provided_value() {
        let mut database = Database::new();
        let values = vec![
            Value::Bulk("test_key".to_string()),
        ];

        database.set("test_key", Value::Integer(1));

        let command = IncrCommand::new(values);
        let _ = command.invoke(&mut database);
        let expected = Value::Integer(2);
        let actual = database.get("test_key").unwrap();

        assert_eq!(*actual, expected);
    }
}