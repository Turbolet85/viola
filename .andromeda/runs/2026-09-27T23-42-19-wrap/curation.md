# Curation — 2026-09-27-hooks-to-normalised-events (resumed wrap, 2026-09-28)

Scope: this window's conversation + the report's *Decisions & corrections*. The halted window's conversation (implement,
P4, the first fan-out) is gone, so a correction only it held is not curated here.

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-win32.md: "The Bash guard refuses a heredoc whose payload carries a doubled backslash, so a source edit holding escape sequences goes through the Edit tool, never a shell heredoc."
    Proof: report.md *Decisions & corrections*, last bullet (the guard blocked the Rust-escape payload at implement; the edit landed through the Edit tool). Score: measured +0.4, specific detail +0.2, no-other-home +0.2 (no master, route annotation or ledger note carries it) = 0.8. host-win32.md loads on every turn, so the entry is held to Tier 1's bar: one sentence, under 600 B.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters:
    - dup 2: the copied-target `CARGO_BIN_EXE_*` sweep hazard (this wrap's cascade wrote it into the generated body of `verification-harness.md` and the masters: dedup-reject-only); `cargo test` vs nextest for process-global capture (testing.md 2026-09-24's `OnceLock` isolation entry, same rule).
    - task-specific 1: `TestHome` removed on `Wrapper::stop()`, use `stop_keep()` (test-support function names, not public API).
    - below threshold 2: an ad-hoc `cargo test` in the default `target/` planted the stale binary (0.4: obsolete now that the mutation run has its own target dir); Windows `ExitCode` is `PartialEq` but not `Eq` (0.3: one-off, no emphasis).
    - conflict 0 · deferred 0.
  No-other-home: "The Bash guard refuses a heredoc whose payload carries a doubled backslash …"
  Not candidates (telemetry, not learnings): the re-fan and the obs §1 in-pass correction (evolve records).
