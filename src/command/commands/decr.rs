extern crate resp;

use crate::storage::Database;
use crate::command::Runnable;
use self::resp::Value as DataValue;

const INVALID_DECR_ERROR: &'static str = "ERR value is not an integer or out of range";

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
        let key = Self::hash_key(&self.values);
        let new_value: i64;

        match database.get(key.clone()) {
            Some(value) => {
                if let DataValue::Integer(int) = value {
                    new_value = int.clone() - 1;
                } else {
                    return Err(DataValue::Error(INVALID_DECR_ERROR.to_string()));
                }
            }
            None => {
                new_value = -1;
            }
        }

        let integer = DataValue::Integer(new_value);
        database.set(key.clone(), integer.clone());

        Ok(integer)
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

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_creates_value_if_does_not_exist() {
        let mut database = Database::new();
        let values = vec![Value::Bulk("test_key".to_string())];

        let command = Decr::new(values);
        let _ = command.invoke(&mut database);
        let expected = Value::Integer(-1);
        let actual = database.get("test_key".to_string()).unwrap();

        assert_eq!(actual, expected);
    }
}
