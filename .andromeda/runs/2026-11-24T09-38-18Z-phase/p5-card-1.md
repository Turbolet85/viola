# P5 review card 1 — «`viola adopt`» (2026-11-24-viola-adopt)

**Plan:** `doc:2026-11-24-viola-adopt/plan` rev 3. Its render, to open in an editor, is
`.andromeda/cache/docs/2026-11-24-viola-adopt/plan.md`. No plan stands in a chunk's folder. Writing in that file
changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-24-viola-adopt` diffs the chunk's records.
**Forks:** `.andromeda/runs/2026-11-24T09-38-18Z-phase/founder-forks.md` Block 1. W1–W4 are his, each built on its
arm 1, PROVISIONAL. R5 is his live reading. T1–T6 are technical, each on its lean, for the overseer's relay.

## Goal
`viola adopt <session-id> --name <n> [--parent <p>] [--model] [--effort] [--json]` takes a plain claude session that
was started outside viola and has already ended, and resumes it under a viola wrapper in the terminal it is typed in.
The session lands on the board under that name and parent, driven like any wrapped session. It is `revive`'s start
without a log to read. `revive`'s live-elsewhere reading guards it: a listed id is refused `session-live`, and no
process viola did not start is ended, signalled or taken over. The chunk also folds `e7703af0`'s CI kill: the two
verify-driven `security_negatives_env_capture` cases are renamed into the `verify_window_` class.

## Top acceptance criteria
- Resumed under name and parent: `wheel` → `lineage{parent}` → `launch` → `session-start{cause:resume, id}`, then
  driven by one send and one wait (`adopt_resumes_the_session_under_its_name_and_parent`).
- A listed id is refused `session-live`, exit 1, with no child (control `adopt-live`). A failed agents read starts.
  A second adopt of a live adopted id is refused.
- A malformed id is a usage exit 2 before anything is read (control `adopt-shape`). The home's strict modes are
  checked before any log.
- The help's `setup:` row reads `open, run, revive, adopt, up, verify, plugin install`; the exit and error-body tables
  stay green.
- The CI fold: both cases stand in the class. `evidence/ci-fold.md` holds their measured time against the 80 s kill,
  locally and under coverage.

## Touchpoints
- New: `src/cmd/adopt.rs`, `tests/cli_adopt.rs`, `scripts/r5-adopt.sh`.
- Modified: `src/cmd/mod.rs`, `src/main.rs`, `src/cmd/revive.rs` (two readers shared, bodies unchanged),
  `tests/cli_output_plain.rs`, `tests/cross_exit_causes.rs`, `tests/contract_error_bodies.rs`,
  `tests/security_negatives_env_capture.rs`.
- `src/cmd/run.rs`, `src/run/` and `.config/nextest.toml` are untouched.

## Test commands (22 entries)
| Entries | What |
|---|---|
| 1–2 | fmt, clippy |
| 3–4 | root units; adopt units |
| 5 | integration cases `^adopt_` |
| 6 | the table readers |
| 7 | the folded binary under `ci` |
| 8–9 | the R5 script's parse and its rehearsal |
| 10 | host read |
| 11–13 | the harness's three layers |
| 14 | the gated reading of `evidence/r5.md` (`defer = gated: R5`) |
| 15–19 | operator pass: pre-push, hygiene, push, `ci.py`, the CI-fold read |
| 20–22 | `cargo build -p viola` and R5a / R5b, on his word only |

Baselines on the untouched tree:
- green: 3 (1345 passed), 6 (129 passed), 10 (recorded, CONTENDED);
- red, vacuous until the work lands: 4 (0 of 1345), 5 (0 of 4320), 7 (old names), 8 and 9 (exit 127, no script);
- 14: red on no record, its predicate controlled by hand (`control-entry-14.md`).

## Concretization preview — `matrix#v2-13` (claimed GATED on R5)
- **Current acceptance:** A plain claude session named by its session id ends and resumes under viola with the given
  name and parent and is then driven like any wrapped session; a session live in another claude is refused; no
  running process's terminal is taken over.
- **Proposed:** The same outcome, unweakened, read on the operator's host as:
  - `r5a: green resumed=yes lineage=yes turn=ended` — a plain session he started and ended himself, resumed by `viola
    adopt <id> --name adopt-probe --parent r5-parent` from another folder, then driven by one send and one wait;
  - `r5b: green refused=session-live started=none` — a plain session he leaves running is refused, with no child.

  Both lines go in `.andromeda/cache/chunks/2026-11-24-viola-adopt/evidence/r5.md`, read by entry 14. «No running
  process's terminal is taken over» holds by construction (W1 arm 1, PROVISIONAL) and is witnessed over the fakes.
