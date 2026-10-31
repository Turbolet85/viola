proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 30
    section: '## 4. Span / Trace Coverage'
    change: 'Replace the refused instance check''s `--json` form `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the rest of the line stands (exit 21, the instance pair, no process log, no line).'
    sidecar: 'span-trace-coverage: the CLI instance check''s exit-21 `--json` document names `strict-modes-failed`, `detail` null retired.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols says `refuse_untrusted` writes `StrictModes` through `unreachable_document`, `v` still 1 and no exit moved; the Outcome has no refused check gaining a line.'
    basis: src/cmd/client.rs:222-226
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 33
    section: '## 4. Span / Trace Coverage'
    change: 'Replace `(a refusal returns `instance-unreachable` with `detail: null`, no frame and no line)` with `(a refusal returns `instance-unreachable` with `detail` `strict-modes-failed`, no frame and no line)`.'
    sidecar: 'span-trace-coverage: MCP `check_instance` refusal carries `detail` `strict-modes-failed`, `detail: null` retired.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols, MCP: `tools.rs` `frame` reads a refused `check_instance` as `StrictModes`, the `isError` result the CLI document less `v`, no frame and no line.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 33
    section: '## 4. Span / Trace Coverage'
    change: 'Replace `returns the codes-only `isError` `instance-unreachable` with no frame and no log line, as the unreachable path does` with `returns the codes-only `isError` `instance-unreachable` with `detail` `server-verify-failed`, no frame and no log line, as the unreachable path does with its own cause`.'
    sidecar: 'span-trace-coverage: MCP server-check failure names `server-verify-failed` in its `isError` body, each unreachable path its own cause.'
    rationale: 'Report Symbols, MCP: a failed `server_verified` is `ServerVerify`, and no `live_endpoint` is the reading of `without_endpoint`; no frame and no line on any of them, `isError` the record, so the body no longer reads alike across causes.'
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 57
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'Replace `detail` ∈ `already-live|squatted-name|pinned-hash-mismatch|batch-script-child` with `detail` ∈ `already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|batch-script-child` (`already-live` a live holder or a fresh replayed beat, `stale-heartbeat` a stale holder).'
    sidecar: 'span-trace-coverage: the start''s failure `detail` set gains `stale-heartbeat`, `already-live` retired as a stale holder''s code.'
    rationale: 'Report Retired names `already-live` as a stale holder''s refusal; Symbols: `collision_check` prints and logs `stale-heartbeat` for a `stale` holder and `already-live` for a live holder and a fresh replayed beat; the span `outcome` at the attributes entity stays unmoved.'
    basis: schemas/diag-line.v1.json:78
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 57
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'In the `viola revive` preflight''s refusals, replace `(and `already-live`, from the same collision check)` with `(and `already-live` or `stale-heartbeat`, from the same collision check)`.'
    sidecar: 'span-trace-coverage: a revive''s preflight collision check logs `stale-heartbeat` for a stale holder.'
    rationale: 'Report Symbols: the codes of `viola revive --json` now include `stale-heartbeat`, from the `collision_check` it shares with `run`; the witness is `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` in the New text section.'
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 78
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'Add after this line the sub-bullet: `  - `viola_state::liveness::without_endpoint` reads a no-endpoint instance by `symlink_metadata`: a directory is `instance-dead`, anything else (absent, a regular file, a symlink) `unwrapped-name`; it opens nothing and writes no line, and the strict-modes check stays first at every caller (CLI `Unreachable::without_endpoint`, MCP `frame`).`'
    sidecar: 'span-trace-coverage: new detail, the no-endpoint path reads its cause through `without_endpoint`, no span and no line.'
    rationale: 'Report Symbols: `without_endpoint` as stated, with the six no-endpoint arms and MCP `frame` its callers; this entity already holds the path that ends as unreachable, so the reading sits beside it.'
    basis: crates/viola-state/src/liveness.rs:107-115
    fact: F5
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 95
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'Replace `(no live snapshot, a replayed one included, or the connect failed) or `call` (the connection lost after it), or` with `(no live snapshot, a replayed one included, or the connect failed), `unwrapped-name` with the same `during` (no instance directory), `endpoint-lost` with `during` `call` (the connection lost after it), or`; the rest of the line stands.'
    sidecar: 'span-trace-coverage: `send.client` exit 21 splits `instance-dead` `during` `call` into `unwrapped-name` and `endpoint-lost`, each its own `during`.'
    rationale: 'Report Retired names `instance-dead` with `during` `call`; Counts `dead-call` names span-trace-coverage.md:95 changed; Symbols: `answer_of` reads `Connect` as `Dead` and `Closed | Io` as `Lost`, and a directory absent at no endpoint reads `unwrapped-name`.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:95
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 96
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'Add after this line the sub-bullet: `  - The `during` of a `process-exit` is `call` for `endpoint-lost` only; `instance-dead`, `unwrapped-name` and `server-verify-failed` are `during` `connect`.`'
    sidecar: 'span-trace-coverage: new detail, the rule of `during` over the exit-21 causes.'
    rationale: 'Report Symbols, A process-exit `during`: a wrapper lost with the call open is `endpoint-lost` `during` `call`, and the other three logged causes are `during` `connect`; it sits beside the `send.client` close that writes them.'
    fact: F4
note: Not in this group's listing, so not proposed here - the report's dead-call site span-trace-coverage.md:112 and the wait/last `detail` null at obs-plan.md:361; span-trace-coverage.md:95 stands near the 2000-character line cap after its edit, so it takes no further clause.
