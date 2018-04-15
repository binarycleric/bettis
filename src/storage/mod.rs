use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum DataType<'a> {
    SimpleString(&'a str),
    BulkString(String),
    Integer(i64),
}

#[derive(Debug, Hash)]
pub struct DataKey {
    key: String,
}

impl DataKey {
    pub fn new<'a>(key: &'a str) -> DataKey {
      return DataKey {
          key: key.to_string(),
      }
    }
}

impl PartialEq for DataKey {
    fn eq(&self, other: &DataKey) -> bool {
        self.key == other.key
    }
}

impl Eq for DataKey {}




#[derive(Debug)]
pub struct DataTable<'vlife> {
    pub data_map: HashMap<String, String>,

    value_map: HashMap<DataKey, DataType<'vlife>>,
}

impl<'vlife> DataTable<'vlife> {
    pub fn new() -> DataTable<'vlife> {
        return DataTable {
            data_map: HashMap::new(),
            value_map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: DataKey, value: DataType<'vlife>) {
        self.value_map.insert(key, value);
    }

    pub fn get(&self, key: DataKey) -> Option<&DataType<'vlife>> {
        return self.value_map.get(&key);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_sets_and_gets_values() {
        let mut table = super::DataTable::new();
        let value = super::DataType::SimpleString("test");
        let data_key = super::DataKey::new("example");

        table.set(data_key, value);

        let data_key = super::DataKey::new("example");
        let expected = super::DataType::SimpleString("test");

        assert_eq!(Some(&expected), table.get(data_key));
    }

    #[test]
    fn it_returns_none_when_fetching_empty_key() {
        let mut table = super::DataTable::new();
        let data_key = super::DataKey::new("example");

        assert_eq!(None, table.get(data_key));
    }
}
