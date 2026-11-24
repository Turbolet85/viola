# P5 review card 2 (Revision 1) — 2026-11-24-viola-up («`viola up`») · plan rev 10 · research rev 5 · scope rev 3

**Read the plan:** `.andromeda/cache/docs/2026-11-24-viola-up/plan.md` (rendered from `doc:2026-11-24-viola-up/plan` rev
10). No plan stands in a chunk's folder: writing in that file changes no record until a `put`. A diff of the chunk's
records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-24-viola-up`.

**What folded since card 1** comes from the overseer's review (`inputs#I7`, a delegate's word, the operator away). It
answered `.andromeda/runs/2026-11-24T05-47-00Z-phase/p5-card-1.md`.
- **(1) The control's entry:**
  - **New entry 16, `ctl-fds`:** `bash scripts/agent-run.sh run --e2e --filter 'binary(tui_channel_fds)'`, with
    role `e2e`, `timeout = 1800` and expect `exit 0` / `contains "ok":true`. It selects the guard's binary alone and
    keeps the harness's workspace build, which the tool's clone needs (testing.md 2026-11-13).
  - **Its baseline**, fired now with `gate.py run --entry 16`: `16 e2e green · exit 0 · 0.81s · 666 B → 16.log`. The
    log reads `Starting 1 test across 1 binary (84 binaries skipped)`, with the case `PASS`. The red it guards is
    intermittent, so this green proves the selection, not the race.
  - **`fds-held-red`** now fires on entry 16 with `--timeout 1800` named, the same as the entry's own key. The
    whole-layer entry 15 stays step 21's entry, and its note now points at entry 16.
  - **Renumbering:** the gated entry moves 16 → 17 and the operator entries 17–25 → 18–26. The one prose reference
    to the gated entry ("Must-pass readings") is renumbered and says so. No other entry changes.
- **(2) No research for this revision** is confirmed (`inputs#I7` item 2). Research's Revision 1 section cites it in
  place of my reading (research rev 5). Card 1's open item 1 is closed.

**Unchanged from card 1:** step 21 and its arm (the case-side settle wait, its assertions unmoved), the one
touchpoint, U2 cited as the operator's own word, and the re-entry. The re-entry now reads: step 21 alone, entry 16
and `fds-held-red` on it, entries 13–15, then the operator pass WITHOUT any R4 entry (`inputs#I6` item 4).

Research: lists synced to the revision's delta (+1 −0), rev 5; not re-derived since the take-up's rev 3.

## Claims
matrix#v2-12 stays claimed GATED on R4. The revised plan still proves it, its concretization is unchanged, and nothing
is re-claimed or un-claimed. Its gated entry is now entry 17; the claim names the gate's text, not the entry number.

