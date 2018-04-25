use types::QDataType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QArray<T> where T: QDataType + Clone {
    values: Vec<T>,
}

impl<T: QDataType + Clone> QArray<T> {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn from_vec(values: Vec<T>) -> Self {
        Self { values: values }
    }
}

impl<T: QDataType + Clone> QDataType for QArray<T> {
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
    use types::QString;

    #[test]
    fn it_exports_into_protocol() {
        let bulk_strings = vec![
            QString::from_string("Example"),
            QString::from_string("Example"),
        ];

        let qarray: QArray<QString> = QArray::from_vec(bulk_strings);
        let protocol = qarray.to_protocol();

        assert_eq!("*2\r\n+Example\r\n+Example\r\n", protocol);
    }
}
