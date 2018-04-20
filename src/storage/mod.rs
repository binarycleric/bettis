use std::collections::HashMap;
use types::{DataType};

#[derive(Debug, Hash, Copy, PartialEq, Eq)]
pub struct DataKey {
    key: &'static str,
}

impl Clone for DataKey {
    fn clone(&self) -> DataKey {
        *self
    }
}

impl DataKey {
    pub fn new(key: &'static str) -> DataKey {
        return DataKey { key: key };
    }
}

// ----

#[derive(Debug)]
pub struct DataTable<'dt> {
    value_map: HashMap<DataKey, DataType<'dt>>,
}

impl<'dt> DataTable<'dt> {
    pub fn new() -> DataTable<'dt> {
        return DataTable {
            value_map: HashMap::new(),
        };
    }

    pub fn set(&mut self, key: &'static str, value: DataType<'dt>) {
        let data_key = DataKey::new(key);
        self.value_map.insert(data_key, value);
    }

    pub fn get(&self, key: &'static str) -> Option<&DataType<'dt>> {
        let data_key = DataKey::new(key);
        return self.value_map.get(&data_key);
    }
}

#[cfg(test)]
mod tests {
    use super::DataTable;

    #[test]
    fn it_does_stuff() {

    }
}
