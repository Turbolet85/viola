//! `<home>/budget.json`: the newest usage reading across wrapped sessions, replaced whole under an
//! exclusive lock on the `budget.json.lock` sibling; last writer wins, and `viola hook statusline`
//! is its only writer (architecture §Occupied Resources, Viola home files).

use std::path::Path;

use chrono::{DateTime, Utc};
use serde::Serialize;
use viola_core::{BudgetReading, BudgetWindow, Reading};

use crate::StateError;
use crate::fs::{FILE_MODE, open_private_lock, replace_private};

pub const BUDGET: &str = "budget.json";
const BUDGET_LOCK: &str = "budget.json.lock";
const BUDGET_V: u32 = 1;

#[derive(Serialize)]
struct Envelope<'a> {
    v: u32,
    five_hour: &'a Reading<BudgetWindow>,
    seven_day: &'a Reading<BudgetWindow>,
    read_at: String,
}

/// A window's `used_percentage`, when both the window and the figure were read.
fn used(window: &Reading<BudgetWindow>) -> Option<f64> {
    match window {
        Reading::Known(BudgetWindow {
            used_percentage: Reading::Known(used),
            ..
        }) => Some(*used),
        _ => None,
    }
}

/// Replaces `<home>/budget.json` with `reading` and the instant it was read. The home must exist:
/// a reading never creates one.
#[tracing::instrument(
    skip_all,
    name = "state.budget_write",
    fields(five_hour_pct = used(&reading.five_hour), seven_day_pct = used(&reading.seven_day))
)]
pub fn write_budget(
    home: &Path,
    reading: &BudgetReading,
    read_at: DateTime<Utc>,
) -> Result<(), StateError> {
    let envelope = Envelope {
        v: BUDGET_V,
        five_hour: &reading.five_hour,
        seven_day: &reading.seven_day,
        read_at: crate::timestamp(read_at),
    };
    let bytes = serde_json::to_vec(&envelope)?;
    let lock = open_private_lock(&home.join(BUDGET_LOCK))?;
    lock.lock()?;
    replace_private(&home.join(BUDGET), &bytes, FILE_MODE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone as _;
    use std::fs;

    fn read_at() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 10, 18, 0, 0)
            .single()
            .expect("valid date")
            + chrono::Duration::milliseconds(7)
    }

    fn window(used_percentage: Reading<f64>, resets_at: Reading<String>) -> Reading<BudgetWindow> {
        Reading::Known(BudgetWindow {
            used_percentage,
            resets_at,
        })
    }

    fn at(text: &str) -> Reading<String> {
        Reading::Known(text.to_owned())
    }

    fn on_disk(home: &Path) -> String {
        fs::read_to_string(home.join("budget.json")).expect("budget.json")
    }

    #[test]
    fn budget_write_is_the_v_1_envelope_with_both_windows_and_read_at() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let reading = BudgetReading {
            five_hour: window(Reading::Known(91.0), at("2025-02-01T16:00:00.000Z")),
            seven_day: window(Reading::Known(23.5), at("2025-02-06T16:00:00.000Z")),
        };
        write_budget(tmp.path(), &reading, read_at()).expect("write");
        assert_eq!(
            on_disk(tmp.path()),
            r#"{"v":1,"five_hour":{"used_percentage":91.0,"resets_at":"2025-02-01T16:00:00.000Z"},"seven_day":{"used_percentage":23.5,"resets_at":"2025-02-06T16:00:00.000Z"},"read_at":"2026-10-10T18:00:00.007Z"}"#
        );
        assert!(tmp.path().join("budget.json.lock").is_file());
    }

    #[test]
    fn budget_write_keeps_each_unknown_form_as_the_word() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let reading = BudgetReading {
            five_hour: Reading::UNKNOWN,
            seven_day: window(Reading::UNKNOWN, Reading::UNKNOWN),
        };
        write_budget(tmp.path(), &reading, read_at()).expect("write");
        assert_eq!(
            on_disk(tmp.path()),
            r#"{"v":1,"five_hour":"unknown","seven_day":{"used_percentage":"unknown","resets_at":"unknown"},"read_at":"2026-10-10T18:00:00.007Z"}"#
        );
    }

    #[test]
    fn budget_write_a_second_write_replaces_the_first_whole() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let first = BudgetReading {
            five_hour: window(Reading::Known(91.0), at("2025-02-01T16:00:00.000Z")),
            seven_day: window(Reading::Known(23.5), at("2025-02-06T16:00:00.000Z")),
        };
        write_budget(tmp.path(), &first, read_at()).expect("first");
        let second = BudgetReading {
            five_hour: Reading::UNKNOWN,
            seven_day: Reading::UNKNOWN,
        };
        let later = read_at() + chrono::Duration::seconds(1);
        write_budget(tmp.path(), &second, later).expect("second");
        assert_eq!(
            on_disk(tmp.path()),
            r#"{"v":1,"five_hour":"unknown","seven_day":"unknown","read_at":"2026-10-10T18:00:01.007Z"}"#
        );
    }

    #[cfg(unix)]
    #[test]
    fn budget_write_leaves_both_files_owner_only() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("budget.json");
        fs::write(&path, b"{}").expect("seed");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).expect("chmod");
        let reading = BudgetReading {
            five_hour: Reading::UNKNOWN,
            seven_day: Reading::UNKNOWN,
        };
        write_budget(tmp.path(), &reading, read_at()).expect("write");
        for file in ["budget.json", "budget.json.lock"] {
            let mode = fs::metadata(tmp.path().join(file))
                .expect("meta")
                .permissions()
                .mode()
                & 0o777;
            assert_eq!(mode, 0o600, "{file}");
        }
    }

    #[test]
    fn budget_write_into_a_missing_home_fails_and_leaves_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("missing");
        let reading = BudgetReading {
            five_hour: Reading::UNKNOWN,
            seven_day: Reading::UNKNOWN,
        };
        assert!(write_budget(&home, &reading, read_at()).is_err());
        assert!(!home.exists());
        assert_eq!(fs::read_dir(tmp.path()).expect("the parent").count(), 0);
    }

    #[test]
    fn budget_write_percentages_are_the_ones_that_were_read() {
        assert_eq!(
            used(&window(Reading::Known(91.0), Reading::UNKNOWN)),
            Some(91.0)
        );
        assert_eq!(
            used(&window(Reading::Known(0.0), at("2025-02-01T16:00:00.000Z"))),
            Some(0.0)
        );
        assert_eq!(used(&window(Reading::UNKNOWN, Reading::UNKNOWN)), None);
        assert_eq!(used(&Reading::UNKNOWN), None);
    }
}
