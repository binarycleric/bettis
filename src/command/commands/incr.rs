extern crate resp;

use storage::Database;
use command::Runnable;

pub struct Incr {
    values: Vec<resp::Value>,
}

impl Incr {
    pub fn new(values: Vec<resp::Value>) -> Self {
        Self { values: values }
    }
}

impl Runnable for Incr {
    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        database.incr(Self::hash_key(&self.values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::resp::Value;

    #[test]
    fn it_incroments_provided_value() {
        let mut database = Database::new();
        let values = vec![Value::Bulk("test_key".to_string())];

        database.set("test_key".to_string(), Value::Integer(1));

        let command = Incr::new(values);
        let _ = command.invoke(&mut database);
        let expected = Value::Integer(2);
        let actual = database.get("test_key".to_string()).unwrap();

        assert_eq!(*actual, expected);
    }

    #[test]
    fn it_creates_value_if_does_not_exist() {
        let mut database = Database::new();
        let values = vec![Value::Bulk("test_key".to_string())];

        let command = Incr::new(values);
        let _ = command.invoke(&mut database);
        let expected = Value::Integer(1);
        let actual = database.get("test_key".to_string()).unwrap();

        assert_eq!(*actual, expected);
    }
}
