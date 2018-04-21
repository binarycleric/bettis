#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    SimpleString(String),
    BulkString(String),
    Integer(i64),
    Array(Vec<DataType>),
}

impl DataType {
    pub fn to_redis_protocol<'p>(&self) -> String {
        match *self {
            DataType::SimpleString(ref x) => {
                format!("+{}\r\n", x)
            }
            DataType::BulkString(ref x) => {
                format!("${}\r\n{}\r\n", x.len(), x)
            }
            DataType::Integer(ref x) => {
                format!(":{}\r\n", x)
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

#[cfg(test)]
mod tests {
    use super::DataType;

    #[test]
    fn it_parses_simple_string_to_redis_protocol() {
        let redis_type = DataType::SimpleString("Ok".to_string());
        assert_eq!("+Ok\r\n", redis_type.to_redis_protocol())
    }

    #[test]
    fn it_parses_bulk_string_to_redis_protocol() {
        let redis_type = DataType::BulkString("Ok".to_string());
        assert_eq!("$2\r\nOk\r\n", redis_type.to_redis_protocol())
    }

    #[test]
    fn it_parses_integer_to_redis_protocol() {
        let redis_type = DataType::Integer(42);
        assert_eq!(":42\r\n", redis_type.to_redis_protocol())
    }

    #[test]
    fn it_parses_array_to_redis_protocol() {
        let redis_type_1 = DataType::Integer(42);
        let redis_type_2 = DataType::Integer(43);
        let redis_array = DataType::Array(vec![redis_type_1, redis_type_2]);

        assert_eq!("*2\r\n:42\r\n:43\r\n", redis_array.to_redis_protocol())
    }

}
