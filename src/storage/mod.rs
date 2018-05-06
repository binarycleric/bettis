extern crate chrono;
extern crate resp;

mod data_key;
mod data_value;

use self::data_value::LifetimeDatum;
use self::data_key::DataKey;
use self::chrono::Duration;
use self::resp::Value as DataValue;

use std::collections::HashMap as DataHash;
use std::collections::BTreeMap;

const INVALID_INCR_ERROR: &'static str = "ERR value is not an integer or out of range";





#[derive(Debug)]
pub struct KeyStore {
    keys: BTreeMap<String, DataKey>,
}

impl<'k> KeyStore {
    pub fn new() -> Self {
        Self { keys: BTreeMap::new() }
    }

    pub fn upsert(&mut self, k: &'k str) -> DataKey {
        match self.get(k) {
            Some(data_key) => data_key,
            None => {
                self.insert(k);
                self.upsert(k)
            }
        }
    }

    pub fn get(&mut self, k: &'k str) -> Option<DataKey> {
        match self.keys.get(k) {
            Some(k) => Some(k.clone()),
            None => None,
        }
    }

    pub fn insert(&mut self, k: &'k str) {
        self.keys.insert(
            k.to_string(),
            DataKey::new(k.to_string())
        );
    }
}




#[derive(Debug)]
pub struct Database {
    values: DataHash<String, DataValue>,
    ttls: DataHash<String, LifetimeDatum>,
    data_keys: KeyStore,
}

impl Database {
    pub fn new() -> Self {
        Self {
            values: DataHash::new(),
            ttls: DataHash::new(),
            data_keys: KeyStore::new(),
        }
    }

    pub fn set(&mut self, key: String, value: DataValue) {
        let data_key = self.data_keys.upsert(&key);
        self.values.insert(data_key.ident(), value);
    }

    pub fn get(&mut self, key: String) -> Option<&DataValue> {
        match self.ttl(key.clone()) {
            Some(remaining) => {
                if remaining.is_zero() {
                    None
                } else {
                    self.values.get(&key)
                }
            }
            None => {
                self.values.get(&key)
            }
        }
    }

    pub fn del(&mut self, key: String) -> Option<DataValue> {
        self.values.remove(&key)
    }

    pub fn set_ttl(&mut self, key: String, duration: Duration) -> Result<DataValue, DataValue> {
        let key_exists: bool = self.values.contains_key(&key);

        if key_exists {
            let ttl_datum = LifetimeDatum::new(duration);
            let data_key = self.data_keys.upsert(&key);

            self.ttls.insert(data_key.ident(), ttl_datum);
            Ok(DataValue::Integer(1))
        } else {
            Err(DataValue::Integer(0))
        }
    }

    pub fn ttl(&mut self, key: String) -> Option<Duration> {
        let key = self.data_keys.upsert(&key);

        match self.ttls.get(&key.ident()) {
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
