# P5 review — 2026-11-27-epoch-5-cleanup-2 «Epoch 5 cleanup — the three splits»

The plan's render to open in an editor: `.andromeda/cache/docs/2026-11-27-epoch-5-cleanup-2/plan.md` (rev 3). No plan
stands in a chunk's folder; writing in that file changes no record until a `put`. A diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-27-epoch-5-cleanup-2`.

## Goal
Three pure moves (the audit's M2 and M3, carry `cry-vkfw2tqe`): `upped` split three ways (`select` · `plan` · `act`),
the switch's respawn out of `pump.rs` into `pump/restart.rs` with ONE helper taking the request block out of
`Pumping::run` (`inputs#I2`), the window and trust steps out of `run.rs` beside `src/run/window.rs` and
`src/trust_record.rs`; the two over-800 test modules into `test_`-stemmed files. No behaviour, span, line, wait or
order moves; no test body edited.

## Top acceptance criteria
- `sizes` entry last line 0 (now 2: `pump.rs` 2042, `run.rs` 1744); `cx` entry last line 0 (now 2: `upped` 30,
  `Pumping::run` 17); `up-cx` and `pty-cx` record the parts (predicted `upped` 5 · `select` 10 · `plan` 6 · `act` 9;
  `run` and its helper each under 15).
- Cases counted equal: `pump::tests` 33 · viola-pty lib 49 · `cmd::run::tests` + the one moved to `trust_record` 84 ·
  `cmd::up::tests` 23 · root bin 1359 · the profraw-census case's path 1.
- `bodies`: no moved function's text differs beyond whitespace (step 11's record).
- Unit, integration and e2e layers `"ok":true`; fmt, clippy, the orphans gate; the smoke trio and CI green (operator).

## Touchpoints
New: `crates/viola-pty/src/pump/restart.rs`, `pump/test_cases.rs`, `pump/test_cases_paste.rs`,
`pump/test_cases_respawn.rs`, `src/cmd/run/test_cases.rs`, `src/cmd/run/test_cases_start.rs`. Modified:
`src/cmd/up.rs`, `crates/viola-pty/src/pump.rs`, `src/cmd/run.rs`, `src/run/window.rs`, `src/trust_record.rs`. One
implement window, three ordered groups (A up · B pump · C run), each closed by its crate's layer (`inputs#I2`).

## Re-points (the only test-side changes)
`pump::tests::paste::*`, `pump::tests::respawn::*`, `cmd::run::tests::start::*` (all but
`start_opens_the_scenario_one_spans_under_run_start`, which `scripts/profraw-census.sh:20` names), and
`cmd::run::tests::trust_word_on_an_unverified_cli_reads_no_file` →
`trust_record::tests::trust_word_on_an_unverified_cli_reads_no_file`. `WindowAsk` `pub(super)` → `pub(crate)`.

## Test commands
27 entries (`gate.py run --dry-run` exit 0): fmt · clippy · orphans · the two crate units · six case counts · sizes ·
cx · up-cx · pty-cx · bodies · the host read · the three harness layers · the operator pass (smoke trio, pre-push,
hygiene, push, CI). No round, no live entry, no mutation run, NO `gate.py control`: a pure move adds and changes no
guard; the guards a moved case carries keep their names, and the count entries hold that none is lost.

## Capabilities
None claimed: the pool (v2-14, v2-15, v2-28, v2-30, v2-31) holds no capability a pure move proves.

## The block the directive asks for (item 8), verbatim where a tool printed it
- **Union lines:**
  - `architecture: rows 1049 · packs 11 · picked 147 (65 by every draw) · standing 11 · 44051 B`
  - `security-plan: rows 667 · packs 7 · picked 101 (60 by every draw) · standing 8 · 33403 B`
  - `design-system: rows 361 · packs 4 · picked 49 (27 by every draw) · standing 0 · 14259 B`
  - `layout-templates: rows 238 · packs 3 · picked 31 (10 by every draw) · standing 0 · 7307 B`
  - `test-plan: rows 1388 · packs 14 · picked 194 (65 by every draw) · standing 6 · 49709 B`
  - `obs-plan: rows 1056 · packs 11 · picked 225 (118 by every draw) · standing 5 · 58779 B`
  - `a11y-plan: rows 244 · packs 3 · picked 41 (17 by every draw) · standing 22 · 13512 B`
  - `rows-union: 7 master(s) · picked 788 of 5003 · rule any`
- **Every cited row in the union:** yes — the plan's 19 cited handles each read 1 in its master's rows file (a
  `grep -c` per handle over `{x}-rows.md` + `{x}-standing.md`); the rows files' sum 221 020 B, under 250 000 B, so
  each was read whole.
