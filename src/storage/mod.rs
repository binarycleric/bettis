extern crate chrono;
extern crate resp;

mod data_key;
mod data_value;

use self::data_value::DataValue as DataValueExp;
use self::data_value::LifetimeDatum;
use self::data_key::DataKey;

use std::collections::HashMap;
use self::chrono::Duration;
use self::resp::Value as DataValue;

const INVALID_INCR_ERROR: &'static str = "ERR value is not an integer or out of range";

#[derive(Debug)]
pub struct Database {
    values: HashMap<DataKey, DataValue>,
    ttls: HashMap<DataKey, LifetimeDatum>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            ttls: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: DataValue) {
        self.values.insert(Self::data_key(key), value);
    }

    pub fn get(&self, key: String) -> Option<&DataValue> {
        let data_key = Self::data_key((*key).to_string());

        match self.ttl(key) {
            Some(remaining) => {
                if remaining.is_zero() {
                    None
                } else {
                    self.values.get(&data_key)
                }
            }
            None => {
                self.values.get(&data_key)
            }
        }
    }

    pub fn del(&mut self, key: String) -> Option<DataValue> {
        self.values.remove(&Self::data_key(key))
    }

    pub fn exist<'l>(&self, key: &'l str) -> bool {
        self.values.contains_key(&Self::data_key(key.to_string()))
    }

    pub fn set_ttl(&mut self, key: String, duration: Duration) -> Result<DataValue, DataValue> {
        if self.exist(key.as_str()) {
            let ttl_datum = LifetimeDatum::new(duration);
            self.ttls.insert(Self::data_key(key), ttl_datum);
            Ok(DataValue::Integer(1))
        } else {
            Err(DataValue::Integer(0))
        }
    }

    pub fn ttl(&self, key: String) -> Option<Duration> {
        match self.ttls.get(&Self::data_key(key)) {
            Some(duration) => Some(duration.remaining()),
            None => None,
        }
    }

    pub fn incr(&mut self, key: String) -> Result<DataValue, DataValue> {
        let new_value: i64;

        match self.get(key.clone()) {
            Some(value) => {
                if let &DataValue::Integer(ref int) = value {
                    new_value = int.clone() + 1;
                } else {
                    return Err(DataValue::Error(INVALID_INCR_ERROR.to_string()));
                }
            }
            None => {
                new_value = 1;
            }
        }

        self.set(key.clone(), DataValue::Integer(new_value));
        Ok(DataValue::Integer(new_value))
    }

    pub fn decr(&mut self, key: String) -> Result<DataValue, DataValue> {
        let new_value: i64;

        match self.get(key.clone()) {
            Some(value) => {
                if let &DataValue::Integer(ref int) = value {
                    new_value = int.clone() - 1;
                } else {
                    return Err(DataValue::Error(INVALID_INCR_ERROR.to_string()));
                }
            }
            None => {
                new_value = -1;
            }
        }

        self.set(key.clone(), DataValue::Integer(new_value));
        Ok(DataValue::Integer(new_value))
    }

    fn data_key(key: String) -> DataKey {
        DataKey::new(key)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_KEY : &'static str  = "example";

    #[test]
    fn it_sets_a_keys_time_to_live() {
        let mut database = Database::new();
        let expected_ttl = Duration::seconds(5);

        database.set(TEST_KEY.to_string(), resp::Value::Integer(999));
        database.set_ttl(TEST_KEY.to_string(), expected_ttl);

        let actual_ttl = database.ttl(TEST_KEY.to_string());

        assert!(actual_ttl != None);
        assert!(actual_ttl.unwrap() < expected_ttl);
    }

    #[test]
    fn it_returns_nothing_on_expired_key() {
        let mut database = Database::new();

        let _ = database.set(TEST_KEY.to_string(), resp::Value::Integer(999));
        let _ = database.set_ttl(TEST_KEY.to_string(), Duration::nanoseconds(0));

        assert!(database.get(TEST_KEY.to_string()) == None);
    }
}
