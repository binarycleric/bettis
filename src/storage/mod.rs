use std::collections::HashMap;


pub trait DataValue {}

#[derive(Debug)]
pub struct DataTable {
    pub data_map: HashMap<String, String>,
}

impl DataTable {
    pub fn new() -> DataTable {
        return DataTable {
            data_map: HashMap::new(),
        }
    }

    pub fn set<T: DataValue>(&self, key: String, value: T) {

    }
}
