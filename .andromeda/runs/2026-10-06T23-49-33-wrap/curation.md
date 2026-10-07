# Curation — 2026-10-06-local-command-send-outcomes wrap

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "When a matrix acceptance sentence holds only in substance under its named witness … close it at implement by extending the witness … by the test, not by the wording" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none new
  Filters: 2 dup · 2 task-specific · 0 conflict · 0 deferred
  No-other-home: "by the test, not by the wording" · "a test with no window prefix runs under the mutants profile's 10 s kill" · "hygiene takes a quoted Windows drive path for a host path"
  Extended: T2/testing.md: "A test wait that a mutant can reach …" + "a root test with no window prefix runs under the mutants profile's 10 s kill; size a new stamped-home case against it"
  Extended: T3/session-learnings.md: "gate.py hygiene reads `/home/<x>/` in prose as a POSIX user home" + "the same read takes a quoted Windows drive path, a test's own literal included"
```

## Applied
1. T2 `testing.md`, new — the acceptance sentence closed by the test.
   Signals: explicit user correction +0.4 · specific technical detail with context +0.2 = 0.6 · reached no other
   durable home +0.2 = 0.8.
   Proof: the operator's word after the implement report, "close the gap you named, by the test and not by the
   wording"; the fix commit `690aefa` extended `send_window_local_command_is_not_presumed_delivered` with the
   `/remote-control` `--json` read (`report.md`, Deviations 7; `evidence/operator-pass.md`, "The fix commit").
   The generalisation from that one directive to a standing rule is this wrap's reading of it.
2. T2 `testing.md`, extension of the 2026-09-24 entry on mutant-reachable waits — the `mutants` profile's 10 s kill.
   Signals: verified by measurement (it changed the chunk's design) +0.4 · specific technical detail +0.2 = 0.6 ·
   reached no other durable home +0.2 = 0.8.
   Proof: `send_under_the_paste_hint_on_a_verified_cli::case_1_hint` read 10.549 s at the planned 6 000 ms hold and
   7.437 s at 3 000 ms (`evidence/paste-hint-send.md`, "The hold, and each case's length");
   `.config/nextest.toml` `[profile.mutants]` `slow-timeout = { period = "5s", terminate-after = 2 }`.
3. T3 `session-learnings.md`, extension of the 2026-09-29 entry on the hygiene read — the drive-path form.
   Signals: proven by a real gate refusal +0.4 · specific technical detail +0.2 = 0.6 · reached no other durable
   home +0.2 = 0.8.
   Proof: `gate.py hygiene` at 2026-10-06T23:28:53Z, `refused 1 files — P1 1`, row
   `evidence/rewritten-path-warning-control.md:9 ×2 · drive`; clean after the rewording
   (`evidence/operator-pass.md`, "Entry 22").

## Filtered
- duplicate: "anchor every path, a `cd` in a compound moves the working directory" (`host-win32.md`, Paths &
  argument conversion, already says to rely on neither).
- duplicate, a recurrence: "a `cat` heredoc redirected to a file is refused by the Bash guard" (`host-win32.md`
  2026-09-28, its 2026-09-29 extension). Logged to the handoff as `recurrence-despite-learning`.
- recurrence: "a cut-limited view of a multi-KB line answered a membership question" (`host-win32.md` §Long
  single-line files): the report said a test-plan line carried no count, from a clipped grep row. Logged to the
  handoff as `recurrence-despite-learning`.
- task-specific or below the threshold: a backticked word inside a double-quoted pattern is command-substituted
  (one event, 0.2 − 0.3); red-first cases that hang on a fixed clock end at the `ci` profile's 120 s kill
  (one event, 0.2).
