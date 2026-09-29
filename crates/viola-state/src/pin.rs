//! `bin/<VERSION>-<hash>/viola(.exe)`: the running exe copied once, keyed by a truncated SHA-256 of
//! its bytes, and re-hashed before every reuse; a copy that no longer matches is never run or
//! overwritten (security-plan §Data Protection, Code-bearing artefacts).

use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use viola_core::VERSION;

use crate::StateError;
use crate::fs::{DIR_MODE, create_private_dir, replace_private_shared};
#[cfg(windows)]
use crate::fs::{REPLACE_ATTEMPTS, REPLACE_PAUSE};

#[derive(Debug, thiserror::Error)]
pub enum PinError {
    #[error("the pinned copy failed its integrity check")]
    HashMismatch,
    #[error("the pinned copy could not be written")]
    State(#[from] StateError),
}

impl From<io::Error> for PinError {
    fn from(error: io::Error) -> Self {
        Self::State(StateError::Io(error))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pinned {
    /// `<VERSION>-<16 hex>`, shared with `plugin/<key>/`.
    pub key: String,
    /// The absolute path of the copy.
    pub path: PathBuf,
    /// The same path with forward slashes, as it is written into config and the child's env.
    pub path_fwd: String,
}

fn hex16(digest: &[u8]) -> String {
    digest[..8].iter().map(|b| format!("{b:02x}")).collect()
}

/// The first 16 hex digits of the SHA-256 of `bytes`.
pub fn content_key(bytes: &[u8]) -> String {
    hex16(&Sha256::digest(bytes))
}

/// The whole SHA-256 of what `reader` yields, read in chunks, so a swapped-in file of any size is
/// hashed, not held.
fn digest(mut reader: impl io::Read) -> io::Result<Vec<u8>> {
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            return Ok(hasher.finalize().to_vec());
        }
        hasher.update(&buf[..n]);
    }
}

/// The same key over a file.
fn file_key(path: &Path) -> io::Result<String> {
    Ok(hex16(&digest(File::open(path)?)?))
}

/// Copies `exe` to `<home>/bin/<key>/viola(.exe)` (0700) when absent; when present, re-hashes it
/// and refuses on any difference, leaving the file as found.
pub fn pin_exe(home: &Path, exe: &Path) -> Result<Pinned, PinError> {
    let bytes = std::fs::read(exe)?;
    let hash = content_key(&bytes);
    let key = format!("{VERSION}-{hash}");
    let dir = std::path::absolute(home.join("bin").join(&key))?;
    create_private_dir(&dir)?;
    let path = dir.join(format!("viola{}", std::env::consts::EXE_SUFFIX));
    match file_key(&path) {
        Ok(found) if found == hash => {}
        Ok(_) => return Err(PinError::HashMismatch),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            replace_private_shared(&path, &bytes, DIR_MODE)?
        }
        Err(e) => return Err(e.into()),
    }
    let path_fwd = path.to_string_lossy().replace('\\', "/");
    Ok(Pinned {
        key,
        path,
        path_fwd,
    })
}

/// A file pinned in a subdirectory of the pinned dir, with the whole SHA-256 it must carry.
#[cfg(windows)]
#[derive(Debug, Clone, Copy)]
pub struct Companion {
    pub name: &'static str,
    pub bytes: &'static [u8],
    pub sha256_hex: &'static str,
}

/// Verified companions, each held open with read sharing only: until this is dropped no process can
/// open one for writing, renaming or deleting, so the bytes checked are the bytes later used.
#[cfg(windows)]
#[derive(Debug)]
pub struct HeldCompanions {
    /// The absolute path of `bin/<key>/<subdir>/`.
    pub dir: PathBuf,
    _handles: Vec<File>,
}

/// Win32 `FILE_SHARE_READ`: every other opener may read, none may write or delete.
#[cfg(windows)]
const SHARE_READ_ONLY: u32 = 0x0000_0001;

/// Win32 `ERROR_SHARING_VIOLATION`. Two first starts of one version write the same companion, and
/// for a few ms after one lands it another opener still holds it with write access (measured 5-11
/// ms on two concurrent first starts; never over companions already written).
#[cfg(windows)]
const SHARING_VIOLATION: i32 = 32;

