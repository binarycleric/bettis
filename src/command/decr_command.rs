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
    use super::Command;
    use super::DecrCommand;
    use super::Database;
    use super::resp::Value;

    #[test]
    fn it_decrements_provided_value() {
        let mut database = Database::new();
        let values = vec![
            Value::Bulk("decr".to_string()),
            Value::Bulk("test_key".to_string()),
        ];
        let command = DecrCommand::new(values);

        database.set("test_key", Value::Integer(1));
        command.invoke(&mut database);

        let expected = Value::Integer(0);
        let actual = database.get("test_key").unwrap();

        assert_eq!(*actual, expected);
    }
}