- **Gated criterion:** `BLOCKED-ON: R5, viola adopt once of a real plain claude session the operator started and ended
  himself (R5a) and once of a real live one, refused (R5b), at the operator host, unlocked, its moment his to name
  (founder-forks R5)`. A stand-in, stub or recorded artifact never verifies the gated part: the fakes prove every clause
  now, and only R5's own lines verify the cap.

## Directive item 11 — the tool lines, verbatim
- **The union:**
  - `architecture: rows 1037 · packs 11 · picked 339 (254 by every draw) · standing 25 · 91701 B`
  - `security-plan: rows 657 · packs 7 · picked 202 (151 by every draw) · standing 28 · 56702 B`
  - `design-system: rows 358 · packs 4 · picked 149 (110 by every draw) · standing 5 · 36155 B`
  - `layout-templates: rows 233 · packs 3 · picked 77 (58 by every draw) · standing 19 · 17059 B`
  - `test-plan: rows 1383 · packs 14 · picked 336 (219 by every draw) · standing 62 · 82907 B`
  - `obs-plan: rows 1051 · packs 11 · picked 402 (251 by every draw) · standing 57 · 94278 B`
  - `a11y-plan: rows 242 · packs 3 · picked 125 (103 by every draw) · standing 17 · 32131 B`
  - `rows-union: 7 master(s) · picked 1630 of 4961 · rule any`
- **Every cited row in the union:** yes, all 31 rows the plan and the forks block cite stand in their master's rows
  file (checked by handle against each `{x}-rows.md`; none needed the standing lists). The rows files sum to about
  411 000 B, past the 250 000 B whole-read bound, so they were read by search.
- **`[RETYPED]` lines and re-spawns:** four notes, no re-spawn.
  - `arch-pick-p4.d2 · [RETYPED] line 82: contract/viola-verify-exit-codes/1 is no row of the pack — read as contract/viola-verify-exit-codes/1-relied-row-failed, the pack's row 82`
  - `security-pick-p4 · [RETYPED] line 95: decision/plugin-files-rewritten-every-start is no row of the pack — read as decision/plugin-files-rewritten-each-start, the pack's row 95`
  - `tests-pick-p2.d2 · [RETYPED] line 90: contract/mcp-tools/list is no row of the pack — read as contract/mcp-tools-list/tool-set, the pack's row 90`
  - `tests-pick-p6 · [RETYPED] line 51: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 51`

  The batches: task 7/7, pick 159/159 in three parts (75 · 75 · 9), history 7/7, each first try, 0 dead draws.
- **Non-zero exits, refusals, UNPARSED / INDETERMINATE / NOT DERIVED:** no UNPARSED, INDETERMINATE or NOT DERIVED
  line. The Bash guard refused three calls, each re-issued once in a form it passes: a `cd` into a skill references
  dir, a `cd` into the run dir, and a pipe of `rec.py find` into a filter. History check:
  `a11y-history · not a marker: **2026-10-20T06-58-31-wrap** — history:16`, a wrap run dir's name, read and set
  aside. Baseline reds 4, 5, 7, 8, 9 and 14 are listed above. The host read CONTENDED (escher building, 235 s IO
  stalls in 30 min).
- **Where a letter was wrong or silent:**
  - The directive's P3 block («ONE block … at P3's END») met history files that P4 reads, after the block: W4 and
    the `cwd` row were added to Block 1 at P4, marked as from the history read.
  - The phase letter reads the history files at P4; I read them after the plan's first put. That cost one plan
    revision, recorded as `retry.synthesis-rework`.
  - Neither the directive nor the letter says which home a live adopt runs under. R5 puts that choice to him.
- **Scratch left on disk:**
  - `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/bf141908-d0f7-4620-8749-b69bf23eaac1/scratchpad/` (JUnit
    downloads, `junit_times.py`, `ctl14/`, the two rendered plans, `baselines.sh`);
  - `$TMPDIR/andromeda-gate/2026-11-24-viola-adopt/` (gate logs);
  - `target/run-archive/641` (the local env_capture run).

  None is removed by a recursive delete here.

## Beyond the plan
- **A route note for «Switching a session's model and effort» (v2-30):** v2-30's acceptance names «after … an adopt
  the session runs on the pair its instance recorded». This chunk records adopt's launch pair, like every start,
  and claims no part of v2-30. The cap stays with its entry.
- **W1–W4 PROVISIONAL rows at the wrap:** each widening lands as built, marked not ratified, his own word owed (escalate
  patterns `rul-r74hshif` / `rul-p2zl2kgl`); the wrap halts on them in the plain «Boundary widening» form.

Apply? (yes / review / cancel)
