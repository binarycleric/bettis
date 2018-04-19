use std::collections::HashMap;
use types::{DataKey, DataType};

#[derive(Debug)]
pub struct DataTable<'dkey, 'dval> {
    value_map: HashMap<DataKey<'dkey>, DataType<'dval>>,
}

impl<'dkey, 'dval> DataTable<'dkey, 'dval> {
    pub fn new() -> DataTable<'dkey, 'dval> {
        return DataTable {
            value_map: HashMap::new(),
        };
    }

    pub fn set(&mut self, key: DataKey<'dkey>, value: DataType<'dval>) {
        self.value_map.insert(key, value);
    }

    pub fn get(&self, key: &DataKey<'dkey>) -> Option<&DataType<'dval>> {
        return self.value_map.get(&key);
    }
}
