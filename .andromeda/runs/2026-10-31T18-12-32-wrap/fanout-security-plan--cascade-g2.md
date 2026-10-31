proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 12
    section: '## Error Handling'
    change: 'Replace the clause after the colon with: `isError` `instance-unreachable` with `detail` `server-verify-failed`, text `refused: instance-unreachable`; `error` itself stays `instance-unreachable`, and it writes no log line.'
    sidecar: 'Error Handling: a failed MCP server check now reads `server-verify-failed` in `detail`; the null `detail` and the "adds no error word" clause are gone.'
    rationale: 'Report **Retired** names the exit-21 `detail: null`; **Symbols / APIs** MCP has a failed `server_verified` as `ServerVerify`, and **Counts** `no-own-code` marks this line changed.'
    basis: .andromeda/security-plan-detail/error-handling.md:12
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 13
    section: '## Error Handling'
    change: 'Replace `returns the same unreachable result: isError instance-unreachable with detail: null` with: returns the unreachable result, `isError` `instance-unreachable` with `detail` `strict-modes-failed` for a refused check and, for no live endpoint, `without_endpoint`''s cause, `instance-dead` for an instance directory and `unwrapped-name` for anything else; the text `refused: instance-unreachable` and no log line stand.'
    sidecar: 'Error Handling: an MCP frame tool''s refused check and no-endpoint result each name their cause in `detail`.'
    rationale: 'Report **Symbols / APIs** MCP: in `frame`, a refused `check_instance` is `StrictModes` and no `live_endpoint` is `without_endpoint`''s reading, with no frame and no line; **Retired** names the null `detail`.'
    basis: .andromeda/security-plan-detail/error-handling.md:13
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 22
    section: '## Error Handling'
    change: 'Replace the `--json` document with exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the exit, the instance pair, nothing opened and no line stand.'
    sidecar: 'Error Handling: the CLI''s strict-modes refusal on a frame verb now reads `strict-modes-failed` under `--json`.'
    rationale: 'Report **Symbols / APIs**: the exit-21 document now carries its cause, and `refuse_untrusted` writes `StrictModes` through `unreachable_document`; **Retired** names the null `detail`.'
    basis: .andromeda/security-plan-detail/error-handling.md:22
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan.md
    near: 413
    section: '## Error Handling'
    change: 'Replace `instance-unreachable` with `detail: null` with: `instance-unreachable` with `detail` its closed cause, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; every other clause of the line stands.'
    sidecar: 'Error Handling (body): MCP `instance-unreachable` names its cause in `detail`.'
    rationale: 'Report **Expected amendments** names `security-plan.md:413`; **Symbols / APIs** has MCP''s `isError` result as the CLI document less `v`, with `detail` one of the five causes.'
    basis: .andromeda/security-plan.md:413
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 37
    section: '## Input Validation'
    change: 'In the **Frame verbs** clause, the document printed under `--json` for a refused check becomes exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the rest of the clause stands.'
    sidecar: 'Input Validation: the frame verbs'' strict-modes refusal reads `strict-modes-failed` under `--json`.'
    rationale: 'Report **Retired** names the exit-21 `detail: null`; **Symbols / APIs** has `refuse_untrusted` writing `StrictModes`, and **Expected amendments** names `input-validation.md:37`.'
    basis: .andromeda/security-plan-detail/input-validation.md:37
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 37
    section: '## Input Validation'
    change: 'In the `viola mcp` sentence, replace `a refusal returns instance-unreachable with detail: null` with: a refusal returns `instance-unreachable` with `detail` `strict-modes-failed`; no frame and no line stand.'
    sidecar: 'Input Validation: an MCP frame tool''s strict-modes refusal reads `strict-modes-failed`.'
    rationale: 'Report **Symbols / APIs** MCP: in `frame`, a refused `check_instance` is `StrictModes`, with no frame and no line; a second occurrence of the retired null `detail` on this line.'
    basis: .andromeda/security-plan-detail/input-validation.md:37
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 38
    section: '## Input Validation'
    change: 'In the `revive` `--json` clause: `a refused strict-modes check or a preflight refusal` becomes `a refused strict-modes check or any refusal of the start it shares with run`; the five-code list becomes `one of the closed codes the next record lists`, `client::refused_document` kept; delete the clause that a refusal of the start after a passed preflight prints no document and its owed-to parenthesis.'
    sidecar: 'Input Validation: the `revive --json` post-preflight gap is closed and its owed clause removed.'
    rationale: 'Report **Retired** names the `revive --json` post-preflight gap, and **Symbols / APIs** says the sentence is no longer true; `v1-39` is met in **Outcome**.'
    basis: .andromeda/security-plan-detail/input-validation.md:38
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 75
    section: '## Input Validation'
    change: 'After `the two fixed unable:/hint: stderr lines with no path or pid`, add: under `viola revive --json` the document `{"v":1,"error":"batch-script-child","detail":null}` on stdout in place of the pair, stderr empty (`refused_with`), and a `claude` not found prints `internal-error`, each returning before `spawn_child`; `run` and the human pair stand as they were.'
    sidecar: 'Input Validation: a refused child under `revive --json` prints its document.'
    rationale: 'Report **Symbols / APIs** `run`''s start under `--json`: `refused_with` prints `refused_document` in place of the pair for `batch-script-child` and `internal-error` for a program not found, all before `spawn_child`; `run` passes false and the human pairs are byte for byte as they stood.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 24
    section: '## Error Handling'
    change: 'Keep `no line` for the refused home tree alone: `update_stamps`''s `StampsError::StrictModes` still exits 1 with the home pair, no step line and no summary, and its `process-exit`, under a parsed `VIOLA_NAME`, carries `strict-modes-failed`.'
    sidecar: 'Error Handling: `verify`''s stamps refusal now records `strict-modes-failed`; its refused home tree stays lineless.'
    rationale: 'Report **Symbols / APIs** `verify`''s codes lists `strict-modes-failed` on `process-exit` and says a refused home tree stays lineless, leaving the stamps refusal as the code''s site; deviation 7 names the witness `verify_window_a_widened_stamps_file_refuses_with_its_code`. This is a reading: the report does not name `update_stamps` here.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 22
    section: '## Error Handling'
    change: '- Exit 21''s `detail` is a closed set of five causes, `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`, the same on CLI `--json` and MCP `isError`, by the founder''s own word (`inputs#I2` Q1).'
    sidecar: 'Error Handling: new detail, the five exit-21 causes.'
    rationale: 'Report **Symbols / APIs**, the exit-21 document moved; the frame-verb refusal row is its natural neighbour.'
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 22
    section: '## Error Handling'
    change: '- `viola_core::UnreachableCause` has no serde path and every cause word is written by the two clients: `unreachable_document` is the CLI''s one writer of the exit-21 document, and `crates/viola-mcp/src/result.rs` `unreachable` is MCP''s.'
    sidecar: 'Error Handling: new detail, the cause type and its two writers.'
    rationale: 'Report **Symbols / APIs**: `viola_core::UnreachableCause`, `unreachable_document`, and MCP''s `unreachable(cause: UnreachableCause)`.'
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 24
    section: '## Error Handling'
    change: '- Where the `cli` catch site prints its internal-error line, `json_asked` (reading `--json` from argv before a `--`) prints the `internal-error` document with `detail` null on stdout instead, stderr empty; `run`, `revive`''s start, `mcp` and `hook` stay silent there.'
    sidecar: 'Error Handling: new detail, the catch site''s `--json` form.'
    rationale: 'Report **Symbols / APIs**, the `cli` catch site under `--json`; this row names the catch-site line.'
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 24
    section: '## Error Handling'
    change: '- Exit 1 tells a stale holder from a live one: `collision_check` prints and logs `stale-heartbeat` for a stale holder, while a frame verb against a stale instance reads `instance-dead`, stale heartbeat being exit 1''s cause alone.'
    sidecar: 'Error Handling: new detail, a stale holder''s code.'
    rationale: 'Report **Symbols / APIs**, a stale holder''s code; this row holds the start''s exit-1 refusals.'
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 22
    section: '## Error Handling'
    change: '- The `unwrapped-name` and `endpoint-lost` hints come last on stderr in plain ASCII, the only variable being the parsed `ViolaName`; the `endpoint-lost` wording is the delegate''s word, PROVISIONAL (`inputs#I2` Q3).'
    sidecar: 'Error Handling: new detail, the two cause hints.'
    rationale: 'Report **Coverage of new surfaces** and **Decisions & corrections**; this row holds the same verbs'' human refusal form.'
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 29
    section: '## Error Handling'
    change: '- `tests/cross_exit_causes.rs`, a root `cross_` binary requiring `fake-agent`, holds the exit-21 and exit-1 cause table, with `every_code_is_a_code_word`, `no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`, and `no_hint_names_release`.'
    sidecar: 'Error Handling: new detail, the exit-cause table.'
    rationale: 'Report **Changes** and **New text, by line** name the new file and its tests; it stands beside the error-body inventory.'
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 38
    section: '## Input Validation'
    change: ' **`revive --json` codes:** `viola revive --json`''s closed exit-1 codes are `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `internal-error`, with one document for every refusal of the start it shares with `run`.'
    sidecar: 'Input Validation: new detail, the closed `revive --json` codes.'
    rationale: 'Report **Symbols / APIs** `viola revive --json` lists the ten codes; the record stands under the `revive` arguments line, which points to it.'
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 38
    section: '## Input Validation'
    change: ' **`revive --json` errors:** an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints `internal-error` and returns the same error so the catch site still logs it, while a `pump_child` error prints no document because the child has held the terminal.'
    sidecar: 'Input Validation: new detail, `revive --json` on an anyhow error.'
    rationale: 'Report **Symbols / APIs** `viola revive --json` and **Reverted / negative API facts**.'
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 70
    section: '## Data Protection'
    change: ' **`verify`''s refusal codes:** `verify` writes `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` or `unclean-fixture` on its `process-exit` under a parsed `VIOLA_NAME`; a refused home tree stays lineless, a failing row is no refusal and no code, the fixture''s `<file>` enters no code, and `verify` takes no `--json`.'
    sidecar: 'Data Protection: new detail, `verify`''s refusal codes and the fixture name left out of them.'
    rationale: 'Report **Symbols / APIs** `verify`''s codes and **Coverage of new surfaces** (the fixture''s `<file>` enters no code); the row holds the recording refusal.'
    fact: F12
note: 'D-security-input, D-security-auth and D-security-deps found no drift: no new external input, no auth or secret change, no dependency; the error-handling line 24 proposal rests on a reading of the report, so confirm it against `verify`; F7, F8 and F9 are authentication-authorization facts, not this group''s.'
