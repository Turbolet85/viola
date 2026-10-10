# Scope — 2026-10-09-epoch-3-cleanup-ii · Epoch 3 cleanup II

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry, under
`### Epoch 4 — Session state & governance`): Epoch 3 cleanup II — test scaffolding shared once, three functions within
the cognitive ceiling, viola-e2e whole-unit score, eighteen Linux survivors disposed — plus three CARRY blocks (all
three folded below; completeness check `route.py pins` → the run dir's trail: three rows on the entry, 1534 · 1123 ·
943 chars, no abstention).

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- **no live `claude` session**: the plan holds zero, and there is no cap to spend;
- **a mutation run in this chunk is a one-shot measurement with its record in `evidence/`, never a `[[gate]]`
  entry**: the mutation gate is the boundary's;
- a gate entry that only reads a record takes no `artifact` key, and a preservation guard lists no file a step edits;
- the chunk is sized against one builder window and its machine time is named; if the three CARRYs do not fit, a
  priced split card goes to the operator at P4. A split is the founder's word, so nothing leaves this scope before
  it.

**Settled at P4** (`inputs#I2`, the operator's answers in the fork dialog, verbatim in the snapshot):
- **the chunk stays one**: no split, all three CARRYs. Before each long mutation run its start, its output directory
  and the next step are written into the run dir, and read back from disk when the run returns;
- **the four survivors no test can kill are restated**, behaviour unchanged: `ledger.rs:685:51`,
  `viola-fake-agent.rs:473:46`, `ledger.rs:670:33`, `events.rs:209:24`. Each restatement names its guarantee in its
  comment, and the witness run over the file shows no new missed mutant. No survivor is disposed by argument.

**Where the earlier reads stand.** This entry was split off chunk `2026-10-09-epoch-3-cleanup` on the founder's word
(2026-10-09T15:27:46Z, the third snapshotted input of that chunk). That chunk's `scope.md` §1, §2, §3 and §5 and its `research.md` hold
the reads made for this work, with coordinates at `59e791e`. Two chunks landed after that commit. Re-read at P1
(`git diff --numstat e304994 HEAD -- '*.rs'`): nine `.rs` files moved — `crates/viola-agent-claude/src/hook.rs`,
`crates/viola-core/src/lib.rs`, `crates/viola-e2e/tests/harness_lifecycle.rs`, `src/cmd/send.rs`, `src/human.rs`,
`src/run/send.rs` (258 added, 35 removed), `tests/channel_paste_validation.rs`, `tests/cli_send.rs` (202 added, 5
removed), `tests/contract_lints.rs`. No other `.rs` file moved, so a coordinate in any other file is the audit's own.

## What this chunk builds

### 1. Audit M1 — test scaffolding shared once
- Source: the Epoch 3 boundary audit, `.andromeda/runs/2026-10-08T10-08-51-code-audit/proposals.md` §M1, read at
  `e304994` (CARRY 1; placed at the 2026-10-09 0-pending wrap on the operator's answer, relay
  `e3-route-adaptation.md` item 1).
- The measured movement at the audit: duplication 1.35 % → 3.13 % (clones 53 → 189; 1668 duplicated lines of 53261).
  By duplicated lines a file takes part in: `tests/cli_verify.rs` 363, `tests/cli_answer.rs` 336, `src/run/send.rs`
  254, `tests/cli_send.rs` 198, `tests/cli_wheel.rs` 188, `tests/hook_fail_open.rs` 170, `tests/tui_wheel.rs` 150.
- The largest cross-file fragments are per-file copies of one scaffolding, and their shared home is `tests/support/`
  (present at P1: `fake.rs`, `home.rs`, `hygiene.rs`, `mod.rs`, `ndjson.rs`, `outer_pty.rs`, `piped.rs`, `verify.rs`,
  `watch.rs`):
  - a child-run-and-wait helper (`tests/cli_answer.rs:134` / `tests/cli_wait_last.rs:132`);
  - a wrapper boot that waits for `session-start`, with an `events()` reader (`tests/cli_answer.rs:56` /
    `tests/tui_wheel.rs:33` / `tests/cli_wheel.rs:36`).
- 104 of the 189 pairs sit inside a single file; the audit names `tests/cli_verify.rs` and `src/run/send.rs` as
  candidates for one helper each.
- [premise-corrected: research.md §Measured facts — the audit's command re-run at HEAD] The scalar at HEAD is not the
  audit's: 3.36 %, 1814 duplicated lines of 53968, 205 clones, 119 pairs inside one file. `src/run/send.rs` takes
  part in 394 lines and `tests/cli_send.rs` in 334; the other five named files read the audit's numbers. The named
  cross-file fragments stand at the audit's coordinates.
- The copies, re-derived at HEAD (research.md): an `events` reader in 7 root test files (`channel_paste_validation`,
  `cli_answer`, `cli_instance_state`, `cli_send`, `cli_wheel`, `hook_events`, `tui_wheel`), `wait_events` in 6,
  `struct Running` with its `Drop` in 2 (`cli_answer`, `cli_wait_last`), `struct Ran` in 4, a `viola(…) -> Ran`
  runner in 3. Six of the seven readers call `support::ndjson::read_lines`; `cli_instance_state`'s goes through that
  file's own helper. Every wait copy uses `Watch` and `WITHIN`.
- `src/run/send.rs`'s 25 pairs all sit in its inline test module; `tests/cli_verify.rs`'s five largest are one block
  standing in three tests (research.md).
- Done is read on the audit's own instrument: the duplication scalar re-taken with the audit record's duplication
  command (`record.json` `commands.duplication`, `jscpd` 5.0.16) over the same file population, lower than the
  reading at HEAD, with the named cross-file fragments gone from its list. The entry states a direction and no
  target number; P4 named both: below the audit's own 3.13 %, and no clone pair of 20 lines or more left either
  between two of the eight lifted test files or inside `tests/cli_verify.rs` (7 such pairs at HEAD).
- Leaned at P4: `src/run/send.rs`'s repeats are left where they are. All 25 pairs sit in its inline test module,
  whose case counts three earlier plans pin; the entry's title asks for the scaffolding shared across files.
  `tests/cli_verify.rs` takes its one in-file helper.
- Lifting shared scaffolding moves no assertion: every test that used a per-file copy asserts what it asserted
  before.

### 2. Audit M2 — three functions within the cognitive ceiling
- Source: the same audit §M2 (CARRY 1). Three functions entered over cognitive 15 in Epoch 3, each re-read at P1 at
  the line the entry names (none of the three files moved since `e304994`):
  - `dialog_variants` 19 (`crates/viola-agent-claude/src/ledger.rs:874`);
  - `record` 17 (`src/cmd/verify.rs:388`);
  - `submit` 16 (`src/bin/viola-fake-agent.rs:452`).
- `sgr_attributes` 23 (`tests/cli_output_plain.rs:36`, re-read at P1) stood over the ceiling at the Epoch 2b baseline
  too. The entry's title counts three, so it is named here and is not one of the three. Leaned at P4: it is not
  split and is recorded as standing, so `over_ceiling` is expected to read 1 after this chunk.
- The audit's direction: split each of the three where its branches already separate. No behaviour changes.
- Done is read on the audit's own instrument: `complexity.over_ceiling` re-taken with the audit record's complexity
  command (`record.json` `commands.complexity`, summarizer `cx.py`, `rust-code-analysis-cli` 0.0.25), the three no
  longer over 15. Re-taken at HEAD: 4 over, the audit's four at the audit's lines.

### 3. Audit M3 — both scalars rose at each of the last two boundaries
- Source: the same audit §M3 (CARRY 1): `duplication.pct` 1.29 → 1.35 → 3.13; `complexity.over_ceiling` 0 → 1 → 4.
- No separate work: §1 and §2 are its subjects. Its own statement in this chunk is the two re-taken scalars, recorded
  in `evidence/` beside the three earlier records' values.
- The chunk writes no record into `.andromeda/code-metrics.ndjson`: that ledger is the boundary audit's.

### 4. Audit F1, the score's half — `viola-e2e` whole-unit score on the dev host
- Source: the same audit §F1 and its report-only run (CARRY 2). `viola-e2e` is still unscored.
- "That chunk removed the kill: the harness test is
  `verify_window_boot_with_an_unknown_cli_version_is_verify_failed`, the `verify_window_` override stands first in the
  `mutants` profile, and the unit's unmutated baseline reads 258 of 258 with no TIMEOUT under that profile on the dev
  host (measured at that chunk, its gate entry and `evidence/planted-hang.md`)". Re-read at P1 in
  `.config/nextest.toml`: the `verify_window_` override is the first `[[profile.mutants.overrides]]` table, 15 s × 3,
  and `package(viola-e2e)` the second. The count stands in that chunk's `report.md` and its plan's gate entry; its
  `evidence/planted-hang.md` holds the kill and pass pair. No viola-e2e source moved since (research.md).
- [premise-corrected: research.md §Measured facts — one whole-unit wall is on record] "The boundary tier's form,
  `run --mutants --package viola-e2e`, has returned no score yet: the audit's report-only run tested 165 of 718
  mutants in 1314 s and put the whole unit at about 2.5 h" (of the 165: 139 caught, 0 missed, 0 timeout, 26
  unviable; files graded whole: `viola-harness.rs`, `boot.rs`, `cleanup.rs`, `mod.rs`). The unit lists 718 mutants
  at HEAD. Beside the audit's extrapolation stands one measured whole-unit wall, 78 min for 711 mutants
  (2026-10-04), taken before the gate maximum moved to 8.5 s. The plan carries 2.5 h as the upper figure.
- This chunk takes the whole-unit score on the dev host: the boundary tier's form run once over the whole unit, its
  harness document and cargo-mutants' outcome lines copied into `evidence/`. Per `inputs#I1` it is a one-shot
  measurement and no `[[gate]]` entry carries it.
- What the score reads is not known before the run. A mutant it reads missed or timed out is listed in `evidence/`
  by name. Leaned at P4: this chunk does not kill such a mutant; the entry's title asks for the score, and the
  report names each one for the wrap to place on the route.
- The unit's Windows grade (the 68 mutants) is the next entry's.

### 5. Eighteen Linux survivors, each killed or disposed
- Source: the same audit's Linux survivor table (CARRY 3; relay item 4, split by host on the operator's answer). 18 of
  the 24 survivors on `x86_64-unknown-linux-gnu` are this chunk's. Each is killed by a test or disposed with a recorded
  argument. None of the eight files below moved since `e304994`.
