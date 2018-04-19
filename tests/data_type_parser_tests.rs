extern crate quarrie;

use quarrie::types::DataType;
use quarrie::parser::Parser;

#[test]
fn it_returns_simple_string_from_request() {
    let request = "+Ok\r\n";
    let simple_string = Parser::new(request).to_data_type();

    assert_eq!(Ok(DataType::SimpleString("Ok")), simple_string);
}

#[test]
fn it_returns_array_from_parser() {
    let request = "*1\r\n+Ok\r\n";
    let array = Parser::new(request).to_data_type();
    let expected = DataType::Array(vec![DataType::SimpleString("Ok")]);

    assert_eq!(Ok(expected), array);
}

#[test]
fn it_returns_bulk_string_from_parser() {
    let request = "$2\r\nOk\r\n";
    let bulk_string = Parser::new(request).to_data_type();
    let expect_bulk_string = DataType::BulkString("Ok".to_string());

    assert_eq!(Ok(expect_bulk_string), bulk_string);
}
