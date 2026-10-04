//! viola's disk state: owner-only creation, the one atomic replace, the append-only event log, the
//! snapshot envelope, the heartbeat, pid + start-time liveness, the pinned exe copy and the
//! capability stamps' bytes (architecture §Standard Contracts, §Occupied Resources → Filesystem).
//! Sync only: no Tokio.

pub mod events;
pub mod fs;
pub mod heartbeat;
pub mod liveness;
pub mod pin;
pub mod snapshot;
pub mod stamps;
pub mod strict;

use chrono::{DateTime, SecondsFormat, Utc};

/// Fixed messages only: no path and no payload reaches a `Display` (security-plan §Error Handling).
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("state file i/o failed")]
    Io(#[from] std::io::Error),
    #[error("state record could not be encoded")]
    Encode(#[from] serde_json::Error),
}

/// RFC 3339 UTC with milliseconds and `Z` (architecture §Conventions).
pub fn timestamp(at: DateTime<Utc>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone as _;

    #[test]
    fn timestamp_is_rfc3339_utc_millis_with_z() {
        let at = Utc
            .with_ymd_and_hms(2026, 9, 27, 1, 2, 3)
            .single()
            .expect("valid date")
            + chrono::Duration::milliseconds(45);
        assert_eq!(timestamp(at), "2026-09-27T01:02:03.045Z");
    }

    #[test]
    fn state_error_messages_are_fixed() {
        let io = StateError::from(std::io::Error::other("C:/secret/path"));
        assert_eq!(io.to_string(), "state file i/o failed");
        let encode =
            StateError::from(serde_json::from_str::<u8>("\"payload\"").expect_err("not a number"));
        assert_eq!(encode.to_string(), "state record could not be encoded");
    }
}
