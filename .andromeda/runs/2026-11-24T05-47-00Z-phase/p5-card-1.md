# P5 review card 1 (Revision 1) — 2026-11-24-viola-up («`viola up`») · plan rev 8 · research rev 4 · scope rev 3

**Read the plan:** `.andromeda/cache/docs/2026-11-24-viola-up/plan.md` (rendered from `doc:2026-11-24-viola-up/plan` rev
8). No plan stands in a chunk's folder: writing in that file changes no record until a `put`. A diff of the chunk's
records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-24-viola-up`.

**The stop:** implement run `2026-11-24T05-12-02Z-implement` stopped before its operator pass on one red outside the
diff, the e2e case `tui_channel_fds_the_child_holds_nothing_of_violas`: the fake agent held
`plugin/<key>/hooks/hooks.json` at one read and passed on the next. **The word:** `inputs#I5` (this revision's
invocation) and `inputs#I6` (the overseer's relay, a delegate's, technical, the operator away): fold it into this chunk
as one added step, with nothing else of the plan moving.

## What the revision does
- **Step 21 (new, section H2):** the case waits out a viola home file the fake has open during its own launch. It uses
  the same bounded retry it already applies to a link that vanished mid-read, inside the one 2 s deadline the case
  already holds, which is below the root package's 10 s kill line and the 20 s floor `tests/contract_lints.rs` reads.
  At the deadline the last whole table is asserted as today. **No change to what the guard asserts:** a file still held
  after the launch settles reads the same red, `fd {fd} holds a viola home file`. The case's comment says why.
- **Which arm, from the code:** the case-side wait, not «the fake closes it first». The fake already closes it:
  `fs::read_to_string` holds the handle only for the read (`src/bin/viola-fake-agent.rs:639`). The read sits in
  `launch`, after the `start` receipt by construction: the receipt is at :1547, before the trust branch, and the launch
  at :1569. An untrusted start writes its receipt and launches only once trust is taken, and every case waiting on
  `start` relies on that order. `fire_answered` also re-reads `hooks.json` at every fired event, so no earlier read could
  keep every later sample clear. Only a settle wait tells a transient open from a hold.
- **Remove-the-guard reading (`inputs#I6` item 2):** control `fds-held-red` (12 chars). Its edit is in
  `src/bin/viola-fake-agent.rs`: at the head of `launch`, it opens `<plugin dir>/hooks/hooks.json` and keeps it open
  for the process's life. With step 21 in, case `tui_channel_fds_the_child_holds_nothing_of_violas` must read `failed`.
  It fires on entry 15 (`run --e2e`) as itself with `--timeout 1800` named, though entry 15's own `timeout = 3600`
  governs each firing (gate-contract §Control). Entry 15 runs past a failure (15.2.log read 359/359), so the case's line
  is read.
- **One touchpoint:** `tests/tui_channel_fds.rs`. **One acceptance line:** «(e2e guard, Revision 1)». **Its entry:**
  the e2e layer, entry 15, whose note alone names step 21; its command, expect and timeout are unchanged.
- **U2 (`inputs#I6` item 3):** cited as the operator's own word, arm 1 as built, wherever the plan named it
  PROVISIONAL: Metadata, Goal, the window acceptance line, and two Expected-amendment lines. Its security-plan amendment
  now reads «the wrap halts on it once, as the answer card». U1 and U3–U8 stand PROVISIONAL as before.
- **Implement re-enters at step 21:** it does step 21 alone, fires `fds-held-red`, re-reads the whole layers (entries
  13–15), then runs the operator pass **without any R4 entry** (`inputs#I6` item 4). Steps 1–20 stand done
  (`outcome-2026-11-24-viola-up.md`).

Research: lists synced to the revision's delta (+1 −0), rev 4; not re-derived since the take-up's rev 3.

## Claims
matrix#v2-12 stays claimed GATED on R4. The revised plan still proves it, its concretization is unchanged, and nothing
is re-claimed or un-claimed. Its acceptance names U1 only, not U2.

