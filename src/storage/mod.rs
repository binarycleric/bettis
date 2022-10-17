extern crate chrono;
extern crate resp;
extern crate sled;

use self::chrono::{DateTime, Utc, NaiveDateTime, Duration};
use self::resp::Value as DataValue;

use std::str;
use std::io::Cursor;
use std::io::BufReader;
use self::resp::Decoder;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
    ttls: sled::Tree,
    db: sled::Tree,
}

impl Database {
    pub fn new() -> Self {
        let config = sled::Config::new().temporary(true);
        let db = config.open().unwrap();
        let tree = db.open_tree(format!("db-{}", 15)).unwrap();
        let ttls = db.open_tree(format!("ttls-{}", 15)).unwrap();

        Self {
            ttls: ttls,
            db: tree,
        }
    }

    pub fn set(&mut self, key: String, value: DataValue) {
        self.db.insert(&key, value.encode());
    }

    pub fn get(&mut self, key: String) -> Option<DataValue> {
        match self.ttl(key.clone()) {
            Some(remaining) => {

                println!("{:?}", remaining);
                println!("{:?}", remaining.is_zero());

                if remaining.is_zero() {
                    self.db.remove(&key);

                    None
                } else {
                    self.get_no_ttl(key.clone())
                }
            }
            None => {
                self.get_no_ttl(key.clone())
            }
        }
    }

    fn get_no_ttl(&mut self, key: String) -> Option<DataValue> {
        let vector = self.db.get(&key).unwrap().as_deref()?.to_vec();
        let file = Cursor::new(vector);
        let reader = BufReader::new(file);
        let mut decoder = Decoder::new(reader);
        let values = decoder.decode().unwrap();

        Some(values)
    }

    pub fn del(&mut self, key: String) -> Option<DataValue> {
        let value = self.get_no_ttl(key.clone());
        self.db.remove(&key);

        return value;
    }

    pub fn set_ttl(&mut self, key: String, duration: Duration) -> Result<DataValue, DataValue> {
        match self.get_no_ttl(key.clone()) {
            Some(_) => {
                let ttl = Utc::now() + duration;
                self.ttls.insert(key, &*ttl.to_rfc3339());

                Ok(DataValue::Integer(1))
            }
            None => {
                Err(DataValue::Integer(0))
            }
        }
    }

    pub fn ttl(&mut self, key: String) -> Option<Duration> {
        match self.ttls.get(&key) {
            Ok(None) => None,
            Ok(t) => {
                let datestring = str::from_utf8(t.as_deref().unwrap()).unwrap();
                let end_datetime = DateTime::parse_from_rfc3339(datestring)
                    .unwrap()
                    .with_timezone(&Utc);

                if end_datetime < Utc::now() {
                    Some(Duration::nanoseconds(0))
                } else {
                    Some(end_datetime - Utc::now())
                }
            },
            Err(_) => None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_KEY : &'static str  = "example";

    #[test]
    fn it_sets_a_keys_time_to_live() {
        let mut database = Database::new();
        let expected_ttl = Duration::seconds(5);

        database.set(TEST_KEY.to_string(), resp::Value::Integer(999));
        database.set_ttl(TEST_KEY.to_string(), expected_ttl);

        let actual_ttl = database.ttl(TEST_KEY.to_string());

        assert!(actual_ttl != None);
        assert!(actual_ttl.unwrap() < expected_ttl);
    }

    #[test]
    fn it_returns_nothing_on_expired_key() {
        let mut database = Database::new();

        let _ = database.set(TEST_KEY.to_string(), resp::Value::Integer(999));
        let _ = database.set_ttl(TEST_KEY.to_string(), Duration::nanoseconds(0));

        assert!(database.get(TEST_KEY.to_string()) == None);
    }
}