## The plan's diff against rev 7
```diff
--- plan rev 7
+++ plan rev 10
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
+    - Implement re-enters here (Metadata, Revision 1): step 21 alone, entry 16 (`ctl-fds`) and the `fds-held-red`
+      control on it, the whole layers (entries 13–15), then the operator pass with no R4 entry fired.
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
@@ -249,8 +280,9 @@
 
 **Must-pass readings owed to implement:** entry 10 (the rehearsal) asserts literals step 18's script prints; its first
 firing once the script exists, before any graded use, is their reading; so is entry 11's (`r4b-not-granted`), fired
-after it. Entry 16's record, `evidence/r4.md`, is written by the operator from R4's output (step 19), a step outside
-this block; entry 16 reads it and carries no `artifact`.
+after it. Entry 17's record, `evidence/r4.md`, is written by the operator from R4's output (step 19), a step outside
+this block; entry 17 reads it and carries no `artifact` (entry 16 in rev 7 and before: card 2 of the revision inserted
+entry 16, `ctl-fds`, ahead of it).
 
 **The remove-the-guard controls** are implement's one-shot `gate.py control` runs, never entries. Each is fired on
 the `ctl-root` entry (entry 4), its case named, `--timeout 900`; each case fails on its own bounded assertion, so a
@@ -262,6 +294,15 @@
 | `up-order` | `src/cmd/up.rs` | the parent-first ordering of the candidates | `up_plan_orders_parents_before_children` | `ctl-root` |
 | `up-boot` | `src/cmd/up.rs` | the `beat_before` test on an unnamed candidate | `up_plan_takes_only_a_beat_older_than_the_boot` | `ctl-root` |
 | `up-ended` | `src/cmd/up.rs` | the `ended_after_last_start` test on an unnamed candidate | `up_plan_leaves_out_an_instance_ended_after_its_last_start` | `ctl-root` |
+| `fds-held-red` | `src/bin/viola-fake-agent.rs` | the absence of a hold: at the head of `launch` (anchor `fn launch(agent: &Agent) {`, once in the file) an open of `<plugin dir>/hooks/hooks.json` is added and kept for the process's life (`std::mem::forget`) | `tui_channel_fds_the_child_holds_nothing_of_violas` | `ctl-fds` (entry 16) |
+
+`fds-held-red` (step 21, Revision 1) is fired on entry 16, `ctl-fds`, as itself: the e2e harness layer filtered to the
+`tui_channel_fds` binary alone, the guard's own cases and no other (`inputs#I7` item 1), never on the whole layer.
+Only a workspace-building harness layer reproduces a case that spawns viola and the fake agent in the tool's clone
+(testing.md 2026-11-13), and the harness's `--filter` keeps that build while selecting one binary (testing.md, «an
+E2E-prefixed binary through `run --e2e --filter 'binary(path1_run_start)'`»). It is fired with `--timeout 1800`
+named, the same as entry 16's own `timeout = 1800`, which governs each firing (gate-contract §Control). It holds when
+the case reads `failed` with the edit in. Entry 15, the whole layer, stays step 21's entry.
 
 ```toml
 [[gate]]
@@ -377,7 +418,16 @@
 role = 'e2e'
 timeout = 3600
 expect = ['exit 0', 'contains "ok":true']
-note = 'the contract and e2e suites; atom from .claude/rules/verification-harness.md'
+note = 'the contract and e2e suites; step 21: tui_channel_fds read in the whole layer (Revision 1; its control on entry 16, ctl-fds); atom from .claude/rules/verification-harness.md'
+
+[[gate]]
+run = '''bash scripts/agent-run.sh run --e2e --filter 'binary(tui_channel_fds)' '''
+role = 'e2e'
+new = true
+timeout = 1800
+expect = ['exit 0', 'contains "ok":true']
+baseline = "green — exit 0 in 0.81 s on the tree the stopped implement run left (step 21 not yet done): 1 test across 1 binary, 84 binaries skipped, tui_channel_fds_the_child_holds_nothing_of_violas passed; the red it guards is intermittent (15.2.log), so this green reads the filter's selection, not the race"
+note = 'ctl-fds, step 21 (Revision 1, inputs#I7 item 1): the e2e harness layer filtered to the tui_channel_fds binary alone, the guard own cases; the fds-held-red control is fired on it; atom from .claude/rules/verification-harness.md'
 
 [[gate]]
 run = '''test -s .andromeda/cache/chunks/2026-11-24-viola-up/evidence/r4.md && grep -q '^r4: green phase=windows ' .andromeda/cache/chunks/2026-11-24-viola-up/evidence/r4.md && grep -c '^r4: green phase=after ' .andromeda/cache/chunks/2026-11-24-viola-up/evidence/r4.md'''
@@ -482,8 +532,8 @@
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
@@ -504,11 +554,21 @@
   and exit 1, starting nothing, and the script holds no credential path — the two `r4b-refuses` entries.
 - (output) After the lines, one `hint:` line when any reads `not-live` (founder-forks U7, U8, PROVISIONAL) — the
   `up_*` output cases.
+- (e2e guard, Revision 1) The wrapped child holds nothing of viola's once its launch settles:
+  `tui_channel_fds_the_child_holds_nothing_of_violas` reads green in entry 15 (the whole layer) and in entry 16
+  (`ctl-fds`, its binary alone) with its assertions unchanged, a home file the fake opens only during its own launch
+  waited out inside the case's 2 s deadline; the `fds-held-red` control holds on entry 16 (a fake keeping `hooks.json`
+  open reads the case red) — step 21 (test-plan · decision/claude-child-holds-nothing-of-viola; `inputs#I6` items
+  1–2, `inputs#I7` item 1).
 
 ## Provenance
 - `inputs#I1` — the directive (scope v2-12, nothing live, the forks' away form, the measurements, the live readings).
 - `inputs#I2` — the overseer's review of card 1 (T1–T7 answered; the live root, R4b's refusal and U6, R4a's stand-in
   agent, U7's new words, R4a's workspaces as grant parameters, U8).
