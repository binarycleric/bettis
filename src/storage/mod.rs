use std::collections::HashMap;
use types::DataType;

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

    pub fn incr<'kl>(&mut self, key: &'kl str) {
        let incr_value: i64;

        match self.get(key) {
            Some(value) => {
                if let DataType::Integer(ival) = *value {
                    incr_value = ival + 1;
                } else {
                    panic!("Not sure what's up! {:?}", value);
                }
            }
            None => {
                incr_value = 1;
            }
        }

        self.set(key, DataType::Integer(incr_value))
    }

    pub fn decr<'kl>(&mut self, key: &'kl str) {
        let decr_value: i64;

        match self.get(key) {
            Some(value) => {
                if let DataType::Integer(ival) = *value {
                    decr_value = ival - 1;
                } else {
                    panic!("Not sure what's up! {:?}", value);
                }
            }
            None => {
                decr_value = 0;
            }
        }

        self.set(key, DataType::Integer(decr_value))

    }
}

#[cfg(test)]
mod tests {
    use super::Database;
    use types::DataType;

    #[test]
    fn it_sets_data_value_key() {
        let mut database = Database::new();
        let data_type = DataType::SimpleString("Ok".to_string());

        database.set("example", data_type);
    }
}
