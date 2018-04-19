use types::DataType;

const BULK_STRING_TOKEN: char = '$';
const SIMPLE_STRING_TOKEN: char = '+';
const ARRAY_TOKEN: char = '*';

#[derive(Debug)]
struct ArrayParser<'a> {
    array_data: &'a str,
    size: usize,
    current_idx: usize,
}

impl<'a> ArrayParser<'a> {
    fn new(array_data: &'a str) -> Result<ArrayParser<'a>, &'static str> {
        println!("New ArrayParser");
        println!("\n\n");

        let rtype: char = array_data.get(0..1).unwrap().parse().unwrap();
        // TODO: Check for correct rtype;
        let array_size: usize;
        let values_idx: usize;

        match array_data.find(super::REDIS_SEPARATOR) {
            Some(idx) => {
                array_size = array_data.get(1..idx).unwrap().parse().unwrap();
                values_idx = 1 + array_size.to_string().len() + super::REDIS_SEPARATOR.len();

                println!("array_size --> {:?}", array_size);
                println!("values_idx --> {:?}", values_idx);

                Ok(ArrayParser {
                    array_data: array_data,
                    size: array_size,
                    current_idx: values_idx,
                })
            }
            None => {
                Err("malformed string or something")
            }
        }
    }

    pub fn start_of_values_idx(&self) -> usize {
        return self.current_idx;
    }

    pub fn value_seperator_len(&self) -> usize {
        return super::REDIS_SEPARATOR.len();
    }

    fn get_current_value_string(&self) -> Option<&'a str> {
        return self.array_data.get(self.current_idx..);
    }

    fn get_current_type(&self) -> char {
        match self.get_current_value_string() {
            Some(value) => {
                value.get(..1).unwrap().parse::<char>().unwrap()
            }
            None => {
                panic!("I haven't figured the out yet");
            }
        }
    }

    fn next_type_value(&self) -> (&'a str, usize) {
        match self.get_current_value_string() {
            Some(value) => {
                let rtype = self.get_current_type();

                match rtype {
                    SIMPLE_STRING_TOKEN => {

                        println!("simple_string_value --> {:?}", value);

                        let end_of_value_idx = value.find(super::REDIS_SEPARATOR).unwrap() + self.value_seperator_len();
                        let current_value = value.get(..end_of_value_idx).unwrap();

                        println!("current_value --> {:?}", current_value);


                        return (current_value, end_of_value_idx);
                    }
                    _ => {
                        panic!("Fail to parse type: {:?}", rtype)
                    }
                }
            }
            None => {
                panic!("Fail to get next value")
            }
        }
    }
}

impl<'a> Iterator for ArrayParser<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        println!("Start of ArrayParser.next");

        if self.array_data.len() == self.current_idx {
            return None
        }

        let (type_value, next_value_idx) = self.next_type_value();

        println!("new_current_idx (Next) --> {:?}", next_value_idx);
        println!("raw_data_length (Next) --> {:?}", self.array_data.len());

        self.current_idx += next_value_idx;
        return Some(type_value);
    }
}

pub struct Parser<'a> {
    incoming: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(incoming: &'a str) -> Parser<'a> {
        return Parser { incoming: incoming };
    }

    pub fn get_type(&self) -> char {
      let rtype: char = self.incoming.get(0..1).unwrap().parse().unwrap();
      return rtype;
    }

    pub fn get_size(&self) -> Option<usize> {
        match self.incoming.find(super::REDIS_SEPARATOR) {
            Some(size_idx) => {
                let chunk = self.incoming.get(1..size_idx).unwrap();
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
    pub fn to_data_type(&self) -> Result<DataType<'a>, &'static str> {
        match self.get_type() {
            SIMPLE_STRING_TOKEN => {
                let value_idx: usize;

                match self.incoming.get(1..).unwrap().find(super::REDIS_SEPARATOR) {
                    Some(idx) => { value_idx = (idx + 1) }
                    None => { value_idx = self.incoming.len() }
                }

                match self.incoming.get(1..value_idx) {
                    Some(value) => { Ok(DataType::SimpleString(value)) },
                    None => {
                      println!("something went wrong! {:?}", self.incoming);

                      Err("Something went wrong!")
                    }
                }
            }
            BULK_STRING_TOKEN => {
                let size: usize = self.get_size().unwrap();
                let start_idx = 1 + size.to_string().len() + super::REDIS_SEPARATOR.len();

                println!("size --> : {:?}", self.get_size().unwrap().to_string().len());
                println!("size_idx --> : {:?}", start_idx);
                println!("incoming --> {:?}", self.incoming);


                let value = self.incoming.get(start_idx..(start_idx + size)).unwrap();

                return Ok(DataType::BulkString(value.to_string()));
            }
            ARRAY_TOKEN => {
                let size: usize = self.get_size().unwrap();
                let mut array: Vec<DataType<'a>> = Vec::with_capacity(size);
                let mut array_values = ArrayParser::new(self.incoming);

//                for row in array_values {
//                    array.push(row);
//                }

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
    fn it_builds_new_array_parser() {
        let request = "*1\r\n+Ok\r\n";
        let mut array_parser = ArrayParser::new(request).unwrap();

        assert_eq!(Some("+Ok\r\n"), array_parser.next());
        assert_eq!(None, array_parser.next());
    }

    #[test]
    fn array_parser_handles_two_simple_strings() {
        let request = "*1\r\n+Ok\r\n+Ok\r\n";
        let mut array_parser = ArrayParser::new(request).unwrap();

        assert_eq!(Some("+Ok\r\n"), array_parser.next());
        assert_eq!(Some("+Ok\r\n"), array_parser.next());
        assert_eq!(None, array_parser.next());
    }


/*
    #[test]
    fn it_returns_simple_string_from_request() {
        let request = "+Ok\r\n";
        let simple_string = super::Parser::new(request).to_data_type();
        let expected = super::DataType::SimpleString("Ok");

        assert_eq!(Ok(expected), simple_string);
    }

    #[test]
    fn it_parses_array_with_simple_string() {
        let request = "*1\r\n+Ok\r\n";
        let mut parser = ArrayParser::new(1, request);
        let expected = super::DataType::SimpleString("Ok");

        assert_eq!(Some(expected), parser.next());
        assert_eq!(None, parser.next());
    }

    #[test]
    fn it_parses_array_with_bulk_string() {
        let request = "*1\r\n$2\r\nOk\r\n";
        let mut parser = ArrayParser::new(1, request);
        let expected = super::DataType::BulkString("Ok".to_string());

        assert_eq!(Some(expected), parser.next());
        assert_eq!(None, parser.next());
    }

    #[test]
    fn it_returns_array_from_request() {
        let request = "*1\r\n+Ok\r\n";
        let array = super::Parser::new(request).to_data_type();
        let expect_simple_string = super::DataType::SimpleString("Ok");
        let expected = super::DataType::Array(vec![expect_simple_string]);

        assert_eq!(Ok(expected), array);
    }
*/

    #[test]
    fn it_returns_bulk_string_from_request() {
        let request = "$2\r\nOk\r\n";
        let bulk_string = super::Parser::new(request).to_data_type();
        let expect_bulk_string = super::DataType::BulkString("Ok".to_string());

        assert_eq!(Ok(expect_bulk_string), bulk_string);
    }
}