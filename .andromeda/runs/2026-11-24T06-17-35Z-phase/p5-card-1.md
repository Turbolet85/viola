# P5 review card 1 (Revision 2) — 2026-11-24-viola-up («`viola up`») · plan rev 12 · research rev 7 · scope rev 4

**Read the plan:** `.andromeda/cache/docs/2026-11-24-viola-up/plan.md` (rendered from `doc:2026-11-24-viola-up/plan` rev
12). No plan stands in a chunk's folder: writing in that file changes no record until a `put`. A diff of the chunk's
records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-24-viola-up`.

**Why this revision:** the operator pass of implement run `2026-11-24T05-58-30Z-implement` stopped red at `pre-push`.
Every test passed (4320 of 4320), but the coverage merge refused a cut-short fake-agent profile. The overseer's word
(`inputs#I12`, a delegate's, technical, the operator away; opened by
`.andromeda/runs/2026-11-24T06-17-35Z-phase/relay-1.md`): fold it into this chunk as one added step, nothing else of
the plan moving, no research.

**The added step (22):** one shared teardown, `tests/support/terminal.rs`, used by `cli_open`'s `Terminal` guard and
`cli_up`'s `Terminals` guard.
- **Why today's kill lands inside an exit** (read from the code):
  - Both guards SIGKILL each recorded process in `recorded_tree`'s order: the wrapper, then its child, the foot last.
  - Killing the wrapper closes its PTY. The fake `claude` reads EOF and starts its own exit. The stand-in foot sees its
    child end and exits too.
  - Both are the fake agent's binary, and the same loop SIGKILLs them microseconds later, inside their exit-time
    profile write.
  - That matches the census: every refused profile is the fake agent's, and cases with no terminal read 0.
- **The mend:** end the tree from its leaves.
  - A leaf whose parent still runs is SIGKILLed first. Nothing has touched it, so no exit is under way.
  - Every parent then gets a bounded wait to end by itself: one `WITHIN` deadline (12 s, the bound `gone_within`
    already holds, under the 20 s lint).
  - SIGKILL stays only as the last resort at that deadline.
  - Start-time checks stay, so a pid whose start time moved is still refused, and the guard still ends everything it
    started.
  - No assertion, case, merge setting or threshold moves.
