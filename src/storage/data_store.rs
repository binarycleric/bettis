use super::data_key::DataKey;
use resp::Value as DataValue;

pub trait DataStore {
    fn set(&mut self, key: DataKey, value: DataValue);
    fn get(&mut self, key: DataKey) -> Option<DataValue>;
    fn del(&mut self, key: DataKey) -> Option<DataValue>;
}
