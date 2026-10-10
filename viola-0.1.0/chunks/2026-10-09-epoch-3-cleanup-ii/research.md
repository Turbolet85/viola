# Codebase Research — 2026-10-09-epoch-3-cleanup-ii

## Scope
- **Depth:** deep · **Reads:** 31 · **Globs/Greps:** 22 (this take-up; the sibling chunk's `research.md` was read
  whole as a lead, and every count taken from it is re-derived below)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — whole, 11 Session Additions (never pipe
  `boot`; a mutation run's `TMPDIR` short and NOCOW on this host; judge a mutation run by its verdict, never by its
  counts against another run; `--in-diff` never regenerates an unchanged survivor line; a `test(=name)` filter needs
  the full path; a stalled-start red is a finding about the backing). `.claude/rules/testing.md` — whole, 35 Session
  Additions (no `[[gate]]` entry holds a mutation run, 2026-10-04; a guard that repeats what the call already
  guarantees is left out, 2026-09-24 ext. 2026-09-27; no reshaping to take a body out of the generated set,
  2026-09-28; every new guard test carries its remove-the-guard run, 2026-09-25; an I/O-error case must fail at the
  same call on every OS, 2026-09-24; a root test with no window prefix runs under the `mutants` profile's 10 s kill,
  2026-09-24 ext. 2026-10-06). `.claude/rules/host-linux.md` — whole.
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg. The
  two CI runs Setup read are both green (below).
- **External inputs:** `inputs#I1` — the operator's direction at take-up: no live `claude` session and no cap; a
  mutation run is a one-shot measurement with its record in `evidence/`, never a `[[gate]]` entry; a gate entry that
  only reads a record takes no `artifact` key; a preservation guard lists no file a step edits; the chunk is sized
  against one builder window with its machine time named, and a priced split card goes to P4 if the three CARRYs do
  not fit. `inputs#I2` — the P4 fork answers: the chunk stays one, with each mutation run's start, output directory
  and next step written to the run dir before the wait and re-read from disk when the run returns; all four
  survivors no test can kill are restated, each restatement naming its guarantee in its comment, the witness run
  over the file showing no new missed mutant.

