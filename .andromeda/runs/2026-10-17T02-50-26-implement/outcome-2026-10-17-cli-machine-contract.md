# Implement outcome — 2026-10-17-cli-machine-contract

**Outcome: surfaced** — the block is green (18 of 18 non-leg entries, `pre-push` included), smoke ✓, and one
spec↔reality gap is surfaced (below). Nothing was authored in a spec. The operator pass ran on the operator's word
in this session's invocation: CI on the final HEAD `f738cbc` reads `verdict: green · checks 15/15`
(`ci#38999642102`), `test (ubuntu-latest)` among them.

## Gates
- Whole block, second firing: `entries 21 · green 18 · red 0 · recorded 0 · timeout 0 · not-run 3` (19-21
  `not run — leg operator`). Trail: `.andromeda/runs/2026-10-17T02-50-26-implement/gate-2026-10-17-cli-machine-contract.json`.
- First firing: red 3, 11, 18 — 3 the Windows-target lint (imports used only by a `cfg(unix)` case), 11 and 18 one
  case (`path1_run_start_one_home_in_order`: the test oracle `resolved()` of a home not created yet). Both fixed in
  test code; `--only 3,11` green, then the block green.
- No deferral: every gate ran.
- Smoke: entries 12-14 and 17 (cleanup, boot 4.49 s, status `state ready` / `last_error null`, cleanup
  `processes_gone` / `endpoint_gone`) ran as P2 gates; not re-driven.

## Part A — the last wrap's red CI (`ci#38994749835`)
- **Cause, at file:line** (`chunks/2026-10-17-cli-machine-contract/evidence/ci-red.md`): the capture arm claims
  `SessionStart.<k>.json` EMPTY (`src/cmd/hook.rs:390` `create_private_new`) and lands the bytes only after a write,
  `sync_all` and rename (`src/cmd/hook.rs:396` → `crates/viola-state/src/fs.rs:292`). Run E's wait tested a NAME
  count — `count_of(dir, HookEvent::SessionStart) > 0` (`src/cmd/verify/typed.rs:467-468`) — true on the empty claim;
  Run E then settled, killed the agent and read the captures (`src/cmd/verify.rs:253`), getting `payload: None`, so
  `session_resumed` (`crates/viola-agent-claude/src/ledger.rs:872-878`) failed. The window is the hook's disk write
  under load (two concurrent verify runs, a coverage build), not a shared input.
- **Fix**: `count_of` counts a capture only once its bytes landed (non-empty file). No bound raised, no retry, no
  test serialised.
- **Witness**: `captured_reads_no_capture_while_the_claim_is_empty` (typed.rs unit) red before the fix, green with
  it, red with the fix reverted in place, green restored. The plan's concurrent-start case
  `stamped_home_concurrent_verify_runs_each_stamp_the_home` read green 20/20 before the fix and 20/20 after
  (4.27-4.29 s, under the 10 s mutants kill): it does not open the window on this host's tmpfs home base. Its
  red-on-revert reading is unobtainable here by measurement; the runner leg is the operator pass's CI read.

## Operator pass (record: `chunks/2026-10-17-cli-machine-contract/evidence/operator-pass.md`)
- Entry 18 `pre-push` green (75.77 s) → entry 19 hygiene clean → pre-CI commit `e248533` → entry 20 push
  (`f6bcce5b→e2485333`) → entry 21 `ci#38998933306`: **red**, `test (windows-2025)` only, one unit,
  `cmd::tests::canonical_home_resolves_an_existing_home_and_keeps_a_new_one_absolute` (`src\cmd\mod.rs:347`): its
  oracle took Windows' raw `\\?\` canonical form where the product correctly returns the plain drive form. On that
  run `test (ubuntu-latest)` completed success.
- Fix in the test (the oracle through `plain_drive`, plus a no-prefix assertion); units, entries 1-3 and entry 18
  (74.38 s) green; hygiene clean after one runner drive path in the evidence was respelt; fix commit `f738cbc`; entry
  20 push (`e2485333→f738cbc0`); entry 21: `f738cbc0f307 verdict: green · checks 15/15 · wall 542 s · runs
  ci#38999642102 completed/success`.
- Part A's acceptance, last half: `test (ubuntu-latest)` green on the final HEAD.