- **Acceptance = the census (entries 27–37, appended at the block's end so no earlier number moves):**
  - Entry 27 builds the instrumented binaries.
  - Entries 28–36 are the nine cases that refused profiles: this chunk's seven and `cli_open`'s two. Each must read
    `refused 0` at 192 × 16.
  - Entry 37 is the no-terminal control (`up_dry_run_…`).
  - They are gate entries: the census's own exit and last line carry the number. Each census takes 16–60 s
    (`timeout = 900`); entries 27 and 30 build, so they get `timeout = 3600`.
- **The old-kill reading (item 4):** a one-shot `gate.py control` on entry 30 with the bounded wait removed.
  - The census prints no runner result line, so the tool's word reads `INDETERMINATE: … no result line` by
    construction.
  - The reading is the removed log's `refused` count, copied to `evidence/old-kill.md`. It is a rate, never an atom.
- **Re-entry:** step 22 alone, entries 27–37 and `old-kill`, the whole layers (13–15), `pre-push` (18) once, then the
  operator pass WITHOUT any R4 entry.

**Baselines (fired now, pre-step tree):**

| Entry | Case | Reading |
|---|---|---|
| 27 | build | green, 22 of 22 passed, 6.9 s (cached) |
| 28 | `up_reopens_…` | red: refused 2 / 5308 |
| 29 | `up_opens_a_hand_started_…` | red: refused 7 / 2151 |
| 30 | `up_skips_…` | red: refused 7 / 2075 |
| 31 | `up_leaves_out_…` | red: refused 5 / 4345 |
| 32 | `up_names_no_session_…` | red: refused 7 / 5532 |
| 33 | `up_json_…` | red: refused 14 / 3840 |
| 34 | `up_writes_neither_…` | red: refused 8 / 1479 |
| 35 | `open_spawns_…` | red: refused 9 / 1274 |
| 36 | `open_records_…` | red: refused 5 / 1489 |
| 37 | `up_dry_run_…` (control) | green: refused 0 / 3137 |

All 1920 runs passed. The stop reproduces independently of the stopped run's census.

**Side effect on disk (step 22's own order, done now because entry 27's baseline would have cleaned it):** the pre-push's
corrupt profile was moved to `target/profraw-census/pre-push-3061597/viola-3061597-5781154654293742489_29.profraw`,
kept. The census dirs under `target/profraw-census/` are new and git-ignored.

Research: lists synced to the revision's delta (+3 −0), rev 7; not re-derived since the take-up's rev 3.

## Claims
matrix#v2-12 stays claimed GATED on R4. The revised plan still proves it, its concretization is unchanged, and nothing
is re-claimed or un-claimed. The gated entry is still entry 17.

## The plan's diff against rev 10
```diff
--- plan rev 10
+++ plan rev 12
@@ -15,6 +15,14 @@
   read its expect on the final tree; one red open outside the diff, the e2e layer's `tui_channel_fds` case). One step
   is added, step 21; implement re-enters there, does it alone, then re-reads the whole layers and goes on to the
   operator pass WITHOUT any R4 entry (`inputs#I6` item 4).
+- **Revision 2** (run `2026-11-24T06-17-35Z-phase`, `inputs#I12`; the opening word
+  `.andromeda/runs/2026-11-24T06-17-35Z-phase/relay-1.md`): steps 1–21 stand done (implement runs
+  `2026-11-24T05-12-02Z-implement` and `2026-11-24T05-58-30Z-implement`, the latter's outcome green, then its operator
+  pass stopped red at `pre-push`: every test passed, the coverage merge refused a cut-short fake-agent profile; the
+  census `evidence/profraw-census.md`). One step is added, step 22; implement re-enters there and does it alone, then
+  the census entries (27–37) and the `old-kill` reading, the whole layers (entries 13–15), `pre-push` once, then the
+  operator pass WITHOUT any R4 entry (`inputs#I12` item 6). Every fork, answer, claim and R4 entry stands as approved
+  at rev 10 (item 5).
 
 ## Goal
 `viola up` re-opens, after a reboot, every instance of the home that was live before the host went down: each is
@@ -238,6 +246,49 @@
     - Implement re-enters here (Metadata, Revision 1): step 21 alone, entry 16 (`ctl-fds`) and the `fds-held-red`
       control on it, the whole layers (entries 13–15), then the operator pass with no R4 entry fired.
 
+### H3. One teardown for a stand-in terminal's tree — `tests/support/terminal.rs` (new), `tests/support/mod.rs`, `tests/cli_open.rs`, `tests/cli_up.rs` (Revision 2; keeps tests · decision/claude-child-holds-nothing-of-viola's sibling discipline: no assertion moves)
+22. One shared teardown, `tests/support/terminal.rs` (declared in `tests/support/mod.rs` beside `tied`), ends a
+    stand-in terminal's recorded tree; `cli_open`'s `Terminal` guard (tests/cli_open.rs:157-183) and `cli_up`'s
+    `Terminals` guard (tests/cli_up.rs:294-319) each keep how they find their roots (`foot-pid`, `foot-spawns.ndjson`)
+    and hand the recorded tree to it in place of their own kill loops (`inputs#I12` item 1).
+    - **Why the present kill lands inside an exit** (scope §Revision 2, closed from the code): both loops SIGKILL every
+      recorded member in `recorded_tree`'s order (tests/support/tied.rs:46-70) — the foot's descendants as found (the
+      wrapper, then its child), the foot pushed last. The wrapper's kill closes its PTY; the fake agent playing
+      `claude` reads EOF and runs its own exit (src/bin/viola-fake-agent.rs:1417, :1471-1474), and the stand-in foot,
+      whose `child.status()` returns when the wrapper dies, exits with its code (src/bin/viola-fake-agent/foot.rs:41-53).
+      The same loop then SIGKILLs both, microseconds later, inside the exit-time profile write the fake's own SIGHUP
+      note names (src/bin/viola-fake-agent.rs:1279-1283). Both are the fake agent's binary: the census's every refused
+      profile carries its signature (5781154654293742489, 4096–32232 B of a whole 32576 B), and the cases that spawn no
+      terminal or never reach the guard read 0 refused.
+    - **The end that never kills an exiting process:** the tree is ended from its leaves. A member is signalled only
+      while nothing of the tree has yet ended around it: a leaf whose recorded parent still runs, with its own start
+      time unchanged, is SIGKILLed first (no exit of its own is under way, since nothing above or below it has been
+      touched — a SIGKILLed process writes no profile, so nothing is cut). Every other member — a parent whose recorded
+      children are gone, or a member whose recorded parent is already gone — is given a bounded wait to end by
+      itself (foot and the wrapper end on their child's end, the fake agent on its PTY's EOF), the next level up
+      waited on only once its children are gone. The wait is ONE deadline for the whole teardown, `WITHIN`
+      (tests/support/watch.rs:16, 12 s, the bound `gone_within` already holds, below the 20 s floor
+      `tests/contract_lints.rs` reads, `MUTANTS_FLOOR_SECS`), reading with `std::thread::yield_now` between reads as
+      `gone_within` does; at that deadline a member still running with its recorded start time is SIGKILLed, the kill
+      kept only as the last resort for a process that did not end. `gone_within` then runs as today and keeps its
+      panic naming the survivors.
+    - **What stays:** the guard still ends everything it started; every signal still goes only to a recorded pid whose
+      OS start time reads the one recorded (`process_start`, tests/support/home.rs:40), so a pid whose start time
+      moved is refused; the roots are read as today; no assertion of either binary moves, and no case changes
+      (`inputs#I12` items 1 and 5). The coverage merge, its failure mode and every gate's threshold stay as they are
+      (item 3).
+    - **Its acceptance is the census** (`inputs#I12` item 2), entries 27–37 below: after the step, every case that
+      refused a profile in `evidence/profraw-census.md` — this chunk's seven and `cli_open`'s two — reads `refused 0`
+      at the same 192 × 16, and `up_dry_run_prints_the_plan_and_spawns_nothing`, a no-terminal case, is re-read as the
+      unchanged control. Before the build entry, the pre-push's corrupt profile
+      (`target/llvm-cov-target/viola-3061597-5781154654293742489_29.profraw`) is moved by its full name into
+      `target/profraw-census/pre-push-3061597/`, kept: the build's clean of old profiles would otherwise remove it.
+    - **Its old-kill reading** (`inputs#I12` item 4) is the one-shot `old-kill` below, a recorded reading with its
+      count, never a failing atom (it is a rate).
+    - Implement re-enters here (Metadata, Revision 2): step 22 alone, entries 27–37 and the `old-kill` reading, the
+      whole layers (entries 13–15), `pre-push` (entry 18) once, then the operator pass with no R4 entry fired
+      (`inputs#I12` item 6).
+
 ### I. Close
 20. `cargo fmt`, the block below green, the report written.
 
@@ -246,6 +297,7 @@
 - `src/cmd/up.rs` — the verb
 - `tests/cli_up.rs` — its integration cases over the fakes
 - `scripts/r4-up.sh` — the gated live readings R4a and R4b, with a rehearse form
+- `tests/support/terminal.rs` — step 22: the one teardown of a stand-in terminal's tree, from its leaves (Revision 2)
 ### Files to modify
 - `src/cmd/mod.rs` — `Command::Up`, its dispatch, the `HELP_TEMPLATE` `setup:` row
 - `src/cmd/revive.rs` — hidden `--origin` feeding `Launch.window` with the recorded workspace
@@ -264,6 +316,9 @@
 - `tests/cli_machine_contract.rs` — `up --json` and the new flags in the controls table
 - `schemas/diag-line.v1.json` — the `process-exit` detail word `boot-time-unread`
 - `tests/tui_channel_fds.rs` — step 21: the fd table read again while the fake's launch has a home file open (Revision 1)
+- `tests/support/mod.rs` — step 22: `pub mod terminal;` (Revision 2)
+- `tests/cli_open.rs` — step 22: the `Terminal` guard hands its tree to the shared teardown (Revision 2)
+- `tests/cli_up.rs` (new in this chunk) — step 22: the `Terminals` guard hands its tree to the shared teardown (Revision 2)
 
 ## Test Commands
 The block runs the formatter and the linter; the root binaries' units and the filtered new cases (0 on the untouched
@@ -304,6 +359,32 @@
 named, the same as entry 16's own `timeout = 1800`, which governs each firing (gate-contract §Control). It holds when
 the case reads `failed` with the edit in. Entry 15, the whole layer, stays step 21's entry.
 
+**The census entries (step 22, Revision 2)** are gate entries, not recorded readings: each is one call of
+`scripts/profraw-census.sh --case` whose own verdict is its exit (0 only when every run passed and no profile was
+refused) and its last line (`profraw-census: runs 192 · passed 192 · profiles P · refused R`), so the tool reads the
+number itself and the trail keeps it. They are appended at the block's end as entries 27–37, so no earlier entry
+number moves. Entry 27 builds the instrumented binaries the census reads (`target/llvm-cov-target/`), running the two
+guarded binaries once; entries 28–36 are the nine cases that refused (this chunk's seven, `cli_open`'s two); entry 37 is
+the no-terminal control. A measurement of minutes: one case read about 27 s at 192 × 16 (the census
+`2026-11-24T06:08:38Z..06:15:54Z` for 16 cases), so each census entry carries `timeout = 900`; entry 27 carries 3600,
+and entry 30 (`up_skips_a_live_and_a_stale_instance`, the case that refused most, 16) builds first in the same
+statement, so it reads alone in the tool's clone. Every entry removes `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` from
+its environment, as the stopped run's census did: a test spawning viola under the session's own name would resolve its
+home. Their baselines are the stopped census's red on the pre-step tree (P5's baseline runs).
+
+**The `old-kill` reading** (`inputs#I12` item 4) is a one-shot `gate.py control` fired on entry 30 with `--timeout
+3600`: its edit, in `tests/support/terminal.rs`, removes the bounded wait so every recorded member is SIGKILLed at once
+as before step 22. The tool fires entry 30 in its clone with the wait in place (it must read green, `refused 0`), then
+with it removed. The census prints no runner result line, so the tool's word reads `INDETERMINATE: case
+up_skips_a_live_and_a_stale_instance no result line` by construction; the reading is the removed log's last line,
+copied into `evidence/old-kill.md` with its count — `refused` > 0 is the expected reading, and a `refused 0` is
+recorded as read, a rate's draw, never a red of the chunk and never re-fired for a number. It is not a row of the
+controls table above, whose word must read `holds`.
+
+| id | File | The edit removes | Case | Entry |
+|---|---|---|---|---|
+| `old-kill` | `tests/support/terminal.rs` | the bounded wait: every recorded member SIGKILLed at once, in `recorded_tree`'s order, as the two guards did before step 22 | `up_skips_a_live_and_a_stale_instance` | entry 30 |
+
 ```toml
 [[gate]]
 run = 'cargo fmt --all --check'
