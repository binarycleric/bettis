use types::data_type::DataType;
use types::QDataType;

#[derive(Debug, Clone)]
pub struct QSimpleString {
    value: String,
}

impl QSimpleString {
    pub fn from_string(string: String) -> QSimpleString {
        QSimpleString {
            value: string
        }
    }
}

impl QDataType for QSimpleString {
    fn to_data_type(&self) -> DataType {
        DataType::BulkString(self.value.to_string())
    }

    fn to_protocol(&self) -> String {
        format!("+{}\r\n", self.value)
    }
}
