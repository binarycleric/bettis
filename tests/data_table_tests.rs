extern crate quarrie;

use quarrie::storage::Database;
use quarrie::types::QType;

#[test]
fn it_sets_and_gets_values() {
    let mut table = Database::new();
    let value = QType::build_integer(42);
    let expected = QType::build_integer(42);

    table.set("example", value);
    assert_eq!(Some(&expected), table.get("example"));
}

#[test]
fn it_returns_set_bulk_string() {
    let mut table = Database::new();
    let value = QType::build_simple_string("test bulk string");
    let expected = QType::build_simple_string("test bulk string");

    table.set("example", value);
    assert_eq!(Some(&expected), table.get("example"));
}

#[test]
fn it_returns_set_integer() {
    let mut table = Database::new();
    let value = QType::build_simple_string("test");
    let expected = QType::build_simple_string("test");

    table.set("example", value);
    assert_eq!(Some(&expected), table.get("example"));
}

#[test]
fn it_returns_none_when_fetching_empty_key() {
    let table = Database::new();

    assert_eq!(None, table.get("example"));
}
