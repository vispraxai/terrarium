use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign};

/// Simulation time is independent of wall-clock time.
/// One tick currently represents one simulated second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SimTime(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Duration(pub u64);

impl Duration {
    pub const fn seconds(n: u64) -> Self { Self(n) }
    pub const fn minutes(n: u64) -> Self { Self(n * 60) }
    pub const fn hours(n: u64) -> Self { Self(n * 60 * 60) }
    pub const fn days(n: u64) -> Self { Self(n * 24 * 60 * 60) }
}

impl Add<Duration> for SimTime {
    type Output = SimTime;

    fn add(self, rhs: Duration) -> Self::Output {
        SimTime(self.0 + rhs.0)
    }
}

impl AddAssign<Duration> for SimTime {
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += rhs.0;
    }
}
