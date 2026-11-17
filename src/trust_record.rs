//! The trust record's writer, built once: `viola verify`'s trust-record run writes and removes its
//! own record through it. A write or a removal is at most [`ATTEMPTS`] attempts of: the file
//! checked through `symlink_metadata` (present, no link, a regular file, this user's); the CLI's own
//! lock taken only while free; the file read without following a link through the frame cap; the
//! pure change (`viola_agent_claude::trust`); the one atomic replace with the file's own mode; this
//! call's lock removed; the file read back. Its outcome is one closed word, and it writes no log
//! line of its own: the caller's record carries it.

use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::Read as _;
use std::os::unix::fs::{DirBuilderExt as _, MetadataExt as _, OpenOptionsExt as _};
use std::path::{Path, PathBuf};
use std::time::Duration;

use viola_agent_claude::trust::{self, LOCK_SUFFIX, TrustWord};
use viola_core::MAX_FRAME;
use viola_state::fs::replace_private;

/// A write or a removal makes at most three attempts.
pub(crate) const ATTEMPTS: u32 = 3;
/// The pause after an attempt that found the CLI's lock held.
const LOCK_PAUSE: Duration = Duration::from_millis(100);

/// The nearest ancestor of `cwd` (itself included) holding a `.git` entry that is a directory or a
/// regular file, read without following a link; a link there is no root.
pub(crate) fn git_root(cwd: &Path) -> Option<PathBuf> {
    cwd.ancestors()
        .find(|dir| {
            fs::symlink_metadata(dir.join(".git"))
                .is_ok_and(|meta| meta.file_type().is_dir() || meta.file_type().is_file())
        })
        .map(Path::to_path_buf)
}

/// `projects[key] = {"hasTrustDialogAccepted": true}` written into `file`: `written` once it reads
/// back, else the closed word for what stood in the way.
pub(crate) fn write_record(file: &Path, key: &str) -> TrustWord {
    change(file, key, Change::Write, effective_uid(), &mut |_| {})
}

/// The whole entry at `projects[key]` removed from `file`: `removed` once it reads back gone.
pub(crate) fn remove_record(file: &Path, key: &str) -> TrustWord {
    change(file, key, Change::Remove, effective_uid(), &mut |_| {})
}

fn effective_uid() -> u32 {
    // SAFETY: `geteuid` takes no arguments, cannot fail and touches no memory.
    unsafe { libc::geteuid() }
}

/// Whether a file owned by `owner` is this user's, `euid`.
fn owned_by(owner: u32, euid: u32) -> bool {
    owner == euid
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Change {
    Write,
    Remove,
}

/// One attempt's end: a word that stands, or the next attempt with the word the last one leaves.
enum Attempt {
    Done(TrustWord),
    Again(TrustWord),
}

/// [`write_record`] / [`remove_record`] over `euid`, with `read_back` called on `file` between the
/// replace and the read-back: the window a racing writer would take.
fn change(
    file: &Path,
    key: &str,
    change: Change,
    euid: u32,
    read_back: &mut dyn FnMut(&Path),
) -> TrustWord {
    let mut last = TrustWord::Lost;
    for n in 1..=ATTEMPTS {
        match attempt(file, key, change, euid, read_back) {
            Attempt::Done(word) => return word,
            Attempt::Again(word) => {
                last = word;
                if word == TrustWord::LockHeld && n < ATTEMPTS {
                    std::thread::sleep(LOCK_PAUSE);
                }
            }
        }
    }
    last
}

fn attempt(
    file: &Path,
    key: &str,
    change: Change,
    euid: u32,
    read_back: &mut dyn FnMut(&Path),
) -> Attempt {
    if let Err(word) = checked(file, euid) {
        return Attempt::Done(word);
    }
    let Some(lock) = Lock::take(file) else {
        return Attempt::Again(TrustWord::LockHeld);
    };
    let (bytes, mode) = match read(file) {
        Ok(read) => read,
        Err(word) => return Attempt::Done(word),
    };
    if change == Change::Remove && !trust::holds_entry(&bytes, key) {
        return Attempt::Done(TrustWord::Removed);
    }
    let changed = match change {
        Change::Write => trust::with_record(&bytes, key),
        Change::Remove => trust::without_record(&bytes, key),
    };
    let changed = match changed {
        Ok(changed) => changed,
        Err(word) => return Attempt::Done(word),
    };
    let replaced = replace_private(file, &changed, mode & 0o7777).is_ok();
    drop(lock);
    if !replaced {
        return Attempt::Again(TrustWord::Lost);
    }
    read_back(file);
    let held = read(file).is_ok_and(|(bytes, _)| match change {
        Change::Write => trust::holds_record(&bytes, key),
        Change::Remove => !trust::holds_entry(&bytes, key),
    });
    match (held, change) {
        (true, Change::Write) => Attempt::Done(TrustWord::Written),
        (true, Change::Remove) => Attempt::Done(TrustWord::Removed),
        (false, _) => Attempt::Again(TrustWord::Lost),
    }
}

/// The file as it stands, read through `symlink_metadata` before any open.
fn checked(file: &Path, euid: u32) -> Result<(), TrustWord> {
    let meta = match fs::symlink_metadata(file) {
        Ok(meta) => meta,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(TrustWord::ConfigAbsent);
        }
        Err(_) => return Err(TrustWord::ConfigUnreadable),
    };
    if meta.file_type().is_symlink() {
        Err(TrustWord::ConfigLink)
    } else if !meta.file_type().is_file() {
        Err(TrustWord::ConfigNotRegular)
    } else if !owned_by(meta.uid(), euid) {
        Err(TrustWord::ConfigForeign)
    } else {
        Ok(())
    }
}

