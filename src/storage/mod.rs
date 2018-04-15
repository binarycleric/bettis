use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum DataType<'a> {
    SimpleString(&'a str),
    BulkString(String),
    Integer(i64),
}

#[derive(Debug, Hash, Copy)]
pub struct DataKey<'a> {
    key: &'a str,
}

impl<'a> Clone for DataKey<'a> {
    fn clone(&self) -> DataKey<'a> {
        *self
    }
}

impl<'a> DataKey<'a> {
    pub fn new(key: &'a str) -> DataKey {
        return DataKey { key: key };
    }
}

impl<'a> PartialEq for DataKey<'a> {
    fn eq(&self, other: &DataKey) -> bool {
        self.key == other.key
    }
}

impl<'a> Eq for DataKey<'a> {}

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


pub struct RequestParser;

impl RequestParser {

    // TODO: This method probably leaks memory because I still don't fully
    //       understand lifetimes.
    fn from_request<'a>(request: &'a str) -> DataType<'a> {
        let mut chars = request.chars();

        match chars.next().unwrap() {
            '+' => {
                let mut bundle = chars.as_str().split("\r\n");
                let data: &'a str = bundle.next().unwrap();

                return DataType::SimpleString(data);
            }
            _ => panic!("Invalid type.")
        }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_returns_simple_string_from_request() {
        let request = "+Ok\r\n";
        let simple_string = super::RequestParser::from_request(request);
        let expected = super::DataType::SimpleString("Ok");

        assert_eq!(expected, simple_string);
    }

    #[test]
    fn it_sets_and_gets_values() {
        let mut table = super::DataTable::new();
        let value = super::DataType::Integer(42);
        let data_key = super::DataKey::new("example");
        let expected = super::DataType::Integer(42);

        table.set(&data_key, value);
        assert_eq!(Some(&expected), table.get(&data_key));
    }

    #[test]
    fn it_returns_set_bulk_string() {
        let mut table = super::DataTable::new();
        let value = super::DataType::BulkString("test bulk string".to_string());
        let data_key = super::DataKey::new("example");
        let expected = super::DataType::BulkString("test bulk string".to_string());

        table.set(&data_key, value);
        assert_eq!(Some(&expected), table.get(&data_key));
    }

    #[test]
    fn it_returns_set_integer() {
        let mut table = super::DataTable::new();
        let value = super::DataType::SimpleString("test");
        let data_key = super::DataKey::new("example");
        let expected = super::DataType::SimpleString("test");

        table.set(&data_key, value);
        assert_eq!(Some(&expected), table.get(&data_key));
    }

    #[test]
    fn it_returns_none_when_fetching_empty_key() {
        let mut table = super::DataTable::new();
        let data_key = super::DataKey::new("example");

        assert_eq!(None, table.get(&data_key));
    }
}
