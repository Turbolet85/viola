//! The doctests, which nextest cannot run: `cargo test --doc`.

use std::process::Command;

use super::nextest::exit_must_agree;
use super::{Runner, Suite, Workspace};

pub(super) fn doctest(ws: &Workspace, runner: &mut Runner<'_>) -> Suite {
    let (code, out) = runner(
        Command::new("cargo")
            .args([
                "test",
                "--workspace",
                "--doc",
                "--features",
                "viola/fake-agent",
            ])
            .env("CARGO_TARGET_DIR", ws.cargo_target())
            .current_dir(&ws.root),
    );
    let mut suite = parse_doctest(&out);
    suite.suite = "doctest".to_owned();
    exit_must_agree(&mut suite, code, "doctest");
    suite
}

fn count_before(line: &str, word: &str) -> u64 {
    line.split(';')
        .find_map(|part| part.trim().strip_suffix(word))
        .and_then(|n| n.trim().rsplit(' ').next())
        .and_then(|n| n.parse().ok())
        .unwrap_or(0)
}

/// Sums every `test result: …` line; `ignored` counts as skipped.
pub fn parse_doctest(out: &str) -> Suite {
    let mut suite = Suite::default();
    for line in out.lines().filter(|l| l.contains("test result:")) {
        suite.passed += count_before(line, "passed");
        suite.failed += count_before(line, "failed");
        suite.skipped += count_before(line, "ignored");
    }
    suite
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_doctest_sums_result_lines() {
        let out = "running 2 tests\n\
test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.1s\n\
test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out\n";
        let s = parse_doctest(out);
        assert_eq!((s.passed, s.failed, s.skipped), (5, 1, 1));
        assert_eq!(parse_doctest("nothing here"), Suite::default());
    }
}
