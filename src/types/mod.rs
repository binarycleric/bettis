#[derive(Debug, PartialEq, Clone)]
pub enum DataType<'dtype> {
    SimpleString(&'dtype str),
    BulkString(&'dtype str),
    Integer(i64),
    Array(Vec<DataType<'dtype>>),
}

