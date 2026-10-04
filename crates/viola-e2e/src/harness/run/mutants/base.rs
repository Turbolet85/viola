//! The chunk diff's base: the master flip before the pending chunk's operator pass, and the diff
//! of the working tree against it (test-plan §3 `run` step 4).

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

pub(super) fn git(repo: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

/// The full sha a revision names, when it names a commit.
fn commit_sha(repo: &Path, rev: &str) -> Option<String> {
    git(
        repo,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("{rev}^{{commit}}"),
        ],
    )
    .map(|sha| sha.trim().to_owned())
}

const MASTER_ROUTE: &str = ".andromeda/master-route.md";

/// `AGENT_RUN_CHUNK_BASE` when set; else the chunk's last master flip (`chunk_flip`); else the
/// merge-base with `origin/main`. `None` when the chosen revision is not a commit.
pub fn resolve_base(repo: &Path, from_env: Option<String>) -> Option<String> {
    let base = from_env
        .map(|b| b.trim().to_owned())
        .filter(|b| !b.is_empty())
        .or_else(|| chunk_flip(repo))
        .or_else(|| {
            git(repo, &["merge-base", "HEAD", "origin/main"]).map(|b| b.trim().to_owned())
        })?;
    commit_sha(repo, &base)
}

/// The last commit that flipped a master record, found from the parent of the oldest operator
/// pre-CI commit of a pending chunk (else from HEAD), so no commit of an operator pass can move it —
/// not even one editing a `complete` record. The wrap push is itself the flip and reads its parent.
fn chunk_flip(repo: &Path) -> Option<String> {
    let bound = pre_ci_parent(repo).unwrap_or_else(|| "HEAD".to_owned());
    let found = git(
        repo,
        &[
            "log",
            "-1",
            "--format=%H",
            "-G",
            " · complete · ",
            &bound,
            "--",
            MASTER_ROUTE,
        ],
    )?;
    let flip = found.trim();
    if flip.is_empty() {
        return None;
    }
    if Some(flip.to_owned()) == commit_sha(repo, "HEAD") && !uncommitted_promotion(repo) {
        return commit_sha(repo, "HEAD^");
    }
    Some(flip.to_owned())
}

/// A chunk promoted but not yet committed: the working tree's master route holds a pending record
/// HEAD's copy lacks, so the chunk's work sits on top of HEAD even when HEAD is the flip.
fn uncommitted_promotion(repo: &Path) -> bool {
    let Ok(tree) = fs::read_to_string(repo.join(MASTER_ROUTE)) else {
        return false;
    };
    let head = git(repo, &["show", &format!("HEAD:{MASTER_ROUTE}")]).unwrap_or_default();
    let committed: Vec<&str> = head.lines().filter_map(pending_marker).collect();
    tree.lines()
        .filter_map(pending_marker)
        .any(|marker| !committed.contains(&marker))
}

/// `{sha}^` of the oldest commit in HEAD's history whose message carries a pending chunk's
/// `chore({marker}): operator pre-CI commit`.
fn pre_ci_parent(repo: &Path) -> Option<String> {
    let master = git(repo, &["show", &format!("HEAD:{MASTER_ROUTE}")])?;
    let greps: Vec<String> = master
        .lines()
        .filter_map(pending_marker)
        .map(|marker| format!("chore({marker}): operator pre-CI commit"))
        .collect();
    if greps.is_empty() {
        return None;
    }
    let mut args = vec!["log", "--reverse", "--format=%H", "-F"];
    for grep in &greps {
        args.extend(["--grep", grep.as_str()]);
    }
    args.push("HEAD");
    let oldest = git(repo, &args)?.lines().next()?.to_owned();
    Some(format!("{oldest}^"))
}

/// The marker of a `{marker} · pending · …` master record line.
fn pending_marker(line: &str) -> Option<&str> {
    line.split_once(" · pending · ")
        .map(|(marker, _)| marker)
        .filter(|m| !m.is_empty() && !m.contains(' '))
}

/// The chunk as the working tree holds it: tracked changes since the merge-base plus every
/// untracked, non-ignored file (equal to `<base>...HEAD` once the chunk is committed). The
/// prefixes are pinned: `diff_paths` and cargo-mutants' `--in-diff` read `a/`/`b/` headers, which a
/// user's `diff.mnemonicprefix` or `diff.noprefix` would otherwise rewrite.
pub fn chunk_diff(repo: &Path, base: &str) -> Option<String> {
    let merge_base = git(repo, &["merge-base", base, "HEAD"])?;
    let mut diff = git(
        repo,
        &[
            "diff",
            "--no-color",
            "--no-ext-diff",
            "--src-prefix=a/",
            "--dst-prefix=b/",
            merge_base.trim(),
        ],
    )?;
    let untracked = git(repo, &["ls-files", "--others", "--exclude-standard", "-z"])?;
    for file in untracked.split('\0').filter(|f| !f.is_empty()) {
        let out = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args([
                "diff",
                "--no-color",
                "--no-ext-diff",
                "--src-prefix=a/",
                "--dst-prefix=b/",
                "--no-index",
                "--",
                "/dev/null",
                file,
            ])
            .stderr(Stdio::null())
            .output()
            .ok()?;
        diff.push_str(&String::from_utf8_lossy(&out.stdout));
    }
    Some(diff)
}

