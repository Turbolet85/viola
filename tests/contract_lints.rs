//! Lint coverage and panic-hook placement (obs-plan §3 `obs-ci-gate-wire`, §7 Panic hooks).

use std::fs;
use std::path::{Path, PathBuf};

fn workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The body of a `[name]` table: the lines after its header, up to the next header.
fn table(manifest: &str, name: &str) -> Option<Vec<String>> {
    let header = format!("[{name}]");
    let mut lines = manifest.lines().skip_while(|l| l.trim() != header);
    lines.next()?;
    Some(
        lines
            .take_while(|l| !l.trim_start().starts_with('['))
            .map(|l| l.trim().to_owned())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect(),
    )
}

fn inherits_workspace_lints(manifest: &str) -> bool {
    table(manifest, "lints").is_some_and(|body| body == ["workspace = true"])
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

#[test]
fn workspace_members_inherit_the_lint_table_except_the_harness_crate() {
    let root = workspace();
    assert!(inherits_workspace_lints(&read(&root.join("Cargo.toml"))));

    let mut checked = Vec::new();
    for entry in fs::read_dir(root.join("crates")).expect("crates/") {
        let dir = entry.expect("entry").path();
        let manifest = dir.join("Cargo.toml");
        if !manifest.is_file() {
            continue;
        }
        let name = dir
            .file_name()
            .expect("name")
            .to_string_lossy()
            .into_owned();
        let text = read(&manifest);
        if name == "viola-e2e" {
            assert!(
                table(&text, "lints").is_none(),
                "viola-e2e must not inherit the print bans"
            );
            assert!(
                table(&text, "lints.clippy").is_some(),
                "viola-e2e carries its own clippy table"
            );
        } else {
            assert!(
                inherits_workspace_lints(&text),
                "{name} lacks [lints] workspace = true"
            );
        }
        checked.push(name);
    }
    assert!(
        checked.iter().any(|n| n == "viola-core"),
        "checked {checked:?}"
    );
    assert!(
        checked.iter().any(|n| n == "viola-e2e"),
        "checked {checked:?}"
    );
}

#[test]
fn panic_hook_is_the_first_statement_of_main() {
    let main = read(&workspace().join("src").join("main.rs"));
    let first = main
        .lines()
        .skip_while(|l| l.trim() != "fn main() -> ExitCode {")
        .skip(1)
        .find(|l| !l.trim().is_empty())
        .expect("fn main body");
    assert!(
        first.trim().starts_with("std::panic::set_hook("),
        "first statement of main: {first}"
    );
}

#[test]
fn table_reads_the_body_up_to_the_next_header() {
    let text = "[a]\nx = 1\n\n# note\n[lints]\nworkspace = true\n[b]\ny = 2\n";
    assert_eq!(
        table(text, "lints"),
        Some(vec!["workspace = true".to_owned()])
    );
    assert_eq!(table(text, "missing"), None);
    assert!(inherits_workspace_lints(text));
    assert!(!inherits_workspace_lints("[lints]\nworkspace = false\n"));
    assert!(!inherits_workspace_lints(
        "[lints.rust]\nunused_must_use = \"deny\"\n"
    ));
}
