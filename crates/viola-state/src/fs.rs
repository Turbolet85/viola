//! Owner-only creation and the single atomic replace: modes are set explicitly, never left to the
//! umask (security-plan §Authentication & Authorization, `~/.viola/` row). On Windows the modes are
//! no-ops; the DACL check belongs to the strict-modes work.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write as _};
use std::path::Path;

use crate::StateError;

pub const DIR_MODE: u32 = 0o700;
pub const FILE_MODE: u32 = 0o600;

/// Sets `mode` on `path` on Unix; nothing elsewhere.
fn restrict(path: &Path, mode: u32) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(path, fs::Permissions::from_mode(mode))
    }
    #[cfg(not(unix))]
    {
        let _ = (path, mode);
        Ok(())
    }
}

/// Creates `path` and its missing parents one component at a time, each created 0700 and then set
/// 0700 on Unix (the umask can only narrow the first); an existing `path` is narrowed too.
pub fn create_private_dir(path: &Path) -> io::Result<()> {
    let missing: Vec<&Path> = path
        .ancestors()
        .take_while(|p| !p.as_os_str().is_empty() && !p.exists())
        .collect();
    #[cfg(unix)]
    let builder = {
        let mut builder = fs::DirBuilder::new();
        std::os::unix::fs::DirBuilderExt::mode(&mut builder, DIR_MODE);
        builder
    };
    #[cfg(not(unix))]
    let builder = fs::DirBuilder::new();
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

/// Append-only, created 0600 on Unix and narrowed to 0600 when it already existed.
pub fn open_private_append(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.create(true).append(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, FILE_MODE);
    let file = options.open(path)?;
    restrict(path, FILE_MODE)?;
    Ok(file)
}

/// A `.lock` sibling, 0600 on Unix, opened for write because Windows refuses an exclusive lock on
/// an append-only handle.
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
    tmp.persist(path).map_err(|e| e.error)?;
    Ok(())
}

/// `replace_private` for a file every concurrent writer fills with the same bytes (the pinned copy,
/// the plugin files): a replace that fails — on Windows while another start holds the target open —
/// is still done when the file already holds exactly `bytes`.
pub fn replace_private_shared(path: &Path, bytes: &[u8], mode: u32) -> Result<(), StateError> {
    match replace_private(path, bytes, mode) {
        Err(e) if fs::read(path).ok().as_deref() != Some(bytes) => Err(e),
        _ => Ok(()),
    }
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

    /// Windows refuses to replace a read-only file: the stand-in for a target another start holds.
    #[cfg(windows)]
    #[test]
    fn replace_private_shared_accepts_a_target_that_already_holds_the_bytes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("held");
        let read_only = |ro: bool| {
            let mut perms = fs::metadata(&path).expect("meta").permissions();
            perms.set_readonly(ro);
            fs::set_permissions(&path, perms).expect("set");
        };
        fs::write(&path, b"same").expect("seed");
        read_only(true);
        assert!(
            replace_private(&path, b"same", FILE_MODE).is_err(),
            "the stand-in holds"
        );
        replace_private_shared(&path, b"same", FILE_MODE).expect("already there");
        assert!(replace_private_shared(&path, b"other", FILE_MODE).is_err());
        read_only(false);
        assert_eq!(fs::read(&path).expect("read"), b"same");
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
