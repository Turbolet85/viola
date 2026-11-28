# P5 review card 2 — «Console scaffold» (`2026-11-28-console-scaffold`), written 2026-11-28T07:37:36Z

Card 1 (`.andromeda/runs/2026-11-28T00-46-52Z-phase/p5-card-1.md`) was answered «review» on his answers
(inputs#I2): F3, his own text, moved the start from a verb to bare `viola`. This card shows the revised plan.

The plan to open in an editor: `.andromeda/cache/docs/2026-11-28-console-scaffold/plan.md` (rev 4 of
`doc:2026-11-28-console-scaffold/plan`, 467 lines with its 24-entry fence). No plan stands in a chunk's folder; writing
in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff
--root . 2026-11-28-console-scaffold` is a diff of the chunk's records. Research rev 2 carries the bare-`viola`
measurement as `## Revision 1`; scope rev 4 carries his answers.

## What changed from card 1
- **The start (F3, his word):** no `viola console` verb, no alias, no help change. Bare `viola` (or `viola --home
  <dir>`) with stdin AND stdout a terminal starts the console; with either not a terminal it gives today's answer
  byte for byte (the overseer's settlement, inputs#I2 item 2).
- **Everything else his own word, nothing PROVISIONAL:** F1 no dependency, F2 four named exceptions, F4 the chrono
  ignore to the Epoch 7 boundary, F5 one chunk, Q1 80×24; Q2 (M3 is the literal sites only — no second table; the
  carry's «stamped-set table held once» met by `RELIED_COUNT`) and Q3 (`console`) the overseer's.
- **Plan:** step 4 `role_of` reads no verb plus two terminal facts; step 5 makes the verb optional, the no-verb
  non-terminal branch writing today's answer; step 6 keeps one refusal (the widened home), the «needs a terminal»
  refusal gone (off a terminal is today's answer); step 8 starts bare `viola --home <home>` and adds
  `bare_viola_off_a_terminal_answers_as_today` (three cases); the help-row change and `tests/cli_output_plain.rs`'s
  edit are gone, replaced by a guard that the board row stays; a fourth control, `both-tty`.

## The bare-`viola` measurement (research §Revision 1)
- Today, on a terminal or off one, bare `viola` writes the help text on stderr (362 B, byte-equal to `viola --help`'s
  stdout), nothing on stdout, and exits 2: `role_of` → `Role::Other` (`src/main.rs:120`), `Cli::try_parse` fails on the
  required verb (`src/cmd/mod.rs:61`), `usage.exit()` (`src/main.rs:53`). Measured on the untouched tree's build,
  piped and on a pseudo-terminal of the probe's own, a scratch HOME, nothing under a home read.
- Pins: `tests/run_cli.rs:572` `viola_without_a_verb_is_usage` runs bare `viola` off a terminal, exit 2 — it stands
  unchanged and keeps reading green. No script, CI step or harness spawn calls bare `viola`.
- **The settlement holds.** At his keyboard the only change is: bare `viola` on a terminal opens the console where it
  printed the help and exited 2; `viola --help` still prints the help.
- **The help cannot name the console** without moving the non-terminal answer, which is the help text itself, byte
  for byte. The console is named in no `--help` (a «Beyond the plan» row below).

## His to confirm — a NEW widening the bare start reaches (inputs#I2 item 4)
`obs-plan · decision/role-classified-before-clap` — «The role is the first argument that is neither a global flag nor
a flag's value and names a role subcommand, so viola --home <dir> hook stop is still a hook whose pre-init panic
exits 0.» The console's role is read from NO verb with both stdin and stdout a terminal, which this rule does not
cover. Its conservative arm, the same as F2's: an exception NAMED for «the terminal fleet console, bare `viola` on a
terminal», the rule never generalised. Escalate rows `rul-r74hshif` / `rul-p2zl2kgl`: the wrap halts on it as his.
Read, not reached (terms: the no-argument answer, `Usage:`, `usage error`, `exit 2`, `no verb`, `first argument`,
`role subcommand`): the exit-2 rows (`architecture · contract/cli-exit-codes/2`, `a11y-plan ·
contract/cli-usage-error/exit-2`, `a11y-plan · contract/cli-exit-codes/2`) — the non-terminal answer keeps exit 2;
`design-system · decision/no-interactive-prompt-or-pager` and `layout-templates · decision/no-prompt-or-pager` — no
verb; `layout-templates · contract/viola-help/usage-line`, `a11y-plan · contract/cli-help/usage-line` — about
`--help`, unchanged (their `<verb>` is not required on a terminal any more: a note for the wrap, not a widening).

## Goal
Bare `viola` on a terminal takes it: raw mode and the alternate screen; one frame, whole at 80×24 and a single
«needs 80x24» line below it; `q` or Ctrl-C quits and restores the terminal; every frame and key a codes-only line in
a new `console` role file. Off a terminal, today's answer. A keystroke driver (`tests/tui_console.rs`) proves each
behaviour from the role file and the exit, never the screen. The carries: M1 (`paste_turn`), M3 (`RELIED_COUNT`, one
pin), the chrono re-read recorded.

## Top acceptance criteria
1. On a terminal at 80×24 → `console-frame` `fits` true; `q` → `console-key quit`, `process-exit` 0, exit 0.
2. Below 80×24 `fits` false; a resize redraws; an unbound key logged by class alone; 0x03 exits 0.
3. The terminal's modes restored on quit (`restore` control).
4. Off a terminal (none / stdin alone / stdout alone): stderr byte-equal to `viola --help`'s stdout, stdout empty,
   exit 2, nothing made under the home (`both-tty` control); `viola_without_a_verb_is_usage` green.
5. `viola --help` and the verbs unchanged (the board-row guard reads 1; `cli_output_plain` green, untouched).
6. A widened home → exit 1, two literal lines, no role line.
7. `process` `console` in `console.ndjson`, schema-valid; `ObsProcess::ALL` five roles.
8. M3 `\b29\b` count 27 → 3 (predicted); M1 `run.paste_text(` 6 → 2 (predicted); `cli_verify` green.
9. `cargo deny check advisories` → `advisories ok`, the ignore standing.

## Touchpoints
New: `src/cmd/console.rs`, `src/console/`, `tests/tui_console.rs`. Modify: `crates/viola-core/src/obs.rs`,
`schemas/diag-line.v1.json`, `schemas/diag-detail.v1.json`, `src/obs.rs`, `src/main.rs`, `src/cmd/mod.rs`,
`tests/contract_error_bodies.rs`, `tests/cross_exit_causes.rs`, `tests/contract_bounded_readers.rs` (if its census
needs the reader named), `Cargo.toml`, `crates/viola-agent-claude/src/ledger.rs`,
`crates/viola-agent-claude/src/ledger/test_cases_stamps.rs`, `src/cmd/verify.rs`, `src/cmd/verify/typed/turns.rs`.

## Test commands
24 entries: fmt, clippy; units of `viola-core`, `viola-agent-claude`, the root bins (green: 41, 806, 1413);
`binary(tui_console)` whole and three control filters (red, exit 94: no binary yet); the board-row guard (green, 1);
the M3 and M1 counts (red: 27, 6); `cargo deny check advisories` (green); the host read (recorded, QUIET); the
harness's unit, integration and e2e layers (seen green at the last wrap); operator: the smoke trio, pre-push,
hygiene, the push, `ci.py`. Controls (implement's one-shot `gate.py control`): `quit-key`, `floor-cmp`, `restore`,
`both-tty`. No round, no live entry, no real `claude`, no mutation run. CI runs with no terminal, so CI witnesses the
non-terminal answer.

## Capability claims
None. Concretization preview: none.

## The directive's block (inputs#I1 item 8), for this card
**The union's per-master lines and `rows-union`:** unchanged from card 1 (no re-run: the revision adds no surface the
pinned scope did not name — the start was the scope's own fork):
```
architecture: rows 1049 · packs 11 · picked 276 (149 by every draw) · standing 24 · 80668 B
security-plan: rows 667 · packs 7 · picked 160 (111 by every draw) · standing 24 · 46378 B
design-system: rows 361 · packs 4 · picked 95 (62 by every draw) · standing 1 · 22592 B
layout-templates: rows 238 · packs 3 · picked 45 (31 by every draw) · standing 12 · 9369 B
test-plan: rows 1388 · packs 14 · picked 302 (181 by every draw) · standing 33 · 79705 B · dead draws 1
obs-plan: rows 1056 · packs 11 · picked 406 (257 by every draw) · standing 43 · 95960 B
a11y-plan: rows 244 · packs 3 · picked 80 (49 by every draw) · standing 3 · 22158 B
rows-union: 7 master(s) · picked 1364 of 5003 · rule any
```
**Every cited row in the union or its standing list:** yes — card 1's 33, less `layout-templates ·
contract/viola-help/verb-groups` (no longer cited), plus `obs-plan · decision/role-classified-before-clap`, which
`obs-rows.md` holds (line 257).

**planlint's own lines, whole (plan rev 4):**
```
planlint v1.25 · a61c2794
fence: 24 entries · plan: doc:2026-11-28-console-scaffold/plan · research: doc:2026-11-28-console-scaffold/research
check 9 · CI-outcome reads: 1 · leg operator exempt: 1 · Platform issues consulted: filled · `- crates.io index `https…`
check 9 · runner-only scope bullet: not read — the letter's
check 4 (5) · leg = 'live' entries: 0 · Harness rules consulted: none · `none — no live leg in this chunk (the directi…`
check 4 (5) · leg entries with a content atom: 4 · note without `atom from`: 0
check 4 (13) · operator CI-outcome reads: 1 · not ci.py conclusion: 0
check 4 (13) · job-log reads by pattern: 0
check 4 (14) · base-less git change reads outside the operator and round legs: 0
check 4 (15) · grep → wc -l counts asserting exit 0 and last line 0: 0
check 4 (16) · guards whose exclusions leave a new file in: 0 of 0 with an exclusion
check 4 (18) · entries reading a master's or a base's path the store now holds: 0
check 4 (19) · absence probes against the test names their own plan adds: 0 · probes read 0
check 4 (17) · entries reading a path the wrap writes: 0 of 16 asserting — listed, P5's read
check 4 (17) · a path the wrap writes, read from inside a program an entry runs (a suite, the ledger's tool): not read
check 2 · ledger-note entries (matrix#): 0
check 2 · entries naming no master and no matrix#: 0
check 5 · placeholders outside code: 0
check 12 (c) · step references the plan's steps lack: 0 of 2
check 12 (d) · «unchanged» claims on a name another part of the plan names: 12 of 17 — listed, P5's read
  line 22 · «byte for byte» · viola (step 5)
  line 41 · «unchanged» · deny.toml (step 11)
  line 63 · «byte for byte» · src/cmd/mod.rs (Files to modify) · command (Test Commands) · src/cmd/console.rs (step 6)
  line 64 · «same bytes» · src/cmd/console.rs (step 6)
  line 65 · «unchanged» · Console (step 1)
  line 108 · «byte-equal» · bare_viola_off_a_terminal_answers_as_today (Test Commands)
  line 113 · «unchanged» · tests/run_cli.rs (Test Commands) · viola_without_a_verb_is_usage (Test Commands)
      viola (step 5)
  line 120 · «untouched» · pass (Test Commands)
  line 126 · «unchanged» · runs.rs (Test Commands)
  line 377 · «byte-equal» · viola (step 5)
  line 381 · «untouched» · tests/cli_output_plain.rs (Test Commands)
  line 434 · «untouched» · role_of (step 4)
  a claim on a name no other step, Files to modify or Test Commands names is not listed — P5's read
check 4 (4) · criterion evidence paths the plan names nowhere else: 0 of 0
hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12 (c)); 0 is not a pass
```
The twelve 12 (d) claims were read against the steps, and each holds. The «byte for byte» / «same bytes» /
«byte-equal» claims are what step 5's non-terminal branch must write, pinned by `bare_viola_off_a_terminal_answers_as_today`
and its `both-tty` control. Line 65's «unchanged» is the help and the verbs, not step 1's `ObsProcess::Console`. No
step touches `deny.toml`, `runs.rs`'s call, `tests/run_cli.rs`, `tests/cli_output_plain.rs`, the summary's counting
rule or `role_of`'s `revive --list` branch.

**The claim's lines:** none.

**`[RETYPED]` lines and re-spawns:** as on card 1 (nine `[RETYPED]` lines, three re-spawns, one dead draw); no batch
ran since.

**Non-zero exits, refusals, `UNPARSED` / `INDETERMINATE` / `NOT DERIVED` since card 1:**
- New baselines: entry 9 (`ctl-bare`) `red · exit 94` (no `tui_console` binary yet), the untouched tree's expected red.
- The bare-`viola` measurement's own exit 2 (piped and on the pty) is the reading, not a failure.
- No refusal, `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line.

**Every place a letter was wrong or silent, since card 1:**
- The phase's letter has no door for a review that changes a fork's answer after P5's checks; I took P4's: «a lean the
  operator REJECTS at the P5 review → back to P4 re-synthesis», wrote research's addition in §Revision's
  `## Revision 1` form (the letter's form for a post-implement revision, borrowed), and re-ran the whole P5 set.
- The relay calls M3 «the 21 literal sites»; the count entry reads 27 lines (27 → 3), the same change by its
  derivation (some lines hold the literal twice, as index and total).
- Card 1's places stand (the directive's quoted stub and `ui` role, the history files read by search, the
  scratchpad writes during a batch, the row read not re-run after the closure named new files).

## Beyond the plan
| # | Proposal the word would approve | Owner |
|---|---|---|
| 1 | The console is named in no `viola --help`: a user learns of bare `viola`'s console only from the masters' new `Surface: console` area. An alias or a help line is his to ask for (inputs#I2). | his word, no entry proposed |

Card 1's rows: row 1 (M3's stamped-set table) is ENDED — met by `RELIED_COUNT`, no second table standing (Q2,
inputs#I2). Row 2 (one chunk) is ENDED as a row — now his own answer (F5), in the plan.

Apply? (yes / review / cancel)
