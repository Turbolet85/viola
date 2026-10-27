# Cascade compare — security-plan · wrap · 2026-10-26-home-and-code-bearing-file-integrity
E items 83 · held 18 · lost at classify 4 · lost at strong 1 · beside a placed fact 0 · cascade only 2 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### authentication-authorization#3.7 · lost at classify · .andromeda/security-plan-detail/authentication-authorization.md:79
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 79
    section: '## Authentication & Authorization'
    change: 'After line 79 add a row: **Requirement:** `~/.viola/` access control, the tests holding the strict-modes controls; **Implementation:** `tests/security_negatives_strict_modes.rs` holds `frame_verb_refuses_a_widened_instance_before_any_log` (the six frame verbs, both modes, exit 21, no frame, no line, mode or ACL as found), `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps` (byte-identical after every other writer-capable entry point, the wrapper stopped), `a_start_under_umask_000_makes_every_home_path_owner_only` (Unix), `a_home_another_user_may_read_refuses_run` (Unix 0750, Windows a Users read grant) and `a_home_viola_makes_outside_the_profile_takes_the_protected_dacl` (Windows); beside them `handlers_frame_tools_refuse_a_widened_instance_before_any_frame`, `hook_refused_by_the_instance_check_fails_open_and_writes_nothing`, `hook_statusline_a_widened_instance_runs_and_logs_nothing`, `run_refuses_a_widened_home_tree_before_its_log`, `verify_refuses_a_widened_ledger_before_any_step`, `run_on_a_companion_another_user_may_write_falls_back_to_conhost` (`windows-2025`), `setting_does_not_disable_the_home_check`, and the `strict` unit tables (`unix_read_verdict_*`, `windows_read_verdict_*`, `check_home_tree_*`, `check_diagnostics_*`, the `cfg(windows)` cases); green on three OSes (ci#40403389629). **Stack reference:** `tests/security_negatives_strict_modes.rs`, `crates/viola-mcp/tests/handlers.rs`, `tests/{hook_fail_open,hook_statusline,run_cli,cli_verify,conpty_sideload,cli_controls_not_disableable}.rs`, `crates/viola-state/src/strict.rs`.'
    sidecar: 'Auth & Authz: the strict-modes control tests registered.'
    rationale: 'Report Changes **Files** (tests) and New text, by line (the named test functions); Outcome (security) and (tests) rows MET on ci#40403389629.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:134

### error-handling#1.2 · lost at strong · .andromeda/security-plan-detail/error-handling.md:9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: '## Error Handling'
    change: 'New item after line 9: «- A `viola hook` event or dialog whose instance snapshot is replayed or absent at `connect` (`read_snapshot_or_replay`: only a read `snapshot.json` carries `endpoint`, `pid` and `started_at`) sends no frame and fails open as `channel-unreachable`, a SessionEnd appended through the direct `append_session_end` as before.»'
    sidecar: 'error-handling: the hook connect over a replayed or absent snapshot fails open channel-unreachable (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols (src/cmd/hook.rs): hook `connect` (224-226) reads `read_snapshot_or_replay`; `Replayed` / `Absent` → `NotSent::Unreachable` (fail open, `channel-unreachable`, SessionEnd''s direct append as before); Count 5 moves the reader onto the replay.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16

### error-handling#1.2 · lost at strong · .andromeda/security-plan-detail/error-handling.md:9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: '## Error Handling'
    change: 'New item after line 9: «- `viola hook statusline` (without `--capture`) over a replayed snapshot finds no `statusline_command`, so nothing runs and nothing prints; it exits 0 with empty stderr.»'
    sidecar: 'error-handling: hook statusline over a replayed snapshot runs and prints nothing (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols (src/cmd/hook/statusline.rs): `serve` (128-132) reads `read_snapshot_or_replay`; a replayed snapshot has no `statusline_command`, so nothing runs or prints (unit `statusline_over_a_replayed_snapshot_runs_nothing`).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:17

### error-handling#1.3 · lost at classify · .andromeda/security-plan-detail/error-handling.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 10
    section: '## Error Handling'
    change: 'New item after line 10: «- A `viola mcp` frame tool holds the instance to the strict-modes check (`check_instance`) before its liveness pre-check, and a refused check, like a snapshot that is replayed or absent (no live endpoint), returns the same unreachable result: `isError` `instance-unreachable`, `detail: null`, text `refused: instance-unreachable`, no frame and no log line (the founder''s own answer, `inputs#I3` Q2).»'
    sidecar: 'error-handling: viola mcp frame tools refuse a strict-modes-failed or replayed instance as instance-unreachable with no line (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols: `viola-mcp` `tools.rs` `frame` (337-339) runs `check_instance` before `live_endpoint`, a refusal returning `result::unreachable()` (`instance-unreachable`, `detail: null`), no frame, no line; `live_endpoint` returns `None` for `Replayed`, `Absent` or an error; Retired item (8).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14

### error-handling#1.4 · lost at classify · beside a placed fact F20 (near 11, error-handling#1.4) · .andromeda/security-plan-detail/error-handling.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 11
    section: '## Error Handling'
    change: 'New item after line 11: «- CLI `send` · `wait` · `last` · `answer` · `pause` · `release` hold the instance to the strict-modes check (`client::refuse_untrusted`, shared with `link` / `unlink`) before any frame: a refusal exits 21, prints in human mode the instance pair (`human::instance_state_pair`) through `human::refuse` and under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, opens nothing under the home and writes no line (the founder''s own answers, `inputs#I3` Q2 and Q3).»'
    sidecar: 'error-handling: the six CLI frame verbs'' strict-modes refusal (exit 21, the instance pair or the instance-unreachable document, no line) recorded (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols (src/cmd/client.rs): `refuse_untrusted` (20-41) runs `check_instance`; refused → `refused_document("instance-unreachable")` under `--json`, else `human::refuse` of `human::instance_state_pair(name)`, `Some(21)`, opening nothing and writing no line; called first by the six verbs; Retired items (4)-(7); Count 8.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15