/// Every path a `diff --git a/<old> b/<new>` header names, both sides (a rename away from `.rs`
/// still removes Rust source).
pub fn diff_paths(diff: &str) -> Vec<&str> {
    diff.lines()
        .filter_map(|l| l.strip_prefix("diff --git a/"))
        .filter_map(|rest| rest.rsplit_once(" b/"))
        .flat_map(|(old, new)| [old, new])
        .collect()
}

/// The number of files the diff changes (one `diff --git` header each).
pub fn diff_files(diff: &str) -> usize {
    diff.lines()
        .filter(|l| l.starts_with("diff --git "))
        .count()
}

pub fn rust_delta(diff: &str) -> bool {
    diff_paths(diff).iter().any(|p| p.ends_with(".rs"))
}

/// Every Rust path the diff names, once each, in diff order.
pub fn rust_paths(diff: &str) -> Vec<&str> {
    let mut paths: Vec<&str> = Vec::new();
    for path in diff_paths(diff).into_iter().filter(|p| p.ends_with(".rs")) {
        if !paths.contains(&path) {
            paths.push(path);
        }
    }
    paths
}

/// A file of an auto-discovered test, bench or example target of the root package or a
/// `crates/<member>`: cargo-mutants 27.1.0 mutates lib and bin targets only (measured: `--list-files`
/// over a package with `src/`, `tests/`, `benches/` and `examples/` files lists `src/lib.rs` alone).
pub fn test_target(path: &str) -> bool {
    let within = match path.strip_prefix("crates/") {
        Some(member) => member.split_once('/').map_or("", |(_, rest)| rest),
        None => path,
    };
    ["tests/", "benches/", "examples/"]
        .iter()
        .any(|dir| within.starts_with(dir))
}

#[cfg(test)]
mod tests {
    use super::super::super::test_support::git_repo;
    use super::*;

    #[test]
    fn resolve_base_needs_a_real_commit() {
        let repo = git_repo();
        let head = git(repo.path(), &["rev-parse", "HEAD"]).expect("head");
        assert_eq!(
            resolve_base(repo.path(), Some(format!(" {} ", head.trim()))),
            Some(head.trim().to_owned())
        );
        let zeros = "0".repeat(40);
        assert_eq!(resolve_base(repo.path(), Some(zeros)), None);
        assert_eq!(resolve_base(repo.path(), None), None);
        assert_eq!(resolve_base(repo.path(), Some("  ".to_owned())), None);
    }

    const FLIPPED: &str = "## p-0.1.0\na · complete · first · → x\n";
    const PENDING: &str = "## p-0.1.0\na · complete · first · → x\nm · pending · second · → y\n";
    const EDITED: &str =
        "## p-0.1.0\na · complete · first, edited mid-pass · → x\nm · pending · second · → y\n";
    const WRAPPED: &str =
        "## p-0.1.0\na · complete · first, edited mid-pass · → x\nm · complete · second · → y\n";

    /// A repo whose history an operator pass leaves: each commit rewrites the code file and, when
    /// given, the master route.
    struct Pass(tempfile::TempDir);

    impl Pass {
        fn new() -> Self {
            Self(git_repo())
        }

        fn head(&self) -> String {
            git(self.0.path(), &["rev-parse", "HEAD"])
                .expect("head")
                .trim()
                .to_owned()
        }

        fn commit(&self, subject: &str, master: Option<&str>, code: &str) -> String {
            let dir = self.0.path();
            if let Some(text) = master {
                fs::create_dir_all(dir.join(".andromeda")).expect("mkdir");
                fs::write(dir.join(MASTER_ROUTE), text).expect("master");
            }
            fs::write(dir.join("a.rs"), code).expect("code");
            for args in [vec!["add", "-A"], vec!["commit", "-q", "-m", subject]] {
                let ok = Command::new("git")
                    .arg("-C")
                    .arg(dir)
                    .args([
                        "-c",
                        "user.name=t",
                        "-c",
                        "user.email=t@example.com",
                        "-c",
                        "maintenance.auto=false",
                    ])
                    .args(&args)
                    .output()
                    .expect("git")
                    .status
                    .success();
                assert!(ok, "git {args:?}");
            }
            self.head()
        }