## Files inspected
- `viola-0.1.0/working-route.md` (the taken-up entry, whole, 3764 chars) and
  `.andromeda/runs/2026-10-08T10-08-51-code-audit/proposals.md` (M1 to M3, F1 to F3, CARRY 1, the report-only run),
  `record.json` (`commands`, `tool_versions`), `c-mutation-viola-state.json`, `c-mutation-viola-agent-claude.json`,
  `c-mutation-viola.json` (each unit's `survivors` with its mutation text), `a13.py`, `cx.py`.
- `crates/viola-state/src/events.rs` (80-224) — `current_len` (`:93-99`, called by `end_offset` and
  `append_event_at`), `read_within` (`:140-168`, taking the cap as a parameter), `LoggedLines::next_line`
  (`:203-…`).
- `crates/viola-state/src/stamps.rs` (10-95) — `read_capped` (`:25-35`) feeds `update_stamps` and `read_stamps`;
  `read_stamps_strict` runs `strict::check_stamps` first.
- `crates/viola-state/src/strict.rs` (18-57) — `check_stamps` (`:29-39`): `symlink_metadata` per path, a
  `NotFound` skipped, any other error `Refused::Unreadable`.
- `crates/viola-state/src/fs.rs` (250-320) — `replace_private_shared` (`:288-293`) and its one test (`:300-308`),
  whose failing replace is over a directory, where the read fails too.
- `crates/viola-agent-claude/src/ledger.rs` (628-692, 862-915, 1140-1190) — `both_parallel_answered`,
  `parallel_both_before_first_post`, `clear_start`, `dialog_variants`, `is_local_char`, `has_email`.
- `src/bin/viola-fake-agent.rs` (316-505, and the `mod tests` at `:788`) — `close_hooks`, `run_hook`, `submit`.
- `src/cmd/mod.rs` (80-148), `src/obs.rs` (232-321), `src/cmd/client.rs` (1-100) — `cli_sink`, the catch site's
  `report_internal_error` and `write_detail`, and `client::start`.
- `src/cmd/hook.rs` (186-233, 736-885) — `handle_dialog` and its unit tests with the `served` / `wrapperish`
  fixtures.
- `src/cmd/verify.rs` (376-461) — `record`.
- `tests/cli_answer.rs` (1-170), `tests/cli_wait_last.rs` (1-200, 496-552), `tests/cli_wheel.rs` (1-120),
  `tests/tui_wheel.rs` (1-150), `tests/cli_send.rs` (1-175), `tests/cli_verify.rs` (960-1110),
  `tests/hook_events.rs` (54-72), `tests/support/mod.rs`, `watch.rs`, `piped.rs`, `ndjson.rs` (whole),
  `tests/support/home.rs` (its public items).
- `.config/nextest.toml` (whole), `crates/viola-e2e/src/harness/run/mutants.rs` (the package and file arm, by grep),
  every member's `[features]`.
- `.andromeda/test-plan.md:1212` (the Mutation gate paragraph, whole) and `:1149` (the Mutation row).
- The last chunk's plan (`## Test Commands`, whole) and its implement gate trail
  (`.andromeda/runs/2026-10-09T19-28-37-implement/`), for the standing entries and their walls.

## Graph impact (from the code-graph query; trace `tree-query-2026-10-09-epoch-3-cleanup-ii.json`, plane `rust`)
The graph's lines are zero-based; the lines below are the file's own.
- **`dialog_variants`** — 1 product caller: `record` @ `src/cmd/verify.rs:401`; 4 test call sites in `ledger.rs`
  (`dialog_variants_name_each_call_of_each_turn`, `dialog_variants_keep_the_first_capture_of_a_variant`). Its name
  and signature are architecture's; a split keeps both.
- **`record`** (`src/cmd/verify.rs`) — 1 caller: `measure` @ `:234`. It carries no span and no `obs_event!` call
  (the file's spans are `cli.verify` and `cli.verify_step`, outside it).
- **`submit`** — 2 callers, both in the fake agent: `run_steps` @ `:524`, `read_stdin` @ `:726`.
- **`sgr_attributes`** — 3 test call sites in `tests/cli_output_plain.rs` (`:72`, `:76`, `:104`).
- **`cli_sink`** — 6 call sites, all in `dispatch` @ `src/cmd/mod.rs:87`, `:92`, `:97`, `:102`, `:107`, `:112`.
- **`close_hooks`** — `read_stdin` @ `src/bin/viola-fake-agent.rs:728`, `:737`. **`run_hook`** — `fire_answered`
  @ `:366`.
- **`handle_dialog`** — `hook` @ `src/cmd/hook.rs:123` and 3 unit sites (`dialog_out` @ `:751`,
  `handle_dialog_unreadable_or_foreign_stdin_prints_nothing`, `handle_dialog_refuses_oversize_stdin_and_prints_nothing`).
- **`current_len`** — `end_offset` @ `events.rs:76`, `append_event_at` @ `:87`. **`read_within`** — `read_from` @
  `:137` and the test helper `read` @ `:462`. **`next_line`** — the iterator's `next` @ `:241`.
- **`read_capped`** — `update_stamps` @ `stamps.rs:49`, `read_stamps` @ `:58`. **`check_stamps`** —
  `read_stamps_strict` @ `:78` and 3 unit tests in `strict.rs`.
- **`replace_private_shared`** — `pin_exe` @ `crates/viola-state/src/pin.rs:82`, a second site in `pin.rs` @ `:172`,
  `pin_and_plugin` @ `src/cmd/run.rs:329`; its one unit test.
- **`both_parallel_answered`** — `check` @ `ledger.rs:767`. **`parallel_both_before_first_post`** — `measure` @
  `src/cmd/verify.rs:221` and one unit test (5 call sites). **`clear_start`** — `local_command_cleared` @ `:699`,
  `framing_variants` @ `:718`. **`has_email`** — `screen_text_fault` @ `:1155` and one unit test
  (`has_email_reads_local_at_domain_dot_tld`). **`is_local_char`** — `has_email` @ `:1178`.

## Measured facts (this take-up, 2026-10-09)
- **Source movement since the audit.** `git diff --numstat e304994 HEAD -- '*.rs'` prints nine files:
  `crates/viola-agent-claude/src/hook.rs`, `crates/viola-core/src/lib.rs`,
  `crates/viola-e2e/tests/harness_lifecycle.rs`, `src/cmd/send.rs`, `src/human.rs`, `src/run/send.rs`,
  `tests/channel_paste_validation.rs`, `tests/cli_send.rs`, `tests/contract_lints.rs`. None of the eight survivor
  files and none of the four files of §2 is among them, so each of those coordinates is the audit's.
- **Duplication at HEAD**, the audit record's command (`jscpd` 5.0.16, the same ignore list) into the session
  scratchpad: 3.36 %, 1814 duplicated lines of 53968, 205 clones over 122 sources, 119 pairs inside one file. The
  audit read 3.13 %, 1668 of 53261, 189 and 104. By lines a file takes part in: `src/run/send.rs` 394 (audit 254),
  `tests/cli_verify.rs` 363, `tests/cli_answer.rs` 336, `tests/cli_send.rs` 334 (audit 198), `tests/cli_wheel.rs`
  188, `tests/hook_fail_open.rs` 170, `tests/tui_wheel.rs` 150. The six unchanged files read the audit's own
  numbers, so the per-file method is the audit's.
- **The largest fragments at HEAD** are the audit's, at the audit's coordinates: `tests/cli_answer.rs:134` /
  `tests/cli_wait_last.rs:132` 29 lines; `cli_answer.rs:56` / `tui_wheel.rs:33` 27; `cli_verify.rs:979` / `:1067`
  27; `cli_answer.rs:56` / `cli_wheel.rs:36` 26; `cli_wheel.rs:68` / `tui_wheel.rs:102` 25; `cli_verify.rs:979` /
  `:1026` 24; `cli_wheel.rs:92` / `tui_wheel.rs:125` 21; and, new in the list, `cli_answer.rs:61` /
  `cli_send.rs:65` 19 and `cli_answer.rs:204` / `cli_wait_last.rs:228` 19.
- **The scaffolding copies in root tests** (re-derived: `grep -n -E '^(pub )?fn (events|wait_events|viola|…)\(|^struct
  (Running|Ran)' tests/*.rs`): an `events` reader in 7 files (`channel_paste_validation`, `cli_answer`,
  `cli_instance_state`, `cli_send`, `cli_wheel`, `hook_events`, `tui_wheel`); `wait_events` in 6 (`cli_answer`,
  `cli_send`, `cli_wait_last`, `cli_wheel`, `hook_events`, `tui_wheel`), five of them one predicate form and
  `hook_events`' a count form; `struct Running` with its `Drop` in 2 (`cli_answer`, `cli_wait_last`); `struct Ran`
  in 4; a `viola(…) -> Ran` runner in 3 (`cli_wait_last`, `cli_wheel`, `tui_wheel`). Six of the seven `events`
  copies call `support::ndjson::read_lines`; `cli_instance_state`'s goes through that file's own `lines` helper.
  Every wait copy uses `Watch` and `WITHIN`. Both `Drop` copies kill and wait the child and touch no home.
- **`src/run/send.rs`'s repeats** are 25 pairs, 209 lines, every one at or after `:532`, inside the inline test
  module that opens at `:507`; the largest is 18 lines.
- **`tests/cli_verify.rs`'s repeats** are 16 pairs, 189 lines; the five largest (27, 24, 17, 15, 12 lines) are one
  block, the read of the trusted run's typed prompts from the receipt, standing in three tests (`:979`, `:1026`,
  `:1067`).
- **Complexity at HEAD**, the audit record's command (`rust-code-analysis-cli` 0.0.25, 124 files, `cx.py`):
  `over_ceiling` 4, the audit's four at the audit's lines (`sgr_attributes` 23, `dialog_variants` 19, `record` 17,
  `submit` 16). Two functions read exactly 15 and are not over (`scan_file`, `check_screen`).
- **Mutants at HEAD** (`cargo mutants --list --package <member> [--file <path>]`, cargo-mutants 27.1.0, lines
  counted): `events.rs` 56, `stamps.rs` 26, `strict.rs` 69, `fs.rs` 54 (205 of viola-state's 269); `ledger.rs` 304
  (of viola-agent-claude's 464); `viola-fake-agent.rs` 132, `src/cmd/mod.rs` 4, `src/cmd/hook.rs` 74 (210 of the
  root package's 1050); `src/cmd/verify.rs` 45; `viola-e2e` 718.
- **Walls on record.** The audit, jobs 1: viola-state 1133 s for 269 (4.2 s a mutant), viola-agent-claude 644 s for
  462 (1.4 s), viola 16623 s for 1047 (15.9 s), viola-e2e 1314 s for 165 (8.0 s a mutant by division; the audit's
  own text says about 12 s a mutant and puts the unit at some 2.5 h). One whole-unit viola-e2e wall is measured: 78 min for 711 mutants
  (test-plan's sidecar, 2026-10-04-windows-boundary-mutation-workflow), taken before the gate maximum moved to 8.5 s.
  The last chunk's standing gate block ran in under 3 minutes warm (`run` 49 s, `pre-push` 58 s).
- **The mutation scratch stands:** `<repo parent>/viola-mutants-scratch`, `lsattr -d` reads the `C` (NOCOW) flag,
  511 G free on its volume.
- **The harness's package arm** (`crates/viola-e2e/src/harness/run/mutants.rs:145`, and its own unit test at
  `:842`): `--package <member> --features fake-agent [--file …]` with no `--in-diff`, after the root prebuild in
  `target/mutants`. Every member this chunk mutates declares `fake-agent` (`grep -n '^fake-agent' Cargo.toml
  crates/*/Cargo.toml`: the root, viola-agent-claude, viola-channel, viola-e2e, viola-pty, viola-state).
- **No cargo-mutants exclusion file exists** (`ls .cargo`: absent).
- **A path that holds a NUL byte** fails `metadata`, `symlink_metadata`, `File::open` and `fs::read` with
  `InvalidInput`, before any filesystem call (measured on this host with `rustc` 1.98.1, a four-line program in the
  session scratchpad). Not measured on Windows or macOS.
- **CI.** `f0a0dd1`: green, 15 of 15, wall 460 s, `ci#37991944489`. `14f1fb5` (HEAD): green, 15 of 15, wall 413 s,
  `ci#37993038172`, settled after Setup read it in progress.

## The eighteen survivors, as read at HEAD
Each row: the coordinate, the audit's mutation text, what the mutant changes, and the reading. "Kill" names the
observable a test in the owning package can assert.

| # | Site | Mutation | Reading |
|---|---|---|---|
| 1 | `events.rs:96:19` | `NotFound` guard → `true` in `current_len` | Kill: `current_len` over a path `metadata` fails on with another kind returns an error; the mutant returns 0. Both of its callers (`end_offset` `:73`, `append_event_at` `:85`) open the lock file first, so such a path fails there for the code and the mutant alike; the case calls `current_len` itself (found at P5's read of the frame). |
| 2 | `events.rs:149:19` | `NotFound` guard → `true` in `read_within` | Kill: `read_from` over such a path is an error; the mutant returns no lines. |
| 3 | `events.rs:209:24` | `<` → `>` in `LoggedLines::next_line` | **No test tells them apart.** A line with no `\n` and `len < cap` means the read reached the end of the file; the mutant then calls `skip_line`, which reads the end and breaks too. The guard matters only when a writer appends between the two reads. |
| 4 | `fs.rs:290:19` | content guard → `true` in `replace_private_shared` | Kill (Unix): a replace that fails over a file already holding the bytes (a parent directory without write permission) is done; the mutant is an error. The existing test's failing replace is over a directory, where the read fails as well. |
| 5 | `stamps.rs:28:19` | `NotFound` guard → `true` in `read_capped` | Kill: `read_stamps` over a path `File::open` fails on with another kind is an error; the mutant reads absent. The existing unreadable case uses a directory, which opens on Linux. |
| 6 | `strict.rs:34:23` | `NotFound` guard → `true` in `check_stamps` | Kill: `check_stamps` over such a path is `Refused::Unreadable`; the mutant passes. A stat error read as absent turns a refusal into a pass, so this one is a hole in a control. |
| 7 | `ledger.rs:647:81` | `==` → `!=` in `both_parallel_answered` | Kill: two calls `a`, `b` with PostToolUse captures for `a` and for a third id read false; the mutant reads true. |
| 8 | `ledger.rs:670:33` | `<` → `<=` in `parallel_both_before_first_post` | **Equivalent.** Each index is a PreToolUse's and `first_post` is a PostToolUse's index or the turn's length, so the two are never equal. |
| 9 | `ledger.rs:685:51` | `+` → `*` in `clear_start` | **Equivalent.** The capture at the paste's own index is a UserPromptSubmit, never the SessionStart searched for, so starting the search one earlier finds the same capture at the same absolute index. The `+ 1` repeats what the search already guarantees. |
| 10 | `ledger.rs:900:76` | `&&` → `\|\|` in `dialog_variants` | Kill: a PostToolUse whose id matches no call, beside a call of its tool that has an id, is not recorded; the mutant records it under that call. |
| 11 | `ledger.rs:1172:5` | `is_local_char` → `true` | Kill: a text whose `@` follows a space reads no email; the mutant reads one. |
| 12 | `ledger.rs:1181:74` | `==` → `!=` in `has_email` | Kill: an address with a hyphen in its domain reads an email; the mutant stops the domain at the hyphen and reads none. |
| 13 | `viola-fake-agent.rs:328:9` | `close_hooks` → `()` | Kill: a hook asked for after `close_hooks` is not run; the mutant runs it. Read through `run_hook`'s return in the bin's own test module. |
| 14 | `viola-fake-agent.rs:437:74` | `==` → `!=` in `run_hook` | Kill: with the receipt hold set, a `Stop` hook's `run_hook` takes at least the hold; the mutant returns without it. A lower bound only. |
| 15 | `viola-fake-agent.rs:473:46` | `&&` → `\|\|` in `submit` | **The guard is redundant.** `framing_stem(PROBE_LONG_PASTE)` is a compiled text's stem, so the equality alone already implies `framing.is_some()`. With the mutant every framed turn holds, which only an upper bound on a wait could show. |
| 16 | `viola-fake-agent.rs:473:57` | `==` → `!=` in `submit` | Kill: with the paste hint set, the long paste's turn takes at least the hold; the mutant skips it. A lower bound only. |
| 17 | `src/cmd/mod.rs:133:5` | `cli_sink` → `None` | Kill: a `cli` verb that fails inside a home that is a directory leaves one `process-exit` line with a non-empty `chain` in `instances/<name>/diagnostics/detail-cli.ndjson`; the mutant leaves none. `cli_verbs_print_internal_error_once` uses a home that is a regular file, where the sink can write nothing. This is obs code. |
| 18 | `src/cmd/hook.rs:214:72` | `>` → `>=` in `handle_dialog` | Kill: a dialog payload of exactly `MAX_FRAME` bytes is classified and its answer printed; the mutant refuses it as oversize and prints nothing. |

Read as a set: thirteen are killable by a test as they stand (1, 2, 4, 5, 6, 7, 10, 11, 12, 13, 14, 17, 18); two sit
on an expression that repeats what the code already guarantees (9, 15), and 16 is killable once 15's guard is gone;
two have no test that can tell the mutant from the code (3, 8).

## Patterns detected
- **One wait instrument for root tests** (`tests/support/watch.rs:14`, `:21-57`): `Watch` with the one bound
  `WITHIN` (7 s, below the `mutants` profile's 10 s kill), a streamed report, a panic that carries it. Every wait
  copy is this loop.
- **One boot model** (`tests/support/home.rs:366`, `Wrapper::boot`): the per-file `boot` copies differ only in which
  home they are handed and which extra arguments they pass, and each ends in the same wait for `session-start`.
  `Wrapper::boot` seeds the ConPTY companions on Windows x64, so a helper that calls it keeps the seed.
- **A child the test started sits in a guard** (`tests/cli_answer.rs:121-160`, `tests/support/piped.rs:108-115`):
  killed and waited on drop, its two streams read apart, its exit code kept beside them.
- **An I/O error forced on the real filesystem** (`crates/viola-state/src/fs.rs:300-304`, `stamps.rs:157`): a
  directory or a file planted where the other is expected.
- **Unit fixtures for the dialog tier** (`src/cmd/hook.rs:736-760`): `served`, `wrapperish` and `dialog_out` drive
  `handle_dialog` in-process over a real channel endpoint in a temp dir.
- **Evidence of a mutation run is a path-free record**: the harness's one JSON document and cargo-mutants' outcome
  lines carry repo-relative names; the tool's `outcomes.json` holds absolute argv paths and is never copied.

## Conventions to follow
- **Test names** `<subject>_<condition>_<expected>`; a case table is rstest `#[case::label]`; unit tests stay inline
  as `#[cfg(test)] mod tests`, and a helper split out of a function stays in the same file.
- **A kill test lives in the package that owns the mutated file** (cargo-mutants runs only that package's tests).
- **A route entry is named by its title**, never by a route line number.
- **Human lines only through `src/human.rs`**; no print macro and no new `allow` in a product crate; the fake agent's
  helpers stay in its one file, under its one crate-level allow (`src/bin/viola-fake-agent.rs:7`).
- **A mutation run on this host** is the harness arm, with `TMPDIR` set to the operator's NOCOW scratch.

## Scope premise closure
- Every `[inferred]` bullet of `scope.md` was closed against the reads above; `scope.md` is amended in the same pass.
- Verified, the tag dropped: the four parts of the operator's direction (against `inputs#I1`); both audit
  instruments, their commands and versions; the scaffolding copies and their counts (re-derived, above); the audit's
  direction for §2 and its four functions at their lines; `.andromeda/code-metrics.ndjson` as the boundary audit's
  ledger; the override order in `.config/nextest.toml`; the audit's report-only reading and the 718 (re-derived);
  the per-file mutant counts (re-derived); the absent exclusion file; the three survivors that sit inside functions
  §2 splits (`ledger.rs:900:76` in `dialog_variants` `:874-915`; `viola-fake-agent.rs:473:46` and `:473:57` in
  `submit` `:452-494`).
- Verified with its pointer corrected: the unit's baseline of 258 of 258 under the `mutants` profile stands in the
  sibling chunk's `report.md` (its gate line `258 tests run: 258 passed`) and its plan's gate entry; that chunk's
  `evidence/planted-hang.md` holds the kill and pass pair, not the count. No viola-e2e source moved since.
- Corrected: §1's numbers. Duplication at HEAD is 3.36 %, above the audit's 3.13 %.
- Corrected: §4's machine time. One whole-unit wall is on record, 78 min, older than the gate maximum's move; the
  audit's 2.5 h is an extrapolation from 165 of 718. The plan carries 2.5 h as its upper figure.
- Corrected: §5's run form. A bare `cargo mutants` over a root-package file with the default target dir drives the
  unmutated binary (`architecture.md` §Occupied Resources, `target/mutants/`); the witness is the harness arm
  `run --mutants --package <member> --file <path>`, which takes no `--in-diff` and reaches unchanged lines.
- Corrected: §5's "disposed with a recorded argument". `.andromeda/test-plan.md:1212` names one disposition that
  is not a kill: "A mutant this host cannot compile or reach … is recorded "not measured here; owed to {the route
  entry that measures it}" by coordinate, never "equivalent", and never counted as caught". A reachable survivor
  that is argued stays `missed` and reads red at the next boundary run. Four of the eighteen (3, 8, 9, 15) cannot be
  killed by a test as the code stands; what is done with them is a P4 fork.

## New files to create
- `tests/support/events.rs` — the one `events.ndjson` reader and wait for root tests, and the boot that waits for the session-start record
- `tests/support/cli.rs` — the one started-child guard and run-and-wait runner for root tests

## Files to modify
- `tests/support/mod.rs` — the two new modules declared
- `tests/cli_answer.rs` — its boot, reader, wait, guard and runner copies replaced by the shared ones
- `tests/cli_wait_last.rs` — its wait, guard and runner copies replaced, and the detail-file case for a failing verb
- `tests/cli_wheel.rs` — its boot, reader, wait and runner copies replaced
- `tests/tui_wheel.rs` — its boot, reader, wait and runner copies replaced
- `tests/cli_send.rs` — its boot, reader and wait copies replaced
- `tests/cli_instance_state.rs` — its reader copy replaced
- `tests/hook_events.rs` — its reader copy replaced
- `tests/channel_paste_validation.rs` — its reader copy replaced
- `tests/cli_verify.rs` — one in-file helper for the trusted run's typed prompts
- `crates/viola-agent-claude/src/ledger.rs` — the split of the variant naming, the kill cases, the two restated expressions
- `src/cmd/verify.rs` — the split of the recording function
- `src/bin/viola-fake-agent.rs` — the split of the prompt submit, the redundant guard removed, the kill cases
- `crates/viola-state/src/events.rs` — the kill cases for the two absent-file guards, the torn-line guard restated
- `crates/viola-state/src/stamps.rs` — the kill case for the absent-file guard
- `crates/viola-state/src/strict.rs` — the kill case for the absent-path guard
- `crates/viola-state/src/fs.rs` — the kill case for the content guard
- `src/cmd/hook.rs` — the kill case at the frame bound

## The forks, as answered at P4 (`inputs#I2`)
- **The chunk stays one.** No split; the lists above are the chunk's. The operator's note binds the long runs: each
  mutation run's start, its output directory and the next step are written into the run dir before the wait, and
  read back from disk when the run returns, because an idle session may be compacted after about 55 minutes.
- **All four are restated** (rows 3, 8, 9 and 15 of the table), behaviour unchanged, every body still in the
  generated set. Each restatement names, in its comment, the guarantee it rests on in the same function: the read of
  at most `cap` bytes one line above the events guard; a capture being never both a PreToolUse and a PostToolUse;
  the search that already skips the paste's own capture; the compiled long paste always having a stem. The witness
  run over each restated file must show no new missed mutant.
- Leaned at P4, each shown on the review card: `sgr_attributes` is not split and is recorded as standing (the
  entry's title counts three, and the audit's M2 proposal directs the split at the three entrants);
  `src/run/send.rs`'s repeats are left where they are (all 25 pairs sit in its inline test module, whose case counts
  three earlier plans pin, and the entry's title asks for the scaffolding shared across files); a mutant the
  `viola-e2e` score reads missed or timed out is listed by name and not killed here (the entry's title asks for the
  score, and `inputs#I2` fixed the chunk's size); `src/cmd/verify.rs` joins the root witness run, since its split
  adds functions the next boundary run will mutate (45 more mutants, about 12 minutes).

## Open questions
- none — the two plan decisions were answered at P4 (`inputs#I2`).
