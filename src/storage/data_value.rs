extern crate chrono;
extern crate resp;

use self::chrono::{DateTime, Utc, Duration};

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_remaining_time() {
        let duration = Duration::seconds(1);
        let datum = LifetimeDatum::new(duration);

        assert!(datum.remaining() < duration);
    }
}
