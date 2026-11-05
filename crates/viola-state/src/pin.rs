//! `bin/<VERSION>-<hash>/viola`: the running exe copied once, keyed by a truncated SHA-256 of
//! its bytes, and re-hashed before every reuse; a copy that no longer matches is never run or
//! overwritten (security-plan §Data Protection, Code-bearing artefacts). The absent copy is placed
//! through the no-clobber [`crate::fs::place_private_new`]: it appears once, closed, and a copy a
//! concurrent first start placed first is re-hashed, never replaced.

use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use viola_core::VERSION;

use crate::StateError;
use crate::fs::{DIR_MODE, Placed, create_private_dir, place_private_new};

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

/// `<VERSION>-<16 hex>` of an exe's bytes: the name of its `bin/` and `plugin/` folders.
pub fn pin_key(bytes: &[u8]) -> String {
    format!("{VERSION}-{}", content_key(bytes))
}

/// Copies `exe` to `<home>/bin/<key>/viola` (0700) when absent; when present, re-hashes it
/// and refuses on any difference, leaving the file as found.
pub fn pin_exe(home: &Path, exe: &Path) -> Result<Pinned, PinError> {
    pin_bytes(home, &std::fs::read(exe)?)
}

/// [`pin_exe`] over an exe's bytes already read, so the key a caller checked before is the key
/// pinned.
pub fn pin_bytes(home: &Path, bytes: &[u8]) -> Result<Pinned, PinError> {
    let hash = content_key(bytes);
    let key = pin_key(bytes);
    let dir = std::path::absolute(home.join("bin").join(&key))?;
    create_private_dir(&dir)?;
    let path = dir.join(format!("viola{}", std::env::consts::EXE_SUFFIX));
    match file_key(&path) {
        Ok(found) if found == hash => {}
        Ok(_) => return Err(PinError::HashMismatch),
        Err(e) if e.kind() == io::ErrorKind::NotFound => place_absent(&path, bytes, &hash)?,
        Err(e) => return Err(e.into()),
    }
    let path_fwd = path.to_string_lossy().replace('\\', "/");
    Ok(Pinned {
        key,
        path,
        path_fwd,
    })
}

/// Places the absent copy; when a concurrent start placed one first, that copy is re-hashed like any
/// present copy: equal is reused, different is [`PinError::HashMismatch`] with the file left as found.
fn place_absent(path: &Path, bytes: &[u8], hash: &str) -> Result<(), PinError> {
    match place_private_new(path, bytes, DIR_MODE)? {
        Placed::Written => Ok(()),
        Placed::Present if file_key(path)? == hash => Ok(()),
        Placed::Present => Err(PinError::HashMismatch),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::fs;

    #[test]
    fn pin_key_is_the_version_and_the_content_key() {
        assert_eq!(pin_key(b"abc"), "0.1.0-ba7816bf8f01cfea");
    }

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
        let expected = home
            .join("bin")
            .join("0.1.0-ba7816bf8f01cfea")
            .join("viola");
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
            .join("viola");
        fs::create_dir_all(&slot).expect("a dir where the copy should be");
        assert!(matches!(pin_exe(&home, &exe), Err(PinError::State(_))));
    }

    /// A concurrent first start placed its copy at the path first: the same bytes are reused, other
    /// bytes are refused, and either way the file stands byte-identical.
    #[rstest]
    #[case::same_bytes(b"abc", true)]
    #[case::other_bytes(b"abd", false)]
    fn place_absent_over_a_copy_another_start_placed(
        #[case] standing: &[u8],
        #[case] reused: bool,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("viola");
        fs::write(&path, standing).expect("the other start's copy");
        let placed = place_absent(&path, b"abc", &content_key(b"abc"));
        if reused {
            assert!(placed.is_ok(), "{placed:?}");
        } else {
            assert!(matches!(placed, Err(PinError::HashMismatch)), "{placed:?}");
        }
        assert_eq!(fs::read(&path).expect("left as found"), standing);
    }

    #[test]
    fn place_absent_on_a_free_path_writes_the_copy() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("viola");
        place_absent(&path, b"abc", &content_key(b"abc")).expect("placed");
        assert_eq!(fs::read(&path).expect("copy"), b"abc");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let mode = fs::metadata(&path).expect("meta").permissions().mode() & 0o777;
            assert_eq!(mode, 0o700);
        }
    }

    /// `path` made unreadable to this user while it stays renamable (a rename over it needs only
    /// the directory's rights): mode 000. Undone on drop.
    struct Unreadable(PathBuf);

    impl Unreadable {
        fn set(path: &Path) -> Self {
            {
                use std::os::unix::fs::PermissionsExt as _;
                fs::set_permissions(path, fs::Permissions::from_mode(0o000)).expect("chmod");
            }
            assert!(fs::read(path).is_err(), "the copy is still readable");
            Self(path.to_owned())
        }
    }

    impl Drop for Unreadable {
        fn drop(&mut self) {
            use std::os::unix::fs::PermissionsExt as _;
            let _ = fs::set_permissions(&self.0, fs::Permissions::from_mode(0o600));
        }
    }

    /// A copy that exists but cannot be read is an error, never a missing copy to write over.
    #[test]
    fn pin_exe_refuses_a_copy_it_cannot_read_and_leaves_it() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        let exe = exe_in(tmp.path());
        let pinned = pin_exe(&home, &exe).expect("first pin");
        fs::write(&pinned.path, b"abd").expect("tamper");
        let denied = Unreadable::set(&pinned.path);
        assert!(matches!(pin_exe(&home, &exe), Err(PinError::State(_))));
        drop(denied);
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