- viola-state, 6: a `NotFound` match guard replaced by `true` at `crates/viola-state/src/events.rs:96:19`,
  `events.rs:149:19`, `stamps.rs:28:19` and `strict.rs:34:23` (the last missed on the Windows runner too);
  `events.rs:209:24`, `<` → `>` in `LoggedLines::next_line`; `fs.rs:290:19`, the content guard of
  `replace_private_shared`.
- viola-agent-claude, 6: `crates/viola-agent-claude/src/ledger.rs:647:81`, `:670:33`, `:685:51`, `:900:76`, `:1172:5`,
  `:1181:74`.
- viola, 6: `src/bin/viola-fake-agent.rs:328:9`, `:437:74`, `:473:46`, `:473:57` (a test-side bin of the root package);
  `src/cmd/mod.rs:133:5`; `src/cmd/hook.rs:214:72`.
- [premise-corrected: architecture §Occupied Resources (`target/mutants/`) and `mutants.rs:145`] This chunk's
  evidence for them is the harness arm `run --mutants --package <member> --file <path>` over each survivor's file
  in its owning package, its document and outcome lines copied into `evidence/`. The arm takes no `--in-diff`, so
  it reaches unchanged lines; a bare `cargo mutants` over a root-package file would drive the unmutated binary. Per
  `inputs#I1` each run is a one-shot measurement and no `[[gate]]` entry carries it.
