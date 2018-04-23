use types::data_type::DataType;
use types::QDataType;
use types::QSimpleString;

#[derive(Debug, Clone)]
pub struct QArray<T> where T: QDataType + Clone {
    values: Vec<T>,
}

impl<T: QDataType + Clone> QArray<T> {
    pub fn new() -> QArray<T> {
        QArray { values: Vec::new() }
    }

    pub fn from_vec(values: Vec<T>) -> QArray<T> {
        QArray { values: values }
    }
}

impl<T: QDataType + Clone> QDataType for QArray<T> {
    fn to_data_type(&self) -> DataType {
        let vec_values = Vec::new();

        DataType::Array(vec_values)
    }

    fn to_protocol(&self) -> String {
        let mut retval = format!("*{}\r\n", self.values.len());

        for row in &self.values {
            retval.push_str(&row.to_protocol());
        }
        return retval;
    }
}

#[cfg(test)]
mod tests {
    use super::QDataType;
    use super::QArray;
    use super::QSimpleString;

    #[test]
    fn it_exports_into_protocol() {
        let bulk_strings = vec![
            QSimpleString::from_string("Example".to_string()),
            QSimpleString::from_string("Example".to_string()),
        ];

        let qarray: QArray<QSimpleString> = QArray::from_vec(bulk_strings);
        let protocol = qarray.to_protocol();

        assert_eq!("*2\r\n+Example\r\n+Example\r\n", protocol);
    }
}