        fn base(&self) -> Option<String> {
            resolve_base(self.0.path(), None)
        }
    }

    #[test]
    fn chunk_base_holds_the_flip_across_a_pass() {
        let p = Pass::new();
        p.commit(
            "chore(a): operator pre-CI commit, for the run a's verdict reads",
            None,
            "fn a() { 0; }\n",
        );
        let flip = p.commit("feat(a): wrap a", Some(FLIPPED), "fn a() {}\n");
        p.commit(
            "chore(m): operator pre-CI commit, for the run m's verdict reads",
            Some(PENDING),
            "fn a() { 1; }\n",
        );
        assert_eq!(p.base(), Some(flip.clone()));
        p.commit(
            "fix(m): operator fix after CI run 1, code only",
            None,
            "fn a() { 2; }\n",
        );
        assert_eq!(p.base(), Some(flip.clone()));
        p.commit(
            "fix(m): operator fix after CI run 2, edits a complete record",
            Some(EDITED),
            "fn a() { 3; }\n",
        );
        assert_eq!(p.base(), Some(flip));
    }

    #[test]
    fn chunk_base_is_the_last_flip_before_any_pre_ci_commit() {
        let p = Pass::new();
        let flip = p.commit("feat(a): wrap a", Some(FLIPPED), "fn a() {}\n");
        p.commit("chore(route): adaptation", None, "fn a() { 1; }\n");
        assert_eq!(p.base(), Some(flip.clone()));
        p.commit(
            "docs(m): promoted, not pushed",
            Some(PENDING),
            "fn a() { 2; }\n",
        );
        assert_eq!(p.base(), Some(flip));
    }

    #[test]
    fn chunk_base_of_the_wrap_push_is_its_parent() {
        let p = Pass::new();
        let root = p.head();
        p.commit("feat(a): wrap a", Some(FLIPPED), "fn a() {}\n");
        assert_eq!(p.base(), Some(root));
        p.commit(
            "chore(m): operator pre-CI commit, for the run m's verdict reads",
            Some(PENDING),
            "fn a() { 1; }\n",
        );
        let fix = p.commit(
            "fix(m): operator fix after CI run 1, edits a complete record",
            Some(EDITED),
            "fn a() { 2; }\n",
        );
        p.commit("feat(m): wrap m", Some(WRAPPED), "fn a() { 2; }\n");
        assert_eq!(p.base(), Some(fix));
    }

    #[test]
    fn chunk_base_of_an_uncommitted_promotion_is_the_flip_at_head() {
        let p = Pass::new();
        let flip = p.commit("feat(a): wrap a", Some(FLIPPED), "fn a() {}\n");
        fs::write(p.0.path().join(MASTER_ROUTE), PENDING).expect("promote");
        fs::write(p.0.path().join("a.rs"), "fn a() { 1; }\n").expect("code");
        assert_eq!(p.base(), Some(flip));
    }

    #[test]
    fn chunk_base_of_a_pending_record_head_already_holds_is_the_parent() {
        let p = Pass::new();
        let root = p.head();
        p.commit("feat(a): wrap a", Some(PENDING), "fn a() {}\n");
        assert_eq!(p.base(), Some(root));
    }

    #[test]
    fn chunk_base_bound_is_the_oldest_pre_ci_commit() {
        let p = Pass::new();
        let flip = p.commit("feat(a): wrap a", Some(FLIPPED), "fn a() {}\n");
        p.commit(
            "chore(m): operator pre-CI commit, first",
            Some(PENDING),
            "fn a() { 1; }\n",
        );
        p.commit(
            "fix(m): operator fix after CI run 1, edits a complete record",
            Some(EDITED),
            "fn a() { 2; }\n",
        );
        p.commit(
            "chore(m): operator pre-CI commit, again",
            None,
            "fn a() { 3; }\n",
        );
        assert_eq!(p.base(), Some(flip));
    }

    #[test]
    fn chunk_base_override_wins() {
        let p = Pass::new();
        let root = p.head();
        p.commit("feat(a): wrap a", Some(FLIPPED), "fn a() {}\n");
        p.commit("chore(route): adaptation", None, "fn a() { 1; }\n");
        assert_eq!(
            resolve_base(p.0.path(), Some(root[..12].to_owned())),
            Some(root)
        );
    }

