//! Owner-only creation and the single atomic replace: modes are set explicitly, never left to the
//! umask (security-plan §Authentication & Authorization, `~/.viola/` row). On Windows the modes are
//! no-ops; a folder viola creates outside `%USERPROFILE%` gets an explicit protected user + SYSTEM
//! DACL instead (the same row), and the check is `crate::strict`'s.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write as _};
use std::path::{Component, Path};
use std::time::Duration;

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
/// 0700 on Unix (the umask can only narrow the first); an existing `path` is narrowed too. On
/// Windows the topmost folder this call creates, when it lies outside the user's profile folder,
/// gets the protected user + SYSTEM DACL every folder and file under it inherits; a DACL that cannot
/// be set fails the creation.
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
    let topmost = missing.last().copied();
    for dir in missing.into_iter().rev() {
        created_or_present(builder.create(dir))?;
        restrict(dir, DIR_MODE)?;
    }
    #[cfg(windows)]
    if let Some(topmost) = topmost {
        win::protect_outside_profile(topmost)?;
    }
    #[cfg(not(windows))]
    let _ = topmost;
    restrict(path, DIR_MODE)
}

/// Whether `dir` lies outside `profile`, component by component, case folded (Windows paths): the
/// profile folder itself and anything under it are inside. Both are canonical paths.
pub fn outside_profile(dir: &Path, profile: &Path) -> bool {
    let fold = |c: Component<'_>| c.as_os_str().to_string_lossy().to_lowercase();
    let mut dir = dir.components().map(fold);
    !profile
        .components()
        .map(fold)
        .all(|p| dir.next() == Some(p))
}

#[cfg(windows)]
mod win {
    use std::io;
    use std::os::windows::ffi::OsStrExt as _;
    use std::path::{Path, PathBuf};
    use std::ptr;

    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, LocalFree};
    use windows_sys::Win32::Security::Authorization::{
        ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1, SE_FILE_OBJECT,
        SetNamedSecurityInfoW,
    };
    use windows_sys::Win32::Security::{
        ACL, DACL_SECURITY_INFORMATION, GetSecurityDescriptorDacl,
        PROTECTED_DACL_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR, TOKEN_QUERY,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
    use windows_sys::Win32::UI::Shell::GetUserProfileDirectoryW;

    use super::outside_profile;

    fn wide(text: &std::ffi::OsStr) -> Vec<u16> {
        text.encode_wide().chain(Some(0)).collect()
    }

    /// The protected owner-only DACL, inherited by every folder and file under the folder it is set
    /// on (`OICI`).
    pub(super) fn protected_sddl(user_sid: &str) -> String {
        format!("D:P(A;OICI;FA;;;{user_sid})(A;OICI;FA;;;SY)")
    }

    /// The process user's profile folder, canonical.
    fn profile_dir() -> io::Result<PathBuf> {
        let mut token: HANDLE = ptr::null_mut();
        // SAFETY: the current-process pseudo-handle needs no close; `token` is a local out-pointer.
        if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } == 0 {
            return Err(io::Error::last_os_error());
        }
        let mut len = 0u32;
        // SAFETY: a size query: a null buffer, `len` a local out-pointer.
        unsafe { GetUserProfileDirectoryW(token, ptr::null_mut(), &mut len) };
        let mut buf = vec![0u16; usize::try_from(len).unwrap_or(0)];
        // SAFETY: `buf` holds `len` u16s and outlives the call.
        let filled = unsafe { GetUserProfileDirectoryW(token, buf.as_mut_ptr(), &mut len) };
        let error = io::Error::last_os_error();
        // SAFETY: `token` was opened above and is closed exactly once.
        unsafe { CloseHandle(token) };
        if filled == 0 {
            return Err(error);
        }
        let end = buf.iter().position(|c| *c == 0).unwrap_or(buf.len());
        std::fs::canonicalize(PathBuf::from(String::from_utf16_lossy(&buf[..end])))
    }

    /// `dir` (just created) gets the protected owner-only DACL when it lies outside the profile
    /// folder; a home inside it keeps the profile's inheritance (security-plan, `~/.viola/` row).
    pub(super) fn protect_outside_profile(dir: &Path) -> io::Result<()> {
        let dir = std::fs::canonicalize(dir)?;
        if outside_profile(&dir, &profile_dir()?) {
            protect(&dir)?;
        }
        Ok(())
    }

    fn protect(dir: &Path) -> io::Result<()> {
        let user = crate::strict::win::user_sid().ok_or_else(io::Error::last_os_error)?;
        let sddl = wide(std::ffi::OsStr::new(&protected_sddl(&user)));
        let mut sd: PSECURITY_DESCRIPTOR = ptr::null_mut();
        // SAFETY: `sddl` is NUL-terminated; `sd` is LocalAlloc'd by the call and freed below.
        let made = unsafe {
            ConvertStringSecurityDescriptorToSecurityDescriptorW(
                sddl.as_ptr(),
                SDDL_REVISION_1,
                &mut sd,
                ptr::null_mut(),
            )
        };
        if made == 0 {
            return Err(io::Error::last_os_error());
        }
        let set = dacl_of(sd).and_then(|dacl| {
            let name = wide(dir.as_os_str());
            // SAFETY: `name` is NUL-terminated; `dacl` points into `sd`, still live; no owner, group
            // or SACL is set.
            let status = unsafe {
                SetNamedSecurityInfoW(
                    name.as_ptr(),
                    SE_FILE_OBJECT,
                    DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    dacl,
                    ptr::null(),
                )
            };
            if status == 0 {
                Ok(())
            } else {
                Err(io::Error::from_raw_os_error(
                    i32::try_from(status).unwrap_or(i32::MAX),
                ))
            }
        });
        // SAFETY: `sd` was allocated by the conversion above and is freed exactly once.
        unsafe { LocalFree(sd) };
        set
    }

    fn dacl_of(sd: PSECURITY_DESCRIPTOR) -> io::Result<*mut ACL> {
        let (mut present, mut defaulted) = (0, 0);
        let mut dacl: *mut ACL = ptr::null_mut();
        // SAFETY: `sd` is a valid descriptor; the out-pointers are locals.
        if unsafe { GetSecurityDescriptorDacl(sd, &mut present, &mut dacl, &mut defaulted) } == 0
            || present == 0
            || dacl.is_null()
        {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }
        Ok(dacl)
    }
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

