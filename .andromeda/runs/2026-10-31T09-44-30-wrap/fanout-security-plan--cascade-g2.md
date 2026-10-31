proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan.md
    near: 411
    section: "## Error Handling"
    change: "- The `Display` impls on `PtyError` (hand-written) and the nine `thiserror` types (`AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError`) use fixed messages, `thiserror_messages_interpolate_nothing` gating the `thiserror` ones. Variant fields that hold paths or payloads are excluded from `Display`."
    sidecar: "Error Handling's enum list: `CoreError`, which never existed in the code, leaves it, and the census of nine `thiserror` types replaces it."
    rationale: "The report's Retired bullet and Spec claims disproved measure `CoreError` at 0 hits; its Counts read 9 derives by name and its Expected amendments name this list."
    basis: .andromeda/security-plan.md:411
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 413
    section: "## Error Handling"
    change: "- MCP `isError` results carry only the `structuredContent` codes from Conventions (`refusal`/`detail`, or `error: instance-unreachable | wrapper-fault | invalid-params` — `instance-unreachable` with `detail: null`, `wrapper-fault` with `{code,message,data}` as `viola_channel::sanitised_fault` closes them, never the peer's text, `invalid-params` with a closed field word), a `refusal` that is no code word (`viola_core::is_code_word`) reading `unknown` and a string `detail` that is none reading null, the same screen holding on CLI `--json` through `reply_of`, and every `isError` text is `refused: <code>`. Any other handler fault, a reply of no known shape or a panicked call is the JSON-RPC error `-32603` `internal error` with `data: null` (chunk 2026-10-17-mcp-server-for-drivers)."
    sidecar: "Error Handling's MCP line: `wrapper-fault`'s fields are closed by `sanitised_fault`, and a peer's `refusal` and string `detail` are screened by `is_code_word`, on `of_reply` and `reply_of` alike."
    rationale: "The report replaces both `fault_detail` copies with `sanitised_fault`, screens `refusal` and `detail` in `reply_of` and `of_reply`, and its Expected amendments and `fault-reader` sweep name this line."
    basis: .andromeda/security-plan.md:413
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 14
    section: "## Error Handling"
    change: "- `viola_channel::sanitised_fault(&Value) -> Option<Value>` is the one fault reader of `reply_of` and `of_reply`: `None` without an integer `code`, and for the five known codes `message` is the fault's fixed text and `data` null, except the two `-32602` bodies."
    sidecar: "New detail: `sanitised_fault` as the one fault reader of `reply_of` and `of_reply`."
    rationale: "The report's Symbols / APIs state the reader and its two callers; it sits beside the entity that states the wrapper fault."
    basis: crates/viola-channel/src/lib.rs:117
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 14
    section: "## Error Handling"
    change: "- A `-32602` keeps `data` only as the unsupported-version body, with an integer `data.supported` and a `data.wrapper` of 1 to 64 ASCII letters, digits, `.`, `-` or `+`, or the body whose `data.reason` is exactly `release-from-driver`; any other `-32602` reads `invalid params` with `data` null."
    sidecar: "New detail: the two `-32602` bodies that keep `data`, and the null of every other."
    rationale: "The report's `sanitised_fault` entry states the three `-32602` readings; the entity beside it names a `-32602` reply."
    basis: crates/viola-channel/src/lib.rs:117
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 14
    section: "## Error Handling"
    change: "- A fault code outside the five reads its `code`, the fixed `message` `wrapper fault` and `data` null: a new fixed output word on `--json` and MCP, the human line `error: wrapper fault  <code>` unchanged."
    sidecar: "New detail: the fixed `wrapper fault` message for a code outside the five."
    rationale: "The report's Coverage of new surfaces names this literal as a new fixed output word and the human line as unchanged."
    basis: crates/viola-channel/src/lib.rs:117
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 14
    section: "## Error Handling"
    change: "- `viola_core::is_code_word` admits 1 to 64 bytes of `[a-z0-9-]` with a lower-case letter first; in `reply_of` and `of_reply` a `refusal` failing it reads `unknown`, a string `detail` failing it reads null, and a non-string `detail` is still no known reply."
    sidecar: "New detail: the `is_code_word` screen on a peer's `refusal` and `detail`."
    rationale: "The report's Symbols / APIs state the word's shape and both callers' readings; the entity beside it states what a client does with a reply."
    basis: crates/viola-core/src/lib.rs:350
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 14
    section: "## Error Handling"
    change: "- Exception by design: a code word outside the closed list stays visible in the document and in `refused: <word>`, its exit staying `unknown`'s 14 by `reason_of`."
    sidecar: "New detail: a code word outside the closed list stays visible, by design, at exit 14."
    rationale: "The report's Symbols / APIs and Outcome both state this exception as by design."
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 14
    section: "## Error Handling"
    change: "- `ProtocolError::cause` gives `unsupported-version`, `invalid-params` and `release-from-driver` for the three `-32602` faults and nothing for the other four; `ResponseLine`'s `Drop` writes it as `channel-response`'s `detail`, on both sides."
    sidecar: "New detail: `ProtocolError::cause` and the closed `channel-response` detail it writes."
    rationale: "The report's Symbols / APIs and Schema / config state the three words, the closed enum and both sides' line."
    basis: crates/viola-channel/src/lib.rs:103
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 14
    section: "## Error Handling"
    change: "- Absence: no `-32603` is driven in a scanned home and no failpoint or counted panic was added for it; the unit layer holds it (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`)."
    sidecar: "New detail: `-32603` is held at the unit layer and driven in no scanned home."
    rationale: "The report's Reverted / negative API facts state this absence and name both cases; the entity beside it states the `-32603` answer."
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 15
    section: "## Error Handling"
    change: "- Exception: the exit-2 paths of `viola answer` and `viola send` write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says, no document (`src/cmd/answer.rs:91`, `src/cmd/send.rs:187`), named in `NOT_ERROR_BODIES`."
    sidecar: "New detail: the exit-2 paths of `answer` and `send` write a fixed stderr line and no `--json` document."
    rationale: "The report's Spec claims disproved measures these paths; the entity beside it states the same verbs' `--json` refusal."
    basis: src/cmd/answer.rs:91
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 17
    section: "## Error Handling"
    change: "- `tests/contract_error_bodies.rs` enumerates every error body with its writer and witness (`CLI_DOCUMENTS`, `MCP_RESULTS`, `CHANNEL_FAULTS`, with `NOT_ERROR_BODIES` and `PLUMBING` as the named exclusions), fails on an unnamed product error writer and on an interpolating `thiserror` message, and carries `andromeda:walks-tree`."
    sidecar: "New detail: the enumeration and census of error writers, and the fixed-message gate."
    rationale: "The report's Harness and New text name the binary, its tables and its two census cases; the entity beside it states fixed `Display` messages."
    basis: tests/contract_error_bodies.rs:720
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 17
    section: "## Error Handling"
    change: "- `tests/support/events.rs` holds the error-body check, `error_body_hit` and `assert_error_body_clean`, with classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class and never the bytes it read."
    sidecar: "New detail: the shared error-body check and its four classes."
    rationale: "The report's Symbols / APIs state the check, its classes and that it reports no bytes; the entity beside it states the no-path, no-payload rule."
    basis: tests/support/events.rs:18
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 17
    section: "## Error Handling"
    change: "- Absence: no check runs after every E2E test; the three surface witnesses, the enumeration and the census replace it."
    sidecar: "New detail: no after-every-test error-body check exists; the witnesses, enumeration and census replace it."
    rationale: "The report's Spec claims disproved states the check was measured absent and is replaced, not built."
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 17
    section: "## Error Handling"
    change: "- Open divergences from the error-enum convention: `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused`, owned by a CARRY on the entry Version done-check, on the delegate overseer's word, PROVISIONAL and owed the founder's own."
    sidecar: "New detail: the five open enum divergences and their provisional owner."
    rationale: "The report's inputs#I2 card row and Decisions state the five names, the owner and the PROVISIONAL word; the entity beside it names `StampsError` and `PinError`."
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 17
    section: "## Error Handling"
    change: "- `Refusal` was folded rather than ratified, with no output changed, leaving `src/cmd/revive.rs`'s private `Refusal`, another type, in place."
    sidecar: "New detail: `viola_agent_claude::Refusal` folded into `AgentError`; `revive`'s private `Refusal` stays."
    rationale: "The report's Symbols / APIs and Decisions state the fold, the unchanged outputs and the other type left standing."
    fact: F13
note: "The two body edits at .andromeda/security-plan.md:411 and :413 may meet a body pass's edit of the same lines, so apply one; D-security-input, D-security-auth and D-security-deps fired nothing, the new peer-reply surface being validated, no auth or secret handling moved and no dependency added."
