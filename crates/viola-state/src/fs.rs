//! Owner-only creation and the single atomic replace: modes are set explicitly, never left to the
//! umask (security-plan §Authentication & Authorization, `~/.viola/` row).

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read as _, Write as _};
use std::path::Path;

use crate::StateError;

pub const DIR_MODE: u32 = 0o700;
pub const FILE_MODE: u32 = 0o600;

/// Sets `mode` on `path`.
fn restrict(path: &Path, mode: u32) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt as _;
    fs::set_permissions(path, fs::Permissions::from_mode(mode))
}

/// Creates `path` and its missing parents one component at a time, each created 0700 and then set
/// 0700 (the umask can only narrow the first); an existing `path` is narrowed too.
pub fn create_private_dir(path: &Path) -> io::Result<()> {
    let missing: Vec<&Path> = path
        .ancestors()
        .take_while(|p| !p.as_os_str().is_empty() && !p.exists())
        .collect();
    let mut builder = fs::DirBuilder::new();
    std::os::unix::fs::DirBuilderExt::mode(&mut builder, DIR_MODE);
    for dir in missing.into_iter().rev() {
        created_or_present(builder.create(dir))?;
        restrict(dir, DIR_MODE)?;
    }
    restrict(path, DIR_MODE)
}

/// A component another process created first is as good as one this call created.
fn created_or_present(result: io::Result<()>) -> io::Result<()> {
    match result {
        Err(e) if e.kind() != io::ErrorKind::AlreadyExists => Err(e),
        _ => Ok(()),
    }
}

/// Append-only, created 0600 and narrowed to 0600 when it already existed. A symbolic link at
/// `path` fails the open (`O_NOFOLLOW`), and the mode is set through the open handle, so neither
/// the open nor the narrowing reaches past a link put there after a check.
pub fn open_private_append(path: &Path) -> io::Result<File> {
    use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
    let mut options = OpenOptions::new();
    options
        .create(true)
        .append(true)
        .mode(FILE_MODE)
        .custom_flags(libc::O_NOFOLLOW);
    let file = options.open(path)?;
    file.set_permissions(fs::Permissions::from_mode(FILE_MODE))?;
    Ok(file)
}

/// A new file opened for write, created 0600 on Unix; an existing name fails with `AlreadyExists`, so
/// of two creators of one name exactly one wins.
pub fn create_private_new(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, FILE_MODE);
    options.open(path)
}

/// A `.lock` sibling, 0600 on Unix, opened for write.
pub fn open_private_lock(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.create(true).truncate(false).write(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, FILE_MODE);
    let file = options.open(path)?;
    restrict(path, FILE_MODE)?;
    Ok(file)
}

/// The one atomic replace: a temp file beside `path`, its mode set before any byte is written,
/// synced, then renamed over `path`. Any failure leaves `path` as it was.
pub fn replace_private(path: &Path, bytes: &[u8], mode: u32) -> Result<(), StateError> {
    let parent = path.parent().unwrap_or(Path::new("."));
    let mut tmp = tempfile::NamedTempFile::new_in(parent)?;
    restrict(tmp.path(), mode)?;
    tmp.write_all(bytes)?;
    tmp.as_file().sync_all()?;
    tmp.persist(path).map(|_| ()).map_err(|e| e.error.into())
}

/// What [`place_private_new`] found at its target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placed {
    /// This call placed the bytes.
    Written,
    /// A file already stood at the name; it is left as found.
    Present,
}

/// The pinned copy's absent-copy write: a temp file beside `path`, its mode set before any byte is
/// written, synced and CLOSED, then moved to `path` only if no file stands there (no clobber). The
/// copy appears once, whole, and never open for write, so a concurrent exec of it cannot meet
/// `ETXTBSY`, and a copy another start placed first is never replaced ([`Placed::Present`]). Any
/// failure leaves `path` as it was and removes the temp file. Every other replaced file keeps
/// [`replace_private`] or [`replace_private_shared`].
pub fn place_private_new(path: &Path, bytes: &[u8], mode: u32) -> Result<Placed, StateError> {
    place_private_new_with(path, bytes, mode, &mut |_| {})
}

/// [`place_private_new`] with `placed` called on `path` right after a `Written` placement, before it
/// returns: the window in which a racing reader would meet the placed file.
fn place_private_new_with(
    path: &Path,
    bytes: &[u8],
    mode: u32,
    placed: &mut dyn FnMut(&Path),
) -> Result<Placed, StateError> {
    let parent = path.parent().unwrap_or(Path::new("."));
    let mut tmp = tempfile::NamedTempFile::new_in(parent)?;
    restrict(tmp.path(), mode)?;
    tmp.write_all(bytes)?;
    tmp.as_file().sync_all()?;
    match tmp.into_temp_path().persist_noclobber(path) {
        Ok(()) => {
            placed(path);
            Ok(Placed::Written)
        }
        Err(e) if e.error.kind() == io::ErrorKind::AlreadyExists => Ok(Placed::Present),
        Err(e) => Err(e.error.into()),
    }
}

