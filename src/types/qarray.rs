use types::data_type::DataType;

#[derive(Debug, PartialEq, Clone)]
pub struct QArray {
    values: Vec<DataType>,
}

impl QArray {
    pub fn new() -> QArray {
        QArray { values: Vec::new() }
    }

    pub fn from_vec(values: Vec<DataType>) -> QArray {
        QArray { values: values }
    }
}

trait QDataType {
    fn to_redis_protocol<'p>(&self) -> String;
    fn to_data_type(&self) -> DataType;
}

impl QDataType for QArray {
    fn to_data_type(&self) -> DataType {
        DataType::Array(self.values.to_vec())
    }

    fn to_redis_protocol(&self) -> String {
        let mut retval = format!("*{}\r\n", self.values.len());

        for row in &self.values {
            retval.push_str(&row.to_redis_protocol());
        }
        return retval;
    }
}