@@ -513,6 +594,104 @@
 timeout = 600
 expect = ['exit 0', 'contains r4: green phase=after']
 note = 'step 19, R4b after the reboot: REFUSES as built until the same U6 revision; the gated entry needs this line and R4a line both; atom from step 18'
+
+[[gate]]
+run = '''env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN cargo llvm-cov nextest --no-report --workspace --features viola/fake-agent -E 'binary(cli_up) | binary(cli_open)' '''
+role = 'build'
+new = true
+baseline = "green — exit 0 in 6.9 s on the tree the stopped implement run left (step 22 not yet done), phase run 2026-11-24T06-17-35Z: the instrumented build current (cached), 22 tests run across cli_up and cli_open, 22 passed"
+timeout = 3600
+note = 'step 22 (Revision 2): the instrumented test binaries the census entries 28-37 read, under target/llvm-cov-target/, the two guarded binaries run once; the merge and its thresholds untouched (no report)'
+
+[[gate]]
+run = 'env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN bash scripts/profraw-census.sh --case cli_up up_reopens_each_pre_boot_instance_parents_first 192 16'
+role = 'probe'
+new = true
+baseline = "red — exit 1 in 45.07 s on the tree the stopped implement run left (step 22 not yet done), phase run 2026-11-24T06-17-35Z: runs 192 · passed 192 · profiles 5308 · refused 2"
+timeout = 900
+expect = ['exit 0', 'contains runs 192 · passed 192 ·', 'contains · refused 0']
+note = 'step 22 census (inputs#I12 item 2): refused 5 of 5333 before it; atom from scripts/profraw-census.sh header and .claude/docs/commands.md (last line profraw-census: runs N · passed N · profiles P · refused R)'
+
+[[gate]]
+run = 'env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN bash scripts/profraw-census.sh --case cli_up up_opens_a_hand_started_instance_unplaced 192 16'
+role = 'probe'
+new = true
+baseline = "red — exit 1 in 21.26 s on the tree the stopped implement run left (step 22 not yet done), phase run 2026-11-24T06-17-35Z: runs 192 · passed 192 · profiles 2151 · refused 7"
+timeout = 900
+expect = ['exit 0', 'contains runs 192 · passed 192 ·', 'contains · refused 0']
+note = 'step 22 census: refused 6 of 2152 before it; atom from scripts/profraw-census.sh header and .claude/docs/commands.md'
+
+[[gate]]
+run = '''env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN cargo llvm-cov nextest --no-report --workspace --features viola/fake-agent -E 'binary(cli_up)' && env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN bash scripts/profraw-census.sh --case cli_up up_skips_a_live_and_a_stale_instance 192 16'''
+role = 'probe'
+new = true
+baseline = "red — exit 1 in 30.86 s on the tree the stopped implement run left (step 22 not yet done), phase run 2026-11-24T06-17-35Z: runs 192 · passed 192 · profiles 2075 · refused 7"
+timeout = 3600
+expect = ['exit 0', 'contains runs 192 · passed 192 ·', 'contains · refused 0']
+note = 'step 22 census: refused 16 of 2147 before it; builds first so it reads alone in the control clone (the old-kill reading fires on it); atom from scripts/profraw-census.sh header and .claude/docs/commands.md'
+
+[[gate]]
+run = 'env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN bash scripts/profraw-census.sh --case cli_up up_leaves_out_an_instance_ended_on_purpose_or_this_boot 192 16'
+role = 'probe'
+new = true
+baseline = "red — exit 1 in 37.86 s on the tree the stopped implement run left (step 22 not yet done), phase run 2026-11-24T06-17-35Z: runs 192 · passed 192 · profiles 4345 · refused 5"
+timeout = 900
+expect = ['exit 0', 'contains runs 192 · passed 192 ·', 'contains · refused 0']
+note = 'step 22 census: refused 2 of 4366 before it; atom from scripts/profraw-census.sh header and .claude/docs/commands.md'
+
+[[gate]]
+run = 'env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN bash scripts/profraw-census.sh --case cli_up up_names_no_session_cwd_missing_and_session_live 192 16'
+role = 'probe'
+new = true
+baseline = "red — exit 1 in 59.48 s on the tree the stopped implement run left (step 22 not yet done), phase run 2026-11-24T06-17-35Z: runs 192 · passed 192 · profiles 5532 · refused 7"
+timeout = 900
+expect = ['exit 0', 'contains runs 192 · passed 192 ·', 'contains · refused 0']
+note = 'step 22 census: refused 2 of 5545 before it; atom from scripts/profraw-census.sh header and .claude/docs/commands.md'
+
+[[gate]]
+run = 'env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN bash scripts/profraw-census.sh --case cli_up up_json_is_one_document_of_closed_words 192 16'
+role = 'probe'
+new = true
+baseline = "red — exit 1 in 39.01 s on the tree the stopped implement run left (step 22 not yet done), phase run 2026-11-24T06-17-35Z: runs 192 · passed 192 · profiles 3840 · refused 14"
+timeout = 900
+expect = ['exit 0', 'contains runs 192 · passed 192 ·', 'contains · refused 0']
+note = 'step 22 census: refused 14 of 3874 before it; atom from scripts/profraw-census.sh header and .claude/docs/commands.md'
+
+[[gate]]
+run = 'env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN bash scripts/profraw-census.sh --case cli_up up_writes_neither_snapshot_nor_stamps 192 16'
+role = 'probe'
+new = true
+baseline = "red — exit 1 in 15.71 s on the tree the stopped implement run left (step 22 not yet done), phase run 2026-11-24T06-17-35Z: runs 192 · passed 192 · profiles 1479 · refused 8"
+timeout = 900
+expect = ['exit 0', 'contains runs 192 · passed 192 ·', 'contains · refused 0']
+note = 'step 22 census: refused 9 of 1484 before it; atom from scripts/profraw-census.sh header and .claude/docs/commands.md'
+
+[[gate]]
+run = 'env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN bash scripts/profraw-census.sh --case cli_open open_spawns_the_terminal_with_the_closed_argv 192 16'
+role = 'probe'
+new = true
+baseline = "red — exit 1 in 16.06 s on the tree the stopped implement run left (step 22 not yet done), phase run 2026-11-24T06-17-35Z: runs 192 · passed 192 · profiles 1274 · refused 9"
+timeout = 900
+expect = ['exit 0', 'contains runs 192 · passed 192 ·', 'contains · refused 0']
+note = 'step 22 census, cli_open control of the stopped run: refused 9 of 1289 before it; atom from scripts/profraw-census.sh header and .claude/docs/commands.md'
+
+[[gate]]
+run = 'env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN bash scripts/profraw-census.sh --case cli_open open_records_the_window_found_by_the_terminal_pid 192 16'
+role = 'probe'
+new = true
+baseline = "red — exit 1 in 15.91 s on the tree the stopped implement run left (step 22 not yet done), phase run 2026-11-24T06-17-35Z: runs 192 · passed 192 · profiles 1489 · refused 5"
+timeout = 900
+expect = ['exit 0', 'contains runs 192 · passed 192 ·', 'contains · refused 0']
+note = 'step 22 census, cli_open control of the stopped run: refused 6 of 1481 before it; atom from scripts/profraw-census.sh header and .claude/docs/commands.md'
+
+[[gate]]
+run = 'env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN bash scripts/profraw-census.sh --case cli_up up_dry_run_prints_the_plan_and_spawns_nothing 192 16'
+role = 'probe'
+new = true
+baseline = "green — exit 0 in 28.4 s on the tree the stopped implement run left (step 22 not yet done), phase run 2026-11-24T06-17-35Z: runs 192 · passed 192 · profiles 3137 · refused 0"
+timeout = 900
+expect = ['exit 0', 'contains runs 192 · passed 192 ·', 'contains · refused 0']
+note = 'step 22 census, the unchanged no-terminal control (inputs#I12 item 2): refused 0 of 3131 before it; atom from scripts/profraw-census.sh header and .claude/docs/commands.md'
 ```
 
 ## Acceptance Criteria
@@ -560,6 +739,17 @@
   waited out inside the case's 2 s deadline; the `fds-held-red` control holds on entry 16 (a fake keeping `hooks.json`
   open reads the case red) — step 21 (test-plan · decision/claude-child-holds-nothing-of-viola; `inputs#I6` items
   1–2, `inputs#I7` item 1).
