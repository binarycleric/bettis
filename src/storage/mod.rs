use std::collections::HashMap;
use types::{DataKey, DataType};

#[derive(Debug)]
pub struct DataTable<'vlife> {
    pub data_map: HashMap<String, String>,

    value_map: HashMap<DataKey<'vlife>, DataType<'vlife>>,
}

impl<'vlife> DataTable<'vlife> {
    pub fn new() -> DataTable<'vlife> {
        return DataTable {
            data_map: HashMap::new(),
            value_map: HashMap::new(),
        };
    }

    pub fn set(&mut self, key: &DataKey<'vlife>, value: DataType<'vlife>) {
        self.value_map.insert(*key, value);
    }

    pub fn get(&self, key: &DataKey<'vlife>) -> Option<&DataType<'vlife>> {
        return self.value_map.get(&key);
    }
}
