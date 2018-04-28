extern crate resp;

use storage::Database;
use command::Command;

pub struct DecrCommand {
    values: Vec<resp::Value>,
}

impl Command<DecrCommand> for DecrCommand {
    fn new(values: Vec<resp::Value>) -> Self {
        Self {
            values: values
        }
    }

    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        debug!("Invoke Decr...");
        debug!("KEY --> {:?}", Self::hash_key(&self.values));

        database.decr(&Self::hash_key(&self.values))
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, DecrCommand, Database};
    use super::resp::Value;

    #[test]
    fn it_decrements_provided_value() {
        let mut database = Database::new();
        let values = vec![
            Value::Bulk("test_key".to_string()),
        ];

        database.set("test_key", Value::Integer(1));

        let command = DecrCommand::new(values);
        let _ = command.invoke(&mut database);
        let expected = Value::Integer(0);
        let actual = database.get("test_key").unwrap();

        assert_eq!(*actual, expected);
    }

    #[test]
    fn it_creates_value_if_does_not_exist() {
        let mut database = Database::new();
        let values = vec![
            Value::Bulk("test_key".to_string()),
        ];

        let command = DecrCommand::new(values);
        let _ = command.invoke(&mut database);
        let expected = Value::Integer(-1);
        let actual = database.get("test_key").unwrap();

        assert_eq!(*actual, expected);
    }
}