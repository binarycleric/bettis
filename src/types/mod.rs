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

#[derive(Debug, PartialEq, Eq)]
pub enum QType {
    SimpleString(QString),
    BulkString(QString),
    Integer(QInteger),
    Array(QArray<QString>)
}

impl QType {
    pub fn build_bulk_string<'a>(str: &'a str) -> QType {
        QType::BulkString(QString::from_string(str))
    }

    pub fn build_simple_string<'a>(str: &'a str) -> QType {
        QType::SimpleString(QString::from_string(str))
    }

    pub fn build_integer(int: i64) -> QType {
        QType::Integer(QInteger::from_i64(int))
    }

    pub fn from_string<'a>(str: &'a str) -> QType {
        QType::SimpleString(QString::from_string(str))
    }

    pub fn from_i64(int: i64) -> QType {
        QType::Integer(QInteger::from_i64(int))
    }

    pub fn to_i64(&self) -> Result<i64, &'static str> {
        match self {
            &QType::Integer(ref int) => Ok(int.value()),
            _ =>  Err("Not an integer"),
        }
    }

    pub fn to_protocol(&self) -> String {
        match self {
            &QType::SimpleString(ref x) => x.to_protocol(),
            &QType::BulkString(ref x) => x.to_protocol(),
            &QType::Integer(ref x) => x.to_protocol(),
            &QType::Array(ref x) => x.to_protocol(),
        }
    }
}