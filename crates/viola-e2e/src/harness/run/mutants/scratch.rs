//! The host mutation scratch: on a Windows host, cargo-mutants' temp copies (`TMP`/`TEMP`) and its
//! `--output` go to one dir beside the repo, wiped before every run, instead of `%TEMP%` on C: and
//! `mutants.out/` inside the tree an editor watches.

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// A const, never a `cfg!`-valued fn: a mutated const initializer is killable on either leg.
pub(super) const HOST_SCRATCH: bool = cfg!(windows);

const SCRATCH_NAME: &str = "viola-mutants-scratch";

/// `<repo parent>/viola-mutants-scratch`, from the repo's absolute path; `None` for a repo with no
/// parent.
fn scratch_dir(root: &Path) -> Option<PathBuf> {
    let root = std::path::absolute(root).ok()?;
    Some(root.parent()?.join(SCRATCH_NAME))
}

/// The bytes the host scratch holds now, 0 when it is absent: never its path.
pub(in crate::harness) fn host_scratch_bytes(root: &Path) -> u64 {
    scratch_dir(root).map_or(0, |dir| dir_bytes(&dir))
}

/// The wipe removes whatever `scratch` holds, so it runs only on a dir named exactly
/// `viola-mutants-scratch` that is neither the repo nor one of its ancestors. A drive or filesystem
/// root has no final component, so the name check refuses it too.
pub(super) fn scratch_allowed(root: &Path, scratch: &Path) -> bool {
    scratch.file_name() == Some(OsStr::new(SCRATCH_NAME)) && !root.starts_with(scratch)
}

/// The bytes under `path`, 0 when it is absent; a symlink counts as itself, never followed.
fn dir_bytes(path: &Path) -> u64 {
    let Ok(meta) = fs::symlink_metadata(path) else {
        return 0;
    };
    if !meta.is_dir() {
        return meta.len();
    }
    fs::read_dir(path)
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| dir_bytes(&entry.path()))
        .sum()
}

/// The scratch for this run, emptied: its dir and the bytes it held before the wipe. `Ok(None)` off
/// a Windows host, where cargo-mutants keeps the inherited temp dir and `mutants.out/` in the repo.
pub(super) fn prepare(root: &Path) -> Result<Option<(PathBuf, u64)>, String> {
    if !HOST_SCRATCH {
        return Ok(None);
    }
    let refused = || "scratch-refused".to_owned();
    let root = std::path::absolute(root).map_err(|_| refused())?;
    let scratch = scratch_dir(&root).ok_or_else(refused)?;
    if !scratch_allowed(&root, &scratch) {
        return Err(refused());
    }
    let bytes = dir_bytes(&scratch);
    wipe(&scratch).map_err(|_| "scratch-wipe-failed".to_owned())?;
    Ok(Some((scratch, bytes)))
}

/// A removal that fails (a leaked process holding a file) is an error, never ignored.
fn wipe(scratch: &Path) -> io::Result<()> {
    match fs::remove_dir_all(scratch) {
        Err(e) if e.kind() != io::ErrorKind::NotFound => return Err(e),
        _ => {}
    }
    fs::create_dir_all(scratch)
}

#[cfg(test)]
mod tests {
    use super::super::tests::{CAUGHT, scratch_or_root, stub_run};
    use super::*;

    #[test]
    fn host_scratch_is_the_windows_host_only() {
        assert_eq!(HOST_SCRATCH, cfg!(windows));
    }

    /// One labelled case per refusal, and the accepted sibling.
    #[test]
    fn scratch_allowed_takes_only_the_named_sibling() {
        let cases = [
            ("the sibling", "/w/viola", "/w/viola-mutants-scratch", true),
            ("another name", "/w/viola", "/w/viola-scratch", false),
            (
                "a longer name",
                "/w/viola",
                "/w/viola-mutants-scratch-2",
                false,
            ),
            (
                "the repo itself",
                "/w/viola-mutants-scratch",
                "/w/viola-mutants-scratch",
                false,
            ),
            (
                "an ancestor",
                "/w/viola-mutants-scratch/viola",
                "/w/viola-mutants-scratch",
                false,
            ),
            ("a filesystem root", "/viola", "/", false),
            (
                "a parent step",
                "/w/viola",
                "/w/viola-mutants-scratch/..",
                false,
            ),
        ];
        for (case, root, scratch, allowed) in cases {
            assert_eq!(
                scratch_allowed(Path::new(root), Path::new(scratch)),
                allowed,
                "{case}"
            );
        }
    }

