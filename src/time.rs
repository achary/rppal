//! Timestamp representation and utilities.

use std::ops;
use std::time::Duration;

/// An opaque representation of moment interrupt events.
///
/// Useful only when used together with [`Duration`] type.
/// Used primarily in [`gpio::InputPin::set_async_interrupt`] and the `callback` function.
///
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Instant(pub(crate) u128);

impl Instant {
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        // Accepting a limit that we can only work with time instances not distant more in time
        // than u64 allows expressing in nsecs. This is more than a 500 years or so. Unlikely to
        // hit exceed this limit within a single system run.
        #[allow(clippy::cast_possible_truncation)]
        Duration::from_nanos((self.0 - earlier.0) as u64)
    }

    /// Returns internal representation.
    ///
    /// This is exposed primarily for logging and debugging. Relying on returned values and type is
    /// discouraged.
    pub fn into_inner(self) -> u128 {
        self.0
    }
}

impl ops::Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, rhs: Duration) -> Instant {
        Instant(self.0 + rhs.as_nanos())
    }
}

impl ops::AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        self.duration_since(rhs)
    }
}

impl ops::Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, rhs: Duration) -> Instant {
        Instant(self.0 - rhs.as_nanos())
    }
}

impl ops::SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::time::Instant;
    use std::cmp::Ordering;
    use std::time::Duration;

    #[test]
    fn test_instance_duration_interplay() {
        let dur = Duration::from_nanos(200);
        let ts1 = Instant(100);
        let ts2 = Instant(300);

        assert_eq!(dur, ts2.duration_since(ts1));
        assert_eq!(dur, ts2 - ts1);
        assert_eq!(ts2, ts1 + dur);

        let mut ts = ts1;
        ts += dur;
        assert_eq!(ts2, ts);

        let mut ts = ts2;
        ts -= dur;
        assert_eq!(ts1, ts);
    }

    #[test]
    fn test_instance_properties() {
        let ts1 = Instant(100);
        let ts2 = Instant(300);

        assert_eq!(ts1, ts1);
        assert_eq!(ts1, ts1.clone());
        assert!(ts2 > ts1);
        assert!(ts2 >= ts1);
        assert_eq!(Ordering::Equal, ts1.cmp(&ts1));
        assert_eq!(Ordering::Greater, ts2.cmp(&ts1));
        assert_eq!(Ordering::Less, ts1.cmp(&ts2));
    }
}
