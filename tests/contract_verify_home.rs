//! Every test-started `viola verify` runs under a test-owned `HOME` (test-plan §7): its trust-record
//! run writes the trust record into the config file its child reads, so a verify a test starts
//! outside the one helper file `tests/support/verify.rs` (which sets `HOME` and hands the fake agent
//! `--trust-config`) would write into the user's real `~/.claude.json`. A census over the test
//! sources reads red on a `"verify"` argument literal anywhere else that is not a named exemption,
//! each with why it starts no run. The harness boot (`crates/viola-e2e/src/harness/boot.rs`) is the
//! other stamping start, outside these roots, and sets its session's own `HOME` itself.
//!
//! andromeda:walks-tree — it reads every `.rs` file under the test roots, named or not.

use std::fs;
use std::path::{Path, PathBuf};

use source_scan::{Mask, masked};

#[allow(dead_code)]
#[path = "support/source_scan.rs"]
mod source_scan;

/// The one file that starts `viola verify` for a test, under the test-owned user home.
const HELPER: &str = "tests/support/verify.rs";
/// This census, whose own planted texts name the literal.
const SELF: &str = "tests/contract_verify_home.rs";

/// A `"verify"` literal that starts no verify run: its file, the text of its line, and why.
const EXEMPT: &[(&str, &str, &str)] = &[
    (
        "tests/cli_output_plain.rs",
        "        \"verify\",",
        "a verb of the help table: `viola verify --help` starts no run",
    ),
    (
        "tests/cli_fake_agent.rs",
        "assert_eq!(stamps[\"writer\"], \"verify\");",
        "the stamps' writer field, read",
    ),
    (
        "tests/cli_verify/stamps.rs",
        "assert_eq!(doc[\"writer\"], \"verify\");",
        "the stamps' writer field, read",
    ),
    (
        "tests/cli_verify/runs.rs",
        "viola(&[\"verify\".into(), \"--help\".into()], &[]);",
        "`viola verify --help` starts no run",
    ),
    (
        "tests/cross_exit_causes.rs",
        "viola_on(&bin, home.path(), &[\"verify\"], None, Some(\"verifier\"));",
        "an empty bin dir on the PATH: refused program-not-found before any run",
    ),
    (
        "crates/viola-e2e/tests/harness_tied.rs",
        "args.iter().any(|a| a == \"verify\")",
        "a process-table match, no spawn",
    ),
    (
        "crates/viola-e2e/tests/harness_tied.rs",
        "(\"verify\".to_owned(), mid.verify)",
        "a census label, no spawn",
    ),
];

fn workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// `tests/` and every `crates/<member>/tests/`, each file repo-relative with `/` separators.
fn test_sources(root: &Path) -> Vec<(String, String)> {
    let mut files = Vec::new();
    rs_files(&root.join("tests"), &mut files);
    for member in fs::read_dir(root.join("crates"))
        .into_iter()
        .flatten()
        .flatten()
    {
        rs_files(&member.path().join("tests"), &mut files);
    }
    files
        .into_iter()
        .map(|path| {
            let rel = path
                .strip_prefix(root)
                .expect("under the workspace")
                .to_string_lossy()
                .replace('\\', "/");
            let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}"));
            (rel, text)
        })
        .collect()
}

/// The 1-based lines of `text` whose code (comments masked, literals kept) holds `"verify"`.
fn verify_literals(text: &str) -> Vec<usize> {
    let code = masked(text, Mask::Comments);
    let needle = b"\"verify\"";
    let mut lines: Vec<usize> = code
        .windows(needle.len())
        .enumerate()
        .filter(|(_, w)| *w == needle)
        .map(|(at, _)| code[..at].iter().filter(|&&b| b == b'\n').count() + 1)
        .collect();
    lines.dedup();
    lines
}

/// The `file:line` of each `"verify"` literal no exemption names, and the exemptions used.
fn untied_starts(sources: &[(String, String)]) -> (Vec<String>, Vec<usize>) {
    let mut faults = Vec::new();
    let mut used = Vec::new();
    for (file, text) in sources {
        if file == HELPER || file == SELF {
            continue;
        }
        for line in verify_literals(text) {
            let content = text.lines().nth(line - 1).unwrap_or("");
            match EXEMPT
                .iter()
                .position(|(f, needle, _)| f == file && content.contains(needle))
            {
                Some(n) => used.push(n),
                None => faults.push(format!("{file}:{line}")),
            }
        }
    }
    (faults, used)
}

#[test]
fn every_test_verify_runs_under_a_test_home() {
    let root = workspace();
    let sources = test_sources(&root);
    assert!(
        sources.iter().any(|(f, _)| f == HELPER),
        "the helper file is read"
    );
    let (faults, mut used) = untied_starts(&sources);
    assert!(
        faults.is_empty(),
        "`viola verify` started outside {HELPER} with no exemption: {faults:#?}"
    );
    used.sort_unstable();
    used.dedup();
    let stale: Vec<&str> = (0..EXEMPT.len())
        .filter(|n| !used.contains(n))
        .map(|n| EXEMPT[n].1)
        .collect();
    assert!(stale.is_empty(), "exemptions naming no literal: {stale:#?}");
    for (_, _, why) in EXEMPT {
        assert!(!why.is_empty());
    }
}

/// The census reads red on a planted untied start, in any spelling of the argument, and green on a
/// comment naming it or a start inside the helper file.
#[test]
fn census_finds_a_planted_untied_verify_start() {
    let planted = [
        (
            "tests/planted_a.rs",
            "fn f() {\n    let _ = Command::new(VIOLA).args([\"--home\", h, \"verify\"]).output();\n}\n",
        ),
        (
            "tests/planted_b.rs",
            "fn f() {\n    // a \"verify\" in a comment\n    cmd.arg(\"verify\");\n}\n",
        ),
        (
            HELPER,
            "fn f() {\n    let a = vec![\"verify\".into()];\n}\n",
        ),
        ("tests/planted_c.rs", "/* \"verify\" */\nfn f() {}\n"),
    ];
    let sources: Vec<(String, String)> = planted
        .iter()
        .map(|(f, t)| ((*f).to_owned(), (*t).to_owned()))
        .collect();
    let (faults, used) = untied_starts(&sources);
    assert_eq!(faults, ["tests/planted_a.rs:2", "tests/planted_b.rs:3"]);
    assert!(used.is_empty());
}
