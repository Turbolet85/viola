# Wrap resume point — 2026-10-05-dialog-rows-and-re-probe

**Stopped after P1, on the operator's word** (context 77.6 %, measured; "run P1 only, write the run dir resume point
naming P2 next, and stop; I clear and resume you").

- **Next: P2 (fan-out → validate → apply → cascade).** Resume through Setup step 2a: `chunk_dir/report.md` exists, so
  the wrap offers resume. Take **resume** in this run dir (`.andromeda/runs/2026-10-05T14-27-34-wrap/`). The report
  stands as written; no `fanout-results.md` exists yet, so P2 fans out fresh.
- **Done in this window:**
  - Setup: one pending record; the basis `75198e5` = the oldest pre-CI commit `43e6245`'s parent; branch
    `build/viola-0.1.0`, 0 ahead of upstream at Setup; the code-graph refresh fired in the background.
  - P1: `gate.py scope` `clean — changed 45 · listed 43 · recorded 2`; `inputs.py verify` 3 entries, all `n/a`
    (messages), all cited; `chunk_dir/report.md` authored; the P1 evolve checkpoint appended (2 records).
  - No spec master, sidecar, route, matrix or state file was touched by this wrap.
- **Tree at stop:** pushed through `c914216` (CI ci#37322552375 green twice, attempts 1-2, 15/15 each). Uncommitted:
  `evidence/ci-rounds.md`, `evidence/operator-pass.md`, `report.md`, this run dir, the implement run dir's trails and
  the friction log. All ride the wrap commit.

## What the resume must carry (the report holds each; restated here because the conversation goes)

### The founder's live rulings
- **At P4 (inputs#I3)** — the founder, live, via the overseer's AskUserQuestion, relayed by the operator, 2026-10-05:
  - **M7 = A, hook answers.** The capture arm prints a product-built decision body, never a key into any dialog; the
    probe's `allow` runs one synthetic `touch` in Run C's own 0700 dir; +2 CLI transcripts per verify under
    `~/.claude/projects/` (the accepted residual class).
  - **Live cap 16**, both installed versions stamped (2.1.288, 2.1.287). Spent: 13 of 16 (step 0 = 3, step 11 = 5 + 5),
    3 spare (`evidence/live-sessions.md`).
  - **Sizing: split off W3d + W6 to "Permission end to end"**, a new Epoch 3 entry right after this one; Epoch 3 stays
    one epoch, now 14 entries.
- **On step 0's STOP 7** — the founder, live, 2026-10-05, via the overseer's AskUserQuestion, relayed by the operator:
  - re-run Run D once (1 spare session, 13 of 16 planned);
  - approve body = PreToolUse `allow` + `updatedInput` set to the tool's own input, unchanged (within architecture S7;
    plan step 0 allowed `dialog.rs` and its `plan_approved` snapshot to change);
  - test `plansDirectory` in Run D's `--settings`, pointing into Run D's own 0700 dir, in the same session;
  - if the approve takes effect AND no file lands in `~/.claude/plans`, continue to code; else STOP.
  - Result: both met (the approve took effect, the PostToolUse came 34 ms after it; `~/.claude/plans/` held 17 files
    before and after). `plansDirectory` also held on 2.1.287 in the record run.
  - Spec facts this lands: S7's approve form (`allow` + `updatedInput`), the plan file kept inside the probe dir.

### The overseer's decisions (founder-delegated, 2026-10-05, relayed by the operator)
- **CI round 0 (red) → option 1: the settle before the Run C / Run D kill**, as Run B already does. Kept; red before
  green in `evidence/run-kill-settle.md` (the fake agent's capped `--stop-receipt-hold-ms`, 4 of 6 → 6 of 6). Then
  option 2: measure the hook-count contention on the runner first; "NOT option 3: the bound stays". Round 2 measured
  no contention (`evidence/ci-rounds.md`).
- **After the measurement → the `verify_window_` class** for the verify-driven tests: no test-side bound on the verify
  call, a nextest per-test kill override sized from the measured floor plus a 3x tail. Implemented as a `ci` profile
  kill: 20 s for the twelve verify-driven binaries, 45 s for `verify_window_` tests. The 7 s rule stays for every other
  test. It "reverses my earlier 'not option 3' only because the measurement shows a designed floor, not a regression":
  recorded in `evidence/verify-window-class.md`. Basis: four interactive runs, seven 300 ms settles, about 1.0 s → 2.1 s
  median on the ubuntu coverage leg; both reds fall in runs with a runner-wide 2.5-3x slow tail. Neither verify change
  (parallel C/D, fewer settles) was made. CI was read twice green (`c914216`).
- **Spec facts for P2:**
  - test-plan (the `verify_window_` class grown to every verify-driven test, its kill sizes, the 7 s bound kept
    elsewhere);
  - architecture / security-plan (the settle before the C/D kill; ended by a kill, never a key).

### The route mint (P5)
- **Mint "Permission end to end" right after this entry** (`working-route.md`, Epoch 3, markerless). It carries:
  - **W3d:** the `permission` kind's end-to-end Path 4 case and its `verification-matrix.json#v1-30` wake witness, over
    the ordinary-tool PermissionRequest fixtures this chunk recorded (`PermissionRequest.permission-1.json` /
    `PostToolUse.permission-1.json` at 2.1.288 and 2.1.287); and the PermissionRequest body for a `question` first
    raised by PermissionRequest — measure it, then build a body or keep `null` with a recorded reason.
  - **W6:** the `tests/cli_answer.rs:9` reword (it still names the owner "the live test (working-route `:84`)").
- It goes ahead of "Local-command and paste-framing rows". Epoch 3 stays one epoch (14 entries). The founder's ruling
  at P4 is the authority, so no trajectory dialogue is needed beyond confirming the placement.

### The desk leftovers (for the overseer desk, the founder's word: leave them)
- Step 0's first Run D plan file under `~/.claude/plans/`: one file, `viola-verify-probe-make-greedy-island.md`, written
  twice (11:53Z).
- The empty, gitignored `.viola-verify-2095228/` at the repository root, from the previous chunk's record runs
  (~10:23Z).
- Carried from the prior handoff: the stray recording home `~/.viola-record-20261004T142325Z`; five Run B transcripts
  and now this chunk's Run C / Run D transcripts under `~/.claude/projects/` (the accepted residual class).

### Other P2/P5/P7 inputs
- The plan's `Expected amendments (wrap)` are dispositioned in the report (sites counted per master).
- CLAUDE.md:43's dated-exception line closes with the architecture amendment (cascade).
- `v1-15` is `implemented` (composite ref); P7.3 flips it to `verified`.
- P7's light gate: the `leg = 'live'` / `round` entries re-verify by `evidence/round-131207Z.txt`, and the `operator`
  entries by `evidence/operator-pass.md` + `ci-rounds.md` (the final HEAD `c914216` green twice). Never re-fire a live
  leg: no session remains planned (3 spare are the founder's).
- Deferred learnings to weigh at P3, from this chunk:
  - `pkill -f` self-match: a recurrence of host-win32.md 2026-09-25;
  - a heredoc to a file refused by the Bash guard: a recurrence of host-win32.md 2026-09-28/29;
  - the settle is a 300 ms quiet grace, not a guarantee;
  - a CI timing red read two-sided across runs' JUnit against tests that never touch the changed code.
