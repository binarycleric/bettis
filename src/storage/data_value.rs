extern crate chrono;
extern crate resp;

use std::collections::HashMap;
use std::time::Duration;

use super::Database;
use super::DataKey;
use self::chrono::{DateTime, Utc};
use self::resp::Value as RespValue;

pub trait MetaDatum {}

struct TtlMetaDatum {
    pub ttl: Duration,
}

impl MetaDatum for TtlMetaDatum {}

pub struct MetaData<T: MetaDatum> {
    pub key: DataKey,
    pub datum: T,
}

pub struct DataValue {
    value: RespValue,
}

impl DataValue {
    pub fn new(value: RespValue) -> Self {
        Self {
            value: value,
        }
    }
}

impl Database {
    pub fn set_ttl(&mut self, key: String, ttl: Duration) {

    }

    pub fn ttl(&self, key: String) -> Option<Duration> {
        Some(Duration::from_secs(90))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_md() {
        let mut database = Database::new();
        let expected_ttl = Duration::from_secs(5);

        database.set_ttl("example".to_string(), expected_ttl);

        let actual_ttl = database.ttl("example".to_string());

        assert_eq!(Some(expected_ttl), actual_ttl);
    }

    #[test]
    fn it_does_things() {
        assert_eq!(true, false);
    }
}