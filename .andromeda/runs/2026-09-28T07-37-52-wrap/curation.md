# Curation — 2026-09-28-hook-perf-gate wrap

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "cargo-mutants skips only a module whose attribute is exactly `#[cfg(test)]` …" (confidence 0.8)
                                              + testing.md: "A test that pins a not-yet-built harness selector as `usage` runs the real arm …" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 1 task-specific · 0 conflict · 0 deferred
  No-other-home: "cargo-mutants skips only a bare `#[cfg(test)]` module" · "retarget a usage test's unbuilt selector when the selector lands"

## Applied
- testing.md: feature-gated test modules stack `#[cfg(test)]` and `#[cfg(feature)]` (score: measured +0.4, technical detail +0.2, no-other-home +0.2 = 0.8)
  Proof: implement gate 17 (`run --mutants --file src/cmd/hook/seam.rs`) and gate 28 (`pre-push`, both legs) graded `replace tests::panic_on_anything_but_one_returns with ()` MISSED inside `#[cfg(all(test, feature = "fake-agent"))]`; with the two stacked attributes the re-run graded 0 missed (implement run 2026-09-28T05-42-35-implement, second full block).
- testing.md: retarget a usage test's unbuilt selector when the selector lands (score: measured +0.4, technical detail +0.2, no-other-home +0.2 = 0.8)
  Proof: implement gate 11 (first run) — `crates/viola-e2e/tests/cli.rs` `unbuilt_selectors_and_unknown_commands_are_usage` read exit 0 for `run --perf` after it ran a real perf build, boot and hyperfine inside nextest; swapped to `run --e2e` (the scope record's companion line).

## Rejected
- dup: the Bash guard refusing a `cat >> file` heredoc document write (host-win32.md: documents go through the Write tool).
- dup: a founder ratification covers only what was shown to him (playbook "Boundary widening — what ratifies it").
- task-specific: the Bash auto-mode classifier's no-verdict outage at wrap Setup (an environment event, no rule).

CLAUDE.md size: see the P7 report (health.py check 1).
