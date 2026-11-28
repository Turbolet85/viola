//! The fixtures the board's tests share.

use viola_state::board::{Item, SkippedTotal};
use viola_state::snapshot::Wheel;

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
        dialog_pending: false,
        cli_version: Some("2.1.287".to_owned()),
        cli_verified: Some(true),
        dialog_kind: None,
        join: viola_state::board::JoinKeys::default(),
        parent: None,
        role: None,
        model: None,
        effort: None,
        window: None,
        origin: None,
        last_activity: None,
    }
}
