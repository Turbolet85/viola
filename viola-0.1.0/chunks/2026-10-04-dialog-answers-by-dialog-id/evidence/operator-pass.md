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
- Read 1 left one question open (next): which refusal fires (owner, an inherited write ACE to another SID, persistent ACLs). The refusal reason
  is not logged (the `parse-rejected` detail is the one code), and this Linux host cannot reproduce the runner. Leading
  hypothesis, unmeasured: an inherited allow ACE granting write to a non-trusted SID (e.g. Authenticated Users) from
  the runner's workspace drive.

## Measurement fix commit and CI read 2 — ci#37214281447 on `2080e3f` — RED (as expected), the cause MEASURED
- Before the commit: `viola-state` 103/103 on Linux; pre-push exit 0 (coverage 1370 passed); hygiene clean; scope clean.
- Commit `2080e3f`: `fix(2026-10-04-dialog-answers-by-dialog-id): operator fix after CI run 37213772796, a Windows
  strict-modes measurement test naming the refused ACE`; push `7f1364f..2080e3f`, exit 0.
- `ci.py conclusion`: `verdict: red · checks 15/15 · first-fail +270 s test (windows-2025)`; failed 1:
  `test (windows-2025)` (job 111471525011): 1404 run, 1394 passed, 10 failed — the 9 of read 1 plus the new
  `viola-state strict::win::tests::check_stamps_of_a_home_under_the_workspace_target_passes`.
- Its failure text (SIDs and codes only):
  - `ledger/`: `Err(Writable)`; user `S-1-5-21-…-500` (the runner's built-in Administrator); owner `S-1-5-32-544`
    (Administrators, trusted); persistent ACLs true; 7 allow ACEs:
    - Administrators `0x001F01FF` explicit · Administrators `0x001F01FF` inherited (flags 0x13) · SYSTEM `0x001F01FF`
      inherited · CREATOR OWNER `0x10000000` (GENERIC_ALL) inherited, inherit-only (0x1B) — all trusted;
    - BUILTIN\Users `S-1-5-32-545` `0x001200A9` (read and execute) inherited (0x13) — no write right;
    - **BUILTIN\Users `0x00000004` (FILE_ADD_SUBDIRECTORY / FILE_APPEND_DATA), flags 0x12 (container-inherit,
      inherited) — a write right to an untrusted SID;**
    - **BUILTIN\Users `0x00000002` (FILE_ADD_FILE / FILE_WRITE_DATA), flags 0x12 (container-inherit, inherited) — a
      write right to an untrusted SID.**
  - `ledger/stamps.json`: `Ok(())` — 3 inherited ACEs (Administrators, SYSTEM full; Users read and execute), no
    untrusted write.
  - the home folder: the same 7 ACEs as `ledger/`, also `Err(Writable)`.
- Cause, measured: the runner's workspace drive grants BUILTIN\Users "create files / write data" and "create
  folders / append data" as container-inherit ACEs; every folder viola creates under the checkout inherits them, so
  the Windows strict-modes write rule (security-plan §Authentication & Authorization, `~/.viola/` access control)
  refuses `ledger/` exactly as specified. The stamps file itself passes. The check is behaving as the spec writes it;
  what is missing is the spec's creation half — "A `--home` outside `%USERPROFILE%` gets an explicit protected user +
  SYSTEM DACL at creation (via windows-sys), or is refused" — owned by working-route `:111`.
- Not measured: a default home under `%USERPROFILE%` on Windows (the profile's protected inheritance would carry no
  Users write ACE; the runner's test homes never live there).

## Step 2 — the founder's live ruling (2026-10-04 ~16:40Z, relayed by the overseer; the measured cause shown)
- Word: "pull the creation half forward from :111, so a home outside %USERPROFILE% gets the explicit protected user +
  SYSTEM DACL at creation, exactly as security-plan words it, for the WHOLE home and not ledger/ only. The check is not
  weakened." Pattern: playbook "Boundary widening" — answered by the founder, live.
- Implemented in `viola_state::fs::create_private_dir` (scope record: `widening`, the word quoted): on Windows the
  topmost folder a call creates gets `D:P(A;OICI;FA;;;<user SID>)(A;OICI;FA;;;SY)` (protected, inherited by every
  folder and file beneath) when its canonical path lies outside the canonical profile folder
  (`GetUserProfileDirectoryW`; no environment variable read); a DACL that cannot be set fails the creation. A home
  inside the profile keeps its inheritance. `Cargo.toml`'s windows-sys pin gains the `Win32_UI_Shell` feature.
- Witnesses: the measurement test `strict::win::tests::check_stamps_of_a_home_under_the_workspace_target_passes`
  (must turn green) and the new `fs::tests::create_private_dir_outside_the_profile_sets_the_protected_owner_only_dacl`
  (exactly user + SYSTEM `0x001F01FF`, flags 0x03, strict-modes passes); the pure `outside_profile` decision tested on
  every OS.
- **Route pin owed to the wrap (implement never edits the route):** `:111` — "the creation half moved here: a home
  outside `%USERPROFILE%` gets the protected user + SYSTEM DACL at creation (chunk
  2026-10-04-dialog-answers-by-dialog-id, founder live 2026-10-04 ~16:40Z); `:111` keeps the rest of its Windows set
  (the per-entry-point checks of home, instances, bin, plugin, ui and the trusted files)". Expected amendment:
  security-plan §Authentication & Authorization `~/.viola/` access control — the creation half landed.
