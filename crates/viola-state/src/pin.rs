//! `bin/<VERSION>-<hash>/viola(.exe)`: the running exe copied once, keyed by a truncated SHA-256 of
//! its bytes, and re-hashed before every reuse; a copy that no longer matches is never run or
//! overwritten (security-plan §Data Protection, Code-bearing artefacts).

use std::fs::File;
use std::io::{self, Read as _};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use viola_core::VERSION;

use crate::StateError;
use crate::fs::{DIR_MODE, create_private_dir, replace_private_shared};

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

/// The same key over a file read in chunks, so a swapped-in file of any size is hashed, not held.
fn file_key(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            return Ok(hex16(&hasher.finalize()));
        }
        hasher.update(&buf[..n]);
    }
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
