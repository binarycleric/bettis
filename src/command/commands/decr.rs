extern crate resp;

use storage::Database;
use command::Runnable;

pub struct Decr {
    values: Vec<resp::Value>,
}

impl Decr {
    pub fn new(values: Vec<resp::Value>) -> Self {
        Self { values: values }
    }
}

impl Runnable for Decr {
    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        database.decr(Self::hash_key(&self.values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::resp::Value;

    #[test]
    fn it_decrements_provided_value() {
        let mut database = Database::new();
        let values = vec![Value::Bulk("test_key".to_string())];

        database.set("test_key".to_string(), Value::Integer(1));

        let command = Decr::new(values);
        let _ = command.invoke(&mut database);
        let expected = Value::Integer(0);
        let actual = database.get("test_key".to_string()).unwrap();

        assert_eq!(*actual, expected);
    }

    #[test]
    fn it_creates_value_if_does_not_exist() {
        let mut database = Database::new();
        let values = vec![Value::Bulk("test_key".to_string())];

        let command = Decr::new(values);
        let _ = command.invoke(&mut database);
        let expected = Value::Integer(-1);
        let actual = database.get("test_key".to_string()).unwrap();

        assert_eq!(*actual, expected);
    }
}
