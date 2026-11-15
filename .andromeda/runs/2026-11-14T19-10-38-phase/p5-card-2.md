# P5 card 2 — 2026-11-14-status-line-interception-removed «Status-line interception removed»

The plan to read in an editor: `.andromeda/cache/docs/2026-11-14-status-line-interception-removed/plan.md` (rev 4, its
render). No plan stands in a chunk's folder: writing in that file changes no record until a `put`. A diff of the
chunk's records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-14-status-line-interception-removed`.

## Goal
viola stops intercepting the status line: no override, no `--settings` from `run`, no `statusline_command` in the
snapshot, no source read, no `hook statusline` arm (the word becomes an unknown hook event, exit 0 silent), no
`/bin/sh -c` — viola runs no shell; verify drops Run B's status-line probe, relied rows 23 → 21; the fake agent's
status-line modes, `boot --statusline-echo`, the `StatusLine` fixtures and schema, two `hook_stdin` seeds and the log
words leave. Every own-format `v` stays 1 (`inputs#I3`, the founder's own word, relayed by overseer1). `--settings` moves to the
agent crate's root (Runs C and D keep it).

## Top acceptance criteria
- matrix#v2-05 — `child_launch_passes_no_settings_flag`, `run_writes_no_settings_file_in_the_instance_dir`,
  `hook_statusline_word_is_an_unknown_event_and_writes_nothing`, the identifier probe (entry 9) exits 1 with no output,
  `relied_rows_hold_no_status_line_row` + `contract_ledger_probes` at 21.
- compat — `snapshot_of_a_status_line_era_writer_reads_whole_skipping_its_command`,
  `snapshot_written_holds_no_status_line_key`, `verified_reads_a_stamp_holding_the_status_line_rows_on_the_relied_rows_alone`;
  the old-reader direction measured, not run.
- verify — 21 step lines, `stamped <ver>  21 pass  0 fail`; Runs C and D still pass `--settings`.
- the hook's event/dialog arms, the TUI zero-viola-bytes comparison: unchanged.

## Touchpoints
71 files (research.md «Files to modify»; 10 deletions: `crates/viola-agent-claude/src/statusline.rs`,
`src/cmd/hook/statusline.rs`, `src/bin/viola-fake-agent/statusline.rs`, two `StatusLine.default-1.json`, two
`hook_stdin` seeds, `schemas/claude-statusline-fixture.v1.json`, `tests/hook_statusline.rs`, `tests/support/statusline.rs`). No new file. No control is owed (step 14: the chunk adds no product guard).

## Test commands
29 entries: 21 read `seen green` from chunk 2026-11-14-budget-gate-removed's wrap (byte-equal run and expect), 5
baselined now, 3 operator (hygiene, push, CI read). No live step.

## Concretization preview — matrix#v2-05 (the never-weaken checkpoint)
- **Now:** «A user's own status-line command shows in a wrapped session exactly as it does unwrapped; viola runs no
  shell for any purpose; the relied ledger set holds no status-line row.»
- **Proposed:** a start hands its child no `--settings` (`child_launch_passes_no_settings_flag`) and writes no
  `settings.json` in its instance directory (`run_writes_no_settings_file_in_the_instance_dir`), so the CLI reads its
  status line from the user's own settings exactly as unwrapped — by construction: viola writes nothing under `~/.claude`
  and adds no settings layer; `viola hook statusline` is an unknown hook event that exits 0 and writes nothing; the
  identifier probe over `src crates tests schemas fuzz/fuzz_targets Cargo.toml .config` exits 1 with no output (no
  `BIN_SH`, `shell_argv`, source, override or fixture); the relied set is 21 rows, none a status-line row.
- **What this does not run:** a live reading of a real `claude` status line wrapped vs unwrapped (no live start this
  chunk, `inputs#I1` item 3). The «exactly as unwrapped» clause rests on the by-construction argument above, and the
  live reading is owed as a CARRY on «Live round on the trimmed product» (`inputs#I4`; «Beyond the plan» row 4).

## Directive item 9 — the block

**The union, per master** (`rec.py rows-union`, as printed):
```
architecture: rows 965 · packs 10 · picked 115 (72 by every draw) · standing 13 · 35080 B
security-plan: rows 610 · packs 7 · picked 82 (48 by every draw) · standing 10 · 24841 B
design-system: rows 313 · packs 4 · picked 45 (18 by every draw) · standing 11 · 11280 B
layout-templates: rows 221 · packs 3 · picked 20 (9 by every draw) · standing 2 · 4281 B
test-plan: rows 1340 · packs 14 · picked 196 (107 by every draw) · standing 47 · 50089 B
obs-plan: rows 1049 · packs 11 · picked 90 (44 by every draw) · standing 28 · 23782 B
a11y-plan: rows 237 · packs 3 · picked 18 (9 by every draw) · standing 28 · 6731 B
rows-union: 7 master(s) · picked 566 of 4735 · rule any
```

**Every row the plan cites is in a union rows file or a `*-standing.md`:** yes — checked handle by handle; the
`workspace.repo` exception rows the plan names (`security-plan · decision/fixture-hygiene`, `decision/fixture-scrub-and-refusal`)
stand in `security-standing.md` S3; `test-plan · decision/test-function-naming` in `tests-standing.md` S1; every other
cited handle is a picked row. None in neither.

**`[RETYPED]` lines:** one — `tests-pick-p3.d3 · [RETYPED] line 76: decision/hook-stdin-test-layer is no row of the
pack — read as decision/hook-stdin-test-layers, the pack's row 76` (the pick-p2 check).
**`[EVERY_ROW]` re-spawns:** none. Re-spawns of other checks: `arch-pick-p4.d3` `[ABSENT]` (pick-p1-r1, passed);
`layouts-history` `[CITES] UNRESOLVED: 2026-10-16-resume-and-compact-ledger-rows — history:22` (history-r1, passed;
the raw twin `.raw-layouts-history.md`).

**Non-zero exits, refusals, `UNPARSED` / `INDETERMINATE` / `NOT DERIVED`:**
- The Bash guard refused two calls: a `cd` into the run dir (re-issued with absolute paths) and a `rec.py render` piped
  into `tail` (re-issued bare, by `rec.py find` + `get`).
- The stamp-ahead hook refused the plan's first `Generated:` stamp, an estimate (19:58:00Z against a 19:52:45Z clock);
  rewritten from `date -u`. A recurrence of the session learning on stamps.
- `gate.py baseline` outran the Bash tool's 120 s foreground limit on its first firing (moved to the background, exit
  0); its second firing ran in the background from the start.