## The plan's diff against rev 7
```diff
--- plan rev 7
+++ plan rev 8
@@ -7,8 +7,14 @@
 - **Marker:** 2026-11-24-viola-up · **Version:** viola-0.2.0
 - **Founder forks:** `.andromeda/runs/2026-11-24T04-17-54Z-phase/founder-forks.md` — U1–U5 his, each built on the arm
   taken there, PROVISIONAL until his own word; T1–T7 technical leans. U2 is a boundary widening (playbook
-  `rul-r74hshif` · `rul-p2zl2kgl`) built on its conservative arm.
+  `rul-r74hshif` · `rul-p2zl2kgl`) built on its conservative arm. U2 is now the operator's own word: arm 1, as built
+  (`inputs#I6` item 3, relayed verbatim by the overseer from his dialog of 2026-11-24T05:21:35Z).
 - **Capability:** matrix#v2-12, claimed GATED on R4a and R4b (one of them needs a real reboot).
+- **Revision 1** (run `2026-11-24T05-47-00Z-phase`, `inputs#I5`, `inputs#I6`): steps 1–20 stand done by implement run
+  `2026-11-24T05-12-02Z-implement` (its `outcome-2026-11-24-viola-up.md`: every entry not `operator` and not `defer`red
+  read its expect on the final tree; one red open outside the diff, the e2e layer's `tui_channel_fds` case). One step
+  is added, step 21; implement re-enters there, does it alone, then re-reads the whole layers and goes on to the
+  operator pass WITHOUT any R4 entry (`inputs#I6` item 4).
 
 ## Goal
 `viola up` re-opens, after a reboot, every instance of the home that was live before the host went down: each is
@@ -20,7 +26,7 @@
 The selection is the founder fork U1's arm 1, PROVISIONAL: an instance whose heartbeat mtime is older than this boot
 (`/proc/stat` `btime`) and whose events log holds no `session-end` after its newest `session-start`; or, named on the
 command line, an instance whose heartbeat is older than this boot whatever that reading. The terminal is fork U2's arm
-1, PROVISIONAL: `foot`, spawned directly with a closed argv, running the pinned `viola revive <name> --origin
+1, now the operator's own word (`inputs#I6` item 3): `foot`, spawned directly with a closed argv, running the pinned `viola revive <name> --origin
 <cli|agent>`; revive's new hidden `--origin` runs the wrapper's existing window step with the workspace its own
 snapshot recorded — the window record never leaves the snapshot onto an argv (security-plan ·
 decision/window-record-kept-to-snapshot-and-event-log). A hand-started instance (no window record) opens where the
@@ -208,6 +214,30 @@
     `.andromeda/cache/chunks/2026-11-24-viola-up/evidence/r4.md`. R4b's two entries refuse until a revision built on
     his U6 answer; the gated entry still needs BOTH green lines, so v2-12 cannot verify until R4b has run.
 
+### H2. The fd guard's settle wait — `tests/tui_channel_fds.rs` (Revision 1; keeps tests · decision/claude-child-holds-nothing-of-viola)
+21. `tui_channel_fds_the_child_holds_nothing_of_violas` waits out a viola home file the fake agent has open during its
+    own launch, the same bounded way it already reads again a table whose link vanished mid-read
+    (tests/tui_channel_fds.rs:52-54, :60-70): a table holding a path under the home that is not one of the two fixtures
+    is taken as not yet settled and read again, inside the one deadline the case already holds (2 s,
+    `std::thread::yield_now` between reads — below the root package's 10 s kill line and the 20 s floor
+    `tests/contract_lints.rs` reads, `MUTANTS_FLOOR_SECS`). At the deadline the last whole table is asserted as today,
+    so a file still held reads the same red, `fd {fd} holds a viola home file`. The case's own comment says why: the
+    fake's launch reads `hooks.json` after its `start` receipt (src/bin/viola-fake-agent.rs:1547, :1569, :639), an
+    open that closes inside that one read; a file held after the launch settles stays the guard's red. Nothing the case
+    asserts moves: fds 0–2 the PTY, no socket, neither the endpoint nor its lock, no home path but the two fixtures
+    (`inputs#I6` item 1).
+    - Chosen over making the fake close the file before the point the case reads (the relay's other arm), from the
+      code. The fake already closes it: `fs::read_to_string` holds the handle only for the read (:639). The read sits
+      in `launch`, after the `start` receipt by construction: an untrusted start writes its receipt and launches only
+      once trust is taken (:1547 before the trust branch at :1567-1570), and every case waiting on `start` reads that
+      order. And `fire_answered` reads `hooks.json` again at every fired event (:634-639; the sender's path at :1188),
+      so no earlier read would keep every later sample clear. Only a settle wait in the case tells a transient open
+      from a hold.
+    - Its remove-the-guard reading is the `fds-held-red` control below: with this step in, a fake made to keep
+      `hooks.json` open for its life still reads the case red (`inputs#I6` item 2).
+    - Implement re-enters here (Metadata, Revision 1): step 21 alone, the `fds-held-red` control, the whole layers
+      (entries 13–15), then the operator pass with no R4 entry fired.
+
 ### I. Close
 20. `cargo fmt`, the block below green, the report written.
 
@@ -233,6 +263,7 @@
 - `tests/cli_revive.rs` — revive's window flag over the fakes
 - `tests/cli_machine_contract.rs` — `up --json` and the new flags in the controls table
 - `schemas/diag-line.v1.json` — the `process-exit` detail word `boot-time-unread`
+- `tests/tui_channel_fds.rs` — step 21: the fd table read again while the fake's launch has a home file open (Revision 1)
 
 ## Test Commands
 The block runs the formatter and the linter; the root binaries' units and the filtered new cases (0 on the untouched
@@ -262,6 +293,13 @@
 | `up-order` | `src/cmd/up.rs` | the parent-first ordering of the candidates | `up_plan_orders_parents_before_children` | `ctl-root` |
 | `up-boot` | `src/cmd/up.rs` | the `beat_before` test on an unnamed candidate | `up_plan_takes_only_a_beat_older_than_the_boot` | `ctl-root` |
 | `up-ended` | `src/cmd/up.rs` | the `ended_after_last_start` test on an unnamed candidate | `up_plan_leaves_out_an_instance_ended_after_its_last_start` | `ctl-root` |
+| `fds-held-red` | `src/bin/viola-fake-agent.rs` | the absence of a hold: at the head of `launch` (anchor `fn launch(agent: &Agent) {`, once in the file) an open of `<plugin dir>/hooks/hooks.json` is added and kept for the process's life (`std::mem::forget`) | `tui_channel_fds_the_child_holds_nothing_of_violas` | `run --e2e` (entry 15) |
+
+`fds-held-red` (step 21, Revision 1) is fired on entry 15 as itself, not on `ctl-root`: its case spawns viola and the
+fake agent, so only a workspace-building harness layer reproduces it in the tool's clone (testing.md 2026-11-13). It is
+fired with `--timeout 1800` named; entry 15's own `timeout = 3600` governs each firing (gate-contract §Control). The
+layer runs every case past a failure (15.2.log read 359/359 after the red at 325), so the case's result line is read.
+It holds when the case reads `failed` with the edit in.
 
 ```toml
 [[gate]]
@@ -377,7 +415,7 @@
 role = 'e2e'
 timeout = 3600
 expect = ['exit 0', 'contains "ok":true']
-note = 'the contract and e2e suites; atom from .claude/rules/verification-harness.md'
+note = 'the contract and e2e suites; step 21: tui_channel_fds and its fds-held-red control (Revision 1); atom from .claude/rules/verification-harness.md'
 
 [[gate]]
 run = '''test -s .andromeda/cache/chunks/2026-11-24-viola-up/evidence/r4.md && grep -q '^r4: green phase=windows ' .andromeda/cache/chunks/2026-11-24-viola-up/evidence/r4.md && grep -c '^r4: green phase=after ' .andromeda/cache/chunks/2026-11-24-viola-up/evidence/r4.md'''
@@ -482,8 +520,8 @@
 - (window) A revive under `--origin` moves its own terminal's window to the workspace its snapshot recorded, and
   records it; with no recorded workspace it records `unplaced`; a revive without the flag runs no window step —
   `revive_window_ask_takes_the_recorded_workspace`, `up_moves_each_window_to_its_recorded_workspace`,
-  `up_opens_a_hand_started_instance_unplaced` and the `cli_revive` cases (founder-forks U2 arm 1, U3 arm 1,
-  PROVISIONAL).
+  `up_opens_a_hand_started_instance_unplaced` and the `cli_revive` cases (founder-forks U2 arm 1, the operator's own
+  word, `inputs#I6` item 3; U3 arm 1, PROVISIONAL).
 - (output) One line per instance taken or named, its word from the closed set of step 10, ASCII, no SGR; `--json` one
   document of names and closed words; `--dry-run` spawns nothing; a refused board is exit 21 with no line; no address,
   pid or workspace on a line (design · contract/cli-verb-output/focus's rule; obs ·
@@ -504,11 +542,19 @@
   and exit 1, starting nothing, and the script holds no credential path — the two `r4b-refuses` entries.
 - (output) After the lines, one `hint:` line when any reads `not-live` (founder-forks U7, U8, PROVISIONAL) — the
   `up_*` output cases.
+- (e2e guard, Revision 1) The wrapped child holds nothing of viola's once its launch settles:
+  `tui_channel_fds_the_child_holds_nothing_of_violas` reads green in entry 15 with its assertions unchanged, a home
+  file the fake opens only during its own launch waited out inside the case's 2 s deadline; the `fds-held-red` control
+  holds (a fake keeping `hooks.json` open reads the case red) — step 21 (test-plan ·
+  decision/claude-child-holds-nothing-of-viola; `inputs#I6` items 1–2).
 
 ## Provenance
 - `inputs#I1` — the directive (scope v2-12, nothing live, the forks' away form, the measurements, the live readings).
 - `inputs#I2` — the overseer's review of card 1 (T1–T7 answered; the live root, R4b's refusal and U6, R4a's stand-in
   agent, U7's new words, R4a's workspaces as grant parameters, U8).
+- `inputs#I5` — the word that opened Revision 1 (the invocation, the overseer for the away operator); `inputs#I6` —
+  the overseer's revision relay (the `tui_channel_fds` red folded as one step, its control, U2 answered by the
+  operator, the card's form).
 - Rows: security-plan · decision/open-terminal-direct-spawn, decision/run-window-flags-checked,
   decision/window-record-kept-to-snapshot-and-event-log, decision/window-step-moves-own-terminal-window,
   decision/lineage-parent-authenticates-nothing, decision/list-scan-checks-before-open, decision/claude-agents-read-fail-open;
@@ -572,11 +618,12 @@
   (`inputs#I2` changes 1 and 3).
 - Expected amendments (wrap): security-plan · decision/open-terminal-direct-spawn — `up`'s terminal starts the pinned
   `viola revive <name> --origin <word>`, a boundary widening on founder-forks U2 arm 1 (pattern `rul-r74hshif` /
-  `rul-p2zl2kgl`; delegate overseer, the operator away — the wrap's full escalation, PROVISIONAL).
+  `rul-p2zl2kgl`; was delegate overseer, PROVISIONAL; now the operator's own answer, arm 1 as built, `inputs#I6` item
+  3, relayed verbatim by the overseer — the wrap halts on it once, as the answer card).
 - Expected amendments (wrap): security-plan · decision/run-window-flags-checked — its twin for revive's hidden
   `--origin` (the same widening, the same authority).
 - Expected amendments (wrap): layouts · contract/viola-revive/no-window — a revive places a window only under the
-  hidden `--origin` `up` hands it (PROVISIONAL, U2).
+  hidden `--origin` `up` hands it (U2, the operator's own word, `inputs#I6` item 3).
 - Expected amendments (wrap): architecture · decision/revive-resumes-in-place — under `--origin` revive also replays
   the recorded workspace into its window step; architecture · contract/cli-verbs/revive — the hidden flag.
 - Expected amendments (wrap): architecture · decision/lineage-label-only — `up` reads the parent to order its
```

## Directive item-11 block (`inputs#I1` item 11; tool lines verbatim)
- **The union's per-master lines and the `rows-union` line:** none this run. P2 does not run in a revision
  (SKILL.md §Revision), and the take-up's lines stand as card 1 of run `2026-11-24T04-17-54Z-phase` showed them.
- **Is every cited row in the union or its standing list:** yes. The one row this revision newly cites, test-plan ·
  `decision/claude-child-holds-nothing-of-viola`, was looked up small (`rec.py find` → `dec-huukjbac r1`). It stands
  in the take-up's `tests-rows.md` (1 hit) and not in `tests-standing.md` (0). Every other citation is carried from
  rev 7.
- **`[RETYPED]` lines and re-spawns:** none. No batch ran.
- **Non-zero exits, refusals, `UNPARSED` / `INDETERMINATE` / `NOT DERIVED`:**
  - `gate.py scope --sync`, first fire: `added `tests/tui_channel_fds.rs` — research's lists name it nowhere` ·
    `scope sync: FAIL 1 · delta +1 −0 · research rev 3 · derived rev 3`. Research's lists were then edited (rev 4),
    and the re-fire read `scope sync: PASS · delta +1 −0 · research rev 4 · derived rev 3`. Re-fired after the
    checks, also PASS.
  - bash-guard refused 3 calls, each re-issued once in the passing form: two `cd` into a directory outside the root
    (the skill's references dir; the chunk's inputs dir), and one `rec.py render … | tail -1`.
  - `diff -u` of rev 7 against rev 8 exited 1, which is diff's «they differ».
  - No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed.
  - planlint printed `hits: 0` and listed check 12 (d) `6 of 8`. The new claim at the acceptance line («assertions
    unchanged», step 21) was read true: step 21 changes the read loop, not one assertion. The other five are card 2's,
    unchanged.
  - Every other check exited 0: `gate.py run --dry-run` parsed 25 entries; `gate.py baseline` read seen green 5,
    not seen 0, new (key) 11, leg 9 (entry 15 seen green, so no baseline run is owed); `inputs.py verify` read
    6 entries, unchanged 4, n/a 2, uncited 0; `gate.py scope --lists-only` parsed 20 items.
- **Places a letter was wrong or silent:**
  1. §Revision item 3 re-runs P2 whole when the corrected scope adds a file no take-up task named, and
     `tests/tui_channel_fds.rs` is such a file. The letter's skip is the operator's word «no research». This word
     did not spell it; it said «one added step, nothing else of the plan moving». I read that as the skip, looked
     the one binding row up small, and stated the reading on research's Revision 1 section. That reading is open to
     your word: `review` re-runs the row read whole.
  2. gate-contract §Control applies `--timeout S` only to an entry with no `timeout` key. The relay asks for
     `--timeout` to be named, so it is named (1800), but entry 15's 3600 governs.
  3. The letter is silent on how an added step between H and I is lettered and numbered. It stands as section H2,
     step 21, ahead of Close's step 20. Nothing was renumbered, so nothing else moves.
  4. My own slip, not the letter's: this run dir is named `2026-11-24T05-47-00Z-phase`, but `date -u` read
     05:45:29Z at its creation. The name ran about 90 s ahead and was not read from the clock. Two snaps already
     cite the dir, so it is left as named.

## Beyond the plan
- none (standing — `.andromeda/runs/2026-11-24T04-17-54Z-phase/p5-card-2.md`: `none`)

Apply? (yes / review / cancel)
