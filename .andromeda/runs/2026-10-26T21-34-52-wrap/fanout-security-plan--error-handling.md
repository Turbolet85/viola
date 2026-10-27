proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 8
    section: '## Error Handling'
    change: 'In line 8 replace the clause «strict-modes is owed to "Home and code-bearing file integrity" (Decisions Log `2026-09-28`)» with «the strict-modes check runs first: the event and dialog paths take the canonical instance directory through `check_instance` before any `config.json` read, role file, stdin read or obs init, and a refused or absent instance (a `VIOLA_DIR` naming no existing directory included) exits 0 at once with no body, no line, no frame and no SessionEnd append (the founder''s own answer, `inputs#I3` Q2); the two `--capture` arms stay ahead of it, unchanged»; the rest of the line stands.'
    sidecar: 'error-handling: the hook''s dated strict-modes gap (Decisions Log 2026-09-28) retired; the event and dialog paths now run check_instance first and a refusal fails open with no line (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report **Retired** item (1) retires `viola hook`''s `hook.event` snapshot read without strict-modes; Symbols (src/cmd/hook.rs) states `instance_of(…).and_then(canonical).filter(trusted)` runs before `read_diagnostics_level` / `viola_obs_init`, a refused or absent instance returning exit 0 with no config read, role file, stdin read, line, frame or SessionEnd append, the capture arms unchanged; Decisions: no line on a refused check (`inputs#I3` Q2).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: '## Error Handling'
    change: 'New item after line 9: «- A `viola hook` event or dialog whose instance snapshot is replayed or absent at `connect` (`read_snapshot_or_replay`: only a read `snapshot.json` carries `endpoint`, `pid` and `started_at`) sends no frame and fails open as `channel-unreachable`, a SessionEnd appended through the direct `append_session_end` as before.»'
    sidecar: 'error-handling: the hook connect over a replayed or absent snapshot fails open channel-unreachable (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols (src/cmd/hook.rs): hook `connect` (224-226) reads `read_snapshot_or_replay`; `Replayed` / `Absent` → `NotSent::Unreachable` (fail open, `channel-unreachable`, SessionEnd''s direct append as before); Count 5 moves the reader onto the replay.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: '## Error Handling'
    change: 'New item after line 9: «- `viola hook statusline` (without `--capture`) over a replayed snapshot finds no `statusline_command`, so nothing runs and nothing prints; it exits 0 with empty stderr.»'
    sidecar: 'error-handling: hook statusline over a replayed snapshot runs and prints nothing (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols (src/cmd/hook/statusline.rs): `serve` (128-132) reads `read_snapshot_or_replay`; a replayed snapshot has no `statusline_command`, so nothing runs or prints (unit `statusline_over_a_replayed_snapshot_runs_nothing`).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 10
    section: '## Error Handling'
    change: 'New item after line 10: «- A `viola mcp` frame tool holds the instance to the strict-modes check (`check_instance`) before its liveness pre-check, and a refused check, like a snapshot that is replayed or absent (no live endpoint), returns the same unreachable result: `isError` `instance-unreachable`, `detail: null`, text `refused: instance-unreachable`, no frame and no log line (the founder''s own answer, `inputs#I3` Q2).»'
    sidecar: 'error-handling: viola mcp frame tools refuse a strict-modes-failed or replayed instance as instance-unreachable with no line (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols: `viola-mcp` `tools.rs` `frame` (337-339) runs `check_instance` before `live_endpoint`, a refusal returning `result::unreachable()` (`instance-unreachable`, `detail: null`), no frame, no line; `live_endpoint` returns `None` for `Replayed`, `Absent` or an error; Retired item (8).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 11
    section: '## Error Handling'
    change: 'New item after line 11: «- CLI `send` · `wait` · `last` · `answer` · `pause` · `release` hold the instance to the strict-modes check (`client::refuse_untrusted`, shared with `link` / `unlink`) before any frame: a refusal exits 21, prints in human mode the instance pair (`human::instance_state_pair`) through `human::refuse` and under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, opens nothing under the home and writes no line (the founder''s own answers, `inputs#I3` Q2 and Q3).»'
    sidecar: 'error-handling: the six CLI frame verbs'' strict-modes refusal (exit 21, the instance pair or the instance-unreachable document, no line) recorded (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols (src/cmd/client.rs): `refuse_untrusted` (20-41) runs `check_instance`; refused → `refused_document("instance-unreachable")` under `--json`, else `human::refuse` of `human::instance_state_pair(name)`, `Some(21)`, opening nothing and writing no line; called first by the six verbs; Retired items (4)-(7); Count 8.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 11
    section: '## Error Handling'
    change: 'New item after line 11: «- `viola run` and `revive`''s start refuse a home tree that fails the strict-modes check (`check_home_tree`) before the wrapper log opens: exit 1, the home pair for a home-level refusal and the instance pair for an instance-level one, no log, no line, no child (`revive --json` keeps `strict-modes-failed`); `viola verify` refuses its home tree before its `cli` log, and `update_stamps`''s `StampsError::StrictModes` maps to the same outcome: exit 1, the home pair, no step line, no summary, no line. On Unix a home under a regular file (unstattable) is such a refusal; on Windows that path stats `NotFound` and the `cli` catch-site line stands.»'
    sidecar: 'error-handling: run / revive / verify home-tree refusals (exit 1, the home or instance pair before any log) recorded (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols (src/cmd/run.rs `checked_exe` / `refuse_tree`, revive.rs:125-135, src/cmd/verify.rs 64-70 / 95-99 / 288-296); Count 9 (home pair) and Count 13 (an unstattable home a strict-modes refusal on Unix, the catch-site line on Windows); Deviation 8.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 11
    section: '## Error Handling'
    change: 'New item after line 11: «- The strict-modes errors use fixed messages naming the refused property, never a path: `Refused` gains `Readable` («a trusted state path is readable by another user») and `Symlink` («a trusted state path is a symbolic link»); `StampsError` (`StrictModes` · `State`) reads «the capability stamps failed the strict-modes check» / «… could not be written»; `PinError::StrictModes` reads «the pinned copy failed the strict-modes check».»'
    sidecar: 'error-handling: the fixed messages of Refused::Readable / Symlink, StampsError and PinError::StrictModes recorded (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols: `viola_state::strict` `Refused` gains `Readable` and `Symlink` with fixed messages; `viola_state::stamps` new `StampsError` with fixed messages (unit `stamps_error_messages_are_fixed`); `viola_state::pin` `PinError` gains `StrictModes`; Deviation 2.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 11
    section: '## Error Handling'
    change: 'New item after line 11: «- A `diagnostics/` folder, role file or detail file that fails `strict::check_diagnostics` (a symbolic link, or a mode or grant another user may use) is a failed open: the role writer ends at `std::io::sink` and a detail line is dropped, nothing narrowed or written before the check.»'
    sidecar: 'error-handling: a refused diagnostics path is a failed open (writer at sink, detail line dropped) (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols (src/obs.rs): private `check_diagnostics(dir, file)` (69-76) runs `strict::check_diagnostics` before `ensure_private_dir` in `open_role_file` and `write_detail`; a refusal is an `io::Error`, exactly a failed open (the writer ends at `std::io::sink`, a detail line dropped).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:20
note: D-security-input, D-security-auth and D-security-deps hold for this category (no new input surface, the strict-modes checks are the plan's own, Dependencies none); the body's Error Handling line 408 list of fixed-message error enums does not name StampsError (body, not this file).
