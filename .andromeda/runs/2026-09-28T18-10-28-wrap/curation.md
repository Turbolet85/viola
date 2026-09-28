# Curation — 2026-09-28-capability-ledger-and-viola-verify

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "Never reshape code to take its body out of cargo-mutants' generated set … return a `Default`-deriving type instead." (confidence 0.8)
                                              + testing.md: "A timing red is never fixed by raising a timeout or a test bound … then remove the slow work." (confidence 1.0)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred; below threshold 2 (nextest `success-output` override to see a passing test's output: measured +0.4, detail +0.2 = 0.6, rejects; the nested cargo's package-cache lock under `cargo llvm-cov nextest`: 0.6, rejects — its fix and the remaining 73 s ride the head-of-queue entry's CARRY); homed elsewhere this wrap 2 (rstest 0.27 `#[files]` on an empty glob → test-plan §7; symbolising cost → obs-plan §7 + rules/observability.md)
  CLAUDE.md size: see P7

## Proof
- mutants shape: the overseer's correction in the operator pass ("turning drain from a fn into a closure takes its body out of the set cargo-mutants generates … it lowers coverage to pass the ratio. Prefer keeping drain a fn and killing its mutants with a test"), measured by `cargo mutants --list` before/after (21 → 11 → 15 mutants) and the landed `Drained` return type (report Decisions & corrections; Deviations `drain`). Signals: correction +0.4, measurement +0.4.
- timing reds: the overseer's direction ("never raise a timeout or the 1.0 s bound; measure on the runner … remove the slow work; a breach is surfaced, never absorbed"), proven twice this chunk: raw panic frames took the forced-panic hook from 1.50 s to within the 1.0 s bound (ci#36435153705 → ci#36448654074), and a private `CARGO_HOME` took the macOS harness mutants tests from > 120 s (killed) to 82 s (ci#36429783467 → ci#36448654074). Signals: correction +0.4, "never" +0.3, measurement +0.4 (capped 1.0).