+- `inputs#I5` — the word that opened Revision 1 (the invocation, the overseer for the away operator); `inputs#I6` —
+  the overseer's revision relay (the `tui_channel_fds` red folded as one step, its control, U2 answered by the
+  operator, the card's form); `inputs#I7` — the overseer's review of the revision's card 1 (the control on an entry
+  filtered to its binary; no research for this revision).
 - Rows: security-plan · decision/open-terminal-direct-spawn, decision/run-window-flags-checked,
   decision/window-record-kept-to-snapshot-and-event-log, decision/window-step-moves-own-terminal-window,
   decision/lineage-parent-authenticates-nothing, decision/list-scan-checks-before-open, decision/claude-agents-read-fail-open;
@@ -572,11 +632,12 @@
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
```

## Directive item-11 block (`inputs#I1` item 11; tool lines verbatim)
- **The union's per-master lines and the `rows-union` line:** none this run. P2 does not run in a revision, and «no
  research» is now the word (`inputs#I7` item 2). The take-up's lines stand as card 1 of run
  `2026-11-24T04-17-54Z-phase` showed them.
- **Is every cited row in the union or its standing list:** yes. The one row newly cited, test-plan ·
  `decision/claude-child-holds-nothing-of-viola` (`dec-huukjbac r1`), is in the take-up's `tests-rows.md` (1 hit). The
  fold cites no new row.
- **`[RETYPED]` lines and re-spawns:** none.
- **Non-zero exits, refusals, `UNPARSED` / `INDETERMINATE` / `NOT DERIVED` since card 1:** none.
  - `rec.py put` landed plan rev 9 (tx 4548), plan rev 10 (tx 4549) and research rev 5 (tx 4550).
  - `gate.py run --entry 16`: `entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25`, exit 0.
  - `gate.py scope --sync`: `scope sync: PASS · delta +1 −0 · research rev 5 · derived rev 3`.
  - `gate.py run --dry-run`: exit 0, 26 entries.
  - `gate.py baseline`: `entries 26 · seen green 5 · not seen 0 · new (key) 12 · leg 9`.
  - `planlint`: `hits: 0`. Check 12 (d) is `6 of 8`; the acceptance line's «assertions unchanged» now also names
    `ctl-fds` and was read true.
  - `inputs.py verify`: `7 entries — unchanged 4 · … · n/a 3 · uncited 0 · unparsed 0`.
  - `gate.py scope --lists-only`: `parsed 20 items`.
  - Card 1's lines stand as written there.
- **Places a letter was wrong or silent:**
  1. Card 1 item 1 (the row read's skip) is closed by `inputs#I7` item 2.
  2. Card 1 item 2 no longer applies: entry 16 carries its own `timeout = 1800`, and the named `--timeout 1800`
     matches it.
  3. The letter is silent on renumbering when an entry is inserted mid-block. The insertion moved the gated entry
     16 → 17 and the operator entries 17–25 → 18–26. The plan's one prose reference was renumbered and says so; the
     matrix claim names the gate text, not a number.
  4. Card 1 items 3 and 4 stand as written there: section H2 / step 21 ahead of Close's step 20, and this run dir's
     name running about 90 s ahead of the clock.

## Beyond the plan
- none (standing — `.andromeda/runs/2026-11-24T04-17-54Z-phase/p5-card-2.md`: `none`)

Apply? (yes / review / cancel)
