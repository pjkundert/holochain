//! # Timestamp

#[allow(missing_docs)]
mod error;

use std::time::Duration;
use std::ops::{Add, Sub};
use std::convert::TryFrom;
use serde::{Serialize, Deserialize};
pub use error::{TimestampError, TimestampResult};

/// A UTC timestamp for use in Holochain's headers.
///
/// Timestamp implements `Serialize` and `Display` as rfc3339 time strings.
/// - Field 0: i64 - Seconds since UNIX epoch UTC (midnight 1970-01-01).
/// - Field 1: u32 - Nanoseconds in addition to above seconds.
/// 
/// Supports +/- std::time::Duration directly
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Timestamp(
    // sec
    pub i64,
    // nsec
    pub u32,
);

impl Timestamp {
    /// Create a new Timestamp instance from current system time.
    pub fn now() -> Self {
        chrono::offset::Utc::now().into()
    }
    /// Create a new Timestamp instance from the supplied secs/nsecs.
    pub fn new(secs: i64, nsecs: u32) -> Self {
        Self(secs, nsecs)
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t: chrono::DateTime<chrono::Utc> = self.into();
        write!(
            f,
            "{}",
            t.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true)
        )
    }
}

/// Infallible conversions into a Timestamp.  The only infallible ways to create a Timestamp are
/// `from` a Unix timestamp, or `new` with a timestamp and nanoseconds, or by converting to/from its
/// underlying DateTime<Utc>.

impl From<i64> for Timestamp {
    fn from(secs: i64) -> Self {
        Self::new(secs, 0)
    }
}

impl From<u64> for Timestamp {
    fn from(secs: u64) -> Self {
        Self::new(secs as i64, 0)
    }
}

impl From<i32> for Timestamp {
    fn from(secs: i32) -> Self {
        Self::new(secs.into(), 0)
    }
}

impl From<u32> for Timestamp {
    fn from(secs: u32) -> Self {
        Self::new(secs.into(), 0)
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Timestamp {
    fn from(t: chrono::DateTime<chrono::Utc>) -> Self {
        std::convert::From::from(&t)
    }
}

impl From<&chrono::DateTime<chrono::Utc>> for Timestamp {
    fn from(t: &chrono::DateTime<chrono::Utc>) -> Self {
        let t = t.naive_utc();
        Timestamp(t.timestamp(), t.timestamp_subsec_nanos())
    }
}

impl From<Timestamp> for chrono::DateTime<chrono::Utc> {
    fn from(t: Timestamp) -> Self {
        std::convert::From::from(&t)
    }
}

impl From<&Timestamp> for chrono::DateTime<chrono::Utc> {
    fn from(t: &Timestamp) -> Self {
        let t = chrono::naive::NaiveDateTime::from_timestamp(t.0, t.1);
        chrono::DateTime::from_utc(t, chrono::Utc)
    }
}

impl TryFrom<String> for Timestamp {
    type Error = TimestampError;

    fn try_from(t: String) -> Result<Self, Self::Error> {
        TryFrom::try_from(&t)
    }
}

impl TryFrom<&String> for Timestamp {
    type Error = TimestampError;

    fn try_from(t: &String) -> Result<Self, Self::Error> {
        let t: &str = &t;
        Ok(TryFrom::try_from(t)?)
    }
}

impl TryFrom<&str> for Timestamp {
    type Error = chrono::ParseError;

    fn try_from(t: &str) -> Result<Self, Self::Error> {
        let t = chrono::DateTime::parse_from_rfc3339(t)?;
        let t = chrono::DateTime::from_utc(t.naive_utc(), chrono::Utc);
        Ok(t.into())
    }
}

/// Timestamp +- Into<Duration>: Add anything that can be converted into a std::time::Duration
/// can be used as an overflow-checked offset for a Timestamp.
impl<D: Into<Duration>> Add<D> for Timestamp {
    type Output = TimestampResult<Timestamp>;
    fn add(self, rhs: D) -> Self::Output {
        let dur: Duration = rhs.into();
        Ok(chrono::DateTime::<chrono::Utc>::from(&self)
            .checked_add_signed(chrono::Duration::from_std(dur).or_else(|_e| {
                Err(TimestampError::Overflow)
	    })?)
            .ok_or_else(|| {
                TimestampError::Overflow
            })?
            .into())
    }
}

impl<D: Into<Duration>> Add<D> for &Timestamp {
    type Output = TimestampResult<Timestamp>;
    fn add(self, rhs: D) -> Self::Output {
        self.to_owned() + rhs
    }
}

impl<D: Into<Duration>> Sub<D> for Timestamp {
    type Output = TimestampResult<Timestamp>;
    fn sub(self, rhs: D) -> Self::Output {
        let dur: Duration = rhs.into();
        Ok(chrono::DateTime::<chrono::Utc>::from(&self)
            .checked_sub_signed(chrono::Duration::from_std(dur).or_else(|_e| {
                Err(TimestampError::Overflow)
            })?)
            .ok_or_else(|| {
                TimestampError::Overflow
            })?
            .into())
    }
}

impl<D: Into<Duration>> Sub<D> for &Timestamp {
    type Output = TimestampResult<Timestamp>;
    fn sub(self, rhs: D) -> Self::Output {
        self.to_owned() - rhs
    }
}