/// `replace_private` for a file every concurrent writer fills with the same bytes (the plugin
/// files): a replace that fails is still done when the file already holds exactly `bytes`.
pub fn replace_private_shared(path: &Path, bytes: &[u8], mode: u32) -> Result<(), StateError> {
    match replace_private(path, bytes, mode) {
        Err(e) if !holds_exactly(path, bytes) => Err(e),
        _ => Ok(()),
    }
}

/// Whether `path` holds exactly `bytes`, read through at most one byte past them: a longer file
/// differs at that byte, so the outcome is a whole-file compare's without buffering the file.
fn holds_exactly(path: &Path, bytes: &[u8]) -> bool {
    let Ok(file) = File::open(path) else {
        return false;
    };
    let cap = u64::try_from(bytes.len()).map_or(u64::MAX, |len| len.saturating_add(1));
    let mut read = Vec::new();
    file.take(cap).read_to_end(&mut read).is_ok() && read == bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_private_shared_fails_unless_the_bytes_are_already_there() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let occupied = tmp.path().join("occupied");
        fs::create_dir(&occupied).expect("a dir where the file should go");
        assert!(replace_private_shared(&occupied, b"x", FILE_MODE).is_err());
        let fresh = tmp.path().join("fresh");
        replace_private_shared(&fresh, b"new", FILE_MODE).expect("written");
        assert_eq!(fs::read(&fresh).expect("read"), b"new");
    }

    /// The compare reads at most one byte past `bytes`: a target holding `bytes` and more is no
    /// match, so a failed replace over it stays an error; an equal, a shorter and a different
    /// target are judged as a whole-file compare judges them.
    #[test]
    fn replace_private_shared_reads_no_more_than_one_byte_past_the_bytes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("target");
        let cases: [(&[u8], bool); 4] = [
            (b"same", true),
            (b"same+", false),
            (b"sam", false),
            (b"diff", false),
        ];
        for (seed, expected) in cases {
            fs::write(&path, seed).expect("seed");
            assert_eq!(holds_exactly(&path, b"same"), expected, "{seed:?}");
        }
        assert!(!holds_exactly(&tmp.path().join("absent"), b"same"));
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let dir = tmp.path().join("held");
            fs::create_dir(&dir).expect("dir");
            let held = dir.join("file");
            fs::write(&held, b"same+").expect("seed");
            let set_mode = |mode: u32| {
                fs::set_permissions(&dir, fs::Permissions::from_mode(mode)).expect("chmod")
            };
            set_mode(0o500);
            let refused = replace_private(&held, b"same", FILE_MODE).is_err();
            let longer = replace_private_shared(&held, b"same", FILE_MODE);
            set_mode(0o700);
            assert_eq!(longer.is_err(), refused, "a longer target is no match");
        }
    }

    fn names_in(dir: &Path) -> Vec<std::ffi::OsString> {
        fs::read_dir(dir)
            .expect("dir")
            .flatten()
            .map(|e| e.file_name())
            .collect()
    }

    #[test]
    fn place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp() {
        for mode in [DIR_MODE, FILE_MODE] {
            let tmp = tempfile::tempdir().expect("tempdir");
            let path = tmp.path().join("viola");
            assert_eq!(
                place_private_new(&path, b"copy", mode).expect("placed"),
                Placed::Written
            );
            assert_eq!(fs::read(&path).expect("read"), b"copy");
            assert_eq!(
                names_in(tmp.path()),
                ["viola"],
                "no temp file is left behind"
            );
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt as _;
                let got = fs::metadata(&path).expect("meta").permissions().mode() & 0o777;
                assert_eq!(got, mode);
            }
        }
    }

    #[test]
    fn place_private_new_over_a_present_name_is_present_and_leaves_it() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("viola");
        fs::write(&path, b"another start's copy").expect("seed");
        assert_eq!(
            place_private_new(&path, b"this start's copy", DIR_MODE).expect("not an error"),
            Placed::Present
        );
        assert_eq!(fs::read(&path).expect("read"), b"another start's copy");
        assert_eq!(
            names_in(tmp.path()),
            ["viola"],
            "no temp file is left behind"
        );
    }

    /// A persist that fails for any reason but a present name is an error, never `Present`: a final
    /// name past 255 bytes fails its link `ENAMETOOLONG`, and nothing stays behind.
    #[test]
    fn place_private_new_with_a_too_long_name_fails_and_is_not_present() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("v".repeat(256));
        assert!(place_private_new(&path, b"copy", DIR_MODE).is_err());
        assert!(names_in(tmp.path()).is_empty(), "nothing was created");
    }

    #[test]
    fn place_private_new_under_a_missing_dir_fails() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("missing").join("viola");
        assert!(place_private_new(&path, b"copy", DIR_MODE).is_err());
        assert!(names_in(tmp.path()).is_empty(), "nothing was created");
    }

    /// The placed copy is exec'd from inside the window, right after it appears: it is never open
    /// for write there, so the spawn runs. With the write handle held across the placement the same
    /// spawn fails `ETXTBSY` on Linux (the remove-the-guard reading).
    #[test]
    fn place_private_new_placed_copy_execs_at_once() {
        let program = ["/usr/bin/true", "/bin/true"]
            .into_iter()
            .find(|p| Path::new(p).exists())
            .expect("a host `true`");
        let bytes = fs::read(program).expect("its bytes");
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("viola");
        let mut ran = None;
        let placed = place_private_new_with(&path, &bytes, DIR_MODE, &mut |at| {
            ran = Some(std::process::Command::new(at).status());
        })
        .expect("placed");
        assert_eq!(placed, Placed::Written);
        let status = ran.expect("the hook ran").expect("the placed copy spawned");
        assert!(status.success(), "{status}");
    }

    #[test]
    fn create_private_new_creates_once_and_refuses_an_existing_name() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("claim");
        let mut file = create_private_new(&path).expect("created");
        file.write_all(b"first").expect("write");
        drop(file);
        let again = create_private_new(&path).expect_err("an existing name");
        assert_eq!(again.kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(fs::read(&path).expect("read"), b"first");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let mode = fs::metadata(&path).expect("meta").permissions().mode();
            assert_eq!(mode & 0o777, FILE_MODE);
        }
    }

    /// A directory without its write bit refuses the temp file: the Unix stand-in for a replace
    /// that fails. Over a file already holding the bytes it is done; over other bytes it is still
    /// an error. An effective uid of 0 writes there anyway, so nothing fails and nothing is an error.
    #[test]
    fn replace_private_shared_in_a_read_only_dir_is_done_only_over_the_same_bytes() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("held");
        fs::create_dir(&dir).expect("dir");
        let path = dir.join("file");
        fs::write(&path, b"same").expect("seed");
        let set_mode =
            |mode: u32| fs::set_permissions(&dir, fs::Permissions::from_mode(mode)).expect("chmod");
        set_mode(0o500);
        let refused = replace_private(&path, b"same", FILE_MODE).is_err();
        let same = replace_private_shared(&path, b"same", FILE_MODE);
        let other = replace_private_shared(&path, b"other", FILE_MODE);
        set_mode(0o700);
        assert!(same.is_ok(), "the bytes were already there");
        assert_eq!(other.is_err(), refused);
    }

    #[test]
    fn create_private_dir_creates_missing_parents_and_is_idempotent() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("a").join("b");
        create_private_dir(&dir).expect("create");
        assert!(dir.is_dir());
        create_private_dir(&dir).expect("idempotent");
    }

    #[test]
    fn create_private_dir_under_a_file_fails() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let file = tmp.path().join("file");
        fs::write(&file, b"x").expect("file");
        assert!(create_private_dir(&file.join("sub")).is_err());
        assert_eq!(fs::read(&file).expect("read"), b"x");
    }

    #[test]
    fn created_or_present_passes_only_already_exists() {
        use io::ErrorKind;
        assert!(created_or_present(Ok(())).is_ok());
        assert!(created_or_present(Err(ErrorKind::AlreadyExists.into())).is_ok());
        for kind in [ErrorKind::NotFound, ErrorKind::PermissionDenied] {
            let err = created_or_present(Err(kind.into())).expect_err("kept");
            assert_eq!(err.kind(), kind);
        }
    }

    #[test]
    fn open_private_append_appends_after_prior_bytes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("f.ndjson");
        fs::write(&path, b"one\n").expect("seed");
        open_private_append(&path)
            .expect("open")
            .write_all(b"two\n")
            .expect("write");
        assert_eq!(fs::read(&path).expect("read"), b"one\ntwo\n");
    }

    /// A link at the path fails the open, the target's bytes and mode left as found.
    #[test]
    fn open_private_append_refuses_a_symlink_and_leaves_its_target() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let target = tmp.path().join("elsewhere.ndjson");
        fs::write(&target, b"kept\n").expect("seed");
        fs::set_permissions(&target, fs::Permissions::from_mode(0o644)).expect("chmod");
        let link = tmp.path().join("f.ndjson");
        std::os::unix::fs::symlink(&target, &link).expect("symlink");
        assert!(open_private_append(&link).is_err());
        assert_eq!(fs::read(&target).expect("read"), b"kept\n");
        let mode = fs::metadata(&target).expect("meta").permissions().mode();
        assert_eq!(mode & 0o777, 0o644);
        assert!(
            fs::symlink_metadata(&link)
                .expect("link")
                .file_type()
                .is_symlink()
        );
    }

    /// A file that already stood at 0644 is narrowed to 0600 through the open handle, its bytes kept.
    #[test]
    fn open_private_append_narrows_an_existing_file_through_the_handle() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("f.ndjson");
        fs::write(&path, b"one\n").expect("seed");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).expect("chmod");
        let file = open_private_append(&path).expect("open");
        let mode = file.metadata().expect("meta").permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
        assert_eq!(fs::read(&path).expect("read"), b"one\n");
    }

    #[test]
    fn open_private_lock_locks_exclusively_and_keeps_bytes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("x.lock");
        fs::write(&path, b"kept").expect("seed");
        let file = open_private_lock(&path).expect("open");
        file.lock().expect("exclusive lock");
        let other = open_private_lock(&path).expect("second handle");
        assert!(
            other.try_lock().is_err(),
            "a second exclusive lock was granted"
        );
        drop(file);
        other.try_lock().expect("free once the first handle closed");
        drop(other);
        assert_eq!(fs::read(&path).expect("read"), b"kept");
    }

    #[test]
    fn replace_private_replaces_the_whole_file() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("s.json");
        replace_private(&path, b"first, longer content", FILE_MODE).expect("first");
        replace_private(&path, b"second", FILE_MODE).expect("second");
        assert_eq!(fs::read(&path).expect("read"), b"second");
        let names: Vec<_> = fs::read_dir(tmp.path())
            .expect("dir")
            .flatten()
            .map(|e| e.file_name())
            .collect();
        assert_eq!(names, ["s.json"], "no temp file is left behind");
    }

    #[test]
    fn replace_private_leaves_the_target_when_the_rename_fails() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let target = tmp.path().join("occupied");
        fs::create_dir(&target).expect("a dir where the file should go");
        fs::write(target.join("inside"), b"kept").expect("seed");
        assert!(replace_private(&target, b"new", FILE_MODE).is_err());
        assert_eq!(fs::read(target.join("inside")).expect("read"), b"kept");
    }

    #[test]
    fn replace_private_under_a_missing_dir_fails() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("missing").join("s.json");
        assert!(replace_private(&path, b"x", FILE_MODE).is_err());
        assert!(!path.exists());
    }

    #[cfg(unix)]
    #[test]
    fn fs_private_modes_are_explicit() {
        use std::os::unix::fs::PermissionsExt as _;
        let mode = |p: &Path| fs::metadata(p).expect("meta").permissions().mode() & 0o777;
        let set = |p: &Path, m: u32| {
            fs::set_permissions(p, fs::Permissions::from_mode(m)).expect("chmod");
        };
        let tmp = tempfile::tempdir().expect("tempdir");

        let fresh = tmp.path().join("x").join("y");
        create_private_dir(&fresh).expect("create");
        assert_eq!(mode(&tmp.path().join("x")), 0o700);
        assert_eq!(mode(&fresh), 0o700);

        let open = tmp.path().join("open");
        fs::create_dir(&open).expect("dir");
        set(&open, 0o755);
        create_private_dir(&open).expect("narrow");
        assert_eq!(mode(&open), 0o700);

        let log = tmp.path().join("log.ndjson");
        fs::write(&log, b"").expect("seed");
        set(&log, 0o644);
        open_private_append(&log).expect("append");
        assert_eq!(mode(&log), 0o600);
        set(&log, 0o644);
        open_private_lock(&log).expect("lock file");
        assert_eq!(mode(&log), 0o600);

        let snap = tmp.path().join("snap.json");
        fs::write(&snap, b"{}").expect("seed");
        set(&snap, 0o644);
        replace_private(&snap, b"{}", FILE_MODE).expect("replace");
        assert_eq!(mode(&snap), 0o600);
        replace_private(&snap, b"{}", DIR_MODE).expect("replace as 0700");
        assert_eq!(mode(&snap), 0o700);
    }
}