- The counts, re-derived at HEAD (`cargo mutants --list`, cargo-mutants 27.1.0): `events.rs` 56, `stamps.rs` 26,
  `strict.rs` 69, `fs.rs` 54 (viola-state: 205 of the unit's 269); `ledger.rs` 304 (of viola-agent-claude's 464);
  `viola-fake-agent.rs` 132, `src/cmd/mod.rs` 4, `src/cmd/hook.rs` 74 (root package: 210 of 1050). The audit's
  whole-unit walls: viola-state 1133 s, viola-agent-claude 644 s, viola 16623 s (about 16 s a mutant).
- [premise-corrected: `.andromeda/test-plan.md:1212`] No cargo-mutants exclusion file exists (`.cargo/` is absent),
  and the test plan gives an argued survivor no standing: its one disposition that is not a kill is "not measured
  here; owed to {the route entry that measures it}", for a mutant this host cannot compile or reach, "never
  "equivalent"". A reachable survivor that is argued stays `missed` and reads red at the next boundary run.
- As read at HEAD (research.md, the table): thirteen of the eighteen are killable by a test as the code stands; two
  sit on an expression that repeats what the code already guarantees (`ledger.rs:685:51`,
  `viola-fake-agent.rs:473:46`), and `:473:57` is killable once that guard is gone; two have no test that can tell
  the mutant from the code (`events.rs:209:24`, `ledger.rs:670:33`). Settled at P4 (`inputs#I2`): all four are
  restated, so every one of the eighteen ends caught by a test or gone from the generated set.
- §2 splits `dialog_variants` and `submit`, and three survivors sit inside them (`ledger.rs:900:76`,
  `viola-fake-agent.rs:473:46`, `:473:57`). The split moves their coordinates, so those three are matched by mutation
  text after it, and the order of §2 and §5 on those two files is a plan decision.
- `strict.rs:34:23` is listed in both this entry's Linux table and the next entry's Windows list (missed on both
  hosts). This chunk kills or argues it on Linux; its Windows grade stays the next entry's.

## Excluded — owed to other route entries
- The other six Linux survivors, in `replace_private_with`'s retry loop: "Windows mutation grade".
- The `viola-e2e` Windows grade (68 mutants), the Windows workflow's ceiling and its fifteen Windows-side survivors
  (audit F2, F3): "Windows mutation grade".
- Any live `claude` session (`inputs#I1`).

## Boundaries
- No wire contract, refusal, exit code, event or snapshot shape changes. §1 moves test scaffolding, §2 splits
  functions in place, §5 adds tests and restates four expressions with no change of behaviour.
- A helper split out of a function stays in the same file; unit tests stay inline.
- Nothing left this scope: the chunk was not split (`inputs#I2`).

## CI read at Setup (the last wrap's flip `f0a0dd1` → HEAD `14f1fb5`)
- `f0a0dd16ceb3`: green, 15 of 15 checks, wall 460 s, `ci#37991944489`.
- `14f1fb5f588d` (the setup re-run, U02): in progress at Setup, 15 checks listed, `ci#37993038172`; it was not read
  as green and was not folded. Re-read at P3: green, 15 of 15 checks, wall 413 s.