### error-handling#1.4 · lost at classify · .andromeda/security-plan-detail/error-handling.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 11
    section: '## Error Handling'
    change: 'New item after line 11: «- `viola run` and `revive`''s start refuse a home tree that fails the strict-modes check (`check_home_tree`) before the wrapper log opens: exit 1, the home pair for a home-level refusal and the instance pair for an instance-level one, no log, no line, no child (`revive --json` keeps `strict-modes-failed`); `viola verify` refuses its home tree before its `cli` log, and `update_stamps`''s `StampsError::StrictModes` maps to the same outcome: exit 1, the home pair, no step line, no summary, no line. On Unix a home under a regular file (unstattable) is such a refusal; on Windows that path stats `NotFound` and the `cli` catch-site line stands.»'
    sidecar: 'error-handling: run / revive / verify home-tree refusals (exit 1, the home or instance pair before any log) recorded (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols (src/cmd/run.rs `checked_exe` / `refuse_tree`, revive.rs:125-135, src/cmd/verify.rs 64-70 / 95-99 / 288-296); Count 9 (home pair) and Count 13 (an unstattable home a strict-modes refusal on Unix, the catch-site line on Windows); Deviation 8.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18

### error-handling#1.4 · lost at classify · .andromeda/security-plan-detail/error-handling.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 11
    section: '## Error Handling'
    change: 'New item after line 11: «- The strict-modes errors use fixed messages naming the refused property, never a path: `Refused` gains `Readable` («a trusted state path is readable by another user») and `Symlink` («a trusted state path is a symbolic link»); `StampsError` (`StrictModes` · `State`) reads «the capability stamps failed the strict-modes check» / «… could not be written»; `PinError::StrictModes` reads «the pinned copy failed the strict-modes check».»'
    sidecar: 'error-handling: the fixed messages of Refused::Readable / Symlink, StampsError and PinError::StrictModes recorded (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols: `viola_state::strict` `Refused` gains `Readable` and `Symlink` with fixed messages; `viola_state::stamps` new `StampsError` with fixed messages (unit `stamps_error_messages_are_fixed`); `viola_state::pin` `PinError` gains `StrictModes`; Deviation 2.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:11

### error-handling#1.4 · lost at classify · .andromeda/security-plan-detail/error-handling.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 11
    section: '## Error Handling'
    change: 'New item after line 11: «- A `diagnostics/` folder, role file or detail file that fails `strict::check_diagnostics` (a symbolic link, or a mode or grant another user may use) is a failed open: the role writer ends at `std::io::sink` and a detail line is dropped, nothing narrowed or written before the check.»'
    sidecar: 'error-handling: a refused diagnostics path is a failed open (writer at sink, detail line dropped) (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols (src/obs.rs): private `check_diagnostics(dir, file)` (69-76) runs `strict::check_diagnostics` before `ensure_private_dir` in `open_role_file` and `write_detail`; a refusal is an `io::Error`, exactly a failed open (the writer ends at `std::io::sink`, a detail line dropped).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:20

### security-anti-patterns#4.1 · lost at classify · .andromeda/security-plan-detail/security-anti-patterns.md:32
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/security-anti-patterns.md
    near: 32
    section: '## Security Anti-Patterns (NEVER do these) › ### Universal'
    change: '- NEVER let `viola verify` read or write `ledger/stamps.json` before the strict-modes check passes, and NEVER run that check after a create, mode set or log open that narrows the path checked (a widened `ledger/` narrowed to 0700 first would pass): `verify` runs `check_home_tree(home, None, None)` (the home, `ledger/`, `bin/`, `plugin/`) before its `cli` log, and `update_stamps` runs `strict::check_stamps(home)` before `create_private_dir(ledger/)` and again under the lock before its capped read; a refusal (`StampsError::StrictModes`) reads and writes nothing, and `verify` prints the home pair, exit 1, no step line, no summary.'
    sidecar: 'security-anti-patterns.md Universal: new ban on the stamps writer - verify and update_stamps check strict modes before any create, read or log open (chunk 2026-10-26-home-and-code-bearing-file-integrity, Retired item 2, Deviation 1).'
    rationale: 'The Universal stamps ban (line 31) names verify as the only writer but not the strict-modes order its write now keeps; the report Symbols (stamps: StampsError, check_stamps before create_private_dir and under the lock; verify.rs 64-70 check_home_tree before the cli log) and Deviation 1 (the check after the create narrows a widened ledger/ first, the CARRY 14 trap) bring it as built; Retired item 2 ends the update_stamps gap.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:11

## Held

- authentication-authorization#1.6
- authentication-authorization#1.8
- authentication-authorization#3.2
- authentication-authorization#3.3
- authentication-authorization#3.4
- bootstrap-phases#1.1
- data-protection#1.1
- error-handling#1.1
- input-validation#1.12
- input-validation#1.19
- input-validation#1.20
- input-validation#1.4
- input-validation#1.6
- security-anti-patterns#1.1
- threat-model-summary#2.1
- threat-model-summary#5.1
- threat-model-summary#7.1
- threat-model-summary#8.1

## Cascade only

- data-protection#3.1
- threat-model-summary#6.1
