extern crate chrono;
extern crate resp;

use std::collections::HashMap;

use super::Database;
use super::DataKey;
use self::chrono::{DateTime, Utc, Duration};
use self::resp::Value as RespValue;

#[derive(Debug)]
pub struct LifetimeDatum {
    pub duration: Duration,
    pub started: DateTime<Utc>,
}

impl LifetimeDatum {

    pub fn new(duration: Duration) -> Self {
        Self {
            duration: duration,
            started: Utc::now(),
        }
    }

    pub fn remaining(&self) -> Duration {
        let now = Utc::now();
        let expire_on = self.started + self.duration;

        if expire_on < now {
            Duration::nanoseconds(0)
        } else {
            now - expire_on
        }
    }
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