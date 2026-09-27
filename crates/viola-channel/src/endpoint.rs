//! The per-instance endpoint: its name (architecture §Conventions → Endpoint name) and where it
//! lives — a named pipe on Windows, a socket in the per-user 0700 directory on Unix (security-plan
//! §Authentication & Authorization, IPC access control, Decisions Log amendment 2).

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use viola_core::ViolaName;

use crate::ChannelError;

pub const ENDPOINT_KIND: &str = if cfg!(windows) {
    "named-pipe"
} else {
    "unix-socket"
};

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// FNV-1a 64, written out: std's `DefaultHasher` is not stable across Rust releases, and mixed
/// binary versions must agree on the name.
fn fnv1a(bytes: impl IntoIterator<Item = u8>) -> u64 {
    bytes.into_iter().fold(FNV_OFFSET, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME)
    })
}

/// `viola-<h12>`: the first 12 hex characters of FNV-1a 64 over the name, `\0` and the absolute
/// home's UTF-8 bytes. A non-UTF-8 home is refused, never hashed lossily.
pub fn endpoint_name(name: &ViolaName, home: &Path) -> Result<String, ChannelError> {
    let home = home.to_str().ok_or(ChannelError::NonUtf8Path)?;
    let name: &str = name.as_ref();
    let hash = fnv1a(name.bytes().chain([0]).chain(home.bytes()));
    let hex = format!("{hash:016x}");
    Ok(format!("viola-{}", &hex[..12]))
}

/// The per-user socket directory: `$XDG_RUNTIME_DIR/viola` on Linux, `$TMPDIR/viola` on macOS,
/// `/tmp/viola-<uid>` otherwise. A relative or empty value is not a location and falls back.
pub fn socket_dir(xdg: Option<&OsStr>, tmpdir: Option<&OsStr>, uid: u32, macos: bool) -> PathBuf {
    match (macos, absolute(xdg), absolute(tmpdir)) {
        (false, Some(xdg), _) => Path::new(xdg).join("viola"),
        (true, _, Some(tmpdir)) => Path::new(tmpdir).join("viola"),
        _ => PathBuf::from(format!("/tmp/viola-{uid}")),
    }
}

fn absolute(value: Option<&OsStr>) -> Option<&OsStr> {
    value.filter(|v| v.as_encoded_bytes().first() == Some(&b'/'))
}

/// The endpoint `viola run` binds and records in its snapshot: `\\.\pipe\viola-<h12>` on Windows,
/// `<socket dir>/viola-<h12>.sock` on Unix.
pub fn endpoint_path(name: &ViolaName, home: &Path) -> Result<String, ChannelError> {
    let base = endpoint_name(name, home)?;
    #[cfg(windows)]
    {
        Ok(format!(r"\\.\pipe\{base}"))
    }
    #[cfg(unix)]
    {
        host_socket_dir()
            .join(format!("{base}.sock"))
            .into_os_string()
            .into_string()
            .map_err(|_| ChannelError::NonUtf8Path)
    }
}

