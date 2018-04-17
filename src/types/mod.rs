#[derive(Debug, PartialEq)]
pub enum DataType<'a> {
    SimpleString(&'a str),
    BulkString(String),
    Integer(i64),
    Array(Vec<DataType<'a>>),
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