# Implement outcome — 2026-11-14-status-line-interception-removed

**Outcome: stuck** — soft-exit for a re-plan, on the overseer's ruling (`inputs#I7`); the working tree is preserved.

Setup line that reads the phase run's approval trail (quoted from `python -X utf8 {tools_dir}/rec.py runs --root .
--kind '*-phase' --file '*-2026-11-14-status-line-interception-removed.json'`):

    2026-11-14T19-10-38-phase/matrix-2026-11-14-status-line-interception-removed.json · open · 2403 B

## What stuck

The plan's Test Commands hold two entries that no naming can satisfy together:

- the identifier probe (`grep -rn -E 'statusline|StatusLine|STATUSLINE|status_line|statusLine|BIN_SH|shell_argv|bin-sh|shell_exit_status' src crates tests schemas fuzz/fuzz_targets Cargo.toml .config`, expect `exit 1` + `no output`), matrix#v2-05's own acceptance;
- the two new nextest entries, whose `contains` atoms require five test functions the plan names with the probe's own
  tokens: `snapshot_of_a_status_line_era_writer_reads_whole_skipping_its_command`,
  `snapshot_written_holds_no_status_line_key`,
  `verified_reads_a_stamp_holding_the_status_line_rows_on_the_relied_rows_alone`,
  `relied_rows_hold_no_status_line_row` (all four `status_line`) and
  `hook_statusline_word_is_an_unknown_event_and_writes_nothing` (`statusline`). Those names also stand in the plan's
  steps 1, 2 and 4 and its Acceptance Criteria.

Asked as a technical fork, the overseer ruled (verbatim in `.andromeda/runs/2026-11-15T05-14-17-implement/relay-2.md`,
`inputs#I7`): stop for a re-plan through the phase door; the five test names take neutral words (the `retired_…`
family) in the steps, the `contains` atoms, the acceptance criteria and the matrix refs; nothing else in the plan moves.

Next → `/andromeda-phase` (its pending guard offers the plan's revision), then `/andromeda-implement` resumes on this
tree.

## What the tree holds (uncommitted, `git status` 59 files)

Done, per the plan's steps (the four state-crate/agent-crate new cases are written under the plan's current names, so
the revision renames them):

- **Step 1** — `crates/viola-agent-claude/src/statusline.rs` deleted; `pub mod statusline` gone; `SETTINGS_FLAG` at the
  crate root beside `PLUGIN_DIR_FLAG`; `LedgerRow::ALL` `[Self; 21]`; `ShellVerdict`, `StatusLineCapture`,
  `statusline_variants`, `STATUSLINE_DEFAULT`, the probe checks and file-name helpers gone; ledger tests restated at 21,
  the failed-row test replaced by the two new cases (plan names); `hook.rs`'s negative case reads the fixture's word.
  `cargo check -p viola-agent-claude --all-targets` was clean at that point.
- **Step 2** — `InstanceSnapshot.statusline_command` gone with every literal in the state crate; the two field tests
  replaced by the two new cases (plan names); board and `tests/state_replay.rs` edited. `cargo check -p viola-state
  --all-targets` clean.
- **Step 3** — `fixtures/viola-0.1.0/retired-words.json` gains five keys, named neutrally on the overseer's word
  (`inputs#I6`): `retired_snapshot_key`, `retired_ledger_rows`, `retired_hook_word`, `retired_source_file`,
  `retired_diag_subject` (values as step 3 gives them); `snapshot.json` gains
  `"statusline_command":"echo viola-fake-statusline"`.
- **Step 4** — `src/cmd/hook/statusline.rs` deleted; the arm, `--shell`, `STATUSLINE_DEADLINE`, `capture_statusline`
  and their tests gone. The new `tests/hook_fail_open.rs` case is NOT written yet (step 12 territory, and its plan name
  is one of the five).
- **Step 5** — `run.rs`'s source read, override write and snapshot record gone with their unit tests; `ChildLaunch`
  passes no settings; new case `child_launch_passes_no_settings_flag` in `src/run/mod.rs`; snapshot literals in
  `revive.rs`, `dialog.rs`, `send/slot.rs`, `wheel.rs` gone. `run.rs` still holds its now-dead
  `use viola_agent_claude::{StripPlan, statusline}` import and four unused imports (`File`, `Read as _`, `MAX_FRAME`,
  `replace_private`): not yet tidied. `run_writes_no_settings_file_in_the_instance_dir` (tests/) NOT written.
