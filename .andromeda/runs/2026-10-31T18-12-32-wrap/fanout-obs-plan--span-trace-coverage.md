proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 30
    section: '## 4. Span / Trace Coverage'
    change: 'In the refused instance check, replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the rest of the line stands.'
    sidecar: 'Span coverage, the six frame verbs'' instance check: the exit-21 document under --json now reads detail strict-modes-failed (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report''s Retired bullet names the exit-21 document''s `detail: null`; Symbols / APIs gives the five closed causes and says a refused check writes `StrictModes` through `unreachable_document`; Counts row `doc-null`.'
    basis: src/cmd/client.rs:222
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 33
    section: '## 4. Span / Trace Coverage'
    change: 'In the MCP server check group, replace `a refusal returns `instance-unreachable` with `detail: null`, no frame and no line` with `a refusal returns `instance-unreachable` with `detail: "strict-modes-failed"`, no frame and no line`; the rest of the line stands.'
    sidecar: 'Span coverage, MCP frame tools: a refused check_instance now returns detail strict-modes-failed.'
    rationale: 'Report Retired, the exit-21 document''s `detail: null`; Symbols / APIs **MCP**, `tools.rs` `frame`: a refused `check_instance` is `StrictModes`.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 112
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'In the refused instance check of `wait` / `last`, replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`.'
    sidecar: 'Span coverage, wait / last: the refused instance check''s --json document now reads detail strict-modes-failed.'
    rationale: 'Report Retired, the exit-21 document''s `detail: null`; Symbols / APIs, `refuse_untrusted` writes `StrictModes` through `unreachable_document`; Counts row `doc-null`.'
    basis: src/cmd/client.rs:222
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 136
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'Replace `MCP `answer` refused by `check_instance` returns `instance-unreachable` with `detail: null` and writes no line` with `MCP `answer` refused by `check_instance` returns `instance-unreachable` with `detail: "strict-modes-failed"` and writes no line`.'
    sidecar: 'Span coverage, answer: MCP''s refused check_instance now returns detail strict-modes-failed.'
    rationale: 'Report Retired, the exit-21 document''s `detail: null`; Symbols / APIs **MCP**, a refused `check_instance` is `StrictModes`; Counts row `detail-null`.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 95
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'In the closing `send.client` sentence, replace `exit 21 `detail:"instance-dead"` with `during` `connect` (no live snapshot, a replayed one included, or the connect failed) or `call` (the connection lost after it), or `detail:"server-verify-failed"`` with `exit 21 `detail:"instance-dead"` with `during:"connect"` (the instance directory stands with no live snapshot, a replayed one included, or the connect failed), `detail:"unwrapped-name"` with `during:"connect"` (no live snapshot and no instance directory), `detail:"endpoint-lost"` with `during:"call"` (the connection lost after it), or `detail:"server-verify-failed"``; the rest of the line stands.'
    sidecar: 'Span coverage, send: instance-dead no longer carries during call; endpoint-lost takes the lost call and unwrapped-name the absent directory.'
    rationale: 'Report Retired, `instance-dead` · `during` `call`; Symbols / APIs **A process-exit `during`** and `without_endpoint`; Counts row `dead-call` lists this line as changed.'
    basis: src/cmd/client.rs:114
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 112
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'Replace `where exit 21 carries `detail:"instance-dead"` plus `during` ∈ `connect|call` (endpoint vanished mid-wait = `during:"call"`), or `detail:"server-verify-failed"` with `during:"connect"`` with `where exit 21 carries `detail:"instance-dead"` or `detail:"unwrapped-name"` with `during:"connect"`, `detail:"endpoint-lost"` with `during:"call"` (endpoint vanished mid-wait), or `detail:"server-verify-failed"` with `during:"connect"``.'
    sidecar: 'Span coverage, wait / last: an endpoint vanished mid-wait is endpoint-lost with during call, no longer instance-dead.'
    rationale: 'Report Retired, `instance-dead` · `during` `call`; Symbols / APIs **A process-exit `during`**; Counts row `dead-call` lists this line as changed.'
    basis: src/cmd/client.rs:114
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 57
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'In the failure line, replace `with `detail` ∈ `already-live|squatted-name|pinned-hash-mismatch|batch-script-child` (Founder Direction 4; no path or pid)` with `with `detail` ∈ `already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|batch-script-child` (Founder Direction 4; no path or pid; a `stale` holder logs `stale-heartbeat`, a live holder or a fresh replayed beat `already-live`, while the `run.collision_check` span''s `outcome` stays `already-live` for a live or a stale holder)`.'
    sidecar: 'Span coverage, viola run failure: stale-heartbeat joins the process-exit details; the run.collision_check span outcome does not move.'
    rationale: 'Report Retired, `already-live` as a stale holder''s refusal; Symbols / APIs **A stale holder''s code**. This closed list sits outside the `stale-live` pattern''s reach (the row''s no-change on line 45 is the span outcome, kept).'
    basis: schemas/diag-line.v1.json:78
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 57
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'In the `viola revive` preflight clause, replace `(and `already-live`, from the same collision check)` with `(and `already-live` or `stale-heartbeat`, from the same collision check)`.'
    sidecar: 'Span coverage, viola revive preflight: its collision check can now end stale-heartbeat too.'
    rationale: 'Report Symbols / APIs **A stale holder''s code** and **`viola revive --json`**, whose codes list `stale-heartbeat`; Retired, `already-live` as a stale holder''s refusal.'
    basis: schemas/diag-line.v1.json:78
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 33
    section: '## 4. Span / Trace Coverage'
    change: 'Add after this line: `- **ipc-internal (viola-mcp), the cause on `isError`:** `result::unreachable(cause: UnreachableCause)` names the cause in the body''s `detail`, MCP''s text staying `refused: instance-unreachable`. In `frame`, a refused `check_instance` is `strict-modes-failed`, no `live_endpoint` is `without_endpoint`''s reading (`instance-dead` or `unwrapped-name`) and a failed `server_verified` is `server-verify-failed`; in `of_reply`, `Connect` is `instance-dead` and `Closed | Io` is `endpoint-lost`; in `of_board`, the refusal is `strict-modes-failed`. None writes a line, the `isError` result its record; JSON-RPC errors do not move.`'
    sidecar: 'Span coverage: new item, the cause each MCP tool path puts on its isError instance-unreachable body, with no log line.'
    rationale: 'Report Symbols / APIs **MCP**; Coverage of new surfaces, MCP `detail` causes: instrumentation n/a, no line by `inputs#I2` Q4, `isError` the record.'
    basis: crates/viola-core/src/lib.rs:181
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 34
    section: '## 4. Span / Trace Coverage'
    change: 'Add after this line: `- **cli (short-lived verbs), the no-endpoint reading:** the six no-endpoint arms and MCP''s `frame` read `viola_state::liveness::without_endpoint` after the strict-modes check: `symlink_metadata` reads a directory `instance-dead`, anything else (absent, a regular file, a symlink) `unwrapped-name`; it opens nothing and writes no line. A frame verb against a stale instance still reads `instance-dead`, since `live_endpoint` gives none for a stale wrapper; stale heartbeat is exit 1''s cause alone.`'
    sidecar: 'Span coverage: new item, how a frame verb with no live endpoint tells instance-dead from unwrapped-name, and that a stale instance stays instance-dead.'
    rationale: 'Report Symbols / APIs `viola_state::liveness::without_endpoint`, the six no-endpoint arms and **A stale holder''s code**.'
    basis: crates/viola-state/src/liveness.rs:107
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 230
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'Add after this line: `  - `verify`''s refusal codes: `Ended::Refused(&''static str)` puts its code on the `process-exit` `verify` writes under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` or `unclean-fixture`, beside the standing `interrupted`. A refused home tree stays lineless, a failing row is no refusal and carries no code, and the fixture''s `<file>` enters no code; `verify` takes no `--json`.`'
    sidecar: 'Span coverage, verify: new item, the closed refusal codes on its process-exit line.'
    rationale: 'Report Symbols / APIs **`verify`''s codes**; Coverage of new surfaces, `verify`''s `process-exit` codes; Schema / config, the enum''s six new words.'
    basis: schemas/diag-line.v1.json:78
note: 'Line 45 kept per the report''s `stale-live` no-change; line 228''s `error: internal error` left as the human form (catch-site, re-derived; verify takes no --json, plugin install''s --json fault form is §7''s); the body''s own stale sites (e.g. .andromeda/obs-plan.md:361) are outside this file, for the body detector.'
