//! `ledger/stamps.json`: the capability stamps `viola verify` writes and `viola run` reads
//! (security-plan §Data Protection; architecture §Occupied Resources → Filesystem). Bytes only:
//! their shape belongs to viola-agent-claude. `update_stamps` is the one writer path, replacing the
//! file whole under an exclusive lock on its `.lock` sibling.

use std::fs::File;
use std::io::{self, Read as _};
use std::path::{Path, PathBuf};

use viola_core::MAX_FRAME;

use crate::StateError;
use crate::fs::{FILE_MODE, create_private_dir, open_private_lock, replace_private};
use crate::strict::{self, Refused};

pub(crate) const STAMPS: &str = "stamps.json";
const STAMPS_LOCK: &str = "stamps.json.lock";

/// `<home>/ledger`, which also holds `viola verify`'s probe dirs.
pub fn ledger_dir(home: &Path) -> PathBuf {
    home.join("ledger")
}

/// The whole file, or `None` when it is absent; more than `MAX_FRAME` bytes is `over`.
fn read_capped(path: &Path) -> io::Result<Option<(Vec<u8>, bool)>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e),
    };
    let mut bytes = Vec::new();
    file.take(MAX_FRAME + 1).read_to_end(&mut bytes)?;
    let over = u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME;
    Ok(Some((bytes, over)))
}

/// Replaces the stamps with `update(current)` under the lock, for the whole read-modify-write. A
/// current file over the frame cap reads as absent.
#[tracing::instrument(skip_all, name = "state.ledger_write")]
pub fn update_stamps(
    home: &Path,
    update: impl FnOnce(Option<&[u8]>) -> Vec<u8>,
) -> Result<(), StateError> {
    let dir = ledger_dir(home);
    create_private_dir(&dir)?;
    let lock = open_private_lock(&dir.join(STAMPS_LOCK))?;
    lock.lock()?;
    let path = dir.join(STAMPS);
    let current = read_capped(&path)?.filter(|(_, over)| !over);
    let bytes = update(current.as_ref().map(|(b, _)| b.as_slice()));
    replace_private(&path, &bytes, FILE_MODE)
}

/// The stamps as they stand, `None` when absent; no lock is taken. More than `MAX_FRAME` bytes is
/// an error, never a partial read.
#[tracing::instrument(skip_all, name = "state.ledger_read")]
pub fn read_stamps(home: &Path) -> Result<Option<Vec<u8>>, StateError> {
    match read_capped(&ledger_dir(home).join(STAMPS))? {
        None => Ok(None),
        Some((_, true)) => Err(io::Error::from(io::ErrorKind::InvalidData).into()),
        Some((bytes, false)) => Ok(Some(bytes)),
    }
}

/// Why `run`'s stamps read did not return the stamps: fixed messages only.
#[derive(Debug, thiserror::Error)]
pub enum StampsReadError {
    #[error("the capability stamps failed the strict-modes check")]
    StrictModes(Refused),
    #[error("the capability stamps could not be read")]
    State(#[from] StateError),
}

/// [`read_stamps`] behind the strict-modes check of `ledger/` and the stamps file: a path another
/// user could write is refused before a byte is read (security-plan §Security Anti-Patterns ›
/// Universal: `run`'s read before any non-`null` dialog decision).
pub fn read_stamps_strict(home: &Path) -> Result<Option<Vec<u8>>, StampsReadError> {
    strict::check_stamps(home).map_err(StampsReadError::StrictModes)?;
    Ok(read_stamps(home)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const CAP: usize = 16 << 20;

    #[test]
    fn update_stamps_creates_the_file_and_its_lock_sibling() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        let mut seen = Some(b"sentinel".to_vec());
        update_stamps(&home, |current| {
            seen = current.map(<[u8]>::to_vec);
            b"first".to_vec()
        })
        .expect("written");
        assert_eq!(seen, None, "no stamps before the first write");
        let dir = home.join("ledger");
        assert_eq!(fs::read(dir.join("stamps.json")).expect("stamps"), b"first");
        assert!(dir.join("stamps.json.lock").is_file());
        assert_eq!(read_stamps(&home).expect("read"), Some(b"first".to_vec()));
    }

    #[test]
    fn update_stamps_hands_the_current_bytes_to_the_update() {
        let tmp = tempfile::tempdir().expect("tempdir");
        update_stamps(tmp.path(), |_| b"one".to_vec()).expect("first");
        update_stamps(tmp.path(), |current| {
            let mut next = current.expect("current bytes").to_vec();
            next.extend_from_slice(b"+two");
            next
        })
        .expect("second");
        assert_eq!(
            read_stamps(tmp.path()).expect("read"),
            Some(b"one+two".to_vec())
        );
    }

    #[test]
    fn update_stamps_reads_a_file_over_the_cap_as_absent() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("ledger");
        fs::create_dir_all(&dir).expect("dir");
        fs::write(dir.join("stamps.json"), vec![b' '; CAP + 1]).expect("seed");
        let mut seen = Some(Vec::new());
        update_stamps(tmp.path(), |current| {
            seen = current.map(<[u8]>::to_vec);
            b"fresh".to_vec()
        })
        .expect("written");
        assert_eq!(seen, None);
        assert_eq!(
            read_stamps(tmp.path()).expect("read"),
            Some(b"fresh".to_vec())
        );
    }

