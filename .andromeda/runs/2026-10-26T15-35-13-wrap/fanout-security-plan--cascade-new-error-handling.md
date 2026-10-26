proposals:
  - detector: D-new-detail
    severity: warning
    fact: F5
    file: .andromeda/security-plan-detail/error-handling.md
    near: 10
    section: '## Error Handling'
    change: '- The wrapper''s `link` / `unlink` channel methods answer a `record` / `remove` failure as `ProtocolError::Internal`: `-32603` with the fixed message `internal error` and `data: null`, no other detail. At the `viola link` / `viola unlink` verbs, a wrapper error reply (a `-32602` included) is a wrapper fault, exit 20; a `Refused` reply or an `ok` of a shape other than the verb''s own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) is an internal error, exit 1.'
    sidecar: 'error-handling: the served `link` / `unlink` methods'' internal fault (`-32603` `internal error`) and the verbs'' internal error (exit 1) on a reply of no known shape (chunk 2026-10-26-explicit-session-links)'
    rationale: 'the report''s Changes, Symbols / APIs: a `record` / `remove` error is `ProtocolError::Internal` (`-32603`); the verb reads its reply through `client::answer_of`, a wrapper error is `client::fault` exit 20, a `Refused` or an `ok` of another shape is an internal error, exit 1; no item of this file holds the new methods'' error outcomes'
    basis: src/run/links.rs:70
  - detector: D-new-detail
    severity: warning
    fact: F6
    file: .andromeda/security-plan-detail/error-handling.md
    near: 10
    section: '## Error Handling'
    change: '- A `viola link` / `viola unlink` whose `check_instance` on the home and `instances/<driven>` refuses (run before `client::start`) exits 21, as `list` / `plugin install` do and not `revive`''s exit 1: in human mode the fixed instance-state `unable` / `hint:` pair through `human::refuse`; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty, a code only, no path, pid or endpoint. No line is written to any log and the directory''s mode is left as found (the founder''s own word at P4, `inputs#I3`, and his P5 word on card lean 5, `inputs#I4`, chunk 2026-10-26-explicit-session-links).'
    sidecar: 'error-handling: the `viola link` / `viola unlink` strict-modes refusal, exit 21 with the fixed human pair or the exact `--json` code document, no line, mode as found (chunk 2026-10-26-explicit-session-links)'
    rationale: 'the report''s Changes, Symbols / APIs step 1 (refused → exit 21, `human::instance_state_pair(driven)` through `human::refuse`, the exact `--json` document, stderr empty, no line in any log, the mode as found) and Decisions & corrections (exit 21 as `list` / `plugin install`, not `revive`''s exit 1, card lean 5); no item of this file holds a CLI verb''s strict-modes refusal output'
    basis: src/cmd/link.rs:98
note: both proposals follow line 10; apply F5 first, then F6 after it, so the file reads hook, hook server check, mcp, the link methods' faults, the link verbs' strict-modes refusal.