- **planlint's own lines (rev 3):**
  - `fence: 27 entries · plan: doc:2026-11-27-epoch-5-cleanup-2/plan · research: doc:2026-11-27-epoch-5-cleanup-2/research`
  - `check 9 · CI-outcome reads: 1 · leg operator exempt: 1 · Platform issues consulted: none · `none — no runner-only bull…``
  - `check 9 · runner-only scope bullet: not read — the letter's`
  - `check 4 (5) · leg = 'live' entries: 0 · Harness rules consulted: none · `none — no live leg in this chunk; the smoke t…``
  - `check 4 (5) · leg entries with a content atom: 4 · note without `atom from`: 0`
  - `check 4 (13) · operator CI-outcome reads: 1 · not ci.py conclusion: 0`
  - `check 4 (13) · job-log reads by pattern: 0`
  - `check 4 (14) · base-less git change reads outside the operator and round legs: 0`
  - `check 4 (15) · grep → wc -l counts asserting exit 0 and last line 0: 0`
  - `check 4 (16) · guards whose exclusions leave a new file in: 0 of 0 with an exclusion`
  - `check 4 (18) · entries reading a master's or a base's path the store now holds: 0`
  - `check 4 (19) · absence probes against the test names their own plan adds: 0 · probes read 1`
  - `check 4 (17) · entries reading a path the wrap writes: 0 of 17 asserting — listed, P5's read`
  - `check 4 (17) · a path the wrap writes, read from inside a program an entry runs (a suite, the ledger's tool): not read`
  - `check 2 · ledger-note entries (matrix#): 0`
  - `check 2 · entries naming no master and no matrix#: 0`
  - `check 5 · placeholders outside code: 0`
  - `check 12 (c) · step references the plan's steps lack: 0 of 7`
  - `check 12 (d) · «unchanged» claims on a name another part of the plan names: 6 of 6 — listed, P5's read` — read:
    each claim is a moved body or an order kept, true of the step it sits in
  - `check 4 (4) · criterion evidence paths the plan names nowhere else: 0 of 0`
  - `hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12 (c)); 0 is not a pass`
- **`[RETYPED]` lines and re-spawns:** one — `arch-pick-p3.d2 · [RETYPED] line 99:
  decision/socket-dir-check-verdicts-never-chmods is no row of the pack — read as decision/socket-dir-check-verdicts,
  the pack's row 99`; re-spawns 0 (census `first-rowtask:7 · first-pick:159 · first-rowhistory:7`).
- **Non-zero exits, refusals, UNPARSED / INDETERMINATE / NOT DERIVED:** none of the three words printed. Refusals:
  the Bash guard refused four calls, each re-issued once in the form it passes (a `cd` into the skill's references
  dir; a piped `rec.py find`; a piped `gate.py run`; a `cd` into the run dir). Red baselines: entry 12's first firing
  read red on a refused command (`Error: 'crates/viola-pty/src/pump' not found.`, tokei refusing a not-yet-created
  path), fixed in the plan (rev 2, `ls -d … 2>/dev/null`) and re-fired to the red the plan predicts (last line 2);
  entries 12, 13 and 16 read red as predicted, each with its must-pass control (`controls-p5.md`).
- **Where a letter was wrong or silent:** the carry's «(the same word and record)» names neither (it was decoded from
  the earlier chunk's scope Sources; a take-up friction record); `scope` said «its plan and its act», measured short
  (a two-way plan half reads 16) and corrected on `inputs#I2`; the scope's «those three» was silent on
  `Pumping::run`, answered (b) on `inputs#I2`; the size measure's earlier figures (608 + 1129, 779 + 932, 391 + 163)
  were superseded at this HEAD (610 + 1432, 812 + 932, 382 + 163).

## Escalate rows
Read before the P3 block (`rul-r74hshif`, `rul-p2zl2kgl`, `rul-o2tr55aj`, `rul-bznresq5`): none reached — the moves
widen no boundary. Expected amendments (wrap): none to the masters (they name none of the moved paths or symbols); the
leaf `.claude/docs/services/viola-pty.md` may name `pump/restart.rs`.

## Left in the scratchpad (nothing removed)
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/70c72b6e-1c7e-4929-b2f6-77d700394705/scratchpad/clone/` (the
controls' clone, its inputs uncommitted) and `…/scratchpad/proto/`, `split/`, the measurement scripts.

## Beyond the plan
none

Apply? (yes / review / cancel)
