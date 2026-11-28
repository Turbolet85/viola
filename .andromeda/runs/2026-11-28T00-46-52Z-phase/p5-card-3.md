# P5 review card 3 — «Console scaffold» (`2026-11-28-console-scaffold`), written 2026-11-28T07:45:13Z

Card 2 (`.andromeda/runs/2026-11-28T00-46-52Z-phase/p5-card-2.md`) was answered «review» on his later F3 word,
«Вернуть viola console» (inputs#I3): the bare start is withdrawn. This card shows plan rev 5.

The plan to open in an editor: `.andromeda/cache/docs/2026-11-28-console-scaffold/plan.md` (rev 5 of
`doc:2026-11-28-console-scaffold/plan`, 445 lines with its 23-entry fence). No plan stands in a chunk's folder; writing
in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff
--root . 2026-11-28-console-scaffold` is a diff of the chunk's records. Research rev 3 (its `## Revision 2`), scope
rev 5.

## What changed from card 2 (back to rev 2's shape on F3, keeping rev 4's gains)
- **The start (F3, his later word):** the verb `viola console`, in the `board:` help row beside `list, focus`. `Cli`'s
  `command` stays required; `role_of` maps the first word `console` to `Role::Console`; the no-argument branch
  (`src/main.rs:120`) is untouched. Bare `viola` keeps today's answer on a terminal and off one (the help on stderr,
  exit 2 — research §Revision 1). No bare-start test, no `both-tty` control.
- **Back as rev 2 wrote them:** step 5 (`Command::Console`, the `board:` row), step 6's «needs a terminal» refusal,
  step 9's help pin in `tests/cli_output_plain.rs`, the board-row count entry (red 0/0 on the untouched tree).
