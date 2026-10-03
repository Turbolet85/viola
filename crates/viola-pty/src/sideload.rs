//! Which ConPTY this process uses. portable-pty 0.8.1 loads `conpty.dll` by bare name once per
//! process (its `src/win/psuedocon.rs:54`, behind the `lazy_static` at :61-63) and falls back to
//! kernel32's inbox ConPTY when that load fails. [`restrict_dll_search`] keeps a `conpty.dll`
//! planted in the working directory or on `PATH` out of that load; [`preload`] makes it return a
//! module the caller has already verified. This crate knows no path of its own.

use std::io;
use std::os::windows::ffi::OsStrExt as _;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use windows_sys::Win32::System::LibraryLoader::{
    LOAD_LIBRARY_SEARCH_SYSTEM32, LoadLibraryExW, SetDefaultDllDirectories,
};

use crate::PtyError;

static PRELOADED: AtomicBool = AtomicBool::new(false);
static RESTRICTED: AtomicBool = AtomicBool::new(false);

/// Restricts every later DLL search of this process to System32: process-wide and one-way.
/// Whether it took effect.
pub fn restrict_dll_search() -> bool {
    // SAFETY: passes a flags value; no pointer crosses.
    let restricted = unsafe { SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_SYSTEM32) != 0 };
    if restricted {
        RESTRICTED.store(true, Ordering::SeqCst);
    }
    restricted
}

/// Whether [`restrict_dll_search`] has taken effect in this process.
pub fn search_restricted() -> bool {
    RESTRICTED.load(Ordering::SeqCst)
}

/// Loads `dll` by absolute path and keeps it loaded for the life of the process, so a later
/// bare-name `conpty.dll` load returns this module. It takes effect only before the process's first
/// ConPTY use: portable-pty resolves its functions once, on the first pseudo-terminal it opens.
pub fn preload(dll: &Path) -> Result<(), PtyError> {
    if !dll.is_absolute() {
        return Err(PtyError::Load(Box::new(io::Error::from(
            io::ErrorKind::InvalidInput,
        ))));
    }
    let wide: Vec<u16> = dll.as_os_str().encode_wide().chain([0]).collect();
    // SAFETY: `wide` is NUL-terminated and outlives the call; the module is never freed, so the
    // handle is not kept.
    let module = unsafe { LoadLibraryExW(wide.as_ptr(), std::ptr::null_mut(), 0) };
    if module.is_null() {
        return Err(PtyError::Load(Box::new(io::Error::last_os_error())));
    }
    PRELOADED.store(true, Ordering::SeqCst);
    Ok(())
}

pub(crate) fn preloaded() -> bool {
    PRELOADED.load(Ordering::SeqCst)
}