/// How many times the rename is tried while Windows refuses it for a reader holding the target.
pub const REPLACE_ATTEMPTS: u32 = 100;
pub(crate) const REPLACE_PAUSE: Duration = Duration::from_millis(10);

/// `ERROR_ACCESS_DENIED`: Windows' answer to a rename over a file a reader holds open, under every
/// reader share mode, `std::fs::File::open`'s included (measured with the pinned tempfile).
const ACCESS_DENIED: i32 = 5;
const HOST_IS_WINDOWS: bool = cfg!(windows);

/// Only a Windows host's reader refusal is waited out; every other failure is final at once.
fn retry_replace(raw_os_error: Option<i32>, host_is_windows: bool) -> bool {
    host_is_windows && raw_os_error == Some(ACCESS_DENIED)
}

/// The one atomic replace: a temp file beside `path`, its mode set before any byte is written,
/// synced, then renamed over `path`. Any failure leaves `path` as it was.
pub fn replace_private(path: &Path, bytes: &[u8], mode: u32) -> Result<(), StateError> {
    replace_private_with(path, bytes, mode, &mut || std::thread::sleep(REPLACE_PAUSE))
}

/// `replace_private` with the pause between two rename attempts supplied by the caller.
pub(crate) fn replace_private_with(
    path: &Path,
    bytes: &[u8],
    mode: u32,
    pause: &mut dyn FnMut(),
) -> Result<(), StateError> {
    let parent = path.parent().unwrap_or(Path::new("."));
    let mut tmp = tempfile::NamedTempFile::new_in(parent)?;
    restrict(tmp.path(), mode)?;
    tmp.write_all(bytes)?;
    tmp.as_file().sync_all()?;
    let mut attempts = 0;
    loop {
        attempts += 1;
        match tmp.persist(path) {
            Ok(_) => return Ok(()),
            Err(e)
                if attempts < REPLACE_ATTEMPTS
                    && retry_replace(e.error.raw_os_error(), HOST_IS_WINDOWS) =>
            {
                tmp = e.file;
                pause();
            }
            Err(e) => return Err(e.error.into()),
        }
    }
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
    fn retry_replace_waits_only_for_a_windows_reader_refusal() {
        assert!(retry_replace(Some(5), true));
        assert!(
            !retry_replace(Some(5), false),
            "only a Windows host retries"
        );
        assert!(
            !retry_replace(Some(32), true),
            "a sharing violation is not the refusal"
        );
        assert!(!retry_replace(None, true));
        assert_eq!(REPLACE_ATTEMPTS, 100);
        assert_eq!(HOST_IS_WINDOWS, cfg!(windows));
    }

    #[test]
    fn replace_private_with_never_pauses_a_replace_that_lands() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("snapshot.json");
        fs::write(&path, b"old").expect("seed");
        let mut pauses = 0;
        replace_private_with(&path, b"new", FILE_MODE, &mut || pauses += 1).expect("replaced");
        assert_eq!(pauses, 0);
        assert_eq!(fs::read(&path).expect("read"), b"new");
    }

    /// A reader holds `snapshot.json` open across the replace (a plain `File::open`, the way a hook
    /// reads it): Windows refuses the rename until the reader lets go. The pause lets go the first
    /// time it runs, which also proves the refusal happened.
    #[cfg(windows)]
    #[test]
    fn replace_private_lands_once_a_holding_reader_lets_go() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("snapshot.json");
        fs::write(&path, b"old").expect("seed");
        let mut holder = Some(File::open(&path).expect("reader"));
        let mut pauses = 0;
        replace_private_with(&path, b"new", FILE_MODE, &mut || {
            pauses += 1;
            holder.take();
        })
        .expect("replaced once the reader let go");
        assert!(pauses >= 1, "the reader never held the target");
        assert_eq!(fs::read(&path).expect("read"), b"new");
    }

    #[cfg(windows)]
    #[test]
    fn replace_private_gives_up_after_the_attempts_and_keeps_the_old_bytes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("snapshot.json");
        fs::write(&path, b"old").expect("seed");
        let holder = File::open(&path).expect("reader");
        let mut pauses = 0;
        let replaced = replace_private_with(&path, b"new", FILE_MODE, &mut || {
            pauses += 1;
            assert!(pauses < REPLACE_ATTEMPTS, "paused past the last attempt");
        });
        assert!(replaced.is_err());
        assert_eq!(
            pauses + 1,
            REPLACE_ATTEMPTS,
            "one pause between two attempts"
        );
        drop(holder);
        assert_eq!(fs::read(&path).expect("read"), b"old");
        let names: Vec<_> = fs::read_dir(tmp.path())
            .expect("dir")
            .flatten()
            .map(|e| e.file_name())
            .collect();
        assert_eq!(names, ["snapshot.json"], "no temp file is left behind");
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
    fn outside_profile_compares_whole_components_case_folded() {
        use std::path::PathBuf;
        let profile = PathBuf::from("/c/users/runneradmin");
        let at = |p: &str| outside_profile(Path::new(p), &profile);
        assert!(!at("/c/users/runneradmin"));
        assert!(!at("/c/users/runneradmin/.viola"));
        assert!(!at("/C/Users/RunnerAdmin/.viola/instances"));
        assert!(
            at("/c/users/runneradmin2/.viola"),
            "a sibling sharing a prefix"
        );
        assert!(at("/d/a/viola/viola/target/e2e-home/x/home"));
        assert!(at("/c/users"), "the profile's parent");
        assert!(at("/c"));
    }

    /// A home outside the profile folder, created under the workspace's `target/e2e-home` the way
    /// the tests' homes are: its topmost created folder carries exactly the protected user + SYSTEM
    /// DACL, nothing inherited, and the strict-modes check passes on it and on what it holds.
    #[cfg(windows)]
    #[test]
    fn create_private_dir_outside_the_profile_sets_the_protected_owner_only_dacl() {
        let base = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("target")
            .join("e2e-home");
        fs::create_dir_all(&base).expect("e2e-home");
        let tmp = tempfile::tempdir_in(&base).expect("tempdir");
        let home = tmp.path().join("home");
        create_private_dir(&home.join("ledger")).expect("created");
        let user = crate::strict::win::user_sid().expect("user sid");
        let aces = crate::strict::win::allows(&home).expect("a readable DACL");
        let got: Vec<(String, u32, u8)> = aces
            .iter()
            .map(|a| (a.sid.clone(), a.mask, a.flags))
            .collect();
        assert_eq!(
            got,
            [
                (user, 0x001F_01FF, 0x03),
                ("S-1-5-18".to_owned(), 0x001F_01FF, 0x03)
            ],
            "protected, owner-only, object- and container-inherit"
        );
        assert_eq!(crate::strict::check_path(&home), Ok(()));
        assert_eq!(crate::strict::check_path(&home.join("ledger")), Ok(()));
        assert_eq!(
            win::protected_sddl("S-1-5-21-1"),
            "D:P(A;OICI;FA;;;S-1-5-21-1)(A;OICI;FA;;;SY)"
        );
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
