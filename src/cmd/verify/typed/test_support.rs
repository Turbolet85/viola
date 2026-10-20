//! The fixtures the typed runs' tests share.

use std::time::{Duration, Instant};

use viola_core::Clock;

pub(super) fn ms_(n: u64) -> Duration {
    Duration::from_millis(n)
}

pub(super) struct FixedClock(pub(super) Instant);

impl Clock for FixedClock {
    fn now(&self) -> Instant {
        self.0
    }
}
