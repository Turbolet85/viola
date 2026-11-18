//! The trust record's writer, built once: `viola verify`'s trust-record run writes and removes its
//! own record through it. A write or a removal is at most [`ATTEMPTS`] attempts of: the file
//! checked through `symlink_metadata` (present, no link, a regular file, this user's); the CLI's own
//! lock taken only while free; the file read without following a link through the frame cap; the
//! pure change (`viola_agent_claude::trust`); the one atomic replace with the file's own mode; this
//! call's lock removed; the file read back. Its outcome is one closed word, and it writes no log
//! line of its own: the caller's record carries it. A wrapped start (`run`, `revive`) writes the
//! folder's flag through the same attempts ([`trust_for_start`]), its record never removed.

use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::Read as _;
use std::os::unix::fs::{DirBuilderExt as _, MetadataExt as _, OpenOptionsExt as _};
use std::path::{Path, PathBuf};
use std::time::Duration;

use viola_agent_claude::StripPlan;
use viola_agent_claude::trust::{self, LOCK_SUFFIX, StartWord, TrustWord, Walk};
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

/// The config file the child reads: `<CLAUDE_CONFIG_DIR>/.claude.json` when the strip keeps that
/// name, its one value read here and nowhere else; else `<user home>/.claude.json`.
pub(crate) fn trust_config_file(strip: &StripPlan) -> Result<PathBuf, TrustWord> {
    let kept = strip
        .kept
        .iter()
        .any(|name| name == trust::CONFIG_DIR_VAR)
        .then(|| std::env::var_os(trust::CONFIG_DIR_VAR))
        .flatten();
    trust::config_file(kept.as_deref(), std::env::home_dir().as_deref())
}

/// `projects[key] = {"hasTrustDialogAccepted": true}` written into `file`: `written` once it reads
/// back, else the closed word for what stood in the way.
pub(crate) fn write_record(file: &Path, key: &str) -> TrustWord {
    change(file, key, Change::Write, effective_uid(), &mut |_| {}).unwrap_or(TrustWord::KeyPresent)
}

/// The whole entry at `projects[key]` removed from `file`: `removed` once it reads back gone.
pub(crate) fn remove_record(file: &Path, key: &str) -> TrustWord {
    change(file, key, Change::Remove, effective_uid(), &mut |_| {}).unwrap_or(TrustWord::KeyPresent)
}

/// A wrapped start's trust step for its child's folder (`canonical_cwd`, its `git_root`): nothing
/// when the key viola would write is the user home (`home-folder`, before any open) or when the
/// CLI's own walk already trusts the folder; else only `hasTrustDialogAccepted` set at the CLI's
/// key, read back exact.
pub(crate) fn trust_for_start(
    file: &Path,
    canonical_cwd: &Path,
    git_root: Option<&Path>,
    user_home: Option<&Path>,
) -> StartWord {
    start_trust(
        file,
        canonical_cwd,
        git_root,
        user_home,
        effective_uid(),
        &mut |_| {},
    )
}

fn start_trust(
    file: &Path,
    canonical_cwd: &Path,
    git_root: Option<&Path>,
    user_home: Option<&Path>,
    euid: u32,
    read_back: &mut dyn FnMut(&Path),
) -> StartWord {
    let key = trust::record_key(canonical_cwd, git_root);
    if user_home.is_some_and(|home| home.to_string_lossy() == key) {
        return StartWord::HomeFolder;
    }
    let kind = Change::Trust {
        cwd: canonical_cwd,
        git_root,
    };
    match change(file, &key, kind, euid, read_back) {
        Ok(word) => StartWord::of(word),
        Err(word) => word,
    }
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
enum Change<'a> {
    Write,
    Remove,
    /// The start's flag-only set, read first by the CLI's walk from `cwd`.
    Trust {
        cwd: &'a Path,
        git_root: Option<&'a Path>,
    },
}

