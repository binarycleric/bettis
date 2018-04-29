extern crate resp;

use std::collections::HashMap;

const INVALID_INCR_ERROR: &'static str = "ERR value is not an integer or out of range";

#[derive(Debug)]
pub struct Database {
    value_map: HashMap<String, resp::Value>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            value_map: HashMap::new(),
        }
    }

    pub fn set<'kl>(&mut self, key: &'kl str, value: resp::Value) {
        self.value_map.insert(key.to_string(), value);
    }

    pub fn get<'kl>(&self, key: &'kl str) -> Option<&resp::Value> {
        self.value_map.get(&key.to_string())
    }

    pub fn del<'kl>(&mut self, key: &'kl str) -> Option<resp::Value> {
        self.value_map.remove(&key.to_string())
    }

    pub fn incr<'kl>(&mut self, key: &'kl str) -> Result<resp::Value, resp::Value> {
        let mut new_value: i64;

        match self.get(key) {
            Some(value) => {
                if let &resp::Value::Integer(ref int) = value {
                    new_value = int.clone() + 1;
                } else {
                    return Err(resp::Value::Error(INVALID_INCR_ERROR.to_string()));
                }
            }
            None => {
                new_value = 1;
            }
        }

        self.set(key, resp::Value::Integer(new_value));
        Ok(resp::Value::Integer(new_value))
    }

    pub fn decr<'kl>(&mut self, key: &'kl str) -> Result<resp::Value, resp::Value> {
        let new_value: i64;

        match self.get(key) {
            Some(value) => {
                if let &resp::Value::Integer(ref int) = value {
                    new_value = int.clone() - 1;
                } else {
                    return Err(resp::Value::Error(INVALID_INCR_ERROR.to_string()));
                }
            }
            None => {
                new_value = -1;
            }
        }

        self.set(key, resp::Value::Integer(new_value));
        Ok(resp::Value::Integer(new_value))
    }
}
