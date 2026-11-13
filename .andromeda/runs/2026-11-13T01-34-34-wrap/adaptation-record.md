# Adaptation record — the 2026-11-13T01-34-34-wrap 0-pending wrap

Request: the operator's relay, snapped `inputs#I1` (`.andromeda/runs/2026-11-13T01-34-34-wrap/inputs/I1-wrap-0.2.0-rows-first-relay.md.txt`,
source `../additional/viola-overseer/wrap-0.2.0-rows-first-relay.md`), its five items this wrap's request. The operator is
away: no dialog raised; where an item needed a word it did not carry, the most reversible option was taken, marked
PROVISIONAL below.

## Items

1. Head entry «Verify readiness on the input box», before «Epoch 8 cleanup», and its one requirement added by the tool
   (the founder's word: dialog with overseer1, 2026-11-09, «Первым чанком 0.2.0»; evidence: finding V178).
2. Two Epoch 5 entries — «Model and effort at launch» immediately before «`viola open`», «Switching a session's model
   and effort» immediately after «`viola adopt`» — one requirement each, added by the tool (the founder by dialog,
   2026-11-09T20:57Z, «Два чанка в эпохе 5»).
3. The rows landing `2026-11-13T01-15-10`'s 14 `route-owed.md` lines placed (a PROVISIONAL pre-direction in the relay):
   three on the entries already standing, eleven on a new corrective entry «Rows-proposal rule work» immediately after
   «Epoch 8 cleanup», with the snapshot-replay guard beside them; and the dead `#[cfg(not(unix))]` constants folded
   into «Epoch 8 cleanup» as a CARRY.
4. V174's residue: the five `design-system` placeholder facts retired, if they still stand.
5. Commit after `gate.py hygiene` reads `clean`, push, report.

## Dispositions

1. Applied. `matrix.py add --after v2-03` (theme (a) The corrective head) minted **v2-29** «Verify waits on the input
   box, not the footer», planned, unclaimed. Entry `chk-srpwedyf` «Verify readiness on the input box — … (v2-29)»
   landed in Epoch 1 (tx 832) and placed first by the reorder `chk-zivklxpc after=chk-srpwedyf` (tx 833). The theme
   placement (a) is this wrap's own choice; the relay names none — PROVISIONAL.
2. Applied. `matrix.py add --after v2-15` minted **v2-30** «Model and effort named at launch»; `--after v2-30` minted
   **v2-31** «Switch a session's model and effort»; both theme (c) The fleet, planned, unclaimed. Entries
   `chk-ruio4tl7` «Model and effort at launch» after «Lineage registry» (= immediately before «`viola open`») and
   `chk-ke32dhty` «Switching a session's model and effort» after «`viola adopt`», both in Epoch 5 (tx 832); the route's
   other entries unmoved.
   Both new requirements' acceptance texts and methods (`integration`) are this wrap's authoring at outcome level, not
   the founder's words — PROVISIONAL until the claiming chunk concretizes them.
3. Applied, the placement PROVISIONAL as the relay marks it (every carry says `placement provisional`):
   - `decision/shared-captions-fixed` → CARRY on «Fleet console view» (`chk-brrns5xl`) — the Epoch 6 entry that names
     "each session's state in the board's words"; choosing it over «Console scaffold» is this wrap's reading —
     PROVISIONAL;
   - `decision/no-tmux-or-shell-dependency` → CARRY on «Status-line interception removed» (`chk-tnbdlbiv`);
   - `decision/exact-third-party-pins` → CARRY on the head entry `chk-srpwedyf`, naming the `supply-chain` red it closes;
   - the other eleven lines → one CARRY each on the new entry «Rows-proposal rule work» (`chk-ikt2ehmw`), plus the
     test-plan guard `e5_the_replay_reads_the_link_the_wrapper_wrote` (tests/cli_links.rs:259) and one CARRY naming the
     source and the provisional placement (13 carries);
   - the dead constants (crates/viola-e2e/src/harness/run.rs :349 · :1026 · :1097) → CARRY on «Epoch 8 cleanup».
   Read back: `pins` 17 rows, every introducer `CARRY:`; `abstained` 0 rows.
4. Nothing done: all five facts read `retired at tx 803` (`rec.py get`), the landing having removed them.
5. Below, in the commit and the handoff.

## Unexpected tool behaviour (for overseer1)

- `rec.py put`: a reorder `@chunk id={old} after=@a` naming a chunk minted in the same batch is refused, exit 3,
  `chk-…: stands under no \`contains\` — nothing written` (probed: the same reorder alone, and the new chunk alone,
  each dry-run clean). Routed around by two TXs (832, then 833).
- `rec.py put`: in one batch, `@chunk id=chk-zivklxpc after=chk-srpwedyf` followed by `@chunk … after=chk-zivklxpc`
  placed the new entry by `chk-zivklxpc`'s OLD position (it rendered first in the epoch). Corrected by a third TX
  (834), `@chunk id=chk-ikt2ehmw after=chk-zivklxpc`; the render reads in the relay's order.
- `matrix.py add` lands its TX into the journal file `2026-11-01T02-56-29-no-marker.ndjson`, an existing file of an
  earlier no-marker run, rather than a file of this run.

## Counts

- Markerless entries: 25 before → 29 after (`position` view).
- Requirements minted: v2-29 · v2-30 · v2-31 (28 → 31); the version reads `verified 0/31 · deferred 0 · planned 31 ·
  implemented 0 · unclaimed 31`. Each new entry names its id, so v2-29 · v2-30 · v2-31 stay unclaimed until the
  entry's phase claims it.
