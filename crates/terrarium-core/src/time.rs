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
    pub const fn hours(n: u64) -> Self { Self(n * 3600) }
    pub const fn days(n: u64) -> Self { Self(n * 86400) }

    /// Parse the small human-readable duration syntax used by experiment files.
    /// Examples: `30s`, `15m`, `2h30m`, `1d`.
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut total = 0_u64;
        let mut number = String::new();
        let mut saw_component = false;

        for ch in input.chars() {
            if ch.is_ascii_digit() {
                number.push(ch);
                continue;
            }

            if number.is_empty() {
                return Err(format!("duration has unit `{ch}` without a number"));
            }

            let value: u64 = number
                .parse()
                .map_err(|_| format!("invalid duration number `{number}`"))?;
            number.clear();
            saw_component = true;
            let multiplier = match ch {
                's' => 1,
                'm' => 60,
                'h' => 3_600,
                'd' => 86_400,
                _ => return Err(format!("unknown duration unit `{ch}`")),
            };
            total = total
                .checked_add(value.checked_mul(multiplier).ok_or_else(|| "duration overflow".to_string())?)
                .ok_or_else(|| "duration overflow".to_string())?;
        }

        if !number.is_empty() {
            return Err("duration must end in s, m, h, or d".into());
        }
        if !saw_component {
            return Err("duration is empty".into());
        }
        Ok(Self(total))
    }
}
impl Add<Duration> for SimTime { type Output=SimTime; fn add(self,rhs:Duration)->Self::Output{SimTime(self.0+rhs.0)} }
impl AddAssign<Duration> for SimTime { fn add_assign(&mut self,rhs:Duration){self.0+=rhs.0;} }