- **Step 6** — verify's Run B passes no `--settings`; the probe dir's status-line dir, `read_statusline`, the
  `--record` variants and their tests gone; step-line test at `[19/21]`…`[21/21]`; Runs C and D import `SETTINGS_FLAG`
  from the crate root.
- **Step 7** — `src/run/version_gate.rs`'s test restated on the 21-row relied set.
- **Step 8** — the fake agent's `statusline.rs` deleted; `statusline-echo`, `--statusline-stdin`, `run_statusline`,
  `replay_statusline` gone; `--settings` consumed and ignored; its tests (five modes, three opts and the helper) gone.
- **Step 9** — `boot --statusline-echo` gone from `viola-harness.rs`, `harness/boot.rs`, `harness/session.rs` and the
  literals; `LEDGER_ROWS`/`LIVE_PASS` at 21. `harness/boot.rs`'s `create_private_dir` import is now unused (not yet
  tidied).
- **Step 10** — `schemas/diag-line.v1.json` without `statusline-shell`, `statusline` and `shell_exit_status` (valid JSON).
- **Step 11** — the two `StatusLine.default-1.json`, the two `hook_stdin` seeds and the fixture schema deleted.
- **Step 12, partly** — `tests/hook_statusline.rs` and `tests/support/statusline.rs` deleted; the `[[test]]` and the
  nextest kill-line term gone. NOT done: `tests/support/{mod,home,fake,verify}.rs`, `tests/cli_links.rs` and every
  census / contract / cli / tui / security test file the step names, and the snapshot literal sites already cut in
  `tests/hook_fail_open.rs` and `tests/security_negatives_server_identity.rs` aside.
- **Step 13** — not done. Remaining probe hits at stop: 23 files (the tests/ files above, `src/cmd/run.rs`'s import,
  and the five plan-named test functions).
- **Step 14** — nothing owed.

The tree does not compile at stop (`tests/support/mod.rs` names the deleted module; `run.rs`'s dead import). No gate
was run: P2 and P3 did not start. `cargo fmt --all` ran once after the edit burst and exited 1 on that same missing
module; the crates it could parse are formatted.

No matrix ref was set (v2-05 stays `planned`).

## Files changed

52 listed files changed or deleted (`gate.py scope`: `changed 52 · listed 52 · recorded 0`, `scope: clean`); no
out-of-list edit, so no scope-record line.

## Deviations from the plan, with justification

- Step 3's key names: `statusline_*` → `retired_*` (values unchanged), on the overseer's ruling `inputs#I6`
  (`.andromeda/runs/2026-11-15T05-14-17-implement/relay-1.md`): the plan's names would have put the probe's token in
  the tests that read them.
- Step 1's removed-case count: the plan says "the eight statusline cases" of `ledger/test_cases_runs.rs`; nine
  `#[test]` functions there named or exercised the removed items (the three override, three shell,
  `shell_verdict_reads_only_the_exact_word`, the variants and the file-name round-trip) and all nine went.
- `crates/viola-e2e/src/harness/session.rs`'s default-options test renamed to
  `session_options_default_to_the_pair_recorded_and_stamped` (its old name named the removed echo flag); it now asserts
  `!opts.unstamped` in place of the removed field.

## Process hygiene

| process | started by | final state |
|---|---|---|
| none started (only `cargo check` / `cargo fmt` runs, each exited) | this run | `gate.py census --comm cargo --comm rustc --comm viola-fake-agent --args projects/viola/target/debug/viola`: `census: none running` |

## Decisions & corrections

- The overseer's technical answer, fork 1 (fixture keys): neutral `retired_*` keys, never a split string, which would
  game the probe — `inputs#I6`, relay-1.md.
- The overseer's technical answer, fork 2 (test names): stop for a re-plan; a gate or a `contains` atom left red is no
  outcome; the identifier probe is v2-05's acceptance and must read green as written — `inputs#I7`, relay-2.md.
- Guard refusal: the Bash guard refused one `cd` into a crate directory (it moves the session cwd); re-issued as a
  subshell `( cd … && … )`.