/// Each case changes process-wide loader state, which nextest confines to the case's own process.
#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::process::Command;

    use windows_sys::Win32::System::LibraryLoader::{
        GetModuleFileNameW, GetModuleHandleW, LoadLibraryW,
    };

    use super::*;

    const CASE: &str = "PTY_SIDELOAD_TEST_CASE";
    const REPORT: &str = "PTY_SIDELOAD_TEST_REPORT";

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain([0]).collect()
    }

    fn vendored_dll() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../vendor/conpty/1.24.260710001/x64/conpty.dll")
            .canonicalize()
            .expect("the vendored conpty.dll")
    }

    fn module_path(module: *mut std::ffi::c_void) -> PathBuf {
        let mut buf = vec![0u16; 32_768];
        // SAFETY: `buf` is writable for its whole length, which is the size passed.
        let n = unsafe { GetModuleFileNameW(module, buf.as_mut_ptr(), buf.len() as u32) };
        assert!(n > 0, "module path unreadable");
        PathBuf::from(String::from_utf16_lossy(&buf[..n as usize]))
    }

    /// A bare-name load: the file the loader picked, or the error code when it found none.
    fn bare_load() -> Result<PathBuf, i32> {
        let name = wide("conpty.dll");
        // SAFETY: `name` is NUL-terminated and outlives the call.
        let module = unsafe { LoadLibraryW(name.as_ptr()) };
        if module.is_null() {
            return Err(io::Error::last_os_error().raw_os_error().unwrap_or(0));
        }
        Ok(module_path(module))
    }

    fn system32_conpty() -> Option<PathBuf> {
        let root = std::env::var_os("SystemRoot")?;
        let path = PathBuf::from(root).join("System32").join("conpty.dll");
        path.is_file().then_some(path)
    }

    fn same_file(a: &Path, b: &Path) -> bool {
        match (a.canonicalize(), b.canonicalize()) {
            (Ok(a), Ok(b)) => a.as_os_str().eq_ignore_ascii_case(b.as_os_str()),
            _ => false,
        }
    }

    /// The child half of the planting cases: a no-op in an ordinary run; spawned by one, it makes one
    /// bare-name load and writes what it got to its report file. The `search-restricted` case
    /// instead reports the flag before and after the restriction in this fresh process.
    #[test]
    fn sideload_child_entry() {
        let (Ok(case), Some(report)) = (std::env::var(CASE), std::env::var_os(REPORT)) else {
            return;
        };
        if case == "search-restricted" {
            let before = search_restricted();
            let took = restrict_dll_search();
            let after = search_restricted();
            std::fs::write(report, format!("before={before} took={took} after={after}"))
                .expect("report");
            return;
        }
        if case == "restricted" {
            assert!(restrict_dll_search());
        }
        let line = match bare_load() {
            Ok(path) => format!("loaded={}", path.display()),
            Err(code) => format!("error={code}"),
        };
        std::fs::write(report, line).expect("report");
    }

    /// A copy of a System32 DLL named `conpty.dll`: loadable, and nothing a ConPTY caller could use.
    fn plant(dir: &Path) -> PathBuf {
        let root = std::env::var_os("SystemRoot").expect("SystemRoot");
        let source = PathBuf::from(root).join("System32").join("version.dll");
        let planted = dir.join("conpty.dll");
        std::fs::copy(source, &planted).expect("plant");
        planted
    }

    fn run_child(case: &str, cwd: &Path, path_dir: &Path) -> String {
        let inherited = std::env::var_os("PATH").unwrap_or_default();
        let dirs = std::iter::once(path_dir.to_path_buf()).chain(std::env::split_paths(&inherited));
        let path: OsString = std::env::join_paths(dirs).expect("PATH");
        let report = tempfile::tempdir().expect("tempdir");
        let report = report.path().join("report");
        let out = Command::new(std::env::current_exe().expect("test binary"))
            .args(["--exact", "sideload::tests::sideload_child_entry"])
            .current_dir(cwd)
            .env(CASE, case)
            .env(REPORT, &report)
            .env("PATH", path)
            .output()
            .expect("child");
        assert!(out.status.success(), "{out:?}");
        std::fs::read_to_string(&report).expect("the child's report")
    }

    fn loaded(line: &str) -> Option<PathBuf> {
        line.strip_prefix("loaded=").map(PathBuf::from)
    }

    /// Unrestricted, a bare-name load takes a plant from the working directory, then from `PATH`
    /// (where System32 holds no `conpty.dll`); restricted, it takes neither.
    #[test]
    fn restrict_dll_search_keeps_planted_conpty_out_of_a_bare_name_load() {
        let cwd_dir = tempfile::tempdir().expect("tempdir");
        let path_dir = tempfile::tempdir().expect("tempdir");
        let clean_dir = tempfile::tempdir().expect("tempdir");
        let cwd_plant = plant(cwd_dir.path());
        let path_plant = plant(path_dir.path());
        let system = system32_conpty();

        let from_cwd = run_child("default", cwd_dir.path(), path_dir.path());
        let from_path = run_child("default", clean_dir.path(), path_dir.path());
        let restricted = run_child("restricted", cwd_dir.path(), path_dir.path());
        match &system {
            None => {
                let got = loaded(&from_cwd).expect(&from_cwd);
                assert!(same_file(&got, &cwd_plant), "{from_cwd}");
                let got = loaded(&from_path).expect(&from_path);
                assert!(same_file(&got, &path_plant), "{from_path}");
                assert_eq!(restricted, "error=126");
            }
            Some(system) => {
                let got = loaded(&restricted).expect(&restricted);
                assert!(same_file(&got, system), "{restricted}");
            }
        }
    }

    /// `search_restricted` reads the restriction's own outcome: unset in a fresh process, set once
    /// `restrict_dll_search` took effect.
    #[test]
    fn search_restricted_reads_whether_the_restriction_took_effect() {
        let dir = tempfile::tempdir().expect("tempdir");
        let line = run_child("search-restricted", dir.path(), dir.path());
        assert_eq!(line, "before=false took=true after=true");
    }

    #[test]
    fn preload_makes_a_bare_name_load_return_the_verified_module() {
        let dll = vendored_dll();
        preload(&dll).expect("preload");
        assert_eq!(crate::pty_backend(), "conpty-sideload");
        let name = wide("conpty.dll");
        // SAFETY: `name` is NUL-terminated and outlives both calls.
        let (held, bare) =
            unsafe { (GetModuleHandleW(name.as_ptr()), LoadLibraryW(name.as_ptr())) };
        assert!(!held.is_null());
        assert_eq!(held, bare);
        assert!(same_file(&module_path(held), &dll));
    }

    #[test]
    fn preload_of_a_file_that_is_no_dll_fails_and_keeps_the_inbox_backend() {
        let dir = tempfile::tempdir().expect("tempdir");
        let fake = dir.path().join("conpty.dll");
        std::fs::write(&fake, b"not a module").expect("fake");
        let err = preload(&fake).expect_err("not a module");
        assert!(matches!(err, PtyError::Load(_)));
        assert_eq!(err.to_string(), "pty load failed");
        assert!(matches!(
            preload(Path::new("conpty.dll")),
            Err(PtyError::Load(_))
        ));
        assert_eq!(crate::pty_backend(), "conpty");
    }
}
