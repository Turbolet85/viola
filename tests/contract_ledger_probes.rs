//! The capability-ledger probes over every committed fixture set (test-plan §5 CLI): `viola
//! verify` against the fake agent at each recorded `fixtures/claude/<version>/` passes every row,
//! each literal row id once. The recorded sets are walked at run time, not by `#[files]`, and an
//! empty walk fails.
//! andromeda:walks-tree — it reads every set under `fixtures/claude/`, named or not.

#[allow(dead_code)]
mod support;

use std::path::Path;

use support::home::{TestHome, workspace_path};
use support::verify::verify;

const ROW_IDS: [&str; 6] = [
    "shim-resolution",
    "spine-hooks",
    "session-start-fields",
    "prompt-verbatim",
    "stop-message",
    "largest-hook-payload",
];

/// The recorded CLI versions under `root`, sorted: one dir per version.
fn recorded_versions(root: &Path) -> Vec<String> {
    let mut found: Vec<String> = std::fs::read_dir(root)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().to_str().map(str::to_owned))
        .collect();
    found.sort();
    found
}

#[test]
fn contract_ledger_probes_pass_over_every_recorded_set() {
    let root = workspace_path("fixtures/claude");
    let versions = recorded_versions(&root);
    assert!(!versions.is_empty(), "no recorded fixture set to probe");
    for version in &versions {
        let home = TestHome::new();
        let ran = verify(home.path(), &root, version, &[], &[], &[]);
        let stdout = ran.stdout_text();
        assert_eq!(ran.code, Some(0), "{version}: verify exit\n{stdout}");
        let lines: Vec<&str> = stdout.lines().collect();
        let (last, steps) = lines.split_last().expect("verify printed nothing");
        assert_eq!(
            steps.len(),
            ROW_IDS.len(),
            "{version}: step lines\n{stdout}"
        );
        for id in ROW_IDS {
            let naming: Vec<&&str> = steps
                .iter()
                .filter(|l| l.split(' ').nth(1) == Some(id))
                .collect();
            assert_eq!(
                naming.len(),
                1,
                "{version}: {id} named {} times",
                naming.len()
            );
            assert!(
                naming[0].ends_with("  pass"),
                "{version}: {id} did not pass"
            );
        }
        assert_eq!(*last, format!("stamped {version}  6 pass  0 fail"));
    }
}
