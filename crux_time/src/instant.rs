use serde::{Deserialize, Serialize};

use crate::{error::TimeResult, TimeError};

/// The number of nanoseconds in seconds.
pub const NANOS_PER_SEC: u32 = 1_000_000_000;

/// Represents a point in time (UTC):
///
/// - seconds: number of seconds since the Unix epoch (1970-01-01T00:00:00Z)
/// - nanos: number of nanoseconds since the last second
///
/// Note: We cannot use `std::time::Instant` because it is not deserializable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instant {
    pub seconds: u64,
    pub nanos: u32,
}

/// Create a new `Instant` from the given number of seconds and nanoseconds.
///
/// - seconds: number of seconds since the Unix epoch (1970-01-01T00:00:00Z)
/// - nanos: number of nanoseconds since the last second
///
/// Errors with [`TimeError::InvalidDuration`] if the number of seconds
/// would overflow when converted to nanoseconds.
impl Instant {
    pub fn new(seconds: u64, nanos: u32) -> TimeResult<Self> {
        if nanos >= NANOS_PER_SEC {
            return Err(TimeError::InvalidInstant);
        }
        Ok(Self { seconds, nanos })
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<Instant> for chrono::DateTime<chrono::Utc> {
    type Error = TimeError;

    fn try_from(time: Instant) -> Result<Self, Self::Error> {
        let seconds = i64::try_from(time.seconds).map_err(|_| TimeError::InvalidInstant)?;
        chrono::DateTime::<chrono::Utc>::from_timestamp(seconds, time.nanos)
            .ok_or(TimeError::InvalidInstant)
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<chrono::DateTime<chrono::Utc>> for Instant {
    type Error = TimeError;

    fn try_from(time: chrono::DateTime<chrono::Utc>) -> Result<Self, Self::Error> {
        let seconds = time
            .timestamp()
            .try_into()
            .map_err(|_| TimeError::InvalidTime)?;
        let nanos = time.timestamp_subsec_nanos();
        Ok(Instant { seconds, nanos })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_instant() {
        let instant = Instant::new(1_000_000_000, 10).unwrap();
        assert_eq!(instant.seconds, 1_000_000_000);
        assert_eq!(instant.nanos, 10);
    }

    #[test]
    fn new_instant_invalid_nanos() {
        let instant = Instant::new(1_000_000_000, 1_000_000_000);
        assert_eq!(instant.unwrap_err(), TimeError::InvalidInstant);
    }
}

#[cfg(feature = "chrono")]
#[cfg(test)]
mod chrono_test {
    use chrono::{DateTime, TimeZone, Utc};

    use super::*;

    #[test]
    fn instant_to_datetime_utc() {
        let instant = Instant::new(1_000_000_000, 10).unwrap();
        let chrono_time: DateTime<Utc> = instant.try_into().unwrap();
        assert_eq!(chrono_time.timestamp(), 1_000_000_000);
        assert_eq!(chrono_time.timestamp_subsec_nanos(), 10);
    }

    #[test]
    fn datetime_utc_to_instant() {
        let chrono_time: DateTime<Utc> = Utc.timestamp_opt(1_000_000_000, 10).unwrap();
        let instant: Instant = chrono_time.try_into().unwrap();
        assert_eq!(instant.seconds, 1_000_000_000);
        assert_eq!(instant.nanos, 10);
    }
}