+- (teardown, Revision 2) A stand-in terminal's tree is ended from its leaves by the one shared teardown of step 22,
+  never killing a process already in its own exit: each case that refused a profile in `evidence/profraw-census.md`
+  reads `refused 0` at 192 × 16 — `up_reopens_each_pre_boot_instance_parents_first`,
+  `up_opens_a_hand_started_instance_unplaced`, `up_skips_a_live_and_a_stale_instance`,
+  `up_leaves_out_an_instance_ended_on_purpose_or_this_boot`, `up_names_no_session_cwd_missing_and_session_live`,
+  `up_json_is_one_document_of_closed_words`, `up_writes_neither_snapshot_nor_stamps`,
+  `open_spawns_the_terminal_with_the_closed_argv` and `open_records_the_window_found_by_the_terminal_pid` (entries
+  28–36) — and the no-terminal control `up_dry_run_prints_the_plan_and_spawns_nothing` reads `refused 0` unchanged
+  (entry 37); every census run passes; no assertion of `cli_open` or `cli_up` moves, and both guards still signal
+  only a recorded pid whose start time reads the recorded one. The `old-kill` reading (the bounded wait removed) is
+  recorded with its count in `evidence/old-kill.md`, a rate, not an atom (`inputs#I12` items 1, 2 and 4).
 
 ## Provenance
 - `inputs#I1` — the directive (scope v2-12, nothing live, the forks' away form, the measurements, the live readings).
@@ -569,6 +759,11 @@
   the overseer's revision relay (the `tui_channel_fds` red folded as one step, its control, U2 answered by the
   operator, the card's form); `inputs#I7` — the overseer's review of the revision's card 1 (the control on an entry
   filtered to its binary; no research for this revision).
