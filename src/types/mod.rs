#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    SimpleString(String),
    BulkString(String),
    Integer(i64),
    Array(Vec<DataType>),
}

