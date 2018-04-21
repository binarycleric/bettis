use std::collections::HashMap;
use types::DataType;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DataKey {
    key: String,
}

//impl Clone for DataKey {
//    fn clone(&self) -> DataKey {
//        *self
//    }
//}

impl DataKey {
    pub fn new(key: String) -> DataKey {
        return DataKey { key: key };
    }
}

// ----

#[derive(Debug)]
pub struct Database {
    value_map: HashMap<DataKey, DataType>,
}

impl Database {
    pub fn new() -> Database {
        return Database {
            value_map: HashMap::new(),
        };
    }

    pub fn set<'kl>(&mut self, key: &'kl str, value: DataType) {
        let data_key = DataKey::new(key.to_string());
        self.value_map.insert(data_key, value);
    }

    pub fn get<'kl>(&self, key: &'kl str) -> Option<&DataType> {
        let data_key = DataKey::new(key.to_string());
        return self.value_map.get(&data_key);
    }
}

#[cfg(test)]
mod tests {
    use super::Database;

    #[test]
    fn it_does_stuff() {}
}
