extern crate chrono;

use self::chrono::{DateTime, Utc};

use std::cmp::Ordering;

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct DataKey {
    key: String,
    created: DateTime<Utc>,
    last_used: DateTime<Utc>,
}

impl Ord for DataKey {
    fn cmp(&self, other: &DataKey) -> Ordering {
        self.last_used.cmp(&other.last_used())
    }
}

impl DataKey {
    pub fn new(string: String) -> Self {
        let now = Utc::now();
        Self { key: string, created: now, last_used: now }
    }

    pub fn last_used(&self) -> DateTime<Utc> {
        self.last_used
    }
}
