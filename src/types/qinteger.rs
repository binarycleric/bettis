use types::QDataType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QInteger {
    value: i64,
}

impl QInteger {
    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn from_i64(value: i64) -> Self {
        Self { value: value }
    }
}

impl QDataType for QInteger {
    fn to_protocol(&self) -> String {
        format!(":{}\r\n", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::QDataType;
    use super::QInteger;

    #[test]
    fn it_parses_integer_to_protocol() {
        let redis_type = QInteger::from_i64(42);
        assert_eq!(":42\r\n", redis_type.to_protocol())
    }
}