+- `inputs#I12` — the overseer's Revision 2 relay (the `pre-push` red folded as one step for both terminal guards,
+  its census acceptance, no change to the merge, the old-kill reading, every approval standing, the card's form);
+  the word that opened it, `.andromeda/runs/2026-11-24T06-17-35Z-phase/relay-1.md` («no research»); `inputs#I11` —
+  the overseer's «measure first» relay at the stopped pass, whose census (`evidence/profraw-census.md`) gives step
+  22's before-numbers and the form of entries 28–37.
 - Rows: security-plan · decision/open-terminal-direct-spawn, decision/run-window-flags-checked,
   decision/window-record-kept-to-snapshot-and-event-log, decision/window-step-moves-own-terminal-window,
   decision/lineage-parent-authenticates-nothing, decision/list-scan-checks-before-open, decision/claude-agents-read-fail-open;
```

## Directive item-11 block (`inputs#I1` item 11; tool lines verbatim)
- **The union's per-master lines and the `rows-union` line:** none this run. P2 does not run in a revision, and «no
  research» is the word (`relay-1.md`; `inputs#I12` item 5). The take-up's lines stand as card 1 of run
  `2026-11-24T04-17-54Z-phase` showed them.
- **Is every cited row in the union or its standing list:** yes. The revision cites no new master row.
- **`[RETYPED]` lines and re-spawns:** none.
- **Non-zero exits, refusals, `UNPARSED` / `INDETERMINATE` / `NOT DERIVED`:**
  - `gate.py scope --sync`, first two firings: `scope sync: FAIL 3 · delta +3 −0`. I first entered the added files in
    research's `## Files inspected`; the sync reads `## New files to create` / `## Files to modify`. Third firing:
    `scope sync: PASS · delta +3 −0 · research rev 7 · derived rev 3`.
  - `gate.py run --only 28,…,37`: `entries 37 · green 1 · red 9 (28,29,30,31,32,33,34,35,36)`. These are the expected
    baseline reds, recorded as `baseline` keys.
  - Two Bash-guard refusals: a heredoc to a file, and a `cd` outside the root. Each was re-issued in the guard's form.
    No pipeline tool output was filtered.
  - Landed: `rec.py put` scope rev 4 (tx 4564), plan rev 11 (tx 4566), research rev 6–7 (tx 4567–4568), plan rev 12
    (tx 4570).
  - Checks:
    - `gate.py run --entry 27`: `entries 37 · green 1`.
    - `gate.py run --dry-run`: exit 0, 37 entries.
    - `gate.py baseline`: `entries 37 · seen green 5 · not seen 0 · new (key) 23 · leg 9`.
    - `planlint`: `hits: 0`. Check 12 (d) is `10 of 12`, and each listed claim was read true. The `tests/cli_open.rs`
      «unchanged» lines (137, 811) speak of its `foot-argv` read, which step 22 leaves alone.
    - `inputs.py verify`: `12 entries — unchanged 7 · … · n/a 5 · uncited 0`.
    - `gate.py scope --lists-only`: `parsed 23 items (new 4 · modify 19 · derived 0)`.
- **Places a letter was wrong or silent:**
  1. SKILL.md §Revision item 4 says an added file goes into research's `## Files inspected`. The sync tool reads the
     two touchpoint lists, so I entered the files in both.
  2. The letters give no control word for an entry whose runner prints no result line. The census is such an entry,
     so the `old-kill` reading's tool word is `INDETERMINATE` by construction. The plan names that in advance and
     reads the count from the removed log (`inputs#I12` item 4: a reading, not an atom).
  3. Step 22 orders the corrupt pre-push profile moved aside before the build entry. The baseline run of entry 27 came
     first, so the move was made now, in this phase.

## Beyond the plan
- none (standing — `.andromeda/runs/2026-11-24T05-47-00Z-phase/p5-card-2.md`: `none`)

Apply? (yes / review / cancel)
