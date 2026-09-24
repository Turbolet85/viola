//! Fuzz corpus replay (test-plan §3 `--fuzz-replay`).

use std::fs;
use std::path::Path;
use std::process::Command;

use super::{Refusal, Runner, Suite, Workspace};

/// Cargo-fuzz's libFuzzer runs on the Linux runner only (test-plan §3 `--fuzz-replay`). A const,
/// not a fn: a fn body equal to one OS's answer is an unkillable mutant on that OS's leg.
pub const FUZZ_HOST_SUPPORTED: bool = cfg!(target_os = "linux");

/// The `channel` of `fuzz/rust-toolchain.toml`: the one source of the fuzz toolchain.
pub fn fuzz_channel(toml: &str) -> Option<String> {
    toml.lines()
        .filter_map(|l| l.trim().strip_prefix("channel"))
        .filter_map(|rest| rest.trim_start().strip_prefix('='))
        .map(|v| v.trim().trim_matches('"').to_owned())
        .find(|v| !v.is_empty())
}

/// Every `fuzz/fuzz_targets/<target>.rs`, by name, sorted.
fn fuzz_targets(fuzz: &Path) -> Vec<String> {
    let mut targets: Vec<String> = fs::read_dir(fuzz.join("fuzz_targets"))
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .filter_map(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .collect();
    targets.sort();
    targets
}

fn has_entries(dir: &Path) -> bool {
    fs::read_dir(dir).is_ok_and(|mut entries| entries.next().is_some())
}

/// Replays each target's committed corpus once (`-runs=0`); an absent or empty corpus fails its
/// target, never passes it.
pub(super) fn fuzz_replay(ws: &Workspace, runner: &mut Runner<'_>) -> Result<Suite, Refusal> {
    let (probe, _) = runner(
        Command::new("cargo")
            .args(["fuzz", "--version"])
            .current_dir(&ws.root),
    );
    if probe != Some(0) {
        return Err(Refusal::new("tool-missing", Some("cargo-fuzz")));
    }
    let fuzz = ws.root.join("fuzz");
    let channel = fs::read_to_string(fuzz.join("rust-toolchain.toml"))
        .ok()
        .and_then(|t| fuzz_channel(&t))
        .ok_or_else(|| Refusal::new("tool-missing", Some("fuzz/rust-toolchain.toml")))?;
    let targets = fuzz_targets(&fuzz);
    if targets.is_empty() {
        return Err(Refusal::new("corpus-empty", None));
    }
    let mut suite = Suite {
        artifact: Some("fuzz/corpus".to_owned()),
        ..Suite::named("fuzz-replay")
    };
    for target in targets {
        let corpus = fuzz.join("corpus").join(&target);
        if !has_entries(&corpus) {
            suite.failed += 1;
            suite.failures.push(format!("{target}: corpus-empty"));
            continue;
        }
        let (code, _) = runner(
            Command::new("cargo")
                .arg(format!("+{channel}"))
                .args(["fuzz", "run", "--fuzz-dir", "fuzz", &target])
                .arg(format!("fuzz/corpus/{target}"))
                .args(["--", "-runs=0"])
                .current_dir(&ws.root),
        );
        if code == Some(0) {
            suite.passed += 1;
        } else {
            suite.failed += 1;
            suite.failures.push(target);
        }
    }
    Ok(suite)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    use super::super::test_support::{Calls, args_of, has, scratch};
    use super::super::{Selection, run_with};
    use super::*;

    #[test]
    fn fuzz_channel_reads_the_toolchain_file() {
        let toml = "[toolchain]\nchannel = \"nightly-2026-09-20\"\ncomponents = [\"rust-src\"]\n";
        assert_eq!(fuzz_channel(toml).as_deref(), Some("nightly-2026-09-20"));
        assert_eq!(fuzz_channel("channel=\"x\"").as_deref(), Some("x"));
        assert_eq!(fuzz_channel("channels = \"x\"\n"), None);
        assert_eq!(fuzz_channel("channel = \"\"\n"), None);
        assert_eq!(fuzz_channel("[toolchain]\n"), None);
    }

    fn plant_fuzz(ws: &Workspace, targets: &[&str], corpora: &[(&str, bool)]) {
        let fuzz = ws.root.join("fuzz");
        fs::create_dir_all(fuzz.join("fuzz_targets")).expect("mkdir");
        fs::write(
            fuzz.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"nightly-2026-09-20\"\n",
        )
        .expect("toolchain");
        for t in targets {
            fs::write(fuzz.join("fuzz_targets").join(format!("{t}.rs")), "").expect("target");
        }
        fs::write(fuzz.join("fuzz_targets").join("README.md"), "").expect("not a target");
        for (t, seeded) in corpora {
            let dir = fuzz.join("corpus").join(t);
            fs::create_dir_all(&dir).expect("mkdir");
            if *seeded {
                fs::write(dir.join("seed"), "builder").expect("seed");
            }
        }
    }

    /// `version` is `cargo fuzz --version`'s exit; `red` names the targets whose replay fails.
    fn replay(ws: &Workspace, version: i32, red: &[&str]) -> (Result<Suite, Refusal>, Calls) {
        let mut calls = Vec::new();
        let result = {
            let mut runner = |cmd: &mut Command| {
                let args = args_of(cmd);
                let code = if has(&args, &["fuzz", "--version"]) {
                    version
                } else {
                    i32::from(red.iter().any(|t| args.contains(&(*t).to_owned())))
                };
                calls.push(args);
                (Some(code), String::new())
            };
            fuzz_replay(ws, &mut runner)
        };
        (result, calls)
    }

    #[test]
    fn fuzz_replay_without_cargo_fuzz_is_tool_missing() {
        let (_tmp, ws) = scratch();
        plant_fuzz(&ws, &["a"], &[("a", true)]);
        let (result, calls) = replay(&ws, 101, &[]);
        assert_eq!(
            result.expect_err("refused"),
            Refusal::new("tool-missing", Some("cargo-fuzz"))
        );
        assert_eq!(calls.len(), 1);
    }

    #[test]
    fn fuzz_replay_without_its_toolchain_file_is_tool_missing() {
        let (_tmp, ws) = scratch();
        let (result, _) = replay(&ws, 0, &[]);
        assert_eq!(
            result.expect_err("refused"),
            Refusal::new("tool-missing", Some("fuzz/rust-toolchain.toml"))
        );
    }

    #[test]
    fn fuzz_replay_without_targets_is_corpus_empty() {
        let (_tmp, ws) = scratch();
        plant_fuzz(&ws, &[], &[]);
        let (result, _) = replay(&ws, 0, &[]);
        assert_eq!(
            result.expect_err("refused"),
            Refusal::new("corpus-empty", None)
        );
    }

    #[test]
    fn fuzz_replay_counts_each_target_and_never_passes_an_empty_corpus() {
        let (_tmp, ws) = scratch();
        plant_fuzz(
            &ws,
            &["d", "c", "b", "a"],
            &[("a", true), ("b", false), ("d", true)],
        );
        let (result, calls) = replay(&ws, 0, &["d"]);
        let suite = result.expect("replayed");
        assert_eq!(suite.suite, "fuzz-replay");
        assert_eq!(suite.artifact.as_deref(), Some("fuzz/corpus"));
        assert_eq!((suite.passed, suite.failed), (1, 3));
        assert_eq!(suite.failures, ["b: corpus-empty", "c: corpus-empty", "d"]);
        let expected: Vec<String> = [
            "+nightly-2026-09-20",
            "fuzz",
            "run",
            "--fuzz-dir",
            "fuzz",
            "a",
            "fuzz/corpus/a",
            "--",
            "-runs=0",
        ]
        .map(str::to_owned)
        .to_vec();
        assert_eq!(calls[1], expected);
        assert_eq!(calls.len(), 3, "version probe, then a and d only");
    }

    #[test]
    fn fuzz_host_is_linux_only() {
        assert_eq!(FUZZ_HOST_SUPPORTED, std::env::consts::OS == "linux");
    }

    #[test]
    fn run_fuzz_replay_refusals_reach_the_document() {
        let (_tmp, ws) = scratch();
        let mut ran = 0;
        let sel = Selection {
            fuzz_replay: true,
            mutants: true,
            ..Selection::default()
        };
        let out = run_with(&ws, sel, None, None, None, &mut |_: &mut Command| {
            ran += 1;
            (Some(101), String::new())
        });
        if cfg!(target_os = "linux") {
            assert_eq!(out.code, 1);
            assert_eq!(out.doc["reason"], "tool-missing");
            assert_eq!(out.doc["detail"], "cargo-fuzz");
            assert!(
                out.doc.get("mutants").is_none(),
                "mutants skipped after a refusal"
            );
            assert_eq!(ran, 1);
        } else {
            assert_eq!(out.code, 2);
            assert_eq!(out.doc["reason"], "fuzz-linux-only");
            assert_eq!(ran, 0);
        }
    }
}