## Files changed
- Product: `src/cmd/mod.rs` (`Instance` / `instance_of` moved here; `resolve_home` in the documented order with its
  inputs as arguments; `resolved_home`; `canonical_home`; `plain_drive`; revive `--list`'s `cli` detail sink),
  `src/cmd/hook.rs` (imports the moved check), `src/cmd/run.rs` (the home canonicalised in `start`;
  `statusline_source` compares the canonical default; `collision_check` takes `json`), `src/cmd/revive.rs` (`--json`
  on both arms, `ListRow`, `--list`'s `cli` log), `src/cmd/client.rs` (`refused_document`), `src/main.rs`
  (`role_of`: `revive … --list` → `cli`, a `--list` after `--` is the child's), `src/cmd/verify/typed.rs` (Part A),
  `crates/viola-state/src/replay.rs` (`scanned`), `schemas/diag-line.v1.json` (`scanned`), `schemas/config.v1.json`
  (new), `src/obs.rs` (test module: the `CONFIG_KEYS` pin).
- Tests: `tests/cli_machine_contract.rs` (new), `tests/cli_controls_not_disableable.rs`, `tests/cli_revive.rs`,
  `tests/cli_list.rs`, `tests/contract_diag_schema.rs`, `tests/cli_instance_state.rs`, `tests/cli_verify.rs`,
  `tests/support/{cli,home,start,outer_pty}.rs`.
- Ledger: `viola-0.1.0/verification-matrix.json` — `v1-39` `implemented`.

## Deviations from the plan, with justifications
1. **Step 3's premise** ("which shared input two simultaneous verify runs read"): no shared input decides it; the
   equality is the waiter's name count over an empty claim, and the shared resource is disk latency. The fix site,
   `src/cmd/verify/typed.rs`, is one the plan listed.
2. **Step 4's revert reading** holds for the forced unit case, not for the concurrent root case (green on this host
   either way). Forcing the window at the decision follows testing.md 2026-09-27.
3. **Step 12's `cli-<name>`**: the log is the CALLER's (`VIOLA_NAME`), as `list` and `verify` log (obs-plan §4 cli
   row); a case pins it with `VIOLA_NAME=driver` beside the instance `builder`.
4. **Step 15**: `setting_does_not_disable_the_budget_gate` gained `seven_day_over_one_hundred` and `v_two` so the `v`
   and `seven_day` rows are each tried by a case (the first three case labels and order kept). The `VIOLA_*` set is
   read by walking the product sources for `"VIOLA_…"` literals (marked `andromeda:walks-tree`).
5. **Step 16**: outcomes over one live wrapper's life are a labelled case table per world (each assertion names its
   label), not independent rstest rows: the outcomes are sequential (pause → refused send → release → stopped).
6. **Step 8**: `boot_with`'s outer PTY drops `VIOLA_*` through a new `OuterPty::spawn_without`
   (`tests/support/outer_pty.rs`, scope record: `in-intent · serves step 8 · self`); `revive_command` drops them too.
7. **`resolved()`** (a test oracle in `tests/support/home.rs`) spells a home not created yet as `run` will: its name
   under its parent's canonical spelling.

## Surfaced (spec↔reality — the wrap amends)
- **`viola revive --json` beyond the five codes.** architecture §Standard Contracts (CLI `--json` output, as amended
  by `inputs#I2`), plan step 10 and `v1-39`'s acceptance close revive's exit-1 documents at `strict-modes-failed` ·
  `already-live` · `no-session` · `session-live` · `cwd-missing`. Reality: after a passed preflight, `start()`
  (`src/cmd/run.rs`) can still refuse `batch-script-child`, `pinned-hash-mismatch` or `squatted-name` — under
  `--json` these print `run`'s human pair on stderr and no document — and a missing `claude` ends `internal-error`
  with no document. Not fixed in the impl: a document for them would carry a code outside the closed five. The
  "Exit-cause code catalogue" entry extends the same form; the wrap decides whether the gap is owned there.

## Verification matrix
- `v1-39` → `implemented`, ref: `tests/cli_machine_contract.rs` (six cases), `tests/cli_revive.rs::revive_json_*`,
  `::revive_list_json_*`, `::revive_list_under_a_name_logs_its_start_and_exit`,
  `tests/cli_controls_not_disableable.rs::every_flag_variable_and_config_key_is_a_row_of_the_table` and its rows,
  `src/cmd/mod.rs::tests::resolve_home_*`, `src/obs.rs::tests::config_keys_are_the_config_schema_properties`. The
  surfaced gap above stands beside it.

## Process hygiene
| process | started by | final state |
|---|---|---|
| the smoke session `p-cli-smoke` (supervisor, wrapper, fake agent) | a leg (entries 13, 17) | terminated — entry 17 `processes_gone:true`, `endpoint_gone:true` |
| every test-started `viola` / fake agent (the block, `pre-push` ×3) | this run (nextest) | terminated — `ps` at 2026-10-17T03:26Z lists no process whose `/proc/<pid>/exe` is under this repository |
| `viola` processes of `viola-lab/prototype` and the host's `claude` | not this run | left running — the overseer pair's and the operator's; not this run's to stop |

## Decisions & corrections
- Guard refusals: a `cat` heredoc with a file target (re-issued through Edit), a `cd` that would move the cwd (twice;
  re-issued as a subshell); a foreground `sleep` chained to a check (twice; waited on the completion notice instead).
- An evolve record stamped ahead of the clock was refused (`REFUSED ts`) and re-fired with the clock's own reading;
  a journal stamp ahead of the clock was caught by the stamp-ahead hook and corrected from `date -u`.
- A remove-the-guard `sed` anchored on pre-format text matched nothing after rustfmt reflowed the tuple (testing.md
  2026-09-25's recurrence); re-done on the reflowed line, both readings in `evidence/guards.md`.
- The operator's invocation for this run: "No live claude start in this chunk. The folded CI red goes first: name
  its cause at a file:line or stop and report after the plan's forced runs — never raise a timeout, retry or
  serialise to get green. Run each tool bare and read it as printed (the G1 guard). Journal each long run before you
  wait. Run the operator pass with the ci.py conclusion read (leg=operator); stop ready for the wrap." — the
  operator, 2026-10-17, in this session. No live `claude` start was made.

## Next
`/andromeda-wrap-session` — the operator pass is done (HEAD `f738cbc`, pushed, CI green). Uncommitted for the wrap's
commit: this run dir's last records (journal, outcome, evolve) and `evidence/operator-pass.md`'s CI section.
