//! `<home>/budget.json`: the newest usage reading across wrapped sessions, replaced whole under an
//! exclusive lock on the `budget.json.lock` sibling; last writer wins, and `viola hook statusline`
//! is its only writer (architecture §Occupied Resources, Viola home files).

use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use viola_core::{BudgetReading, BudgetWindow, MAX_FRAME, Reading};

use crate::StateError;
use crate::fs::{FILE_MODE, open_private_lock, replace_private};
use crate::strict::check_path;

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

/// The envelope as a reader takes it: a key it does not know is skipped.
#[derive(Deserialize)]
struct Stored {
    v: u32,
    five_hour: Reading<BudgetWindow>,
    seven_day: Reading<BudgetWindow>,
    #[serde(default)]
    read_at: Option<serde_json::Value>,
}

/// A reading with the instant its writer stored beside it.
#[derive(Debug, Clone, PartialEq)]
pub struct StampedReading {
    pub reading: BudgetReading,
    /// `"unknown"` when the envelope holds no string `read_at`.
    pub read_at: Reading<String>,
}

/// The reading `<home>/budget.json` holds, read under a shared lock on its sibling through the
/// frame cap, after the strict-modes check of the home and of the file (security-plan §Input
/// Validation, Own state files on read). `None` for an absent file, a refused check, more than
/// `MAX_FRAME` bytes, bytes that are no envelope, or a `v` other than 1. It never writes
/// `budget.json`, and an absent one leaves no file behind.
pub fn read_budget(home: &Path) -> Option<BudgetReading> {
    read_budget_stamped(home).map(|stamped| stamped.reading)
}

