extern crate quarrie;

use quarrie::storage::Database;
use quarrie::types::DataType;

#[test]
fn it_sets_and_gets_values() {
    let mut table = Database::new();
    let value = DataType::Integer(42);
    let expected = DataType::Integer(42);

    table.set("example", value);
    assert_eq!(Some(&expected), table.get("example"));
}

#[test]
fn it_returns_set_bulk_string() {
    let mut table = Database::new();
    let value = DataType::BulkString("test bulk string".to_string());
    let expected = DataType::BulkString("test bulk string".to_string());

    table.set("example", value);
    assert_eq!(Some(&expected), table.get("example"));
}

#[test]
fn it_returns_set_integer() {
    let mut table = Database::new();
    let value = DataType::SimpleString("test".to_string());
    let expected = DataType::SimpleString("test".to_string());

    table.set("example", value);
    assert_eq!(Some(&expected), table.get("example"));
}

#[test]
fn it_returns_none_when_fetching_empty_key() {
    let table = Database::new();

    assert_eq!(None, table.get("example"));
}
