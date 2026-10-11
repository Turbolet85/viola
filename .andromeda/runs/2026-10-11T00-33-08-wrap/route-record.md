# Route record — wrap of 2026-10-10-session-links (Phase 5)

Only markerless lines of `viola-0.1.0/working-route.md` changed: 123 and 156 (`git diff --unified=0`: two hunks,
`@@ -123 +123 @@` and `@@ -156 +156 @@`). The frozen line 121 is untouched; its two CARRYs were read whole before
any edit. No entry was added, moved or retired. "The board: viola list" is the head (`route.py cursor`: `next
working-route.md:123`).

## The two CARRYs the entry carried

1. chunk 2026-10-10-self-healing-state (the two kinds in the kind table, the replay's link half, test-plan §6
   Scenario E5's `link` step) — discharged: `EventKind::Link` and `::Unlink` with `data` keys `driver`, `driven`,
   `apply_link_line` in the replay's one pass, and `e5_the_replay_reads_the_link_the_wrapper_wrote`.
2. chunk 2026-10-10-budget-governor (the test for `answer`, `wait`, `last` and `pause` under a closed gate) —
   discharged: `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` in `tests/cli_budget.rs`. The three
   masters that named this entry as the owner were amended at Phase 2.

## Pin made (one CARRY block, on a recorded direction naming entry and disposition)

"The board: viola list" — the explicit `viola link` / `viola unlink`, steps 12 to 15 of the chunk's plan, held.
The operator's direction with the resume (`inputs#I6`, the operator, 2026-10-11): "steps 12 to 15 become ONE CARRY
on the route head, The board: viola list, naming the hold: the explicit verbs wait for the founder word on the
widening, its three options quoted." The block names what is owed, the hold and what lifts it, the three places
the widening crosses (the chunk's `scope.md`), the three options as the plan's step 11 words them, what landed,
the dead-code allow on `LinkSet::remove` and where it ends, the operator's answers for the unbuilt part, and the
plan's boundary that v1 `list` shows no link column. The entry holds eleven CARRY blocks now (`route.py pins`:
the new one 2935 chars; no `INDETERMINATE:` row).

No test for the skipped gate entry is pinned apart from it: `tests/channel_links.rs` is part of step 15 and
rides this CARRY.

## Citation row (from the Phase 2 sweep)

`viola-0.1.0/working-route.md:156` `src/cmd/run.rs:445` → 455 — the markerless entry "Windows-only live
measurements". Read at the new number: line 455 is the body `("not-built", None)` of `sideload_outcome` under
`cfg(all(windows, not(target_arch = "x86_64")))`, the place the cited mutants stand (`455:5`). The digits were
re-pointed by hand, nothing else on the line moved.

## Blocks and gates

- `route.py cursor`: gated 0. No `gated` record exists, so no premise was re-verified for one. This chunk's own
  frozen line carries no `BLOCKED-ON`.
- The `BLOCKED-ON` on "Windows-only live measurements" stands: `uname -s` read Linux at this wrap. The entry was
  touched only for the citation's digits.
- No gate deferral, no `PREREQ` and no `WATCH` was on the frozen line, and the report notes no deferral.

## Epoch

`route.py epoch`: Epoch 4 holds 10 entries, 8 frozen complete, this chunk pending, 1 markerless. After the flip
it reads 9 complete and 1 markerless, so it does not close at this wrap: no consolidation, no evolve nudge. The
growth valve's note is carried on the handoff as before (no boundary inside Epoch 4, the founder's word of
2026-10-09).

## Not pinned, carried on the handoff

- The link rule takes a logged `driver` / `driven` as a string and does not parse it as a name again
  (security-plan, row "Own state files on read", states it). Whether that is work for an Epoch 6 entry is not
  decided here; it is listed for the operator's word.
