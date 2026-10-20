//! The fixtures the board's tests share.

use chrono::{DateTime, Utc};
use viola_state::board::{Item, SkippedTotal};
use viola_state::snapshot::Wheel;

pub(super) fn at(text: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(text)
        .expect("an instant")
        .with_timezone(&Utc)
}

pub(super) const NOW: &str = "2025-03-01T12:00:00.000Z";

pub(super) fn counts(unknown_kinds: u64, unknown_fields: u64, torn_lines: u64) -> SkippedTotal {
    SkippedTotal {
        unknown_kinds,
        unknown_fields,
        torn_lines,
    }
}

pub(super) fn item(name: &str) -> Item {
    Item {
        name: name.to_owned(),
        wrapped: true,
        liveness: viola_state::board::RowLiveness::Live,
        status: viola_state::board::SessionStatus::Unknown,
        wheel: Some(Wheel::Driver),
        budget_paused: Some(false),
        dialog_pending: false,
        cli_version: Some("2.1.287".to_owned()),
        cli_verified: Some(true),
        dialog_kind: None,
        join: viola_state::board::JoinKeys::default(),
    }
}
