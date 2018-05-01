extern crate chrono;

use self::chrono::{DateTime, Utc};

use std::cmp::Ordering;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

impl PartialOrd for DataKey {
    fn partial_cmp(&self, other: &DataKey) -> Option<Ordering> {
        Some(self.cmp(other))
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
