mod data_type;
mod qsimple_string;
mod qarray;

pub use self::data_type::DataType;
pub use self::qarray::QArray;
pub use self::qsimple_string::QSimpleString;

pub trait QDataType {
    fn to_data_type(&self) -> DataType;
    fn to_protocol(&self) -> String;
}
