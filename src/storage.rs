extern crate resp;

use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DataKey {
    key: String,
}

impl DataKey {
    pub fn new(key: String) -> DataKey {
        return DataKey { key: key };
    }
}

#[derive(Debug)]
pub struct Database {
    value_map: HashMap<DataKey, resp::Value>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            value_map: HashMap::new(),
        }
    }

    pub fn set<'kl>(&mut self, key: &'kl str, value: resp::Value) {
        let data_key = DataKey::new(key.to_string());
        self.value_map.insert(data_key, value);
    }

    pub fn get<'kl>(&self, key: &'kl str) -> Option<&resp::Value> {
        let data_key = DataKey::new(key.to_string());
        return self.value_map.get(&data_key);
    }

    pub fn del<'kl>(&mut self, key: &'kl str) -> Option<resp::Value> {
        let data_key = DataKey::new(key.to_string());
        return self.value_map.remove(&data_key);
    }
}
