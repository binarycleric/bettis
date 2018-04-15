use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum DataType<'a> {
    SimpleString(&'a str),
    BulkString(String),
}

#[derive(Debug)]
pub struct DataTable<'tablelife> {
    pub data_map: HashMap<String, String>,

    value_map: HashMap<String, DataType<'tablelife>>,
}

impl<'tablelife> DataTable<'tablelife> {
    pub fn new() -> DataTable<'tablelife> {
        return DataTable {
            data_map: HashMap::new(),
            value_map: HashMap::new(),
        }
    }

    pub fn set<'keylife>(&mut self, key: &'keylife str, value: DataType<'tablelife>) {
        self.value_map.insert(key.to_string(), value);
    }

    pub fn get<'keylife>(&self, key: &'keylife str) -> &DataType<'tablelife> {
        return self.value_map.get(&key.to_string()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_does_things() {
      let mut table = super::DataTable::new();
      let value = super::DataType::SimpleString("test");

      table.set("example", value);

      let expected = super::DataType::SimpleString("test");

      assert_eq!(&expected, table.get("example"));
    }
}