/// The file's bytes and mode, opened without following a link, read through the frame cap.
fn read(file: &Path) -> Result<(Vec<u8>, u32), TrustWord> {
    let opened: File = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(file)
        .map_err(|_| TrustWord::ConfigUnreadable)?;
    let mode = opened
        .metadata()
        .map_err(|_| TrustWord::ConfigUnreadable)?
        .mode();
    let mut bytes = Vec::new();
    opened
        .take(MAX_FRAME + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| TrustWord::ConfigUnreadable)?;
    if u64::try_from(bytes.len()).is_ok_and(|n| n > MAX_FRAME) {
        return Err(TrustWord::ConfigOversize);
    }
    Ok((bytes, mode))
}

/// The CLI's lock, `<file>.lock`, a directory this call created; removed on drop, and only then.
struct Lock(PathBuf);

impl Lock {
    /// `None` when the lock stands (held) or cannot be made.
    fn take(file: &Path) -> Option<Self> {
        let mut name = file.as_os_str().to_owned();
        name.push(LOCK_SUFFIX);
        let path = PathBuf::from(name);
        DirBuilder::new().mode(0o700).create(&path).ok()?;
        Some(Self(path))
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        let _ = fs::remove_dir(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use std::os::unix::fs::PermissionsExt as _;

    const KEY: &str = "/t/viola-verify-x";

    fn config(dir: &Path, body: &Value, mode: u32) -> PathBuf {
        let file = dir.join(".claude.json");
        fs::write(&file, body.to_string()).expect("seed");
        fs::set_permissions(&file, fs::Permissions::from_mode(mode)).expect("mode");
        file
    }

    fn doc(file: &Path) -> Value {
        serde_json::from_slice(&fs::read(file).expect("read")).expect("json")
    }

    fn mode(file: &Path) -> u32 {
        fs::metadata(file).expect("meta").permissions().mode() & 0o777
    }

    fn lock_of(file: &Path) -> PathBuf {
        PathBuf::from(format!("{}.lock", file.display()))
    }

    #[test]
    fn trust_record_written_then_removed_keeps_every_other_key_and_the_mode() {
        let tmp = tempfile::tempdir().expect("tmp");
        let seed = json!({"a": 1, "projects": {"/s": {"hasTrustDialogAccepted": true, "k": [2]}}, "z": "x"});
        let file = config(tmp.path(), &seed, 0o644);
        assert_eq!(write_record(&file, KEY), TrustWord::Written);
        let written = doc(&file);
        assert_eq!(
            written["projects"][KEY],
            json!({"hasTrustDialogAccepted": true})
        );
        assert_eq!(written["projects"]["/s"], seed["projects"]["/s"]);
        assert_eq!((&written["a"], &written["z"]), (&json!(1), &json!("x")));
        assert_eq!(mode(&file), 0o644, "the file's own mode kept");
        assert!(!lock_of(&file).exists(), "the lock this call made is gone");
        assert_eq!(remove_record(&file, KEY), TrustWord::Removed);
        assert_eq!(doc(&file), seed);
        assert_eq!(mode(&file), 0o644);
        assert_eq!(
            remove_record(&file, KEY),
            TrustWord::Removed,
            "nothing left to remove"
        );
        assert_eq!(doc(&file), seed);
    }

    #[test]
    fn trust_record_an_absent_config_is_never_created() {
        let tmp = tempfile::tempdir().expect("tmp");
        let file = tmp.path().join(".claude.json");
        assert_eq!(write_record(&file, KEY), TrustWord::ConfigAbsent);
        assert_eq!(remove_record(&file, KEY), TrustWord::ConfigAbsent);
        assert_eq!(fs::read_dir(tmp.path()).expect("dir").count(), 0);
    }

    #[test]
    fn trust_record_refuses_a_link_and_a_dir_untouched() {
        let tmp = tempfile::tempdir().expect("tmp");
        let target = config(tmp.path(), &json!({}), 0o600);
        let link = tmp.path().join("linked.json");
        std::os::unix::fs::symlink(&target, &link).expect("link");
        assert_eq!(write_record(&link, KEY), TrustWord::ConfigLink);
        assert_eq!(doc(&target), json!({}));
        let dir = tmp.path().join("dir.json");
        fs::create_dir(&dir).expect("dir");
        assert_eq!(write_record(&dir, KEY), TrustWord::ConfigNotRegular);
        assert!(!lock_of(&link).exists() && !lock_of(&dir).exists());
    }

    #[test]
    fn trust_record_refuses_an_oversize_file() {
        let tmp = tempfile::tempdir().expect("tmp");
        let file = tmp.path().join(".claude.json");
        let size = usize::try_from(MAX_FRAME + 1).expect("fits");
        fs::write(&file, vec![b' '; size]).expect("big");
        assert_eq!(write_record(&file, KEY), TrustWord::ConfigOversize);
        assert_eq!(fs::metadata(&file).expect("meta").len(), MAX_FRAME + 1);
        assert!(!lock_of(&file).exists());
    }

    #[test]
    fn trust_record_refuses_a_malformed_document_and_a_present_key() {
        let tmp = tempfile::tempdir().expect("tmp");
        for (label, body, word) in [
            ("not an object", json!([1]), TrustWord::ConfigMalformed),
            (
                "projects an array",
                json!({"projects": []}),
                TrustWord::ConfigMalformed,
            ),
            (
                "the key present",
                json!({"projects": {KEY: {"other": 1}}}),
                TrustWord::KeyPresent,
            ),
        ] {
            let file = config(tmp.path(), &body, 0o600);
            assert_eq!(write_record(&file, KEY), word, "{label}");
            assert_eq!(doc(&file), body, "{label}: left as found");
            assert!(!lock_of(&file).exists(), "{label}");
        }
    }

    #[test]
    fn trust_record_held_lock_writes_nothing_and_stays() {
        let tmp = tempfile::tempdir().expect("tmp");
        let file = config(tmp.path(), &json!({}), 0o600);
        let lock = lock_of(&file);
        fs::create_dir(&lock).expect("a lock another process holds");
        fs::write(lock.join("held"), b"x").expect("its content");
        assert_eq!(write_record(&file, KEY), TrustWord::LockHeld);
        assert_eq!(remove_record(&file, KEY), TrustWord::LockHeld);
        assert_eq!(doc(&file), json!({}));
        assert_eq!(fs::read(lock.join("held")).expect("left as found"), b"x");
    }

    #[test]
    fn trust_record_lost_three_times_reads_lost() {
        let tmp = tempfile::tempdir().expect("tmp");
        let file = config(tmp.path(), &json!({"b": 1}), 0o600);
        let mut raced = 0;
        let word = change(&file, KEY, Change::Write, effective_uid(), &mut |path| {
            raced += 1;
            fs::write(path, br#"{"b":2}"#).expect("a racing writer");
        });
        assert_eq!(word, TrustWord::Lost);
        assert_eq!(raced, ATTEMPTS);
        assert!(!lock_of(&file).exists());
        let mut once = 0;
        let word = change(&file, KEY, Change::Write, effective_uid(), &mut |path| {
            once += 1;
            if once == 1 {
                fs::write(path, br#"{"b":3}"#).expect("one racing write");
            }
        });
        assert_eq!(word, TrustWord::Written, "the second attempt holds");
        assert_eq!(once, 2);
        let mut kept = 0;
        let word = change(&file, KEY, Change::Remove, effective_uid(), &mut |path| {
            kept += 1;
            let mut back = doc(path);
            back["projects"][KEY] = json!({"hasTrustDialogAccepted": true});
            fs::write(path, back.to_string()).expect("the record put back");
        });
        assert_eq!(word, TrustWord::Lost);
        assert_eq!(kept, ATTEMPTS);
    }

    #[test]
    fn trust_record_a_file_of_another_owner_is_foreign() {
        assert!(owned_by(1000, 1000));
        assert!(!owned_by(0, 1000));
        let tmp = tempfile::tempdir().expect("tmp");
        let file = config(tmp.path(), &json!({}), 0o600);
        let other = effective_uid().wrapping_add(1);
        assert_eq!(
            change(&file, KEY, Change::Write, other, &mut |_| {}),
            TrustWord::ConfigForeign
        );
        assert_eq!(doc(&file), json!({}));
        assert!(!lock_of(&file).exists());
    }

    #[test]
    fn git_root_takes_a_git_dir_or_file_and_never_a_link() {
        let tmp = tempfile::tempdir().expect("tmp");
        let root = fs::canonicalize(tmp.path()).expect("canonical");
        let deep = root.join("repo").join("a").join("b");
        fs::create_dir_all(&deep).expect("dirs");
        let outside = git_root(&root);
        assert_eq!(git_root(&deep), outside, "no .git under the temp root");
        fs::create_dir(root.join("repo").join(".git")).expect("a .git dir");
        assert_eq!(git_root(&deep), Some(root.join("repo")));
        fs::write(deep.join(".git"), b"gitdir: x").expect("a .git file");
        assert_eq!(git_root(&deep), Some(deep.clone()));
        fs::remove_file(deep.join(".git")).expect("unplant");
        std::os::unix::fs::symlink(root.join("repo"), root.join("repo").join("a").join(".git"))
            .expect("a .git link");
        assert_eq!(
            git_root(&deep),
            Some(root.join("repo")),
            "the link is no root"
        );
        assert_eq!(git_root(&root), outside);
    }
}
