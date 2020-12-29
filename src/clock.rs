pub mod controls;

use crate::{DateTime, Utc};

pub trait Clock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    fn rfc3339(&self) -> String {
        self.now().to_rfc3339()
    }
}

#[cfg(test)]
mod tests {
    use crate::clock::{controls, Clock};
    use chrono::{DateTime, Utc};

    struct TestClock {
        now: DateTime<Utc>,
    }

    impl Clock for TestClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    #[test]
    fn creates_rfc3339_timestamps() {
        let now = controls::time();
        let expected = controls::rfc3339();

        let clock = TestClock { now };

        let time = clock.rfc3339();

        assert_eq!(expected, time);
    }
}
