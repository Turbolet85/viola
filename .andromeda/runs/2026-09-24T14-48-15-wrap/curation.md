# Curation — 2026-09-24-quality-gates

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "cargo-mutants swaps an operator TOKEN in the text … cover each such condition with a case where the leading operand is false" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): + "A gate atom must not match what a green run can print" (confidence 0.8)
  Extended: T2/testing.md: "Keep a `#[cfg(unix)]`-only function to a minimal OS reader…" + the mirror facet (other-OS bodies are unkillable on Linux; the CI two-leg union; a stub as a `#[cfg]` block in one body) (confidence 1.0)
  Filters: 0 dup · 2 task-specific (the llvm-cov JUnit path and `RUSTUP_TOOLCHAIN` precedence were amended into test-plan/architecture this wrap, so their home is the master) · 0 conflict · 1 deferred (→ handoff)
  No-other-home: "cargo-mutants token swap re-parses `a || b || c`" · "gate atom vs test names"
  CLAUDE.md size: 121/200 · T1 1.3 KB, 0 over 600 B

## Proofs
- **Extended testing.md (the cfg mirror):** research M1, `cargo mutants --list --file crates/viola-e2e/src/harness/secret_scan.rs` → `224:5` (the unix reader, 3 mutants) vs `230:5` (the `#[cfg(not(unix))]` stub, 2 mutants). CI run 36005608858 MISSED exactly the `230:5` pair. The operator's correction at phase P5: "230:5 is the non-unix stub; my read was wrong." Fixed by one body; the gate `grep -c 'replace file_mode'` reads 3.
- **Token swap:** the implement mutants log `mutants.out/log/crates__viola-e2e__src__harness__run.rs_line_139_col_36.log` shows `run_reports_unit_integration_and_doctest_suites` PASS under the `||`→`&&` mutant (MISSED). Killed by the coverage-only selection test (`coverage_alone_still_runs_the_doctests`). The second full gate run read 2 missed, both host-equivalent.
- **Gate atom:** plan entry 13 read red at P5 on `contains 0 failed` (the green summary "158 tests run: 158 passed, 0 skipped"), then red on `lacks failed` (test names `boot_with_a_failing_build_reports_build_failed_with_its_exit`, `run_mutants_with_an_unbuildable_root_package_is_build_failed`), then green on `lacks failed,`.

## Deferred (max-3 cap)
- `git check-ignore -q` on a path that does not exist yet reads "not ignored" (exit 1) for a trailing-slash directory pattern (`target/`), because git cannot know the path is a directory. So a pre-build gitignore probe on an unborn dir is a false negative. (confidence 0.8)