/// The platform locations `socket_dir` decides between, read here only (never a configuration
/// channel: security-plan amendment 2 names them).
#[cfg(unix)]
fn host_socket_dir() -> PathBuf {
    // SAFETY: getuid has no preconditions and cannot fail.
    let uid = unsafe { libc::getuid() };
    socket_dir(
        std::env::var_os("XDG_RUNTIME_DIR").as_deref(),
        std::env::var_os("TMPDIR").as_deref(),
        uid,
        cfg!(target_os = "macos"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn name(raw: &str) -> ViolaName {
        ViolaName::try_new(raw.to_owned()).expect("valid")
    }

    #[test]
    fn fnv1a_matches_the_published_vectors() {
        assert_eq!(fnv1a([]), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a(*b"a"), 0xaf63_dc4c_8601_ec8c);
    }

    /// Oracles computed outside the product (a Python FNV-1a over `name + "\0" + home`).
    #[rstest]
    #[case::windows_home("builder", r"C:\Users\u\.viola", "viola-2c5208c9fe52")]
    #[case::unix_home("overseer", "/home/u/.viola", "viola-6a533790db6e")]
    fn endpoint_name_golden_vectors(#[case] raw: &str, #[case] home: &str, #[case] want: &str) {
        assert_eq!(
            endpoint_name(&name(raw), Path::new(home)).expect("utf-8"),
            want
        );
    }

    #[test]
    fn endpoint_name_differs_by_home_and_by_name() {
        let a = endpoint_name(&name("builder"), Path::new("/h/one")).expect("utf-8");
        let b = endpoint_name(&name("builder"), Path::new("/h/two")).expect("utf-8");
        let c = endpoint_name(&name("overseer"), Path::new("/h/one")).expect("utf-8");
        assert!(a != b && a != c && b != c);
        assert_eq!(a.len(), "viola-".len() + 12);
    }

    #[cfg(unix)]
    #[test]
    fn endpoint_name_refuses_a_non_utf8_home() {
        use std::os::unix::ffi::OsStrExt as _;
        let home = Path::new(OsStr::from_bytes(b"/home/\xff"));
        assert!(matches!(
            endpoint_name(&name("builder"), home),
            Err(ChannelError::NonUtf8Path)
        ));
    }

    #[cfg(windows)]
    #[test]
    fn endpoint_name_refuses_a_non_utf8_home() {
        use std::os::windows::ffi::OsStringExt as _;
        let home = std::ffi::OsString::from_wide(&[u16::from(b'C'), u16::from(b':'), 0xd800]);
        assert!(matches!(
            endpoint_name(&name("builder"), Path::new(&home)),
            Err(ChannelError::NonUtf8Path)
        ));
    }

    #[rstest]
    #[case::linux_xdg(Some("/run/user/1000"), Some("/tmp/x"), false, "/run/user/1000/viola")]
    #[case::linux_no_xdg(None, Some("/tmp/x"), false, "/tmp/viola-1000")]
    #[case::linux_relative_xdg(Some("run/user"), None, false, "/tmp/viola-1000")]
    #[case::linux_empty_xdg(Some(""), None, false, "/tmp/viola-1000")]
    #[case::macos_tmpdir(
        Some("/run/user/1000"),
        Some("/var/folders/ab/T/"),
        true,
        "/var/folders/ab/T/viola"
    )]
    #[case::macos_no_tmpdir(Some("/run/user/1000"), None, true, "/tmp/viola-1000")]
    fn socket_dir_picks_the_per_user_dir(
        #[case] xdg: Option<&str>,
        #[case] tmpdir: Option<&str>,
        #[case] macos: bool,
        #[case] want: &str,
    ) {
        let got = socket_dir(xdg.map(OsStr::new), tmpdir.map(OsStr::new), 1000, macos);
        assert_eq!(got, PathBuf::from(want));
    }

    #[cfg(windows)]
    #[test]
    fn endpoint_path_is_the_named_pipe_on_windows() {
        let got = endpoint_path(&name("builder"), Path::new(r"C:\Users\u\.viola")).expect("path");
        assert_eq!(got, r"\\.\pipe\viola-2c5208c9fe52");
        assert_eq!(ENDPOINT_KIND, "named-pipe");
    }

    #[cfg(unix)]
    #[test]
    fn endpoint_path_is_a_socket_in_the_per_user_dir_on_unix() {
        let got = endpoint_path(&name("overseer"), Path::new("/home/u/.viola")).expect("path");
        let path = Path::new(&got);
        assert_eq!(
            path.file_name(),
            Some(OsStr::new("viola-6a533790db6e.sock"))
        );
        let dir = path.parent().expect("socket dir");
        assert!(dir.is_absolute(), "{got}");
        let dir_name = dir.file_name().and_then(OsStr::to_str).expect("dir name");
        assert!(
            dir_name == "viola" || dir_name.starts_with("viola-"),
            "{got}"
        );
        assert_eq!(ENDPOINT_KIND, "unix-socket");
    }
}
