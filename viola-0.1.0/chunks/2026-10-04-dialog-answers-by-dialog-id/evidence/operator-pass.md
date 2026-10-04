# Operator pass — 2026-10-04

## Before the pre-CI commit
- `bash scripts/agent-run.sh pre-push` (re-run before the push): exit 0, `"ok":true`, `"stage":"linux-tests"`,
  coverage suite 1370 passed / 0 failed.
- Entry `grep -c '^overseer review: approved' …/evidence/relayed-fixtures.md`: exit 0, last line `1`.
- Entry `gate.py hygiene`: exit 0, `hygiene: clean — read 64 (runs 57 · evidence 7)`.
- `gate.py scope`: `scope: clean — changed 59 · listed 42 · recorded 17`.
- The stray `fixtures/claude/2.1.288/` was moved out of the repository by the overseer before the commit.

## Pre-CI commit and push
- Commit `7f1364f0cb3340d19b48fc53839185e0656487cb`: `chore(2026-10-04-dialog-answers-by-dialog-id): operator pre-CI
  commit, for the run this chunk's verdict reads`.
- Push entry `git diff --quiet && git diff --cached --quiet && git push origin HEAD`: exit 0, `c540254..7f1364f`.

## CI read 1 — ci#37213772796 on `7f1364f` — RED
- `ci.py conclusion --sha HEAD --wait 1800`: `verdict: red · checks 15/15 · first-fail +263 s test (windows-2025) · wall
  275 s`; failed 1: `test (windows-2025)` (job 111470045714). Every other check green — `msrv` (red B) and
  `test (ubuntu-latest)` (red A) included.
- The Windows job: `Coverage and doctest (pwsh shim)`, 1403 run, 1394 passed, 9 failed:
  - `cli_version_gate`: `run_with_a_verified_version_records_cli_verified` (`cli_verified` false), `run_without_a_verified_stamp_degrades` ×2,
    `run_writes_nothing_of_its_own_on_any_gate_outcome::case_1_stamped`, `run_with_unreadable_stamps_logs_one_rejection`
    (`detail` read `strict-modes-failed`, expected `unreadable`);
  - `cli_instance_state path1_start_writes_state_before_the_spawn` ("the fixture home is stamped": false);
  - `viola-e2e harness_lifecycle harness_session_boots_reports_logs_and_tears_down` ("builder stamped by boot": false);
  - `cli_answer path4_dialogs_are_logged_once_woken_and_answered_by_id` ("viola never exited"),
    `path4_second_concurrent_dialog_is_left_to_the_human` (exit 12, expected 13).
- One cause, read from the assertions: on `windows-2025` the Windows strict-modes check (`viola_state::strict`,
  this chunk's step 11) refuses the `ledger/` of every test home, so every stamped home reads unverified. Test homes
  live under the checkout (`target/e2e-home/…`), outside `%USERPROFILE%`, with the runner workspace's inherited DACL.
- NOT measured: which refusal fires (owner, an inherited write ACE to another SID, persistent ACLs). The refusal reason
  is not logged (the `parse-rejected` detail is the one code), and this Linux host cannot reproduce the runner. Leading
  hypothesis, unmeasured: an inherited allow ACE granting write to a non-trusted SID (e.g. Authenticated Users) from
  the runner's workspace drive.
