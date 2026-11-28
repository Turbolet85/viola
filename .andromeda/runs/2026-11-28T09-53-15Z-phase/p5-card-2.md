# P5 review card 2 — chunk 2026-11-28-fleet-console-view, «Fleet console view» (the review of card 1 folded)

Written 2026-11-28T11:57:05Z. The review of card 1 (`.andromeda/runs/2026-11-28T09-53-15Z-phase/p5-card-1.md`) is
the overseer's technical relay, inputs#I4: two changes on the log volume, then «yes» for that revision on two
conditions — planlint reads 0 hits, and no master row is reached that card 1's table does not name. Both hold
(below), so this card records the revision and the word together; no prompt follows it.
**The plan:** `doc:2026-11-28-fleet-console-view/plan` rev 6, its render at
`.andromeda/cache/docs/2026-11-28-fleet-console-view/plan.md`.

## The two changes, as built into rev 6
1. **`console-frame` on a change** (step 6): written at the first draw, then only when one of `cols`, `rows`,
   `fits`, `shown`, `hidden` differs from the line last written; a `LAST` tick draws and writes no line.
   `shown` and `hidden` now count the fleet's rows (instance rows only, never the caption, the empty-board line or
   the separator) so a row's arrival or leaving moves them. **No case loses its state read:** the scaffold's cases
   read the first frame or the frame after a resize (`tests/tui_console.rs:136-186`), both still written; the new
   `console_draws_a_booted_wrapper_and_its_end` reads `shown` 0 → 1 → 0. No second event is needed.
2. **No replay of an unchanged log** (steps 2, 5). **Measured from the code:** the replay sits in
   `read_snapshot_or_replay` (`crates/viola-state/src/replay.rs:240-263`), called once per instance inside
   `board::read`'s private loop (`crates/viola-state/src/board.rs:404-450`), which also walks every log whole
   (`log_pass`, `board.rs:382`). The skip cannot sit in the console's caller alone; it takes two pub functions in
   viola-state (`read_one`, `relive`) with `board::read` rebuilt on them — `viola list` byte-identical, held by its
   literal cases — and a per-instance stamp map (length, modification time, inode of `snapshot.json` and
   `events.ndjson`) in `src/console/fleet.rs`. **What it adds in lines:** none per second; one `state-recovered` per
   replay that actually runs (a replayed instance whose files moved), as the row says. **Fit:** two functions, one
   map and four unit cases — inside the one window.
- **The `agents-probe` pairs** stand as his F6: one per 5 s, 720 pairs (1 440 lines) an hour.

## The two conditions
- planlint rev 6: `hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12
  (c)); 0 is not a pass` · `exit=0` (12 (d) lists 13 «unchanged» readings, each an output held beside changed code).
- **Rows the changes reach:** searched over all seven masters by «once per draw», «modification time», «mtime», «per
  read», «cache» (`rec.py find --doc all --text`). Moved: (obs-plan · contract/diag-line-fields/console-frame)
  «Logged once per draw» — card 1's table row 9, its change now «at the first draw and then when one of its fields
  changes» beside `shown`/`hidden`, the overseer's technical word (delegate, PROVISIONAL). Read and holding as
  written: (architecture · decision/reader-counts-skipped-lines), (security-plan · decision/events-reader-bounded-lines)
  — the reader still counts per read it makes; (test-plan · decision/snapshot-replayed-from-log),
  (architecture · decision/snapshots-read-through-replay), (obs-plan · contract/state-recovered-detail/snapshot-replayed)
  — every snapshot read still goes through the replay and each replay that runs writes its line. No other row moves.

## Checks re-run on rev 6
Fence parses (21 entries); baseline listing `seen green 5 · not seen 0 · new (key) 9 · leg 7`; entries 4 and 6
re-baselined red, 0 selected (0 of 328, 0 of 1446); inputs `4 entries — unchanged 3 · … · unparsed 0`; scope lists
`parsed 16 items`; cited rows 29 of 29 in the union or a standing list.

## Beyond the plan
- **The console's log volume** (card 1's row): narrowed by inputs#I4 to the `agents-probe` pairs of his F6 (720 an
  hour) and the lines of real changes; no owner proposed — (standing — `.andromeda/runs/2026-11-28T09-53-15Z-phase/p5-card-1.md`).
- **An agents child outliving a quit** (up to its 5 s bound): accepted as built; no owner proposed — (standing —
  `.andromeda/runs/2026-11-28T09-53-15Z-phase/p5-card-1.md`).

The word: «yes» for this revision, on inputs#I4's two conditions, both met.
