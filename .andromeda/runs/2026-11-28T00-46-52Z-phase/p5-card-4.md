# P5 review card 4 — «Console scaffold» (`2026-11-28-console-scaffold`), written 2026-11-28T07:51:02Z

Card 3 (`.andromeda/runs/2026-11-28T00-46-52Z-phase/p5-card-3.md`) was answered «review» with one technical change
(inputs#I4, the overseer's word): at the internal-error catch site `console` stays an ordinary CLI role. That word was
to be the yes for the revision IF planlint read 0 hits AND no new master row was reached. **planlint reads 0 hits; a
new master row IS reached** (below), so this is a card, not a closed phase.

The plan: `.andromeda/cache/docs/2026-11-28-console-scaffold/plan.md` (rev 7 of `doc:2026-11-28-console-scaffold/plan`,
483 lines with its 24-entry fence; rev 6 was the change, rev 7 its new entry's baseline). Writing in that file changes no
record until a `put`; `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-28-console-scaffold` diffs the chunk's records.

## The change (rev 5 → rev 7)
- **Step 4:** no `Role::Console`; `role_of`, `exit_code` and `prints_internal_error` unchanged. `console` takes the
  `Role::Cli` fallback (`src/main.rs:116`). A console dispatch error or caught panic prints `error: internal error` once
  on stderr (`human::internal_error`, `src/main.rs:69`) and exits 1 (`src/main.rs:142`). `role_of`'s rstest table gains
  `console` → `Cli` and `--home x console` → `Cli`. `architecture · decision/silent-roles-at-internal-error-catch` is
  not reached.
- **Step 3:** `internal_error_exit_line` (`src/obs.rs:233`, gate `Run | Cli` at line 235) also writes the catch-site
  `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` for `ObsProcess::Console`. Otherwise a console
  failure would leave the stderr line with no role line (the console's obs process is `console`, not `cli`).
- **Step 7:** the loop and guard take their reader, writer and terminal-mode handle as values (a test double, no
  product seam), plus two unit cases.
- **Fence:** one new unit entry (red, exit 4, 0 of 1413 selected on the untouched tree); 24 entries.

## Which step proves the order restore-then-print
**Step 7's two unit cases, with step 4's cited frame:**
- `console_run_on_a_failing_read_writes_the_restore_before_it_returns_the_error` — the reader fails: `run` returns
  `Err`, and the writer's bytes end with the restore sequence. The restore lands before the error reaches the catch site.
- `console_run_panicking_in_the_loop_writes_the_restore_during_the_unwind` — the reader panics: inside the test's own
  `catch_unwind`, the writer's bytes end with the restore sequence by the time the unwind returns.
- The print's half is main.rs's frame (step 4): the guard lives inside `console::run`, so it drops before
  `std::panic::catch_unwind` (`src/main.rs:49`) returns, and `human::internal_error()` runs only after that return
  (`src/main.rs:66-70`).
- The `restore` control (the guard's drop emptied) reads these two cases red too.

## The new master row this revision reaches (why this is a card)
**`obs-plan · contract/panic-outcome-by-role` — the family gains a `console` item.** Its rows are keyed by the obs
process: `…/cli` reads «After a panic, cli logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} when
an instance resolves, puts the chain in the detail file only, writes exactly "error: internal error\n" on stderr in one
write with no hint, and exits 1»; `…/run`, `…/hook`, `…/mcp-*` the others. The console is a CLI role at the catch site
but obs process `console` with no instance, so no existing item states its outcome. The wrap adds `…/console`: restore
the terminal, then `process-exit` `internal-error` in its home-level role file, `error: internal error` once, exit 1.
- **Not a widening:** it is the cli outcome under the console's process word. It is the same class as the role lists
  growing to five (technical, Q3), and nothing new crosses a boundary.
- **Card 3 did not name it.** Rev 5's silent arm owed such an item too (a run-like one), and card 3 missed it. It is
  named here first.
- **Read and holding, no amendment:** `architecture · contract/cli-internal-error/output`, `obs-plan ·
  contract/cli-internal-error/human-line`, `contract/cli-internal-error/stderr-line`, `contract/cli-stderr/internal-error`.
  Each states the cli-role catch-site line, which the console now prints as written.
- **Not reached:** `architecture · decision/silent-roles-at-internal-error-catch`.
- Terms searched: `panic-outcome-by-role`, `cli-internal-error`, `internal error line`, `internal-error … run|cli`,
  `catch site … process-exit`.

## Whose word, row by row (card 3's table, changed only here)
- **On his word:** unchanged from card 3 — F2's four rows (named exception), the help's verb-group row (F3), the floor
  80×24 (Q1), no `deny.toml` change (F4).
- **On a delegate's technical word:** card 3's list, less `decision/silent-roles-at-internal-error-catch` (not
  reached), plus `obs-plan · contract/panic-outcome-by-role/console` (new) and the catch-site role line's «only run and
  cli own one» gaining the console — both the overseer's (inputs#I4).

## Checks (plan rev 7), verbatim where a tool printed it
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
check 12 (c) · step references the plan's steps lack: 0 of 5
check 12 (d) · «unchanged» claims on a name another part of the plan names: 7 of 10 — listed, P5's read
  line 17 · «unchanged» · viola (step 4)
  line 43 · «unchanged» · deny.toml (step 12)
  line 58 · «no change» · src/main.rs (step 7) · role_of (Files to modify) · exit_code (step 3)
  line 127 · «unchanged» · tests/run_cli.rs (Test Commands) · viola_without_a_verb_is_usage (Test Commands)
  line 134 · «untouched» · pass (Test Commands)
  line 140 · «unchanged» · runs.rs (Test Commands)
  line 450 · «untouched» · role_of (step 4) · console (step 1) · Cli (step 3)
  a claim on a name no other step, Files to modify or Test Commands names is not listed — P5's read
check 4 (4) · criterion evidence paths the plan names nowhere else: 0 of 0
hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12 (c)); 0 is not a pass
```
The seven 12 (d) claims hold: line 58's «no change» is to `role_of`'s, `exit_code`'s and `prints_internal_error`'s
code. `src/main.rs` changes only by `mod console` and two rstest cases. Line 450's «untouched» is `role_of`'s
`revive --list` and no-argument branches.
- `gate.py run --dry-run` exit 0, 24 entries; `gate.py baseline` exit 0, every new non-leg entry carrying `baseline`.
- `inputs.py verify`: 4 entries — unchanged 3, `I4 n/a — a message has no live source`, 0 uncited.
- `gate.py scope --lists-only`: `scope lists: parsed 17 items (new 3 · modify 14 · derived 0)`.
- **Non-zero exits since card 3:** the new entry's baseline, `red · exit 4` («no tests to run»), the untouched tree's
  expected red.
- A `rec.py find` pattern's backticks were read by the shell (`run: command not found`, `cli: command not found`); the
  search still ran on its other terms and printed its 6 rows. No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line.
- Union lines, `[RETYPED]` lines and re-spawns: as card 1 (no batch ran since). Every cited row stands in its rows file:
  `architecture · contract/cli-internal-error/output` in `arch-rows.md`, `obs-plan · contract/panic-outcome-by-role/cli`
  in `obs-rows.md` (the 9 `contract/(cli-internal-error|panic-outcome-by-role)/` lines there).
- **Where a letter was silent:** the word's condition «no new master row is reached» does not say «new» against what.
  I read it against card 3's named rows, and so stopped here.

## Beyond the plan
none.

Apply? (yes / review / cancel)
