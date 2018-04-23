#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    SimpleString(String),
    BulkString(String),
    Integer(i64),
    Array(Vec<DataType>),
}

use types::QDataType;
use types::QString;
use types::QInteger;

impl DataType {
    pub fn to_redis_protocol<'p>(&self) -> String {
        match *self {
            DataType::SimpleString(ref x) => {
                QString::from_string(x).to_protocol()
            }
            DataType::BulkString(ref x) => {
                QString::from_string(x).to_protocol_bulk()
            }
            DataType::Integer(ref x) => {
                QInteger::from_i64(*x).to_protocol()
            }
            DataType::Array(ref array) => {
                let mut retval = format!("*{}\r\n", array.len());
                for row in array {
                    retval.push_str(&row.to_redis_protocol());
                }

                return retval;
            }
        }
    }
}
