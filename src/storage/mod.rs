extern crate chrono;
extern crate resp;
extern crate sled;

mod data_key;
mod data_value;

use self::data_value::LifetimeDatum;
use self::data_key::DataKey;
use self::chrono::Duration;
use self::resp::Value as DataValue;
use self::sled::Config as SledConfig;
use self::sled::Result as SledResult;

use std::io::Cursor;
use std::io::BufReader;
use self::resp::Decoder;

use std::collections::HashMap;
use std::collections::BTreeMap;


#[derive(Debug)]
pub struct KeyStore {
    keys: BTreeMap<String, DataKey>,
}

impl<'k> KeyStore {
    pub fn new() -> Self {
        Self { keys: BTreeMap::new() }
    }

    pub fn upsert(&mut self, k: &'k str) -> DataKey {
        match self.get(k) {
            Some(data_key) => data_key,
            None => {
                self.insert(k);
                self.upsert(k)
            }
        }
    }

    pub fn get(&mut self, k: &'k str) -> Option<DataKey> {
        match self.keys.get(k) {
            Some(k) => Some(k.clone()),
            None => None,
        }
    }

    pub fn insert(&mut self, k: &'k str) {
        self.keys.insert(
            k.to_string(),
            DataKey::new(k.to_string())
        );
    }
}

#[derive(Debug)]
pub struct Database {
    ttls: HashMap<String, LifetimeDatum>,
    data_keys: KeyStore,
    db: sled::Db,
}

impl Database {
    pub fn new() -> Self {
        Self {
            ttls: HashMap::new(),
            data_keys: KeyStore::new(),
            db: SledConfig::new().temporary(true).open().unwrap(),
        }
    }

    pub fn set(&mut self, key: String, value: DataValue) {
        let data_key = self.data_keys.upsert(&key);
        self.db.insert(data_key.ident(), value.encode());
    }

    pub fn get(&mut self, key: String) -> Option<DataValue> {
        match self.ttl(key.clone()) {
            Some(remaining) => {
                if remaining.is_zero() {
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
        let mut file = Cursor::new(vector);
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
                let ttl_datum = LifetimeDatum::new(duration);
                let data_key = self.data_keys.upsert(&key);

                self.ttls.insert(data_key.ident(), ttl_datum);
                Ok(DataValue::Integer(1))
            }
            None => {
                Err(DataValue::Integer(0))
            }
        }
    }

    pub fn ttl(&mut self, key: String) -> Option<Duration> {
        let key = self.data_keys.upsert(&key);

        match self.ttls.get(&key.ident()) {
            Some(duration) => Some(duration.remaining()),
            None => None,
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
