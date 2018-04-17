use types::DataType;

const BULK_STRING_TOKEN: char = '$';
const SIMPLE_STRING_TOKEN: char = '+';
const ARRAY_TOKEN: char = '*';

struct ArrayParser<'a> {
    incoming: &'a str,
    current_idx: usize,
}

impl<'a> ArrayParser<'a> {
    fn new(size: usize, raw_array_data: &'a str) -> ArrayParser<'a> {
        let start_idx = 1 + size.to_string().len() + super::REDIS_SEPARATOR.len();
        let raw_array_contents = raw_array_data.get(start_idx..).unwrap();

        let parser = ArrayParser {
            incoming: raw_array_contents,
            current_idx: 0,
        };

        return parser;
    }

    // TODO: This maybe doesn't belong here.
    // TODO: Support for tested arrays.
    fn size_of_type_string(&self, request: &'a str) -> usize {
        let rtype: char = RequestParser::get_type(request);

        match rtype {
            SIMPLE_STRING_TOKEN => {
                let value_idx: usize = request.find(super::REDIS_SEPARATOR).unwrap_or(0);
                return value_idx;
            }
            BULK_STRING_TOKEN => {
                let size: usize = RequestParser::get_size(request).unwrap();

                return size + super::REDIS_SEPARATOR.len()
            }
            _ => {
                return 0;
            }
        }
    }
}

impl<'a> Iterator for ArrayParser<'a> {
    type Item = DataType<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.incoming.len() {
            return None
        }

        let type_str_len: usize = self.size_of_type_string(self.incoming);
        let raw_type_data = self.incoming.get(self.current_idx..type_str_len).unwrap();

        println!("-> {:?}", self.incoming);
        println!("-> {:?}", type_str_len);
        println!("--> {:?}", raw_type_data);

        self.current_idx = self.current_idx + type_str_len + super::REDIS_SEPARATOR.len();

         match RequestParser::from_str(self.incoming) {
             Ok(value) => {
                 return Some(value)
             }
             Err(_) => {
                 return None
             }
         }
    }
}

pub struct RequestParser;
impl RequestParser {
    fn get_type<'a>(request: &'a str) -> char {
        let rtype: char = request.get(0..1).unwrap().parse().unwrap();
        return rtype;
    }

    fn get_size<'a>(request: &'a str) -> Option<usize> {
        match request.find(super::REDIS_SEPARATOR) {
            Some(size_idx) => {
                let chunk = request.get(1..size_idx).unwrap();
                let size: usize = chunk.parse().unwrap();

                return Some(size);
            }
            None => {
                return None;
            }
        }
    }

    // TODO: This method probably leaks memory because I still don't fully
    //       understand lifetimes.
    pub fn from_str<'a>(request: &'a str) -> Result<DataType<'a>, &'static str> {
        let rtype: char = RequestParser::get_type(request);

        match rtype {
            SIMPLE_STRING_TOKEN => {
                let value_idx = request.get(1..).unwrap().find(super::REDIS_SEPARATOR).unwrap();
                let value = request.get(1..(value_idx + 1)).unwrap();

                return Ok(DataType::SimpleString(value))
            }
            BULK_STRING_TOKEN => {
                let size: usize = RequestParser::get_size(request).unwrap();
                let start_idx = 1 + size.to_string().len() + super::REDIS_SEPARATOR.len();
                let value = request.get(start_idx..(start_idx + size)).unwrap();

                return Ok(DataType::BulkString(value.to_string()));
            }
            ARRAY_TOKEN => {
                let size: usize = RequestParser::get_size(request).unwrap();
                let mut array: Vec<DataType<'a>> = Vec::with_capacity(size);
                let mut array_values = ArrayParser::new(size, request);

                for row in array_values {
                    array.push(row);
                }

                return Ok(DataType::Array(array))
            }
            _ => {
                return Err("Type not supported (yet).")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayParser;

    #[test]
    fn it_parses_array_with_simple_string() {
        let request = "*1\r\n+Ok\r\n";
        let size = 1;
        let mut parser = ArrayParser::new(size, request);
        let expected = super::DataType::SimpleString("Ok");

        assert_eq!(Some(expected), parser.next());
        assert_eq!(None, parser.next());
    }

    #[test]
    fn it_returns_simple_string_from_request() {
        let request = "+Ok\r\n";
        let simple_string = super::RequestParser::from_str(request);
        let expected = super::DataType::SimpleString("Ok");

        assert_eq!(Ok(expected), simple_string);
    }

    #[test]
    fn it_returns_array_from_request() {
        let request = "*1\r\n+Ok\r\n";
        let array = super::RequestParser::from_str(request);
        let expect_simple_string = super::DataType::SimpleString("Ok");
        let expected = super::DataType::Array(vec![expect_simple_string]);

        assert_eq!(Ok(expected), array);
    }

    #[test]
    fn it_returns_bulk_string_from_request() {
        let request = "$2\r\nOk\r\n";
        let bulk_string = super::RequestParser::from_str(request);
        let expect_bulk_string = super::DataType::BulkString("Ok".to_string());

        assert_eq!(Ok(expect_bulk_string), bulk_string);
    }
}