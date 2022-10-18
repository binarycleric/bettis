extern crate resp;
extern crate chrono;

use crate::storage::Database;
use crate::command::Runnable;

use self::chrono::Duration;

pub struct Expire {
    values: Vec<resp::Value>,
}

impl Expire {
    pub fn new(values: Vec<resp::Value>) -> Self {
        Self { values: values }
    }

    fn timeout_set() -> resp::Value {
        resp::Value::Integer(1)
    }

    fn key_does_not_exist() -> resp::Value {
        resp::Value::Integer(0)
    }
}

impl Runnable for Expire {
    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {

        match Self::single_value(&self.values) {
            resp::Value::Integer(ttl) => {
                let duration = Duration::seconds(ttl);
                let key = Self::hash_key(&self.values);

                match database.set_ttl(key, duration) {
                    Ok(_) => Ok(Self::timeout_set()),
                    Err(_) => Err(Self::key_does_not_exist()),
                }
            }
            _ => {
                panic!("Invalid value")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::resp::Value;

    #[test]
    fn it_sets_expiration_on_key() {
        let mut database = Database::new();
        let duration = 5;
        let values = vec![
            Value::Bulk("test_key".to_string()),
            Value::Integer(duration),
        ];

        database.set("test_key".to_string(), Value::Integer(1));

        let response = Expire::new(values).invoke(&mut database);
        let ttl = database.ttl("test_key".to_string());

        assert!(ttl != None);
        assert!(ttl.unwrap() < Duration::seconds(duration));
        assert_eq!(Value::Integer(1), response.unwrap());
    }
}