mod data_type;
mod qstring;
mod qinteger;
mod qarray;

pub use self::data_type::DataType;
pub use self::qarray::QArray;
pub use self::qstring::QString;
pub use self::qinteger::QInteger;

pub trait QDataType {
    fn to_data_type(&self) -> DataType;
    fn to_protocol(&self) -> String;
}

#[derive(Debug)]
pub enum QType {
    SimpleString(QString),
    BulkString(QString),
    Integer(QInteger),
    Array(Vec<QType>)
}
