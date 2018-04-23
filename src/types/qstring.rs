use types::data_type::DataType;
use types::QDataType;

#[derive(Debug, Clone)]
pub struct QString {
    value: String,
}

impl QString {
    pub fn from_string<'a>(string: &'a str) -> Self {
        Self { value: string.to_string() }
    }

    pub fn to_protocol_bulk(&self) -> String {
        format!("${}\r\n{}\r\n", self.value.len(), self.value)
    }
}

impl QDataType for QString {
    fn to_data_type(&self) -> DataType {
        DataType::BulkString(self.value.to_string())
    }

    fn to_protocol(&self) -> String {
        format!("+{}\r\n", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::QDataType;
    use super::QString;

    #[test]
    fn it_parses_bulk_string_to_redis_protocol() {
        let redis_type = QString::from_string("Ok");
        assert_eq!("$2\r\nOk\r\n", redis_type.to_protocol_bulk())
    }

    #[test]
    fn it_parses_simple_string_to_redis_protocol() {
        let redis_type = QString::from_string("Ok");
        assert_eq!("+Ok\r\n", redis_type.to_protocol())
    }
}