/// One attempt's end: a word that stands, the folder found trusted, or the next attempt with the
/// word the last one leaves.
enum Attempt {
    Done(TrustWord),
    Trusted(StartWord),
    Again(TrustWord),
}

/// [`write_record`] / [`remove_record`] / [`trust_for_start`] over `euid`, with `read_back` called
/// on `file` between the replace and the read-back: the window a racing writer would take. `Err` is
/// the start's walk finding the folder trusted, nothing written.
fn change(
    file: &Path,
    key: &str,
    change: Change<'_>,
    euid: u32,
    read_back: &mut dyn FnMut(&Path),
) -> Result<TrustWord, StartWord> {
    let mut last = TrustWord::Lost;
    for n in 1..=ATTEMPTS {
        match attempt(file, key, change, euid, read_back) {
            Attempt::Done(word) => return Ok(word),
            Attempt::Trusted(word) => return Err(word),
            Attempt::Again(word) => {
                last = word;
                if word == TrustWord::LockHeld && n < ATTEMPTS {
                    std::thread::sleep(LOCK_PAUSE);
                }
            }
        }
    }
    Ok(last)
}

fn attempt(
    file: &Path,
    key: &str,
    change: Change<'_>,
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
    if let Change::Trust { cwd, git_root } = change {
        match trust::trust_walk(&bytes, cwd, git_root) {
            Walk::AtKey => return Attempt::Trusted(StartWord::AlreadyTrusted),
            Walk::Parent => return Attempt::Trusted(StartWord::ParentTrusted),
            Walk::Untrusted => {}
        }
    }
    let changed = match change {
        Change::Write => trust::with_record(&bytes, key),
        Change::Remove => trust::without_record(&bytes, key),
        Change::Trust { .. } => trust::with_flag(&bytes, key),
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
        Change::Trust { .. } => trust::same_entry(&changed, &bytes, key),
    });
    match (held, change) {
        (true, Change::Remove) => Attempt::Done(TrustWord::Removed),
        (true, Change::Write | Change::Trust { .. }) => Attempt::Done(TrustWord::Written),
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

    fn written_by(file: &Path, kind: Change<'_>, read_back: &mut dyn FnMut(&Path)) -> TrustWord {
        change(file, KEY, kind, effective_uid(), read_back).expect("no walk for verify's changes")
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
        let word = written_by(&file, Change::Write, &mut |path| {
            raced += 1;
            fs::write(path, br#"{"b":2}"#).expect("a racing writer");
        });
        assert_eq!(word, TrustWord::Lost);
        assert_eq!(raced, ATTEMPTS);
        assert!(!lock_of(&file).exists());
        let mut once = 0;
        let word = written_by(&file, Change::Write, &mut |path| {
            once += 1;
            if once == 1 {
                fs::write(path, br#"{"b":3}"#).expect("one racing write");
            }
        });
        assert_eq!(word, TrustWord::Written, "the second attempt holds");
        assert_eq!(once, 2);
        let mut kept = 0;
        let word = written_by(&file, Change::Remove, &mut |path| {
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
            Ok(TrustWord::ConfigForeign)
        );
        assert_eq!(doc(&file), json!({}));
        assert!(!lock_of(&file).exists());
    }

    const CWD: &str = "/t/run-cwd";

    fn start(file: &Path, euid: u32, read_back: &mut dyn FnMut(&Path)) -> StartWord {
        start_trust(
            file,
            Path::new(CWD),
            None,
            Some(Path::new("/u/me")),
            euid,
            read_back,
        )
    }

    /// Every name in `dir`, sorted: a lock or a temp sibling left shows here.
    fn names(dir: &Path) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(dir)
            .expect("dir")
            .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }

    #[test]
    fn trust_start_writes_the_one_key_and_reads_it_back() {
        let tmp = tempfile::tempdir().expect("tmp");
        let seed = json!({"a": 1, "projects": {"/s": {"hasTrustDialogAccepted": true, "allowedTools": []}}});
        let file = config(tmp.path(), &seed, 0o644);
        assert_eq!(
            start(&file, effective_uid(), &mut |_| {}),
            StartWord::Written
        );
        let after = doc(&file);
        assert_eq!(
            after["projects"][CWD],
            json!({"hasTrustDialogAccepted": true})
        );
        assert_eq!(after["projects"]["/s"], seed["projects"]["/s"]);
        assert_eq!(after["a"], json!(1));
        assert_eq!(mode(&file), 0o644, "the file's own mode kept");
        assert_eq!(
            names(tmp.path()),
            [".claude.json"],
            "no lock, no temp sibling"
        );
    }

    #[test]
    fn trust_start_flag_false_sets_only_the_flag() {
        let tmp = tempfile::tempdir().expect("tmp");
        let seed = json!({"projects": {CWD: {"hasTrustDialogAccepted": false, "allowedTools": ["x"], "k": 1}}});
        let file = config(tmp.path(), &seed, 0o600);
        assert_eq!(
            start(&file, effective_uid(), &mut |_| {}),
            StartWord::Written
        );
        assert_eq!(
            doc(&file)["projects"][CWD],
            json!({"hasTrustDialogAccepted": true, "allowedTools": ["x"], "k": 1})
        );
    }

    #[test]
    fn trust_start_already_trusted_writes_nothing() {
        let tmp = tempfile::tempdir().expect("tmp");
        let file = tmp.path().join(".claude.json");
        let bytes = br#"{"projects":{"/t/run-cwd":{"hasTrustDialogAccepted":true}}}"#;
        fs::write(&file, bytes).expect("seed");
        assert_eq!(
            start(&file, effective_uid(), &mut |_| {}),
            StartWord::AlreadyTrusted
        );
        assert_eq!(fs::read(&file).expect("read"), bytes);
        assert_eq!(names(tmp.path()), [".claude.json"]);
    }

    #[test]
    fn trust_start_parent_trusted_writes_nothing() {
        let tmp = tempfile::tempdir().expect("tmp");
        let file = tmp.path().join(".claude.json");
        let bytes = br#"{"projects":{"/t":{"hasTrustDialogAccepted":true}}}"#;
        fs::write(&file, bytes).expect("seed");
        assert_eq!(
            start(&file, effective_uid(), &mut |_| {}),
            StartWord::ParentTrusted
        );
        assert_eq!(fs::read(&file).expect("read"), bytes);
        assert_eq!(names(tmp.path()), [".claude.json"]);
    }

    #[test]
    fn trust_start_home_folder_opens_nothing() {
        let tmp = tempfile::tempdir().expect("tmp");
        let file = tmp.path().join(".claude.json");
        let home = Path::new(CWD);
        let word = start_trust(&file, home, None, Some(home), effective_uid(), &mut |_| {});
        assert_eq!(
            word,
            StartWord::HomeFolder,
            "read before any open: no config-absent"
        );
        let root = Path::new("/u/me");
        let word = start_trust(
            &file,
            Path::new("/u/me/sub"),
            Some(root),
            Some(root),
            effective_uid(),
            &mut |_| {},
        );
        assert_eq!(
            word,
            StartWord::HomeFolder,
            "the home as the git root's key"
        );
        assert!(names(tmp.path()).is_empty());
    }

    #[test]
    fn trust_start_refusals_write_nothing() {
        let tmp = tempfile::tempdir().expect("tmp");
        let uid = effective_uid();
        for sub in ["u", "l", "f", "o", "m"] {
            fs::create_dir(tmp.path().join(sub)).expect("sub");
        }
        let absent = tmp.path().join("absent.json");
        assert_eq!(start(&absent, uid, &mut |_| {}), StartWord::ConfigAbsent);
        assert!(!absent.exists(), "never created");

        let unreadable = config(&tmp.path().join("u"), &json!({}), 0o600);
        fs::set_permissions(&unreadable, fs::Permissions::from_mode(0o000)).expect("mode");
        assert_eq!(
            start(&unreadable, uid, &mut |_| {}),
            StartWord::ConfigUnreadable
        );
        fs::set_permissions(&unreadable, fs::Permissions::from_mode(0o600)).expect("mode");
        assert_eq!(doc(&unreadable), json!({}));
        assert_eq!(names(&tmp.path().join("u")), [".claude.json"]);

        let target = config(&tmp.path().join("l"), &json!({}), 0o600);
        let link = tmp.path().join("l").join("linked.json");
        std::os::unix::fs::symlink(&target, &link).expect("link");
        assert_eq!(start(&link, uid, &mut |_| {}), StartWord::ConfigLink);
        assert_eq!(doc(&target), json!({}));

        let dir = tmp.path().join("d.json");
        fs::create_dir(&dir).expect("dir");
        assert_eq!(start(&dir, uid, &mut |_| {}), StartWord::ConfigNotRegular);

        let foreign = config(&tmp.path().join("f"), &json!({}), 0o600);
        assert_eq!(
            start(&foreign, uid.wrapping_add(1), &mut |_| {}),
            StartWord::ConfigForeign
        );
        assert_eq!(doc(&foreign), json!({}));

        let big = tmp.path().join("o").join(".claude.json");
        let size = usize::try_from(MAX_FRAME + 1).expect("fits");
        fs::write(&big, vec![b' '; size]).expect("big");
        assert_eq!(start(&big, uid, &mut |_| {}), StartWord::ConfigOversize);
        assert_eq!(fs::metadata(&big).expect("meta").len(), MAX_FRAME + 1);

        for (label, body) in [
            ("not an object", json!([1])),
            ("projects an array", json!({"projects": []})),
            ("the entry not an object", json!({"projects": {CWD: 1}})),
        ] {
            let file = config(&tmp.path().join("m"), &body, 0o600);
            assert_eq!(
                start(&file, uid, &mut |_| {}),
                StartWord::ConfigMalformed,
                "{label}"
            );
            assert_eq!(doc(&file), body, "{label}");
            assert_eq!(names(&tmp.path().join("m")), [".claude.json"], "{label}");
        }
        for sub in ["l", "f", "o"] {
            assert!(
                names(&tmp.path().join(sub))
                    .iter()
                    .all(|n| !n.ends_with(".lock")),
                "{sub}"
            );
        }
        assert!(!lock_of(&dir).exists() && !lock_of(&absent).exists());
    }

    #[test]
    fn trust_start_lost_after_three_read_backs() {
        let tmp = tempfile::tempdir().expect("tmp");
        let file = config(tmp.path(), &json!({"b": 1}), 0o600);
        let mut raced = 0;
        let word = start(&file, effective_uid(), &mut |path| {
            raced += 1;
            fs::write(path, br#"{"b":2}"#).expect("a racing writer");
        });
        assert_eq!(word, StartWord::Lost);
        assert_eq!(raced, ATTEMPTS);
        assert_eq!(names(tmp.path()), [".claude.json"]);
    }

    #[test]
    fn trust_start_read_back_with_an_added_key_is_lost() {
        let tmp = tempfile::tempdir().expect("tmp");
        let file = config(tmp.path(), &json!({}), 0o600);
        let mut race = |path: &Path| {
            let mut raced = doc(path);
            raced["projects"][CWD]["added"] = json!(1);
            fs::write(path, raced.to_string()).expect("a racing writer's key");
        };
        let kind = Change::Trust {
            cwd: Path::new(CWD),
            git_root: None,
        };
        let one = attempt(&file, CWD, kind, effective_uid(), &mut race);
        assert!(
            matches!(one, Attempt::Again(TrustWord::Lost)),
            "the flag alone is no exact read-back"
        );
        let mut raced = 0;
        let word = start(&file, effective_uid(), &mut |path| {
            raced += 1;
            race(path);
        });
        assert_eq!(
            word,
            StartWord::AlreadyTrusted,
            "the next attempt reads the racer's true flag and writes nothing more"
        );
        assert_eq!(
            raced, 0,
            "the record the racer left stands: no second replace"
        );
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
