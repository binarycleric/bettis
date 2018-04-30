extern crate resp;

mod data_key;
mod data_store;

use std::collections::HashMap;
use resp::Value as DataValue;
use self::data_key::DataKey;

pub use self::data_store::DataStore;

const INVALID_INCR_ERROR: &'static str = "ERR value is not an integer or out of range";

#[derive(Debug)]
pub struct Database {
    value_map: HashMap<DataKey, DataValue>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            value_map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: DataValue) {
        self.value_map.insert(Self::data_key(key), value);
    }

    pub fn get(&self, key: String) -> Option<&DataValue> {
        self.value_map.get(&Self::data_key(key))
    }

    pub fn del(&mut self, key: String) -> Option<DataValue> {
        self.value_map.remove(&Self::data_key(key))
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
