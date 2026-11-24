# P5 review — 2026-11-24-adopt-s-resume-folder «Adopt's resume folder»

**Open the plan in an editor:** `.andromeda/cache/docs/2026-11-24-adopt-s-resume-folder/plan.md`. It is a render of
`doc:2026-11-24-adopt-s-resume-folder/plan` rev 3.
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-24-adopt-s-resume-folder`
  is a diff of the chunk's records.

## The finding that reshapes the entry (research M3)
The capability ledger's relied row `resume-session` already resumes a session from a folder other than the one its
maker used:
- verify's Run E resumes from `<cwd>/.viola-verify-<pid>-resume/` the session Run B made in
  `<cwd>/.viola-verify-<pid>/`, which is already removed;
- recorded on 2.1.328 (`fixtures/claude/2.1.328/SessionStart.resume-1.json`) and stamped pass on 2.1.331.

So **hypothesis A is falsified at the measured grain**. The overseer found no transcript for the R5a ids anywhere
under `~/.claude/projects/`, which A alone cannot explain. What remains is B (never kept on disk) or a new **C**: the
session was kept under another Claude config dir set in the operator's terminal. The zero-start readings and the one
live check that tell them apart are priced in `founder-forks.md`'s P3 block (M4), for the overseer to put to him.

## Goal
An adopt whose resumed child ends before any `session-start` reaches the wrapper ends **exit 1**:
- the fixed pair `unable: <name>'s session did not resume` /
  `hint: run viola adopt from the folder the session was made in, under the claude config it was made with`;
- under `--json`, `{"v":1,"error":"not-resumed","detail":null}`;
- one `process-exit{subject:"self", exit_code:1, detail:"not-resumed"}` in the run log.

Today it ends exit 0 and says nothing. The spawn folder stays the caller's (fork 1 (i)). The R5 rig adds `start=` to
`r5a` and a rehearse-only `r5c`.

## Forks of his — the plan is built on the PROVISIONAL arms (`founder-forks.md`, P3 block)
1. **Fork 1, the folder:**
   - (i) is taken: type `adopt` in the session's folder, with the named hint.
   - (ii) a `--cwd` flag, and (iii) a projects-store lookup, are each a boundary widening, not taken.
2. **Fork 2, the not-resumed surface:**
   - «At once» means before any `session-start` (no time bound).
   - Word `not-resumed`, exit 1.
   - The pair is written after the child released the terminal. This departs from «viola writes nothing past it».
3. **Fork 3, hypothesis C:** the hint names the claude config generically. `CLAUDE_CONFIG_DIR` is not kept by default.

His answers come through the overseer. An answer that differs returns the plan to P4 before the word.

## Top acceptance criteria
- A not-resumed adopt exits 1 with exactly that pair, or under `--json` exactly that document, stderr empty (entry 5).
- Its run log holds the `not-resumed` self exit and the child's exit, schema-valid, with no session id (entry 5).
- A resumed adopt still ends exit 0. The name a not-resumed start left is taken over (entry 4, step 6).
- `not-resumed` is in the exit table with its own hint, and its writer is named (entry 4).
- The rehearsal prints `r5a … start=ok`, `r5b` unchanged, and `r5c: green not-resumed=yes started=child ended=yes`
  (entry 7, fake agent only).

## Touchpoints
`src/cmd/run.rs` (a pump sibling; `pump_child` unchanged in behaviour) · `src/cmd/adopt.rs` · `schemas/diag-line.v1.json`
· `src/bin/viola-fake-agent.rs` · `src/bin/viola-fake-agent/test_cases_opts.rs` · `tests/cli_adopt.rs` ·
`tests/cross_exit_causes.rs` · `tests/contract_error_bodies.rs` · `scripts/r5-adopt.sh`

## Test commands (14 entries)
- **Entries 1-10:** fmt and clippy · the root units · the adopt and table binaries · the new cases by name · the rig's
  parse and rehearsal · the host read · the harness unit and integration layers.
- **Entries 11-14, operator:** pre-push · hygiene · push · `ci.py`.
- **Baselines:**
  - 3 green (1401 passed, 6.07 s); 4 green (75 passed, 9.74 s); 6 green; 8 recorded (host QUIET).
  - 5 red, no tests to run (vacuous until step 6).
  - 7 red on the `start=ok` atom, which the untouched rig does not print.
- **Control:** one remove-the-guard one-shot, `not-resumed`, on entry 5, `--timeout 900`.

## Capabilities
No claim. The pool's planned-unclaimed ids (v2-14, v2-15, v2-28, v2-30, v2-31) are not this chunk's. It serves the
gated `matrix#v2-13`, which stays claimed by `2026-11-24-viola-adopt` and closes at the one R5a its door fires on his
word, after this lands. No concretization preview.

