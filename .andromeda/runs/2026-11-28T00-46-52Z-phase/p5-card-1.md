# P5 review card 1 — «Console scaffold» (`2026-11-28-console-scaffold`), written 2026-11-28T01:32:09Z

The plan to open in an editor: `.andromeda/cache/docs/2026-11-28-console-scaffold/plan.md` (rev 2 of
`doc:2026-11-28-console-scaffold/plan`, 432 lines with its 23-entry fence). No plan stands in a chunk's folder; writing
in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff
--root . 2026-11-28-console-scaffold` is a diff of the chunk's records. His forks and the overseer's three technical
questions: `.andromeda/runs/2026-11-28T00-46-52Z-phase/founder-forks.md` (the P3 block).

## Goal
`viola console`, a new verb (F3, PROVISIONAL), takes its terminal: raw mode and the alternate screen; one frame,
whole at 80×24 (Q1, PROVISIONAL) and a single «needs 80x24» line below it; `q` or Ctrl-C quits and restores the
terminal; every frame and key is a codes-only line in a new `console` role file. No new dependency (F1, PROVISIONAL).
A keystroke driver (`tests/tui_console.rs`) proves each behaviour over its own PTY and test home from the role file and
the exit, never the screen. The carries: M1 (`framing_turns` → one `paste_turn`), M3 (`RELIED_COUNT` from
`LedgerRow::ALL`, one pin), and the chrono re-read recorded (no safe upgrade; the ignore stands, F4 PROVISIONAL).

## Top acceptance criteria
1. 80×24 → `console-frame` `fits` true; `q` → `console-key quit`, `process-exit` 0, exit 0 (`tui_console`).
2. Below 80×24 `fits` false; a resize redraws at the new size; an unbound key logged by class alone; 0x03 exits 0.
3. The terminal's modes restored on quit (the `restore` control reads it red with the guard emptied).
4. No terminal, or a widened home → exit 1, two literal lines, no role line (both writers in the error-bodies census).
5. `process` `console` in `console.ndjson`, schema-valid; `ObsProcess::ALL` pins five roles.
6. `--help`'s board row `board:    list, focus, console`.
7. M3: `\b29\b` count 27 → 3 (predicted); M1: `run.paste_text(` count 6 → 2 (predicted); `cli_verify` green.
8. `cargo deny check advisories` → `advisories ok` with the ignore standing.

## Touchpoints
New: `src/cmd/console.rs`, `src/console/`, `tests/tui_console.rs`. Modify: `crates/viola-core/src/obs.rs`,
`schemas/diag-line.v1.json`, `schemas/diag-detail.v1.json`, `src/obs.rs`, `src/main.rs`, `src/cmd/mod.rs`,
`tests/cli_output_plain.rs`, `tests/contract_error_bodies.rs`, `tests/cross_exit_causes.rs`,
`tests/contract_bounded_readers.rs` (if its census needs the reader named), `Cargo.toml`,
`crates/viola-agent-claude/src/ledger.rs`, `crates/viola-agent-claude/src/ledger/test_cases_stamps.rs`,
`src/cmd/verify.rs`, `src/cmd/verify/typed/turns.rs`.

## Test commands
23 entries: fmt, clippy; units of `viola-core`, `viola-agent-claude`, the root bins (green on the untouched tree: 41,
806, 1413); `binary(tui_console)` whole and two control filters (red, exit 94: no binary yet); the help, M3 and M1
counts (red: 0/0, 27, 6); `cargo deny check advisories` (green); the host read (recorded, QUIET); the harness's unit,
integration and e2e layers (seen green at the last wrap); operator: the smoke trio (`role_of` is on every start), the
pre-push, hygiene, the push, `ci.py`. Controls (implement's one-shot `gate.py control`): `quit-key`, `floor-cmp`,
`restore`. No round, no live entry, no real `claude`, no mutation run.

## Capability claims
None. Concretization preview: none. v2-14 needs the view, v2-15 and v2-31's console leg the actions, v2-12 and v2-28
the founder's host, v2-30 no console part.

## The directive's block (inputs#I1 item 8)
**The union's per-master lines, verbatim:**
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
**Every cited row in the union or its standing list:** yes — the plan's 33 row citations each read present in its
master's `{specialty}-rows.md` (a `grep -F` per handle over the rows file and the standing list; 0 out). The rows files
sum 356 830 B, past the 250 000 B bound, so P4 read them by search, never whole.

**planlint's own lines, whole (the final run, on plan rev 2):**
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
check 12 (d) · «unchanged» claims on a name another part of the plan names: 4 of 6 — listed, P5's read
  line 32 · «unchanged» · deny.toml (step 12)
  line 104 · «untouched» · pass (Test Commands)
  line 110 · «unchanged» · runs.rs (Test Commands)
  line 404 · «untouched» · role_of (step 4)
  a claim on a name no other step, Files to modify or Test Commands names is not listed — P5's read
check 4 (4) · criterion evidence paths the plan names nowhere else: 0 of 0
hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12 (c)); 0 is not a pass
```
The four 12 (d) claims were read against the steps and each holds: no step touches `deny.toml`; the verify summary's
`pass`/`absent` counting rule is not changed by M3; `runs.rs` keeps `framing_turns`' signature; `role_of`'s
`revive --list` branch is not changed.

**The claim's lines:** none (no capability claimed).

**Every `[RETYPED]` line and re-spawn:**
```
layouts-pick-p1 · [RETYPED] line 1: decision/amber-means-dialog-only is no row of the pack — read as decision/amber-means-dialog, the pack's row 1
layouts-pick-p1.d3 · [RETYPED] line 91: contract/cli-token-rendering/color-surface-* is no row of the pack — read as contract/cli-token-rendering/color-surface, the pack's row 91
tests-pick-p6 · [RETYPED] line 52: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 52
obs-pick-p3.d2 · [RETYPED] line 1: decision/session-over-unix-socket is no row of the pack — read as contract/viola-verify/home-tree-refused, the pack's row 1
obs-pick-p3.d2 · [RETYPED] line 2: contract/cli-exit-codes/3 is no row of the pack — read as contract/viola-revive/strict-modes-failed, the pack's row 2
obs-pick-p3.d2 · [RETYPED] line 3: decision/one-home-a-test is no row of the pack — read as contract/viola-link-unlink/strict-modes-failed, the pack's row 3
tests-pick-p1.d3 · [RETYPED] line 1: decision/session-over-unix-socket is no row of the pack — read as decision/no-child-screen-assertions, the pack's row 1
tests-pick-p1.d3 · [RETYPED] line 2: contract/cli-exit-codes/3 is no row of the pack — read as contract/cli-exit-codes/12, the pack's row 2
tests-pick-p1.d3 · [RETYPED] line 3: decision/one-home-a-test is no row of the pack — read as contract/cli-exit-codes/10-14, the pack's row 3
```
Re-spawns (3): `arch-pick-p1` `[EVERY_ROW]` (`decision/flat-cargo-workspace` unanswered) → passed in `pick-p1-r1`;
`obs-pick-p3.d2` `[EVERY_ROW]` (its first three lines foreign row names, 90+ rows unanswered) → passed in
`pick-p2-r1`; `tests-pick-p1.d3` `[ABSENT]` → its re-spawn `[EVERY_ROW]` with the same three foreign names → `dead
draws: tests-pick-p1.d3 [EVERY_ROW]`, its pack read on draws 1 and 2. Every `[EVERY_ROW]` sub-line is whole in
`.andromeda/runs/2026-11-28T00-46-52Z-phase/fanout-2026-11-28-console-scaffold.json`. The two foreign-name drafts
open with the same three lines (`decision/session-over-unix-socket`, `contract/cli-exit-codes/3`,
`decision/one-home-a-test`) — rows of no pack of this run.

**Every non-zero exit, refusal, `UNPARSED` / `INDETERMINATE` / `NOT DERIVED` line:**
- Baseline runs: entries 6, 7, 8 `red · exit 94` (`error: operator didn't match any binary names` — no
  `tui_console` binary yet); entry 9 `red · exit 1` (counts 0); entries 10 and 11 `red` on their last line (27, 6).
  All are the untouched tree's expected reds.
- Bash guard refusals, each re-issued once in the form it passes: two `cd` into a non-root directory (P2, P4), one
  pipeline tool's output piped into a filter (P3).
- `cargo deny check advisories --disable-fetch` exit 2 (an unknown flag, scratch probe; re-run as `cargo deny
  --offline check advisories`).
- No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed in this run.

**Every place a letter was wrong or silent:**
- The directive quotes a 0.1.0 stub «deferred to 0.2.0, technology not chosen»: no master row holds it (0 hits); the
  a11y history shows it was replaced by `N/A` lines at the 2026-11-08 census wrap. Scope corrected.
- The directive lists the roles as `run / hook / mcp / ui`: there is no `ui` role in code or the masters (the arch
  history shows a `ui-` role file planned at 2026-09-24 and never built). Scope corrected.
- The P3 closure named files the pinned scope did not (`tests/tui_console.rs`, the two schemas). I read them as
  concretizations of surfaces the pinned scope already named (the driver, the diagnostics role), so the row read was
  NOT re-run. The letter's «adds a file … the scope as pinned did not name» could be read to require the re-run.
- The seven history files (145 KB) were read by search for the chunk's terms, not whole once as fan-out.md says.
- During pick part 1 the orchestrator wrote scratch crates and a script under the session scratchpad (outside the
  repository tree the snapshot pair measures), to price the terminal candidates. The path check passed; the letter's
  «writes nothing» between snapshots was held only for the tree.
- `deny.toml`'s chrono advisory DB on the host is 2026-11-24's; the live advisory was read from GitHub on
  2026-11-28 and agrees.

## Beyond the plan
| # | Proposal the word would approve | Owner |
|---|---|---|
| 1 | M3's «the stamped-set table held once» (`cry-boujwuyr`'s second clause), left out until the overseer's Q2 answer names the table | this chunk, by revision, on Q2's answer; with no answer, «Version done-check» |
| 2 | F5's arm: the scaffold and `cry-boujwuyr` in one chunk, M1 and M3 last | this chunk (PROVISIONAL, his word owed) |

Apply? (yes / review / cancel)
