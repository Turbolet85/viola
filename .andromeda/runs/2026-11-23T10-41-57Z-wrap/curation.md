# Curation — wrap 2026-11-23T10-41-57Z-wrap (chunk 2026-11-23-model-and-effort-at-launch)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   extended testing.md (below)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup (recurrence → handoff) · 4 task-specific · 0 conflict · 0 deferred
  No-other-home: "a counter written as an offset or a key list spread over a multi-line literal escapes a single-line sweep"
  Extended: T2/testing.md: "2026-10-04: A test that counts `events.ndjson` lines after `Wrapper::stop` … Extended 2026-11-14: …" + "a counter written as an offset (`3 + cases.len()`) or a key list spread over a multi-line `json!` literal or array escapes a single-line pattern, so sweep for the old bare count and the item's last key too"
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

## The applied extension
- testing.md, the 2026-10-04 entry (extended 2026-11-14): `Extended 2026-11-23` facet, confidence 0.8 (measured +0.4, specific technical detail +0.2, no other home +0.2). Landed by `rec.py harness land`, tx 4241.
  Proof: the integration-binaries entry's first firing read red, 3 of 206 (`tests/hook_events.rs` `hook_prompts_arrive_normalised_with_their_origin`'s `3 + cases.len()`; `tests/cli_list.rs` `e5_list_reads_a_live_wrapper_through_the_replay`'s multi-line key literal), both outside the plan's single-line search 18 (implement's report, Deviations and Decisions & corrections; `outcome-2026-11-23-model-and-effort-at-launch.md`, Gates).

## Every Decisions & corrections entry, dispositioned
Implement's:
1. The overseer's word to go on (host quiet from 10:21:18Z, hostwatch read `--from`) — not a learning: a one-off host reading of this run.
2. The directive's host rule held the layers once (CONTENDED, escher alive) — not a learning: the host rule stands in the overseer's directive and the handoff's Host note; task-specific.
3. The overseer ruled step 23 a deviation, never a plan revision — not a learning: a ruling of this chunk, its fact amended into the masters at P2 (test-plan decision/snapshot-replay-from-log, decision/snapshot-replay-fields).
4. The overseer re-worded two phase-card lines path-free — not a learning: task-specific (two lines of one run).
5. Recurrence of testing.md 2026-10-04 (extended 2026-11-14) — Filter 1 dedup hit on a DEFECT record: `recurrence-despite-learning` → handoff Deferred learnings; its new facet taken as the extension above.
6. Sweep hazard: a list-key reader as a multi-line `json!` literal escapes a single-line grep — extended (the facet above, folded with entry 5's).
7. Sweep hazard: serde_json built with `preserve_order`, so a parsed `list --json` keeps key order — rejected, Filter 4 (one-off mention, +0.2 −0.3).
8. Guard refusals, each one re-issue — not a learning: the guard put each call on its path (telemetry).
9. The smoke's status read the compiled default pair, no inherited value — rejected, Filter 2 (task-specific reading of one smoke run).
After implement's:
10. The wrap directive (`inputs#I8`): whose word each answer is; «handed», never «runs on»; the config contract says the default once — not a learning: carried into the masters at P2 (architecture decision/launch-pair-handed-not-verified, contract/config-json/default-model / default-effort) and the dispositions; task-specific to this chunk.
11. The readers the plan's search missed, beside testing.md 2026-10-04 — the same candidate as 5 and 6 (extended; recurrence to the handoff).
12. This wrap's dialog (`inputs#I9`): Q1 confirm, Q2/Q3 provisional — not a learning: a ruling recorded in the sidecars and `founder-forks.md`.
