use std::collections::HashMap;
use types::{DataKey, DataType};

#[derive(Debug)]
pub struct DataTable<'dt> {
    value_map: HashMap<DataKey<'dt>, DataType<'dt>>,
}

impl<'dt> DataTable<'dt> {
    pub fn new() -> DataTable<'dt> {
        return DataTable {
            value_map: HashMap::new(),
        };
    }

    pub fn set(&mut self, key: DataKey<'dt>, value: DataType<'dt>) {
        self.value_map.insert(key, value);
    }

    pub fn get(&self, key: &DataKey<'dt>) -> Option<&DataType<'dt>> {
        return self.value_map.get(&key);
    }
}
