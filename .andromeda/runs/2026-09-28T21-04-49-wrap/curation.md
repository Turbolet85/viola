# Curation — 2026-09-28-mutation-testing-to-the-epoch-boundary

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   corrections ×3 (cap-exempt) + 1 extension
  Tier 3 (.claude/docs/session-learnings.md): + "A plan whose test and probe contradict: narrow the probe" (confidence 0.7)
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 2 below threshold (exactly 0.6)
  Extended: T2/testing.md: "2026-09-28: A timing red is never fixed by raising a timeout…" + "move the cold work into the step it replaces; a temporary nextest `success-output` override shows a passing test's lines"
  CLAUDE.md size: 124/200 · T1 1.8 KB, 0 over 600 B

## Corrections (in place, `[corrected 2026-09-28: …]`)
- `verification-harness.md` 2026-09-24 (`--in-diff` witness): "never by a green `mutants` job" → "never by a green `run --mutants`".
  Proof: ci.yml has no `mutants` job (report Changes, Symbols/CI; ci#36483042659 15/15 check-runs).
- `verification-harness.md` 2026-09-25 (`#[cfg(unix)]` bodies on the Windows host): the `--leg windows-2025` + CI-union recipe
  replaced by "read such misses as host-excluded — the code audit classifies them" (also settles scope CARRY 4, the
  host-scoped `--file` expectation rule, rewritten as code-audit guidance).
  Proof: `run --leg` exits 2 usage `arguments` (`crates/viola-e2e/tests/cli.rs` `gate_usage_errors_and_the_retired_leg_flags_are_exit_2`, run --integration green).
- `verification-harness.md` 2026-09-26: "judge a mutation leg" → "judge a mutation run". Proof: as above.
- `testing.md` 2026-09-24 (`#[cfg(unix)]` minimal reader): "killable only on the Linux CI mutants job … CI gates the union" →
  "killable only by a Linux mutation run". Proof: as above.

## Extension
- `testing.md` 2026-09-28 timing-red rule + the measurement technique.
  Proof: the one measurement push (a7c1560, ci#36481260151) read `Unmutated baseline in 0s build` locally and 80.9 s on macOS
  with the cold build moved into `target/mutants` first — the test stayed at 87 s, under the 120 s kill (was 101.5 s);
  the phase lines were visible only through `[[profile.ci.overrides]] success-output = "immediate"`.
  Signals: verified by measurement +0.4 · specific technical detail +0.2 · no other durable home +0.2 = 0.8.

## New (Tier 3)
- "A plan whose test and probe contradict: narrow the probe". Signals: explicit operator decision +0.4 · "never" language
  +0.3 = 0.7. Tier: a directive with no path-scoped rule file covering plan authoring (tiebreaker 3) → Tier 3.
  Proof: the operator + overseer ruling at implement P2 (AskUserQuestion); plan entry 13 narrowed at this wrap, its control
  firing on a copied `cli.rs` and the subject reading exit 1 / no output.

## Rejected
- macOS AF_UNIX ENOTCONN on a close-racing write — 0.6 (measured +0.4, detail +0.2); amended into test-plan §5 this wrap,
  so the no-other-home signal does not apply.
- The throwaway crate inherits cargo-llvm-cov's `RUSTC_WRAPPER` / `LLVM_PROFILE_FILE` — 0.6; carried on the route at P5
  (the route owns it).
