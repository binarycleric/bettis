extern crate quarrie;

use quarrie::storage::DataTable;
use quarrie::types::{DataType, DataKey};

#[test]
fn it_sets_and_gets_values() {
    let mut table = DataTable::new();
    let value = DataType::Integer(42);
    let data_key = DataKey::new("example");
    let expected = DataType::Integer(42);

    table.set(&data_key, value);
    assert_eq!(Some(&expected), table.get(&data_key));
}

#[test]
fn it_returns_set_bulk_string() {
    let mut table = DataTable::new();
    let value = DataType::BulkString("test bulk string".to_string());
    let data_key = DataKey::new("example");
    let expected = DataType::BulkString("test bulk string".to_string());

    table.set(&data_key, value);
    assert_eq!(Some(&expected), table.get(&data_key));
}

#[test]
fn it_returns_set_integer() {
    let mut table = DataTable::new();
    let value = DataType::SimpleString("test");
    let data_key = DataKey::new("example");
    let expected = DataType::SimpleString("test");

    table.set(&data_key, value);
    assert_eq!(Some(&expected), table.get(&data_key));
}

#[test]
fn it_returns_none_when_fetching_empty_key() {
    let mut table = DataTable::new();
    let data_key = DataKey::new("example");

    assert_eq!(None, table.get(&data_key));
}