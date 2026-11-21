# Curation — wrap 2026-11-21T08-50-56Z-wrap (2026-11-20-harness-prefix-and-identity-floor-ledger-rows)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A whole re-record of a fixture set moves its recorded session ids, which test sources pin as literals … search the tests for the old set's ids right after the copy, before the after-landing suites run."
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 2 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  No-other-home: "a re-record moves pinned session ids" · "probe the figure from before a stopped run's move"
  Extended: T2/testing.md: "2026-11-19: When a counted set grows, a retired-shape probe for its old figure …" + "where a cut reverses part of a stopped run's move, probe the figure from before that move as well as the abandoned one"
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

## Applied
- T2 testing.md (new, 2026-11-21): a fixture re-record moves pinned session ids. Confidence 0.8 (measured by a real gate
  failure +0.4 · specific technical detail +0.2 · no other durable home +0.2 — the test-plan proposer's `@row_none`
  type 5 left it to the code; no route annotation, no master row, no matrix note).
  Proof: implement's report, Decisions & corrections «a whole re-record of a fixture set moves its recorded session ids,
  which four test sources pin as literals … red only after the landing (entries «run --unit», «run --integration»,
  «run --e2e», first firing: 1 + 5 + 3 cases)»; Deviations item 4 (the previous re-record `c82bb5bb` made the same move).
- T2 testing.md (extended, the 2026-11-19 count-probe entry): the figure from before a stopped run's move. Confidence
  0.8 (measured +0.4 · technical detail +0.2 · no other durable home +0.2). Filter 1: over the similarity bar with the
  2026-11-19 entry, a new failure mode (a pre-move literal a probe keyed on the abandoned figure misses) — the additive
  facet; the entry stays under 1.5 KB.
  Proof: implement's report, Decisions & corrections «a count literal the stopped run never moved (`pass-count` `27/27`
  in `rig_rehearsal.rs`, while its `rows` went 27 → 29) escapes a retired-shape pattern keyed on the stopped tree's
  figure (C1's `29/29`); measured this run, four sites»; Deviations item 2.

## Every Decisions & corrections entry, disposed
Implement's:
- Sweep hazard, the stopped run's count literal → extended (above).
- Sweep hazard, a re-record moves pinned session ids → applied (above).
- A remove-the-guard control over the floor needs an oracle keeping the floor names → not a learning here: P2 landed it
  as test-plan `decision/strip-control-oracle-keeps-guarded-names` (tx 3605), its master home.
- Guard refusal (a `rec.py query` piped into `sed`) → not a learning: the guard's own refusal, re-issued once
  (`guard-refused`).
- The directive (`inputs#I13`): `HG` the one base start → rejected, Filter 2 task-specific (this chunk's cap).
The close's (After implement):
- The wrap directive's routing (operator away, overseer dialogs, conservative widening) → rejected, Filter 2
  task-specific (this wrap's directive).
- Rows ratified only for the sender or Run B's literal not amended → not a learning: P2's disposition, recorded in
  `fanout-results.md`.
- The floor rows name the thirteen by name; a proposer set dropping a member or qualifier is rejected → not a learning:
  P2's validation rule for this wrap, recorded in `fanout-results.md`.
- The new route entry for the cross-session row → not a learning: P5's route edit.
- Name a compiled screen literal, never print it; no env value; no scheduled wake → duplicate: CLAUDE.md Session
  Learnings already holds both rules.
This conversation: the Bash guard refused a cat heredoc to a run-dir file → not a learning: host-linux.md's 2026-09-28
entry already holds it (`guard-refused`).