/// Only that refusal is waited out, `REPLACE_ATTEMPTS` times; every other failure is final at once.
#[cfg(windows)]
fn retry_open(raw_os_error: Option<i32>, attempts: u32) -> bool {
    attempts < REPLACE_ATTEMPTS && raw_os_error == Some(SHARING_VIOLATION)
}

#[cfg(windows)]
fn open_held(path: &Path) -> io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt as _;
    std::fs::OpenOptions::new()
        .read(true)
        .share_mode(SHARE_READ_ONLY)
        .open(path)
}

#[cfg(windows)]
fn open_held_waiting(path: &Path, pause: &mut dyn FnMut()) -> io::Result<File> {
    let mut attempts = 0;
    loop {
        attempts += 1;
        match open_held(path) {
            Err(e) if retry_open(e.raw_os_error(), attempts) => pause(),
            opened => return opened,
        }
    }
}

/// Writes each companion into `bin/<key>/<subdir>/` when absent (never over a present one, like
/// [`pin_exe`]), then opens it held and compares its whole SHA-256 with the pin. A file that differs
/// is left as found: [`PinError::HashMismatch`].
#[cfg(windows)]
pub fn pin_companions(
    pinned: &Pinned,
    subdir: &str,
    files: &[Companion],
) -> Result<HeldCompanions, PinError> {
    let bin = pinned
        .path
        .parent()
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;
    let dir = bin.join(subdir);
    create_private_dir(&dir)?;
    let mut handles = Vec::with_capacity(files.len());
    let mut pause = || std::thread::sleep(REPLACE_PAUSE);
    for file in files {
        let path = dir.join(file.name);
        let held = match open_held_waiting(&path, &mut pause) {
            Ok(held) => held,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                replace_private_shared(&path, file.bytes, DIR_MODE)?;
                open_held_waiting(&path, &mut pause)?
            }
            Err(e) => return Err(e.into()),
        };
        let found: String = digest(&held)?.iter().map(|b| format!("{b:02x}")).collect();
        if found != file.sha256_hex {
            return Err(PinError::HashMismatch);
        }
        handles.push(held);
    }
    Ok(HeldCompanions {
        dir,
        _handles: handles,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::fs;

    #[rstest]
    #[case::empty(b"", "e3b0c44298fc1c14")]
    #[case::abc(b"abc", "ba7816bf8f01cfea")]
    fn content_key_is_the_sixteen_hex_sha256_prefix(#[case] bytes: &[u8], #[case] key: &str) {
        assert_eq!(content_key(bytes), key);
    }

    fn exe_in(dir: &Path) -> PathBuf {
        let exe = dir.join("source-exe");
        fs::write(&exe, b"abc").expect("source");
        exe
    }

    #[test]
    fn pin_exe_copies_then_refuses_a_tampered_copy() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        let exe = exe_in(tmp.path());

        let pinned = pin_exe(&home, &exe).expect("first pin");
        assert_eq!(pinned.key, "0.1.0-ba7816bf8f01cfea");
        let name = if cfg!(windows) { "viola.exe" } else { "viola" };
        let expected = home.join("bin").join("0.1.0-ba7816bf8f01cfea").join(name);
        assert_eq!(pinned.path, std::path::absolute(&expected).expect("abs"));
        assert!(pinned.path.is_absolute());
        assert!(!pinned.path_fwd.contains('\\'));
        assert_eq!(
            pinned.path_fwd,
            pinned.path.to_string_lossy().replace('\\', "/")
        );
        assert_eq!(fs::read(&pinned.path).expect("copy"), b"abc");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let mode = fs::metadata(&pinned.path)
                .expect("meta")
                .permissions()
                .mode()
                & 0o777;
            assert_eq!(mode, 0o700);
        }

        assert_eq!(pin_exe(&home, &exe).expect("intact copy"), pinned);

        fs::write(&pinned.path, b"abd").expect("tamper one byte");
        assert!(matches!(pin_exe(&home, &exe), Err(PinError::HashMismatch)));
        assert_eq!(fs::read(&pinned.path).expect("left as found"), b"abd");

        fs::remove_file(&pinned.path).expect("delete");
        assert_eq!(pin_exe(&home, &exe).expect("re-copied"), pinned);
        assert_eq!(fs::read(&pinned.path).expect("copy"), b"abc");
    }

    #[test]
    fn pin_exe_hashes_a_copy_longer_than_one_read() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        let exe = tmp.path().join("big-exe");
        let bytes: Vec<u8> = (0..200_000u32).map(|i| (i % 251) as u8).collect();
        fs::write(&exe, &bytes).expect("source");
        let pinned = pin_exe(&home, &exe).expect("first pin");
        assert_eq!(pinned.key, format!("0.1.0-{}", content_key(&bytes)));
        assert_eq!(pin_exe(&home, &exe).expect("intact"), pinned);
        let mut tampered = bytes.clone();
        tampered[150_000] ^= 1;
        fs::write(&pinned.path, &tampered).expect("tamper past the first chunk");
        assert!(matches!(pin_exe(&home, &exe), Err(PinError::HashMismatch)));
    }

    #[test]
    fn pin_exe_reports_an_unreadable_source_or_copy() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        assert!(matches!(
            pin_exe(&home, &tmp.path().join("missing")),
            Err(PinError::State(_))
        ));
        let exe = exe_in(tmp.path());
        let slot = home
            .join("bin")
            .join("0.1.0-ba7816bf8f01cfea")
            .join(if cfg!(windows) { "viola.exe" } else { "viola" });
        fs::create_dir_all(&slot).expect("a dir where the copy should be");
        assert!(matches!(pin_exe(&home, &exe), Err(PinError::State(_))));
    }

    /// A copy that exists but cannot be read is an error, never a missing copy to write over.
    #[cfg(unix)]
    #[test]
    fn pin_exe_refuses_an_unreadable_copy_and_leaves_it() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        let exe = exe_in(tmp.path());
        let pinned = pin_exe(&home, &exe).expect("first pin");
        fs::write(&pinned.path, b"abd").expect("tamper");
        fs::set_permissions(&pinned.path, fs::Permissions::from_mode(0o000)).expect("chmod");
        assert!(matches!(pin_exe(&home, &exe), Err(PinError::State(_))));
        fs::set_permissions(&pinned.path, fs::Permissions::from_mode(0o600)).expect("chmod");
        assert_eq!(fs::read(&pinned.path).expect("left as found"), b"abd");
    }

    #[cfg(windows)]
    const COMPANIONS: &[Companion] = &[
        Companion {
            name: "a.dll",
            bytes: b"alpha",
            sha256_hex: "8ed3f6ad685b959ead7022518e1af76cd816f8e8ec7ccdda1ed4018e8f2223f8",
        },
        Companion {
            name: "b.exe",
            bytes: b"beta",
            sha256_hex: "f44e64e75f3948e9f73f8dfa94721c4ce8cbb4f265c4790c702b2d41cfbf2753",
        },
    ];

    #[cfg(windows)]
    fn pinned_in(tmp: &Path) -> Pinned {
        pin_exe(&tmp.join("home"), &exe_in(tmp)).expect("pin")
    }

    #[cfg(windows)]
    #[test]
    fn pin_companions_writes_then_reverifies_and_rewrites_a_deleted_one() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let pinned = pinned_in(tmp.path());
        let held = pin_companions(&pinned, "side", COMPANIONS).expect("first pin");
        let dir = pinned.path.parent().expect("bin dir").join("side");
        assert_eq!(held.dir, dir);
        assert_eq!(fs::read(dir.join("a.dll")).expect("a"), b"alpha");
        assert_eq!(fs::read(dir.join("b.exe")).expect("b"), b"beta");
        drop(held);

        assert_eq!(
            pin_companions(&pinned, "side", COMPANIONS)
                .expect("intact")
                .dir,
            dir
        );

        fs::remove_file(dir.join("b.exe")).expect("delete");
        drop(pin_companions(&pinned, "side", COMPANIONS).expect("re-written"));
        assert_eq!(fs::read(dir.join("b.exe")).expect("b"), b"beta");
    }

    #[cfg(windows)]
    #[test]
    fn pin_companions_refuses_a_tampered_one_and_leaves_it() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let pinned = pinned_in(tmp.path());
        drop(pin_companions(&pinned, "side", COMPANIONS).expect("first pin"));
        let a = pinned.path.parent().expect("bin dir").join("side/a.dll");
        fs::write(&a, b"alphb").expect("tamper one byte");
        assert!(matches!(
            pin_companions(&pinned, "side", COMPANIONS),
            Err(PinError::HashMismatch)
        ));
        assert_eq!(fs::read(&a).expect("left as found"), b"alphb");
    }

    #[cfg(windows)]
    #[test]
    fn pin_companions_reports_a_directory_where_a_file_should_be() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let pinned = pinned_in(tmp.path());
        let slot = pinned.path.parent().expect("bin dir").join("side/b.exe");
        fs::create_dir_all(&slot).expect("a dir where the file should be");
        assert!(matches!(
            pin_companions(&pinned, "side", COMPANIONS),
            Err(PinError::State(_))
        ));
    }

    /// The handles deny a writer from the hash until they drop: Win32 `ERROR_SHARING_VIOLATION`.
    #[cfg(windows)]
    #[test]
    fn pin_companions_holds_every_file_against_a_writer() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let pinned = pinned_in(tmp.path());
        let held = pin_companions(&pinned, "side", COMPANIONS).expect("pin");
        for name in ["a.dll", "b.exe"] {
            let path = held.dir.join(name);
            let err = fs::OpenOptions::new()
                .write(true)
                .open(&path)
                .expect_err("held");
            assert_eq!(err.raw_os_error(), Some(32), "{name}");
            assert!(!fs::read(&path).expect("readers still read").is_empty());
        }
        drop(held);
        let path = pinned.path.parent().expect("bin dir").join("side/a.dll");
        fs::OpenOptions::new()
            .write(true)
            .open(&path)
            .expect("writable once released");
    }

    #[cfg(windows)]
    #[rstest]
    #[case::sharing_violation_first(Some(32), 1, true)]
    #[case::sharing_violation_before_the_last(Some(32), 99, true)]
    #[case::sharing_violation_at_the_last(Some(32), 100, false)]
    #[case::access_denied(Some(5), 1, false)]
    #[case::not_an_os_error(None, 1, false)]
    fn retry_open_waits_out_only_a_sharing_violation(
        #[case] raw: Option<i32>,
        #[case] attempts: u32,
        #[case] retry: bool,
    ) {
        assert_eq!(retry_open(raw, attempts), retry);
    }

    /// Another opener holding the file with write access, the way a start that has just written a
    /// companion still does for a few ms.
    #[cfg(windows)]
    fn writer_holding(path: &Path) -> fs::File {
        use std::os::windows::fs::OpenOptionsExt as _;
        fs::OpenOptions::new()
            .read(true)
            .write(true)
            .share_mode(0x7)
            .open(path)
            .expect("writer")
    }

    /// The window forced open: the first open meets the writer, the pause releases it.
    #[cfg(windows)]
    #[test]
    fn open_held_waiting_outlasts_a_writer_that_lets_go() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("fresh.exe");
        fs::write(&path, b"beta").expect("file");
        let mut writer = Some(writer_holding(&path));
        let mut pauses = 0;
        let held = open_held_waiting(&path, &mut || {
            pauses += 1;
            drop(writer.take());
        })
        .expect("opened once the writer let go");
        assert_eq!(pauses, 1);
        drop(held);
    }

    #[cfg(windows)]
    #[test]
    fn open_held_waiting_gives_up_on_a_writer_that_stays() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("held.exe");
        fs::write(&path, b"beta").expect("file");
        let _writer = writer_holding(&path);
        let mut pauses = 0;
        let err = open_held_waiting(&path, &mut || pauses += 1).expect_err("still held");
        assert_eq!(err.raw_os_error(), Some(32));
        assert_eq!(pauses, REPLACE_ATTEMPTS - 1);
    }

    #[cfg(windows)]
    #[test]
    fn open_held_waiting_reports_a_missing_file_at_once() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let mut pauses = 0;
        let err =
            open_held_waiting(&tmp.path().join("absent"), &mut || pauses += 1).expect_err("absent");
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
        assert_eq!(pauses, 0);
    }

    #[test]
    fn pin_error_messages_are_fixed() {
        assert_eq!(
            PinError::HashMismatch.to_string(),
            "the pinned copy failed its integrity check"
        );
        assert_eq!(
            PinError::from(io::Error::other("C:/x")).to_string(),
            "the pinned copy could not be written"
        );
    }
}
