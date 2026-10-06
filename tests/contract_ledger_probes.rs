//! The capability-ledger probes over the committed fixture sets (test-plan §5 CLI): `viola verify`
//! against the fake agent at each stamped `fixtures/claude/<version>/` passes every row, each literal
//! row id once. Every recorded set is on exactly one literal list: stamped, or kept for the byte-drift
//! contract only. The recorded sets are walked at run time, not by `#[files]`, and an empty walk
//! fails.
//! andromeda:walks-tree — it reads every set under `fixtures/claude/`, named or not.

#[allow(dead_code)]
mod support;

use std::path::Path;

use support::home::{TestHome, workspace_path};
use support::verify::verify;

const ROW_IDS: [&str; 17] = [
    "shim-resolution",
    "spine-hooks",
    "session-start-fields",
    "prompt-verbatim",
    "stop-message",
    "largest-hook-payload",
    "modal-signature",
    "input-box-signature",
    "quiet-period",
    "confirm-window",
    "question-answer",
    "plan-approve-revise",
    "question-notes",
    "dialog-concurrency",
    "long-paste-wrapper",
    "tag-escaping",
    "local-command-clear",
];
/// The sets recorded whole (spine, screens, dialog and framing variants): each stamps every row.
const STAMPED: [&str; 1] = ["2.1.287"];
/// The sets kept only for the byte-drift contract, never stamped: 2.1.283 carries no screen, and
/// 2.1.288 none of the framing variants.
const DRIFT_ONLY: [&str; 2] = ["2.1.283", "2.1.288"];

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
fn contract_ledger_probes_every_recorded_set_is_on_exactly_one_list() {
    let versions = recorded_versions(&workspace_path("fixtures/claude"));
    assert!(!versions.is_empty(), "no recorded fixture set");
    for version in &versions {
        let lists = usize::from(STAMPED.contains(&version.as_str()))
            + usize::from(DRIFT_ONLY.contains(&version.as_str()));
        assert_eq!(lists, 1, "{version} is on {lists} lists");
    }
    for listed in STAMPED.iter().chain(&DRIFT_ONLY) {
        assert!(
            versions.iter().any(|v| v == listed),
            "{listed} is not recorded"
        );
    }
}

#[test]
fn contract_ledger_probes_pass_over_every_stamped_set() {
    let root = workspace_path("fixtures/claude");
    for version in STAMPED {
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
        assert_eq!(*last, format!("stamped {version}  17 pass  0 fail"));
    }
}
