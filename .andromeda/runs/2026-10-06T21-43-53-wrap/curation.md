# Curation log — 2026-10-06-local-command-and-paste-framing-rows

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A live shape probe's record states, for every settle it waited on, how long the screen took to settle and what it settled on" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific · 0 conflict · 0 deferred · 7 below threshold
  No-other-home: "a live shape probe's record states how long each settle took" · "a common stall across process-spawning tests is a host stall"
  Extended: T2/verification-harness.md: "On this host the local suite grades an identical tree differently run to run" + "a common stall across process-spawning tests points at the host: read other sessions' build dirs by mtime per window; read a red entry's log before a re-run overwrites it"
```

## Applied
- **testing.md, new entry (2026-10-06).** Signals: verified by measurement +0.4, specific technical detail +0.2,
  reached no other durable home +0.2 = 0.8.
  Proof: step 0's record said four settled screens each showed the input box; its own drive log held the long
  turn settling 5.8 s after its Stop against 1.3 s for the others, and that left-out reading is what explained the
  first record round's `15 pass  2 fail` (`evidence/record-round-red.md`; the rehearsal then measured 6.8 s,
  `evidence/rehearsal-shapes.md`).
- **verification-harness.md, the 2026-09-29 entry extended in place (2026-10-06).** Signals: verified by
  measurement +0.4, specific technical detail +0.2, reached no other durable home +0.2 = 0.8. Filter 1: over the
  similarity bar on "a local red on this host", with a facet the entry lacks (a named cause class and a cheaper
  discriminator than paired runs).
  Proof: gate entries 16 and 18 read red in the first full block with every process-spawning test stalled about
  18 s; another project's build wrote 10.6 GB and 5.2 GB to the same volume in exactly those windows and nothing in
  the green ones (`evidence/block-reds-host-contention.md`); the targeted re-run had overwritten both red logs.

## Not applied
- below threshold (0.6 or less, no conditional signal): a probe that pastes several texts waits on the input-box
  literal, not a timer (0.6; amended into architecture and security-plan this wrap) · a runner kill of a
  verify-driven test leaves verify's probe dirs at the repository root (0.6; carried on the route) · dry-run the
  row arms and the record scrub over a rehearsal's real captures before a non-retriable round (0.2) · run a live
  driver against a stand-in first (0.2) · `grep -E` on this host is ugrep and refuses a bounded-repeat context
  window (0.2, two instances) · `pgrep -x` matches nothing past 15 characters (0.2) · the operator CI read is the
  `ci.py conclusion` entry, never a copied poller (0.5; the plan template's operator entries already carry it).
- task-specific: the 20:49Z card was not labelled a widening (recorded in the two sidecar entries) · the
  proposal count on the escalation card (52, shown as 53).

## Recurrences (handoff, Deferred learnings)
- `recurrence-despite-learning: host-win32.md` ("`rm -rf` + … compounds get denied"): an `rm -rf` of a kept test
  home chained with a listing was refused by the permission layer and not retried.
- `recurrence-despite-learning: host-win32.md 2026-09-28/29` (a heredoc with a file target): one `cat` heredoc was
  refused by the Bash guard at this wrap; already carried.
