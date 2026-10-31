proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 36
    section: '## 1. Obs Scope Summary'
    change: 'Append to the entity: `viola_core::UnreachableCause` is the closed set of exit 21''s five cause words (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`); it has no serde path, and every word is written by the two clients.'
    sidecar: 'obs-scope-summary: viola-core also supplies UnreachableCause, the closed set of exit 21''s five cause words, no serde path.'
    rationale: 'Report **Symbols / APIs**: `viola_core::UnreachableCause`, "No serde path; every word is written by the two clients"; **Crates / modules**: `viola-core` stays pure.'
    basis: crates/viola-core/src/lib.rs:181-190
    fact: F11
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 159
    section: '## 1. Obs Scope Summary'
    change: 'In the `send`, `wait`, `last`, `answer`, `pause` and `release` clause, read: a refused check exits 21 with the instance pair, or under `--json` the `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` document, opens nothing under the home and writes no line.'
    sidecar: 'obs-scope-summary: the frame verbs'' refused-check document names strict-modes-failed; detail null retired.'
    rationale: 'Report **Retired**: the exit-21 document''s `detail: null`; **Symbols / APIs**: `refuse_untrusted` writes `StrictModes` through `unreachable_document`.'
    basis: src/cmd/client.rs:222-226
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 163
    section: '## 1. Obs Scope Summary'
    change: 'Append to the note: without `--json` the catch site''s `error: internal error` line stands byte for byte; under `--json` a catch-site fault leaves stderr empty and writes the `{"v":1,"error":"internal-error","detail":null}` document on stdout (`internal_error_document` in `src/cmd/mod.rs`, decided by `json_asked`).'
    sidecar: 'obs-scope-summary: the cli surface''s stderr note states the catch site''s --json form, stderr empty and the document on stdout.'
    rationale: 'Report **Symbols / APIs**, the `cli` catch site under `--json`: where `prints_internal_error` is true, `json_asked` prints the document on stdout in place of the line, stderr then empty.'
    basis: src/cmd/mod.rs:99-102
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 165
    section: '## 1. Obs Scope Summary'
    change: 'Append to the Founder Direction 4 note: the socket-directory cause and macOS''s server check are the named carve-outs from this rule, left with the route entry Unix endpoint and home hardening (the Founder Direction 4 trigger below states them).'
    sidecar: 'obs-scope-summary: the Founder Direction 4 note names the socket-directory and macOS carve-outs beside its strict-modes exception.'
    rationale: 'Report **Outcome**, `v1-18`: met with the named carve-outs (`inputs#I2` Q4), which leave the socket-directory cause and macOS''s server check unmet; the note''s sole exception would otherwise read as the only one.'
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 215
    section: '## 1. Obs Scope Summary'
    change: 'After `instance-unreachable` in the codes list, add: its `detail` one closed cause (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`), the CLI''s document less `v`, MCP''s text still `refused: instance-unreachable`.'
    sidecar: 'obs-scope-summary: the MCP surface''s instance-unreachable isError body carries its cause as detail.'
    rationale: 'Report **Symbols / APIs**, the exit-21 document moved and **MCP**: `unreachable(cause: UnreachableCause)`, MCP''s `isError` result the same less `v`, its text unchanged.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 302
    section: '## 1. Obs Scope Summary'
    change: 'In the exit-1 list, read: `already-live` (a live holder or a fresh replayed beat), `stale-heartbeat` (a stale holder), squatted-name, sha-256 mismatch, `.cmd`/`.bat` child, and for a start made by `viola revive` also no-session, session-live and cwd-missing.'
    sidecar: 'obs-scope-summary: the run start path''s exit-1 codes gain stale-heartbeat for a stale holder.'
    rationale: 'Report **Symbols / APIs**, a stale holder''s code: `collision_check` prints and logs `stale-heartbeat` for a `stale` holder, and `already-live` for a live holder and a fresh replayed beat; **Retired**: `already-live` as a stale holder''s refusal.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 329
    section: '## 1. Obs Scope Summary'
    change: 'In the refused instance check clause, read: CLI `answer` (`refuse_untrusted`, before any frame) exits 21 with the instance pair, or the `instance-unreachable` document with `detail` `strict-modes-failed` under `--json`.'
    sidecar: 'obs-scope-summary: CLI answer''s refused-check document names strict-modes-failed.'
    rationale: 'Report **Symbols / APIs**: `refuse_untrusted` writes `StrictModes` through `unreachable_document`, the one writer of the exit-21 document, and each cause its own `detail`.'
    basis: src/cmd/client.rs:222-226
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 381
    section: '## 1. Obs Scope Summary'
    change: 'End the line: a frame tool (`send`, `wait`, `last`, `answer`) whose instance check (`check_instance`, before `live_endpoint`) refuses returns `instance-unreachable` with `detail` `strict-modes-failed` and writes no frame; no `live_endpoint` (`without_endpoint`''s reading, `instance-dead` or `unwrapped-name`) and a failed server check (`server-verify-failed`) likewise write no frame and no line.'
    sidecar: 'obs-scope-summary: the MCP trigger''s no-line refusals each name a cause; detail null retired.'
    rationale: 'Report **Retired**: the exit-21 document''s `detail: null`; **Symbols / APIs**, **MCP**: `tools.rs` `frame`, a refused `check_instance` is `StrictModes`, no `live_endpoint` is `without_endpoint`''s reading, a failed `server_verified` is `ServerVerify`, "No frame and no line on any of them".'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 399
    section: '## 1. Obs Scope Summary'
    change: 'In the `update_stamps` clause, read: a refused `update_stamps` in `viola verify` (`StampsError::StrictModes`) leaves only verify''s own `process-exit` (with an instance; exit 1, `detail` `strict-modes-failed`).'
    sidecar: 'obs-scope-summary: verify''s refused stamp write now carries strict-modes-failed on its own process-exit.'
    rationale: 'Report **Symbols / APIs**, `verify`''s codes: `Ended::Refused` writes its code, `strict-modes-failed` among them, on the `process-exit` under a parsed `VIOLA_NAME`, a refused home tree staying lineless; **Deviations** 7 names the witness `verify_window_a_widened_stamps_file_refuses_with_its_code`.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan.md
    near: 215
    section: '## 1. Obs Scope Summary'
    change: 'Read: a strict-modes failure has a line only on the `ledger-stamps` read in `run`''s version gate, on a `conpty/` companion its owner/DACL check refuses (`process-start`''s `sideload_fallback:"strict-modes-failed"`, the child on the inbox ConPTY, exit unchanged) and on `verify`''s own `process-exit` for a stamp write it refuses (`detail` `strict-modes-failed`, exit 1).'
    sidecar: 'obs-plan §1: the Vector 7 broad line names verify''s process-exit among a strict-modes failure''s lines.'
    rationale: 'Report **Symbols / APIs**, `verify`''s codes: the broad line''s "only" no longer holds once `verify`''s `process-exit` carries `strict-modes-failed`.'
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 445
    section: '## 1. Obs Scope Summary'
    change: 'Read: endpoint vanished during `wait` (exit 21 / `instance-unreachable`, `detail` `endpoint-lost`, `during` `call`).'
    sidecar: 'obs-scope-summary: the chaos trigger''s vanished endpoint names its code, endpoint-lost with during call.'
    rationale: 'Report **Symbols / APIs**, a process-exit `during`: a wrapper lost with the call open is `endpoint-lost` · `during:"call"`; **Retired**: `instance-dead` · `during` `call`.'
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 490
    section: '## 1. Obs Scope Summary'
    change: 'Read the head: each cause behind exit 21 (`instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`) and exit 1 (`already-live`, `stale-heartbeat`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, and `viola revive`''s `no-session`, `session-live` and `cwd-missing`) gets a distinct `detail` code with no path or pid; and read `viola link` / `viola unlink`''s `--json` form as the `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` document, exit 21.'
    sidecar: 'obs-scope-summary: the Founder Direction 4 trigger lists the built exit-21 and exit-1 codes; link and unlink''s detail null retired.'
    rationale: 'Report **Symbols / APIs**, the five closed causes and a stale holder''s code; **Outcome**: exit 21''s five causes on the eight frame verbs met; **Retired**: the exit-21 document''s `detail: null`.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 491
    section: '## 1. Obs Scope Summary'
    change: 'Add a bullet after this one:   - Exception to the distinct-code rule, by the founder''s named carve-outs: the socket-directory cause has no code (`bind_endpoint`''s `create_private_dir` failure is an anyhow error at the catch site) and macOS has no server check, both left with the route entry Unix endpoint and home hardening; the row leaving the socket-directory code out is the delegate''s word, PROVISIONAL.'
    sidecar: 'obs-scope-summary: the Founder Direction 4 trigger records the socket-directory and macOS carve-outs; the socket-directory row PROVISIONAL.'
    rationale: 'Report **Spec claims disproved by measurement** (`v1-18`, the socket-directory refusal), **Outcome** (`v1-18` with the named carve-outs, `inputs#I2` Q4) and **Decisions** (`inputs#I3`''s Beyond-the-plan row, PROVISIONAL).'
    fact: F12
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 43
    section: '## 2. Telemetry Strategy'
    change: 'End the cell: writes no line and carries no logged `detail` code: its machine-readable form is the exit code, and under `--json` the closed `instance-unreachable` document with `detail` `strict-modes-failed` (the frame verbs and `link` / `unlink`) or the `strict-modes-failed` document (`revive`).'
    sidecar: 'telemetry-strategy: the Founder Direction 4 row''s no-code exception holds of the log; the frame verbs'' document names strict-modes-failed.'
    rationale: 'Report **Symbols / APIs**: the exit-21 document now carries the cause in `detail`; **Outcome**: no refused check gains a line or moves.'
note: F1 to F10 belong to log-coverage, span-trace-coverage and error-capture-reporting, so no entity of this group carries them; no scoped detector fired (no dependency moved, no hot path added, and the report's Coverage rows hold no path, pid or upstream text in a code); alignment gap (a) at the body's §1 was not picked here, and its "can be an additive `detail` field" is now built.