- **Kept from rev 4:** Q2 met by `RELIED_COUNT`, no second table; every PROVISIONAL word on F1 · F2 · F3 · F4 · F5 · Q1
  replaced by his (inputs#I2, inputs#I3); F2's exceptions named «the terminal fleet console (`viola console`), its own
  surface».
- **Gone:** the `obs-plan · decision/role-classified-before-clap` exception — not reached by the verb (inputs#I3).

## Which rows the wrap amends, and on whose word (inputs#I3's order)
**On HIS word (the operator's own answers):**
| Row | Amendment | His answer |
|---|---|---|
| `a11y-plan · decision/static-linear-output` | `except:` the terminal fleet console (`viola console`), its own surface | F2, inputs#I2 — a boundary widening (`rul-r74hshif`, `rul-p2zl2kgl`); the wrap halts once on his own answer card (`rul-bznresq5`) |
| `a11y-plan · decision/cli-not-keyboard-interactive` | the same named exception | F2, as above |
| `a11y-plan · decision/cli-no-interactive-input` | the same named exception | F2, as above |
| `obs-plan · absence/interactive-terminal-interface` | leaves for a decision row naming the console's surface | F2, as above |
| `layout-templates · contract/viola-help/verb-groups` | the board row `board:    list, focus, console` | F3, inputs#I3 |
| the `Surface: console` area's floor (design-system and layout-templates, new) | stays whole at 80×24, one line below it | Q1, inputs#I2 |
| no `deny.toml` change; the chrono ignore stands to the Epoch 7 boundary | (none — no row moves) | F4, inputs#I2 |

**On a delegate's technical word (the overseer's, or the route head's own text):**
| Row | Amendment | Whose word |
|---|---|---|
| the role lists growing to five: `obs-plan · contract/diag-line/process`, `contract/log-line/process`, `contract/process-log/process`, `contract/process-log-line/process`, `decision/structured-json-process-logs`, `decision/zero-unlogged-panics`, `contract/diagnostics-line/role-file-line-size`; `architecture · decision/codes-only-process-logs`; `test-plan · decision/structured-json-process-logs`, `decision/process-logs-via-tracing-subscriber`, `contract/log-line/process` | gain `console` | the role itself is the route head's «its own diagnostics role»; its word `console` is Q3, the overseer's technical answer (inputs#I2) |
| the obs event catalog | gains `console-frame` and `console-key` (an obs-plan Decisions Log entry) | technical, this plan's |
| `architecture · decision/silent-roles-at-internal-error-catch` | gains `console` | technical — see below |
| the rest of the `Surface: console` area (design-system, layout-templates): the frame's two lines, `q quit`, no colour, the owed view and actions | new rows | technical, this plan's, inside F2's named surface |
| `security-plan` §Input Validation | the console's bounded 64-byte key reader, its bytes in no line | technical, this plan's |
| `architecture · overview/project-intent` | unchanged («planned, not built» stays until the view lands) | — |

**`architecture · decision/silent-roles-at-internal-error-catch` gaining `console`: a reach of its own, NOT a named
addition inside F2's exception.** F2's four rows govern redraw and interactive input. This row governs what a role
prints at the internal-error catch site: «Run, revive's start, adopt's start, mcp and hook print nothing at the
internal-error catch site; only a CLI verb prints the internal-error line or document there.» Adding `console` makes a
verb silent there, because its terminal is the frame's. That is no widening — the console prints less there than a CLI
verb does, and nothing new crosses a boundary — so it lands on a technical word, the overseer's to give. **The
technical alternative** that reaches no row at all: `role_of` leaves `console` as `Role::Cli` (the fallback it takes
today), and the catch site prints `error: internal error` once after the restore guard has dropped during the unwind,
on a terminal already back to its own modes. The plan holds the silent arm as rev 2 had it (inputs#I3: «`role_of` reads
the role from the first word `console` as rev 2 had it»). Which arm stands is the overseer's to say; both build whole.

## Goal
`viola console` takes its terminal: raw mode and the alternate screen; one frame, whole at 80×24 and a single
«needs 80x24» line below it; `q` or Ctrl-C quits and restores the terminal; every frame and key a codes-only line in a
new `console` role file. No new dependency. A keystroke driver (`tests/tui_console.rs`) proves each behaviour from the
role file and the exit, never the screen. The carries: M1 (`paste_turn`), M3 (`RELIED_COUNT`, one pin), the chrono
re-read recorded.

## Top acceptance criteria
1. 80×24 → `console-frame` `fits` true; `q` → `console-key quit`, `process-exit` 0, exit 0 (`tui_console`).
2. Below 80×24 `fits` false; a resize redraws; an unbound key logged by class alone; 0x03 exits 0.
3. The terminal's modes restored on quit (`restore` control).
4. No terminal, or a widened home → exit 1, two literal lines, no role line (both writers in the error-bodies census).
5. `process` `console` in `console.ndjson`, schema-valid; `ObsProcess::ALL` five roles.
6. `--help`'s board row `board:    list, focus, console`; bare `viola` keeps exit 2 (`viola_without_a_verb_is_usage`).
7. M3 `\b29\b` count 27 → 3 (predicted); M1 `run.paste_text(` 6 → 2 (predicted); `cli_verify` green.
8. `cargo deny check advisories` → `advisories ok`, the ignore standing.

## Touchpoints
New: `src/cmd/console.rs`, `src/console/`, `tests/tui_console.rs`. Modify: `crates/viola-core/src/obs.rs`,
`schemas/diag-line.v1.json`, `schemas/diag-detail.v1.json`, `src/obs.rs`, `src/main.rs`, `src/cmd/mod.rs`,
`tests/cli_output_plain.rs`, `tests/contract_error_bodies.rs`, `tests/cross_exit_causes.rs`,
`tests/contract_bounded_readers.rs` (if its census needs the reader named), `Cargo.toml`,
`crates/viola-agent-claude/src/ledger.rs`, `crates/viola-agent-claude/src/ledger/test_cases_stamps.rs`,
`src/cmd/verify.rs`, `src/cmd/verify/typed/turns.rs`.

## Test commands
23 entries, each `run` byte-equal to rev 2's and carrying its baseline: fmt, clippy; units of `viola-core`,
`viola-agent-claude`, the root bins (green: 41, 806, 1413); `binary(tui_console)` whole and two control filters (red,
exit 94: no binary yet); the help, M3 and M1 counts (red: 0/0, 27, 6); `cargo deny check advisories` (green); the host
read (recorded, QUIET at 07:19:47Z-07:34:47Z); the harness's unit, integration and e2e layers (seen green at the last
wrap); operator: the smoke trio, pre-push, hygiene, the push, `ci.py`. Controls (implement's one-shot `gate.py
control`): `quit-key`, `floor-cmp`, `restore`. No round, no live entry, no real `claude`, no mutation run.

## Capability claims
None. Concretization preview: none.

## The directive's block (inputs#I1 item 8), for this card
**The union's per-master lines and `rows-union`:** unchanged from card 1 — no batch ran since:
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
**Every cited row in the union or its standing list:** yes — the same 33 citations card 1 read present (rev 5 cites
the rows rev 2 cited; `obs-plan · decision/role-classified-before-clap` is no longer cited).

**planlint's own lines, whole (plan rev 5):**
```
planlint v1.25 · a61c2794
fence: 23 entries · plan: doc:2026-11-28-console-scaffold/plan · research: doc:2026-11-28-console-scaffold/research
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
check 4 (17) · entries reading a path the wrap writes: 0 of 15 asserting — listed, P5's read
check 4 (17) · a path the wrap writes, read from inside a program an entry runs (a suite, the ledger's tool): not read
check 2 · ledger-note entries (matrix#): 0
check 2 · entries naming no master and no matrix#: 0
check 5 · placeholders outside code: 0
check 12 (c) · step references the plan's steps lack: 0 of 2
check 12 (d) · «unchanged» claims on a name another part of the plan names: 6 of 8 — listed, P5's read
  line 17 · «unchanged» · viola (step 4)
  line 41 · «unchanged» · deny.toml (step 12)
  line 109 · «unchanged» · tests/run_cli.rs (Test Commands) · viola_without_a_verb_is_usage (Test Commands)
  line 116 · «untouched» · pass (Test Commands)
  line 122 · «unchanged» · runs.rs (Test Commands)
  line 416 · «untouched» · role_of (step 4) · Cli (step 4)
  a claim on a name no other step, Files to modify or Test Commands names is not listed — P5's read
check 4 (4) · criterion evidence paths the plan names nowhere else: 0 of 0
hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12 (c)); 0 is not a pass
```
The six 12 (d) claims were read against the steps, and each holds. Step 4 adds the `console` arm and leaves
`role_of`'s no-argument and `revive --list` branches and the `Cli` fallback for every other verb. No step touches
`deny.toml`, `tests/run_cli.rs`, `runs.rs`'s call or the summary's counting rule.

**The claim's lines:** none.

**`[RETYPED]` lines and re-spawns:** as on card 1 (nine `[RETYPED]` lines, three re-spawns, one dead draw); no batch
ran since.

**Non-zero exits, refusals, `UNPARSED` / `INDETERMINATE` / `NOT DERIVED` since card 2:**
- `rec.py render doc:…/plan --at-tx 5257` exit 2 (a chunk document's past is read with `rec.py read … --at-tx`, not
  `render`); not retried — rev 5 was composed from rev 2's text in this window.
- No new baseline run: rev 5's 23 entries are byte-equal to rev 2's runs, each carrying rev 2's baseline (the host
  read the later 07:19:47Z-07:34:47Z reading).
- No refusal, `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line.

**Every place a letter was wrong or silent, since card 2:**
- The same door as rev 4 (P4 re-synthesis at a P5 review; research's addition in the borrowed `## Revision {n}` form).
- Card 2's `## Beyond the plan` row 1 (the console named in no `--help`) is ENDED: the verb names it in the board row.
- Cards 1 and 2's places stand (the directive's quoted stub and `ui` role, the history files read by search, the
  scratchpad writes during a batch, the row read not re-run after the closure named new files, the 21-sites / 27-lines
  count).

## Beyond the plan
none. Card 1's rows ended at card 2 (Q2 met by `RELIED_COUNT`; one chunk now his own F5). Card 2's row 1 is ended
above.

Apply? (yes / review / cancel)
