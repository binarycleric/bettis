#[derive(Debug, PartialEq, Clone)]
pub enum DataType<'dtype> {
    SimpleString(&'dtype str),
    BulkString(&'dtype str),
    Integer(i64),
    Array(Vec<DataType<'dtype>>),
}

#[derive(Debug, Hash, Copy)]
pub struct DataKey<'dkey> {
    key: &'dkey str,
}

impl<'dkey> Clone for DataKey<'dkey> {
    fn clone(&self) -> DataKey<'dkey> {
        *self
    }
}

impl<'dkey> DataKey<'dkey> {
    pub fn new(key: &'dkey str) -> DataKey<'dkey> {
        return DataKey { key: key };
    }
}

impl<'dkey> PartialEq for DataKey<'dkey> {
    fn eq(&self, other: &DataKey) -> bool {
        self.key == other.key
    }
}

impl<'dkey> Eq for DataKey<'dkey> {}
