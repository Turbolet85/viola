proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 99
    section: '## Threat Model Summary'
    change: 'Before its frame, each of `send`, `wait`, `last` and `answer` runs the instance strict-modes check (`check_instance` on the home and the instance directory) ahead of the liveness pre-check and the server check; a refusal is `instance-unreachable` with `detail: "strict-modes-failed"`, no frame and no line.'
    sidecar: 'threat-model-summary boundary-mcp: an MCP strict-modes refusal reads `detail: "strict-modes-failed"`, the retired `detail: null` removed (chunk 2026-10-31-exit-cause-code-catalogue, the founder''s own answer `inputs#I2` Q1).'
    rationale: 'The report **Retired** bullet names the exit-21 document `detail: null`, and **Symbols / APIs** (MCP) has a refused `check_instance` in `tools.rs` `frame` read `StrictModes`; **Expected amendments** lists threat-model-summary.md:99 under `doc-null`.'
    basis: .andromeda/security-plan-detail/threat-model-summary.md:99
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 59
    section: '## Threat Model Summary'
    change: 'New item after this line: an `instance-unreachable` on CLI `--json` or an MCP result names its cause in `detail`, one of five closed words from `viola_core::UnreachableCause` (no serde path, every word written by the two clients, no path, pid or upstream text): `strict-modes-failed`; `instance-dead` (no endpoint with the instance directory standing, or a failed connect); `unwrapped-name` (no endpoint with anything else at that path: absent, a regular file or a symlink); `endpoint-lost` (the wrapper lost with the call open); `server-verify-failed` (off macOS). The no-endpoint reading is `viola_state::liveness::without_endpoint`, a `symlink_metadata` that opens nothing and writes no line, with the strict-modes check first at every caller. MCP writes no line for any cause, `isError` the record. The cause in `detail` is the founder''s own answer (chunk 2026-10-31-exit-cause-code-catalogue, `inputs#I2` Q1); the unwrapped reading is the delegate''s, the overseer, PROVISIONAL (`inputs#I2` Q2).'
    sidecar: 'threat-model-summary boundary-ipc: the five closed exit-21 and MCP causes, client-written, and the `without_endpoint` reading (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Symbols / APIs** moves the exit-21 document, adds `UnreachableCause` and `without_endpoint`, and maps MCP `of_reply` and `frame`; **Coverage of new surfaces** records closed enum validation and no path, pid or upstream text. This line 59 states what reaches a CLI `--json` document or an MCP result, and these words now reach both.'
    basis: crates/viola-core/src/lib.rs:181
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 122
    section: '## Threat Model Summary'
    change: 'New item after this line: `src/main.rs` `json_asked` reads `--json` from argv ahead of clap, as `role_of` reads its role, taking the flag only among the arguments before a `--`; it only selects the form of the catch site output, `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty in place of `error: internal error`, where `prints_internal_error` is true. Without `--json` the line stands byte for byte, and `run`, the start of `revive`, `mcp` and `hook` stay silent there.'
    sidecar: 'threat-model-summary boundary-cli: the catch site argv read `json_asked` and its fixed `internal-error` document (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Symbols / APIs** (the `cli` catch site under `--json`) adds an argv read outside the clap parsing this entry names as the trust boundary at line 121; **Coverage of new surfaces** records closed codes, so D-security-input finds no unvalidated boundary and this is detail, not a violation.'
    basis: src/main.rs:154
note: 'D-security-input, D-security-auth and D-security-deps found no violation (no dependency added, no auth library moved); the same retired `detail: null` also stands at .andromeda/security-plan.md:413 (§Error Handling), which the report lists and this file does not hold.'