- Entries 9, 10, 11, 12 baselined red — each the red the plan predicts (below). No `UNPARSED`, `INDETERMINATE` or
  `NOT DERIVED` line was printed by any tool this run.

**Where a letter was wrong or silent:**
- CARRY cry-dc3um6ng's coordinate `statusline.rs:124` is a test line; the shell-out is `BIN_SH` (:17) and `shell_argv`
  (:68). The scope closed it.
- The version census maps C-057 (`proptest-regressions/statusline.txt`) and C-070 (`chrono` in the agent crate) to
  v2-05; neither stands at HEAD — both already left with the budget removal. Closed `premise-corrected`.
- My own slips, corrected before this card: research first left `crates/viola-agent-claude/src/hook.rs` out of «Files
  to modify» while step 4 edits it (caught by entry 9's baseline, added — research rev 2, 71 items); the plan first set
  `new = true` on 15 entries byte-equal to the previous chunk's (dropped; the listing then read them `seen green`); a
  first file comparison read research's «Files inspected» list as well as «Files to modify» (redone on the list alone).

**A new KEY of a master's keyed section, or a NOT YET MEASURED piece of a mark:** neither is needed.

## Builds and test runs (directive item 4), each fired once, never in parallel
- P3: none. One code-graph query (`scripts/code-graph.py query`, 0 s, the plane read as it stood).
- P5 baselines: entry 11 `cargo nextest run -p viola-state -p viola-agent-claude --no-fail-fast` — 20:01:31Z →
  20:02:09Z, 36.8 s, 824 passed, red: the four named cases absent (this chunk's names). Entry 12 `cargo nextest run -p
  viola --features fake-agent --bin viola --test hook_fail_open --test cli_instance_state --no-fail-fast` — 20:02:17Z →
  20:02:49Z, 31.3 s, 1095 passed, red: the three named cases absent. Entries 1, 9, 10 (`df`, the identifier grep, the
  `find`): no build, 0.01 s each — 9 red over 65 files, every one a research item; 10 red on the five paths step 11
  deletes.

## The steps a `v` bump would have added (kept for the record; the founder's word, `inputs#I3`, chose no bump)
- the snapshot envelope written at `v` 2 (`crates/viola-state/src/snapshot.rs` writer) and its reader taking `v` 1 and
  `v` 2 alike (`contract/snapshot-envelope/v` reads «above 1 is unsupported» today);
- the stamps envelope at `v` 2 (`merge_stamp`, `ledger.rs`) and `verified()` / the version gate taking both;
- a compatibility case per envelope over a `v` 1 file and one over a `v` 2 file; the 0.1.0 shape fixtures kept as the
  `v` 1 side;
- the expected amendment changes from widening `decision/protocol-version-bump-rule`'s except to amending
  `contract/snapshot-envelope/v` and `contract/ledger-stamps/*`.

## Expected amendment on the founder's own word
`architecture · decision/protocol-version-bump-rule` — its except widened to this removal (the snapshot's
`statusline_command`, the stamps' two row ids): his own answer to this chunk's P4 fork, «То же исключение
(Recommended)» at 2026-11-14T19:52:54Z, relayed by overseer1 (`inputs#I3`), ratifying the delegate's PROVISIONAL
option 1 (`inputs#I2`). The wrap's card shows him the widened except's text.

## Beyond the plan
- **The box-only screen fixture CARRY (cry-hqyx2rou)** — not recorded this chunk (a live `claude` start, no per-chunk
  cap given; `inputs#I1` item 3). Proposed owner: the existing entry «Live round on the trimmed product», as a CARRY
  beside its `Mcp.tools-list-1.json` re-record.
- **The `workspace.repo` exception rows** — `test-plan · decision/fixture-hygiene-walk`, `test-plan ·
  decision/recorder-scrub`, `test-plan · decision/no-production-data-in-tests`, `security-plan · decision/fixture-hygiene`,
  `security-plan · decision/fixture-scrub-and-refusal`: each row's except loses its subject once the StatusLine variants
  leave (no fixture holds a `workspace.repo`). Proposed: the wrap retires those except clauses, rows kept otherwise.
- **The live home's stale status-line files** — `~/.viola/instances/*/settings.json` overrides and any
  `~/.viola/statusline-source.json`, read by nothing after this chunk: the operator's at the desk, not before the
  pinned product binary is re-pinned (no route entry; as `~/.viola/budget.json`).
- **The live wrapped-vs-unwrapped status-line reading** — v2-05's «exactly as unwrapped» is proven by construction
  here; the reading on a real `claude` (a wrapped session's status line beside an unwrapped one's) is owed as a CARRY
  on the existing entry «Live round on the trimmed product» (`inputs#I4`).

## Folded from card 1's review (`inputs#I4`, the delegate overseer)
- v2-05 as proposed; the live reading a fourth «Beyond the plan» row (above).
- The three earlier «Beyond the plan» rows as carded.
- The `v` fork's authority re-pointed to the founder's own word (`inputs#I3`) in the plan's Goal, its compat
  criterion, entry 11's note, Provenance, Constraints and the architecture Expected amendment (plan rev 4).
- The whole P5 check set re-ran after the fold: the fence parses (dry-run exit 0), planlint hits 0, the lists parse
  (71 items), `inputs.py verify` 4 entries clean (I1, I3 unchanged; I2, I4 messages), the baseline listing re-read
  (no run or expect changed, so no entry re-baselined and no build or test ran).

Apply? (yes / review / cancel)