    #[test]
    fn update_stamps_takes_a_file_at_the_cap() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("ledger");
        fs::create_dir_all(&dir).expect("dir");
        fs::write(dir.join("stamps.json"), vec![b' '; CAP]).expect("seed");
        let mut len = 0;
        update_stamps(tmp.path(), |current| {
            len = current.map_or(0, <[u8]>::len);
            Vec::new()
        })
        .expect("written");
        assert_eq!(len, CAP);
    }

    #[test]
    fn update_stamps_under_a_file_fails_and_never_calls_the_update() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        fs::write(&home, b"x").expect("a file where the home should be");
        let mut called = false;
        let result = update_stamps(&home, |_| {
            called = true;
            Vec::new()
        });
        assert!(result.is_err());
        assert!(!called);
    }

    /// A lock another handle holds makes the writer wait; the stand-in is a second handle whose
    /// `try_lock` fails while the update runs.
    #[test]
    fn update_stamps_holds_the_lock_while_the_update_runs() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let lock_path = tmp.path().join("ledger").join("stamps.json.lock");
        let mut held = None;
        update_stamps(tmp.path(), |_| {
            let other = open_private_lock(&lock_path).expect("second handle");
            held = Some(other.try_lock().is_err());
            Vec::new()
        })
        .expect("written");
        assert_eq!(held, Some(true));
        let after = open_private_lock(&lock_path).expect("handle");
        after.try_lock().expect("free once the writer returned");
    }

    #[test]
    fn read_stamps_is_none_when_absent_and_refuses_over_the_cap() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(read_stamps(tmp.path()).expect("absent"), None);
        let dir = tmp.path().join("ledger");
        fs::create_dir_all(&dir).expect("dir");
        fs::write(dir.join("stamps.json"), vec![b' '; CAP + 1]).expect("seed");
        assert!(read_stamps(tmp.path()).is_err());
        fs::write(dir.join("stamps.json"), vec![b' '; CAP]).expect("seed");
        assert_eq!(
            read_stamps(tmp.path()).expect("at cap").map(|b| b.len()),
            Some(CAP)
        );
    }

    /// A directory where the file should be fails at open on Windows and at read on Unix: an error
    /// either way, never `None`.
    #[test]
    fn read_stamps_reports_an_unreadable_file() {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(tmp.path().join("ledger").join("stamps.json")).expect("dir");
        assert!(read_stamps(tmp.path()).is_err());
    }

    /// A path holding a NUL byte fails the open with a kind other than `NotFound`, before any
    /// filesystem call: only an absent file reads as `None`.
    #[test]
    fn read_stamps_of_a_file_that_cannot_be_opened_is_an_error() {
        assert!(read_stamps(Path::new("home\0dir")).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn update_stamps_writes_owner_only() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        update_stamps(tmp.path(), |_| b"x".to_vec()).expect("written");
        let mode = |p: &Path| fs::metadata(p).expect("meta").permissions().mode() & 0o777;
        let dir = tmp.path().join("ledger");
        assert_eq!(mode(&dir), 0o700);
        assert_eq!(mode(&dir.join("stamps.json")), 0o600);
        assert_eq!(mode(&dir.join("stamps.json.lock")), 0o600);
    }

    #[test]
    fn read_stamps_strict_reads_stamps_viola_wrote_and_an_absent_file() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert!(matches!(read_stamps_strict(tmp.path()), Ok(None)));
        update_stamps(tmp.path(), |_| b"stamped".to_vec()).expect("written");
        assert_eq!(
            read_stamps_strict(tmp.path()).expect("read"),
            Some(b"stamped".to_vec())
        );
    }

    /// A stamps file another user could write is refused before it is read, even with a stamp in
    /// it.
    #[cfg(unix)]
    #[test]
    fn read_stamps_strict_refuses_a_world_writable_stamps_file() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        update_stamps(tmp.path(), |_| b"stamped".to_vec()).expect("written");
        let path = tmp.path().join("ledger").join("stamps.json");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o666)).expect("chmod");
        let got = read_stamps_strict(tmp.path());
        assert!(
            matches!(got, Err(StampsReadError::StrictModes(Refused::Writable))),
            "{got:?}"
        );
        assert!(
            read_stamps(tmp.path()).is_ok(),
            "the plain read still reads it"
        );
    }

    #[test]
    fn stamps_read_error_messages_are_fixed() {
        assert_eq!(
            StampsReadError::StrictModes(Refused::Writable).to_string(),
            "the capability stamps failed the strict-modes check"
        );
        let io = StateError::Io(std::io::Error::other("C:/secret"));
        assert_eq!(
            StampsReadError::State(io).to_string(),
            "the capability stamps could not be read"
        );
    }
}