    #[cfg(windows)]
    #[test]
    fn scratch_allowed_refuses_a_drive_root() {
        assert!(!scratch_allowed(Path::new(r"C:\viola"), Path::new(r"C:\")));
    }

    #[test]
    fn scratch_dir_is_the_repo_parents_named_child() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let root = tmp.path().join("repo");
        assert_eq!(
            scratch_dir(&root),
            Some(
                std::path::absolute(tmp.path())
                    .expect("abs")
                    .join(SCRATCH_NAME)
            )
        );
        assert_eq!(scratch_dir(Path::new("/")), None);
    }

    #[test]
    fn dir_bytes_sums_files_and_reads_an_absent_path_as_zero() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("d");
        fs::create_dir_all(dir.join("sub")).expect("mkdir");
        fs::write(dir.join("a"), [0u8; 3]).expect("write");
        fs::write(dir.join("sub").join("b"), [0u8; 4]).expect("write");
        assert_eq!(dir_bytes(&dir), 7);
        assert_eq!(dir_bytes(&dir.join("a")), 3);
        assert_eq!(dir_bytes(&tmp.path().join("none")), 0);
    }

    #[test]
    fn host_scratch_bytes_reads_the_sibling_scratch() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let root = tmp.path().join("repo");
        assert_eq!(host_scratch_bytes(&root), 0, "absent");
        fs::create_dir_all(tmp.path().join(SCRATCH_NAME)).expect("mkdir");
        fs::write(tmp.path().join(SCRATCH_NAME).join("a"), [0u8; 6]).expect("write");
        assert_eq!(host_scratch_bytes(&root), 6);
    }

    #[test]
    fn prepare_empties_the_scratch_and_reports_what_it_held() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let root = tmp.path().join("repo");
        fs::create_dir_all(&root).expect("mkdir");
        let scratch = tmp.path().join(SCRATCH_NAME);
        if HOST_SCRATCH {
            let (fresh, bytes) = prepare(&root).expect("absent").expect("a scratch");
            assert_eq!(
                (fresh.is_dir(), bytes),
                (true, 0),
                "an absent scratch is created"
            );
        }
        fs::create_dir_all(scratch.join("old")).expect("mkdir");
        fs::write(scratch.join("old").join("copy"), [0u8; 5]).expect("write");
        let prepared = prepare(&root).expect("prepared");
        if HOST_SCRATCH {
            let (dir, bytes) = prepared.expect("a scratch on this host");
            assert_eq!(dir, std::path::absolute(&scratch).expect("abs"));
            assert_eq!(bytes, 5);
            assert!(dir.is_dir());
            assert_eq!(fs::read_dir(&dir).expect("read").count(), 0);
            assert_eq!(prepare(&root).expect("again"), Some((dir, 0)));
        } else {
            assert_eq!(prepared, None);
            assert!(scratch.join("old").join("copy").is_file(), "left alone");
        }
    }

    #[cfg(windows)]
    #[test]
    fn prepare_refuses_a_repo_whose_scratch_is_an_ancestor() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let root = tmp.path().join(SCRATCH_NAME).join(SCRATCH_NAME);
        fs::create_dir_all(&root).expect("mkdir");
        fs::write(root.join("keep"), b"x").expect("write");
        assert_eq!(prepare(&root), Err("scratch-refused".to_owned()));
        assert!(root.join("keep").is_file(), "nothing was wiped");
    }

    /// A file held open without delete sharing cannot be removed on Windows: the wipe fails by name.
    #[cfg(windows)]
    #[test]
    fn prepare_names_a_wipe_that_failed() {
        use std::os::windows::fs::OpenOptionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let root = tmp.path().join("repo");
        fs::create_dir_all(&root).expect("mkdir");
        let scratch = tmp.path().join(SCRATCH_NAME);
        fs::create_dir_all(&scratch).expect("mkdir");
        let held = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .share_mode(0)
            .open(scratch.join("held"))
            .expect("held");
        assert_eq!(prepare(&root), Err("scratch-wipe-failed".to_owned()));
        drop(held);
    }

    /// On a Windows host cargo-mutants' temp copies and `--output` go to the repo's sibling scratch,
    /// wiped first and reported; nothing lands in `mutants.out/` under the repo. Elsewhere nothing
    /// changes.
    #[test]
    fn run_mutants_takes_the_host_scratch_on_windows_only() {
        let s = stub_run(&[], None, Some(CAUGHT), |ws| {
            let scratch = scratch_or_root(ws);
            if cfg!(windows) {
                fs::create_dir_all(&scratch).expect("mkdir");
                fs::write(scratch.join("left"), [0u8; 4]).expect("write");
            }
        });
        let (args, env) = s.mutants.expect("cargo mutants ran");
        let scratch = scratch_or_root(&s.ws);
        let temp = |name: &str| {
            env.iter()
                .find(|(k, _)| k == name)
                .and_then(|(_, v)| v.clone())
        };
        let output = args
            .iter()
            .position(|a| a == "--output")
            .map(|at| args[at + 1].clone());
        if cfg!(windows) {
            let dir = std::path::absolute(&scratch).expect("abs");
            let dir = dir.to_string_lossy().into_owned();
            assert_eq!(temp("TMP").as_deref(), Some(dir.as_str()));
            assert_eq!(temp("TEMP").as_deref(), Some(dir.as_str()));
            assert_eq!(output.as_deref(), Some(dir.as_str()));
            assert_eq!(s.out.doc["mutants"]["scratch_bytes"], 4, "{}", s.out.doc);
            assert!(
                !scratch.join("left").exists(),
                "the scratch was wiped first"
            );
            assert!(!s.ws.root.join("mutants.out").exists());
        } else {
            assert_eq!((temp("TMP"), temp("TEMP"), output), (None, None, None));
            assert!(s.out.doc["mutants"].get("scratch_bytes").is_none());
        }
        assert_eq!(s.out.code, 0, "{}", s.out.doc);
    }
}