/// [`read_budget`] with the stored `read_at`: the same checks, lock and read, in the same order.
pub fn read_budget_stamped(home: &Path) -> Option<StampedReading> {
    let path = home.join(BUDGET);
    check_path(home).ok()?;
    check_path(&path).ok()?;
    let lock = open_private_lock(&home.join(BUDGET_LOCK)).ok()?;
    lock.lock_shared().ok()?;
    let mut bytes = Vec::new();
    File::open(&path)
        .ok()?
        .take(MAX_FRAME + 1)
        .read_to_end(&mut bytes)
        .ok()?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME {
        return None;
    }
    let stored: Stored = serde_json::from_slice(&bytes).ok()?;
    let read_at = match stored.read_at {
        Some(serde_json::Value::String(at)) => Reading::Known(at),
        _ => Reading::UNKNOWN,
    };
    (stored.v == BUDGET_V).then_some(StampedReading {
        reading: BudgetReading {
            five_hour: stored.five_hour,
            seven_day: stored.seven_day,
        },
        read_at,
    })
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

    /// A home laid down the way viola lays one down, so it passes the strict-modes check on the
    /// OS the test runs on.
    fn private_home(tmp: &tempfile::TempDir) -> std::path::PathBuf {
        let home = tmp.path().join("vhome");
        crate::fs::create_private_dir(&home).expect("the home");
        home
    }

    fn both_windows() -> BudgetReading {
        BudgetReading {
            five_hour: window(Reading::Known(91.0), at("2025-02-01T16:00:00.000Z")),
            seven_day: window(Reading::UNKNOWN, Reading::UNKNOWN),
        }
    }

    #[test]
    fn budget_read_returns_what_write_budget_wrote() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = private_home(&tmp);
        write_budget(&home, &both_windows(), read_at()).expect("write");
        assert_eq!(read_budget(&home), Some(both_windows()));
        let unknown = BudgetReading {
            five_hour: Reading::UNKNOWN,
            seven_day: Reading::UNKNOWN,
        };
        write_budget(&home, &unknown, read_at()).expect("write");
        assert_eq!(read_budget(&home), Some(unknown));
    }

    #[test]
    fn budget_read_of_an_absent_file_is_nothing_and_creates_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = private_home(&tmp);
        assert_eq!(read_budget(&home), None);
        assert_eq!(fs::read_dir(&home).expect("the home").count(), 0);
        assert_eq!(read_budget(&tmp.path().join("missing")), None);
        assert_eq!(fs::read_dir(&home).expect("the home").count(), 0);
    }

    /// The written envelope padded with trailing white space: exactly the cap is still that
    /// envelope, one byte more is nothing.
    #[test]
    fn budget_read_over_the_frame_cap_is_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = private_home(&tmp);
        write_budget(&home, &both_windows(), read_at()).expect("write");
        let path = home.join("budget.json");
        let cap = usize::try_from(MAX_FRAME).expect("the cap");
        let mut bytes = fs::read(&path).expect("budget.json");
        bytes.resize(cap, b' ');
        fs::write(&path, &bytes).expect("padded to the cap");
        assert_eq!(read_budget(&home), Some(both_windows()));
        bytes.push(b' ');
        fs::write(&path, &bytes).expect("one byte over");
        assert_eq!(read_budget(&home), None);
    }

    #[test]
    fn budget_read_of_bytes_that_are_no_envelope_is_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = private_home(&tmp);
        write_budget(&home, &both_windows(), read_at()).expect("write");
        let path = home.join("budget.json");
        for bytes in [
            "",
            "not json",
            "[1]",
            r#"{"v":1}"#,
            r#"{"v":1,"five_hour":7,"seven_day":"unknown"}"#,
            r#"{"v":"1","five_hour":"unknown","seven_day":"unknown"}"#,
        ] {
            fs::write(&path, bytes).expect("planted");
            assert_eq!(read_budget(&home), None, "{bytes:?}");
        }
    }

    #[test]
    fn budget_read_of_another_v_is_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = private_home(&tmp);
        write_budget(&home, &both_windows(), read_at()).expect("write");
        let path = home.join("budget.json");
        let written = fs::read_to_string(&path).expect("budget.json");
        for v in ["0", "2"] {
            let other = written.replacen(r#"{"v":1,"#, &format!(r#"{{"v":{v},"#), 1);
            assert_ne!(other, written);
            fs::write(&path, other).expect("planted");
            assert_eq!(read_budget(&home), None, "v {v}");
        }
        fs::write(&path, written).expect("restored");
        assert_eq!(read_budget(&home), Some(both_windows()));
    }

    /// A key a newer writer adds beside the known ones is skipped.
    #[test]
    fn budget_read_skips_a_key_it_does_not_know() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = private_home(&tmp);
        write_budget(&home, &both_windows(), read_at()).expect("write");
        let path = home.join("budget.json");
        let written = fs::read_to_string(&path).expect("budget.json");
        let wider = written.replacen(r#"{"v":1,"#, r#"{"v":1,"later":{"a":1},"#, 1);
        fs::write(&path, wider).expect("planted");
        assert_eq!(read_budget(&home), Some(both_windows()));
    }

    #[cfg(unix)]
    fn set_mode(path: &Path, mode: u32) {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(path, fs::Permissions::from_mode(mode)).expect("chmod");
    }

    #[cfg(unix)]
    fn mode_of(path: &Path) -> u32 {
        use std::os::unix::fs::PermissionsExt as _;
        fs::metadata(path).expect("meta").permissions().mode() & 0o777
    }

    /// A home, or a `budget.json`, that another user could write gives no reading, and the read
    /// narrows neither.
    #[cfg(unix)]
    #[test]
    fn budget_read_under_a_path_another_user_could_write_is_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = private_home(&tmp);
        write_budget(&home, &both_windows(), read_at()).expect("write");
        let path = home.join("budget.json");
        assert_eq!(read_budget(&home), Some(both_windows()));

        set_mode(&home, 0o770);
        assert_eq!(read_budget(&home), None);
        assert_eq!(mode_of(&home), 0o770);
        set_mode(&home, 0o700);

        set_mode(&path, 0o620);
        assert_eq!(read_budget(&home), None);
        assert_eq!(mode_of(&path), 0o620);
        set_mode(&path, 0o600);
        assert_eq!(read_budget(&home), Some(both_windows()));
    }

    #[test]
    fn budget_read_leaves_the_file_as_it_was() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = private_home(&tmp);
        write_budget(&home, &both_windows(), read_at()).expect("write");
        let path = home.join("budget.json");
        let before = fs::read(&path).expect("budget.json");
        let modified = fs::metadata(&path)
            .expect("meta")
            .modified()
            .expect("mtime");
        assert!(read_budget(&home).is_some());
        assert_eq!(fs::read(&path).expect("budget.json"), before);
        assert_eq!(
            fs::metadata(&path)
                .expect("meta")
                .modified()
                .expect("mtime"),
            modified
        );
        #[cfg(unix)]
        assert_eq!(mode_of(&path), 0o600);
    }

    #[test]
    fn budget_stamped_read_returns_the_instant_write_budget_wrote() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = private_home(&tmp);
        write_budget(&home, &both_windows(), read_at()).expect("write");
        assert_eq!(
            read_budget_stamped(&home),
            Some(StampedReading {
                reading: both_windows(),
                read_at: at("2026-10-10T18:00:00.007Z"),
            })
        );
    }

    #[test]
    fn budget_stamped_read_of_an_envelope_without_a_string_read_at_is_unknown() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = private_home(&tmp);
        write_budget(&home, &both_windows(), read_at()).expect("write");
        let path = home.join("budget.json");
        let written = fs::read_to_string(&path).expect("budget.json");
        let stamp = r#","read_at":"2026-10-10T18:00:00.007Z""#;
        assert!(written.contains(stamp));
        for replacement in ["", r#","read_at":7"#, r#","read_at":null"#] {
            fs::write(&path, written.replacen(stamp, replacement, 1)).expect("planted");
            assert_eq!(
                read_budget_stamped(&home),
                Some(StampedReading {
                    reading: both_windows(),
                    read_at: Reading::UNKNOWN,
                }),
                "{replacement:?}"
            );
            assert_eq!(read_budget(&home), Some(both_windows()));
        }
    }

    #[test]
    fn budget_stamped_read_of_an_absent_file_is_nothing_and_creates_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = private_home(&tmp);
        assert_eq!(read_budget_stamped(&home), None);
        assert_eq!(fs::read_dir(&home).expect("the home").count(), 0);
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