## The directive's item-8 block — verbatim where a tool printed it

**The union** (`rec.py rows-union`):
```
architecture: rows 1045 · packs 11 · picked 252 (153 by every draw) · standing 18 · 70033 B
security-plan: rows 665 · packs 7 · picked 165 (110 by every draw) · standing 11 · 49691 B
design-system: rows 360 · packs 4 · picked 93 (63 by every draw) · standing 5 · 22413 B
layout-templates: rows 237 · packs 3 · picked 51 (37 by every draw) · standing 16 · 10557 B
test-plan: rows 1384 · packs 14 · picked 281 (154 by every draw) · standing 27 · 70864 B
obs-plan: rows 1055 · packs 11 · picked 257 (170 by every draw) · standing 58 · 62572 B
a11y-plan: rows 244 · packs 3 · picked 92 (63 by every draw) · standing 21 · 24413 B
rows-union: 7 master(s) · picked 1191 of 4990 · rule any
```
- **Every cited row is in the union:** yes. Each of the plan's sixteen cited handles was found in its rows file by
  search. The sum is 310,543 B, past 250,000 B, so the rows files were read by search, never whole. The standing
  lists were read by their `S{n}` lines, and the seven history files (103 KB) were not read whole.
- **planlint** (rev 3), whole:
```
planlint v1.25 · 1ced29bd
fence: 14 entries · plan: doc:2026-11-24-adopt-s-resume-folder/plan · research: doc:2026-11-24-adopt-s-resume-folder/re…
check 9 · CI-outcome reads: 1 · leg operator exempt: 1 · Platform issues consulted: none · `none — no runner-only bull…`
check 9 · runner-only scope bullet: not read — the letter's
check 4 (5) · leg = 'live' entries: 0 · Harness rules consulted: none · `none — no live leg in this chunk (the one liv…`
check 4 (5) · leg entries with a content atom: 3 · note without `atom from`: 0
check 4 (13) · operator CI-outcome reads: 1 · not ci.py conclusion: 0
check 4 (13) · job-log reads by pattern: 0
check 4 (14) · base-less git change reads outside the operator and round legs: 0
check 4 (15) · grep → wc -l counts asserting exit 0 and last line 0: 0
check 4 (16) · guards whose exclusions leave a new file in: 0 of 0 with an exclusion
check 4 (18) · entries reading a master's or a base's path the store now holds: 0
check 4 (19) · absence probes against the test names their own plan adds: 0 · probes read 0
check 4 (17) · entries reading a path the wrap writes: 0 of 9 asserting — listed, P5's read
check 4 (17) · a path the wrap writes, read from inside a program an entry runs (a suite, the ledger's tool): not read
check 2 · ledger-note entries (matrix#): 0
check 2 · entries naming no master and no matrix#: 0
check 5 · placeholders outside code: 0
check 12 (c) · step references the plan's steps lack: 0 of 4
check 12 (d) · «unchanged» claims on a name another part of the plan names: 7 of 8 — listed, P5's read
check 4 (4) · criterion evidence paths the plan names nowhere else: 0 of 0
hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12 (c)); 0 is not a pass
```
  - There is no separate arm line; the `fence:` line is its head.
  - Rev 2's run had one HIT, check 2, a non-amendment bullet under Expected amendments, fixed in rev 2.
  - 12 (d) is read here: each «unchanged» claim is on behaviour (`pump_child`'s mapping, the `r5b` line, the live arms),
    none on a file the plan changes.
- **`[RETYPED]` lines** (pick-p2's check), no re-spawn anywhere:
```
tests-pick-p6.d2 · [RETYPED] line 49: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 49
tests-pick-p6.d3 · [RETYPED] line 75: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 75
```
- **Non-zero exits and refusals:**
  - `rec.py put --edit-file` exit 2 at the scope closure («`old: x` and `new: y` come in pairs, one line each»). It
    was redone through render, Edit and `put --expect-rev 2`.
  - The Bash guard refused two `cd`s and one pipe of a tool's output into `head`. Each was re-issued in a passing form.
  - `gate.py baseline` outran the 120 s foreground and was read from its background output.
  - The history check printed 50 `not a marker` rows (amendment ids, `design-history`, `fanout-2026-11-24-adopt-s-resume-folder.json`).
  - No `UNPARSED` / `INDETERMINATE` / `NOT DERIVED` line.
- **Where a letter was wrong or silent:**
  - The P2 letter says each result file is read whole after its check. With the rows past 250,000 B, P4's
    search rule was applied to the history and standing files too: a deviation, recorded in the evolve records.
  - The directive's hypothesis A rested on «not measured»: the ledger's own Run E had measured it.

## Beyond the plan
- none

**Apply? (yes / review / cancel)**
