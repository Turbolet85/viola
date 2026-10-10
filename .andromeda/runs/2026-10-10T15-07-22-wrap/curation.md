# Curation — wrap of 2026-10-10-viola-revive

Sources: this session's conversation; the report's Decisions & corrections; and `resume-point.md`'s list "For
Phase 3", which the session that ran Phase 1 wrote down because its conversation would be gone. A correction
that conversation held and that list does not name was not curated.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "a test of a verb that looks its program up by name puts the fake agent as claude first on PATH" (confidence 0.8)
                                              + testing.md: "read the tests/support helpers a root test calls before saying it does not assert something" (confidence 0.8)
                                              + verification-harness.md: "a live rig that needs the stamped CLI puts that version's install directory first on PATH" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 3 task-specific or below the threshold · 0 conflict · 2 deferred (→ handoff) · 3 recurrences (→ handoff)
  Load-bearing: "a live rig that needs the stamped CLI puts that version's install directory first on PATH" → Statusline pass-through
  No-other-home: "a test of a verb that looks its program up by name puts the fake agent as claude first on PATH"
  No-other-home: "read the tests/support helpers a root test calls before saying it does not assert something"
```

## Applied

1. testing.md — the fake agent as `claude` first on `PATH`.
   Signals: verified by measurement 0.4 · specific technical detail 0.2 · reached no other durable home 0.2.
   Proof: `evidence/red-green.md`, control B: with the strict-modes reading neutralised, a revive over an instance
   directory with a group-write bit passed its preflight and started its child. The child was the fake agent only
   because the case put it first on `PATH`.
2. testing.md — read the support helpers before an absence claim about a test.
   Signals: explicit user correction 0.4 · specific technical detail 0.2 · reached no other durable home 0.2.
   Proof: `inputs/I8-relay-3.md.txt`, answer 5: the operator asked for the reading that shows the child gone after
   the kill. It is `Wrapper::kill` in `tests/support/home.rs` (the report's listing, row 561-586); this wrap's card
   had read `tests/chaos_revive.rs` alone and said the case asserts no such thing.
3. verification-harness.md — the stamped CLI first on a live rig's `PATH`.
   Signals: verified by measurement 0.4 · specific technical detail 0.2 · load-bearing for the next entry 0.2.
   Proof: `evidence/live-revive.md`: the bare name `claude` on the dev host read 2.1.289 while the home is stamped
   for 2.1.287, and the rig put the 2.1.287 install directory first on its host's `PATH`. The next promotable
   entry, "Statusline pass-through", lands ledger rows with `viola verify` probes on that host and carries no
   annotation with this fact.

## Deferred (the cap of three; → handoff)

- The key files stand one directory deeper than the registry's top level, under
  `.andromeda/registries/contracts/<master>/`, so an owner map keyed on the top-level names reads every key file
  as unowned (confidence 0.8; a sweep hazard, Tier 3).
- A wrapper a test ends through its drop guard writes no coverage profile, so a case whose start path must count
  toward coverage stops its wrapper cleanly (confidence 0.6 before a conditional signal; test-plan §6 already
  states that a killed instrumented process can leave a short profile).

## Rejected

- A killed wrapper leaves its socket file on Unix, so the kill path waits on a connect probe: this wrap amended
  it into architecture and the test-plan key file, and the masters own it (0.6, no conditional signal).
- Prose appended to `research.md` after the lists check has to keep the list grammar: a fact about the pipeline's
  own artifact, not about this project's code (task-specific).
- The write guard refuses the Write tool under `target/`; a count of test names carrying a filter's tokens also
  counts an integration case (71 against 70): each a one-off below the threshold.

## Recurrences (→ handoff, Deferred learnings)

- `recurrence-despite-learning: host-linux.md` Paths: a `cd` outside a subshell, once in the implementing session
  and three times in this session's orientation.
- `recurrence-despite-learning: host-linux.md 2026-09-28/29`: a heredoc with a file target, once.
- `recurrence-despite-learning: host-linux.md` Transports: an inline python heredoc for a multi-edit of a source
  file, once.