    #[test]
    fn chunk_base_reads_only_pending_record_markers() {
        assert_eq!(pending_marker("m-1 · pending · d · → l"), Some("m-1"));
        assert_eq!(pending_marker("m-1 · complete · d · → l"), None);
        assert_eq!(pending_marker(" · pending · d · → l"), None);
        assert_eq!(pending_marker("a note · pending · d"), None);
    }

    #[test]
    fn chunk_diff_holds_tracked_edits_and_untracked_files() {
        let repo = git_repo();
        fs::write(repo.path().join("a.rs"), "fn a() { let _x = 1; }\n").expect("write");
        fs::write(repo.path().join("b.rs"), "fn b() {}\n").expect("write");
        let diff = chunk_diff(repo.path(), "HEAD").expect("diff");
        assert!(diff.contains("+fn a() { let _x = 1; }"));
        assert!(diff.contains("+fn b() {}"));
        assert!(diff.contains("b.rs"));
        assert_eq!(chunk_diff(repo.path(), &"0".repeat(40)), None);
    }

    #[test]
    fn chunk_diff_pins_its_prefixes_under_a_hostile_config() {
        let cases: [(&str, Option<&str>); 3] = [
            ("default", None),
            ("mnemonicprefix", Some("diff.mnemonicprefix")),
            ("noprefix", Some("diff.noprefix")),
        ];
        let mut failed: Vec<String> = Vec::new();
        for (label, key) in cases {
            let repo = git_repo();
            if let Some(key) = key {
                git(repo.path(), &["config", key, "true"]).expect("config");
            }
            fs::write(repo.path().join("a.rs"), "fn a() { let _x = 1; }\n").expect("write");
            fs::write(repo.path().join("b.rs"), "fn b() {}\n").expect("write");
            let diff = chunk_diff(repo.path(), "HEAD").expect("diff");
            let headers: Vec<&str> = diff
                .lines()
                .filter(|l| l.starts_with("diff --git "))
                .collect();
            let pinned =
                headers.len() == 2 && headers.iter().all(|h| h.starts_with("diff --git a/"));
            if !pinned || rust_paths(&diff) != ["a.rs", "b.rs"] {
                failed.push(format!("{label}: {headers:?}"));
            }
        }
        assert!(failed.is_empty(), "{failed:#?}");
    }

    const RUST_DIFF: &str =
        "diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs\n";
    const DOCS_DIFF: &str = "diff --git a/README.md b/README.md\n--- a/README.md\n+++ b/README.md\n\
        diff --git a/docs/x y.md b/docs/x y.md\n";
    const RENAME_DIFF: &str =
        "diff --git a/src/old.rs b/notes/old.md\nsimilarity index 100%\nrename from src/old.rs\n";

    #[test]
    fn rust_delta_classifies_diff_headers() {
        assert!(rust_delta(RUST_DIFF));
        assert!(!rust_delta(DOCS_DIFF));
        assert!(!rust_delta(""));
        assert!(rust_delta(RENAME_DIFF));
        assert!(!rust_delta("+++ b/src/lib.rs\n--- a/src/lib.rs\n"));
        assert_eq!(
            diff_paths(DOCS_DIFF),
            vec!["README.md", "README.md", "docs/x y.md", "docs/x y.md"]
        );
        assert_eq!(diff_paths(RENAME_DIFF), vec!["src/old.rs", "notes/old.md"]);
        assert_eq!(diff_files(DOCS_DIFF), 2);
        assert_eq!(diff_files(RUST_DIFF), 1);
        assert_eq!(diff_files(""), 0);
    }

    #[test]
    fn rust_paths_lists_each_rust_path_once() {
        let diff = "diff --git a/tests/a.rs b/tests/a.rs\n\
            diff --git a/src/old.rs b/notes/old.md\n\
            diff --git a/README.md b/README.md\n";
        assert_eq!(rust_paths(diff), vec!["tests/a.rs", "src/old.rs"]);
        assert!(rust_paths(DOCS_DIFF).is_empty());
    }

    #[test]
    fn test_target_holds_only_test_bench_and_example_dirs() {
        for path in [
            "tests/it.rs",
            "benches/b.rs",
            "examples/e.rs",
            "crates/viola-core/tests/x.rs",
            "crates/viola-e2e/benches/b.rs",
            "crates/viola-e2e/examples/e.rs",
        ] {
            assert!(test_target(path), "{path}");
        }
        for path in [
            "src/lib.rs",
            "src/bin/viola-fake-agent.rs",
            "crates/viola-core/src/lib.rs",
            "crates/tests/x.rs",
            "crates/viola-core",
            "src/tests/x.rs",
            "tests.rs",
            "fuzz/fuzz_targets/viola_name.rs",
        ] {
            assert!(!test_target(path), "{path}");
        }
    }
}
