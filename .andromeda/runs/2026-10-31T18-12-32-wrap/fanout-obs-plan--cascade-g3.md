proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 112
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'In the client process-exit clause, replace `where exit 21 carries detail:"instance-dead" plus during ∈ connect|call (endpoint vanished mid-wait = during:"call"), or detail:"server-verify-failed"` with `where exit 21 carries detail:"instance-dead" or detail:"unwrapped-name" with during:"connect", detail:"endpoint-lost" with during:"call" (endpoint vanished mid-wait, the wrapper lost with the call open), or detail:"server-verify-failed"`; the rest of the clause stands.'
    sidecar: 'span-trace-coverage wait/last: instance-dead no longer carries during call; a wrapper lost with the call open is endpoint-lost with during call, and unwrapped-name joins instance-dead at during connect.'
    rationale: 'Report **Retired** names `instance-dead` · `during` `call`, and Symbols / APIs (A process-exit `during`) moves it to endpoint-lost; **Counts** dead-call lists this line as changed.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:112
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 112
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'In the refused instance check clause, replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, and replace `return instance-unreachable with no line` with `return the same body less v (error instance-unreachable, detail strict-modes-failed), their text still refused: instance-unreachable, with no line`.'
    sidecar: 'span-trace-coverage wait/last: a refused instance check names its cause, strict-modes-failed, in the --json document and the MCP isError body; still no frame and no line.'
    rationale: 'Report **Retired** names the exit-21 document''s `detail: null`; Symbols / APIs gives a strict-modes site `strict-modes-failed`, on the CLI through `unreachable_document` and on MCP through `frame`, `isError` the record.'
    basis: src/cmd/client.rs:222-226
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 136
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'Replace `or under --json the instance-unreachable document` with `or under --json the instance-unreachable document with detail strict-modes-failed`, and replace `returns instance-unreachable with detail: null and writes no line` with `returns instance-unreachable with detail strict-modes-failed and writes no line`.'
    sidecar: 'span-trace-coverage dialog/answer: CLI and MCP answer refused by the instance check now read detail strict-modes-failed, not null; still no line.'
    rationale: 'Report **Retired** names the exit-21 document''s `detail: null`; the MCP bullet of Symbols / APIs reads a refused `check_instance` as `StrictModes`, with no frame and no line.'
    basis: src/cmd/client.rs:222-226
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 114
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'Add a sub-bullet under Cleanup: `  - A process-exit''s during is call for endpoint-lost only; instance-dead, unwrapped-name and server-verify-failed are during connect.`'
    sidecar: 'span-trace-coverage wait/last: states which exit-21 cause a process-exit''s during names, call for endpoint-lost alone.'
    rationale: 'Report Symbols / APIs, A process-exit `during`: the mid-call disconnect this Cleanup says the client logs once is now named endpoint-lost.'
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 112
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'Add a sub-bullet after this one: `  - viola_state::liveness::without_endpoint reads a no-endpoint instance by symlink_metadata: a directory is instance-dead, anything else (absent, a regular file, a symlink) unwrapped-name; it opens nothing, writes no line, and the strict-modes check stays first at every caller.`'
    sidecar: 'span-trace-coverage wait/last: records without_endpoint, the reading that names a no-endpoint instance''s cause, with no line of its own.'
    rationale: 'Report Symbols / APIs: `without_endpoint` is new, read by `Unreachable::without_endpoint` and by `frame`, behind the liveness-only pre-check this entity names.'
    basis: crates/viola-state/src/liveness.rs:107-115
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 230
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'After `verify maps that refusal (StampsError::StrictModes) to the same home pair` add `, and under a parsed VIOLA_NAME its process-exit carries detail:"strict-modes-failed"; each other refusal past the tree check likewise writes its own code on that process-exit (the Section 6 catalog), while a refused tree stays lineless and a failing row is no refusal and carries no code`.'
    sidecar: 'span-trace-coverage verify: a refusal past the tree check now carries its code on verify''s process-exit; a refused tree stays lineless and a failing row has no code.'
    rationale: 'Report Symbols / APIs, `verify`''s codes: `Ended::Refused` writes its code on the `process-exit` under a parsed `VIOLA_NAME`, `strict-modes-failed` among them; a refused home tree stays lineless and a failing row no refusal and no code.'
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan.md
    near: 361
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'In the refused instance check clause, replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, and replace the closing `and MCP''s returns instance-unreachable with no line` with `and MCP''s returns instance-unreachable with detail strict-modes-failed and no line`.'
    sidecar: 'obs-plan §4 wait/last broad line: a refused instance check names strict-modes-failed in the --json document and the MCP body.'
    rationale: 'Report **Retired** names the exit-21 document''s `detail: null`; **Expected amendments** names this line under obs-plan §4 and the search `doc-null`.'
    basis: .andromeda/obs-plan.md:361
note: 'The report lists span-trace-coverage.md:95 as a changed dead-call site, but no entity in this group holds that line, so it has no proposal here; F2''s code list (log-coverage) is left to its own category, and the near-230 edit only points to that catalog.'
