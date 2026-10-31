proposals:
  - detector: D-new-detail
    severity: warning
    fact: F4
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 39
    section: "## 7. Error Capture & Reporting"
    change: "- `viola_channel::sanitised_fault` is the one reader of a peer's fault for `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`). A fault with no integer `code` reads nothing. The five known codes read their fixed text: `-32602` with an integer `data.supported` and a `data.wrapper` of 1 to 64 ASCII letters, digits, `.`, `-` or `+` reads the unsupported-version body with those two values, `data.reason` exactly `release-from-driver` reads that body, any other `-32602` reads `invalid params` with `data` null, and every other known code reads `data` null. A code outside the five reads message `wrapper fault` with `data` null (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "§7 detail gains `sanitised_fault` as the one reader of a peer's fault for `reply_of` and `of_reply`."
    rationale: "Report, Changes, Symbols / APIs and Coverage of new surfaces: the reader and its closed table are new, and no standing text of this file holds them."
    basis: crates/viola-channel/src/lib.rs:113-142
  - detector: D-new-detail
    severity: warning
    fact: F5
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 39
    section: "## 7. Error Capture & Reporting"
    change: "- `reply_of` and `of_reply` screen a peer's reply by `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first): a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads null, and a non-string `detail` is still no known reply. A code word outside the closed list stays visible in the document and in `refused: <word>`, its exit 14, by design (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "§7 detail gains the `is_code_word` screen of a peer's `refusal` and `detail` on `reply_of` and `of_reply`."
    rationale: "Report, Changes, Symbols / APIs, and Outcome's acceptance that a code word outside the closed list stays visible by design."
    basis: crates/viola-core/src/lib.rs:347-357
  - detector: D-new-detail
    severity: warning
    fact: F8
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 39
    section: "## 7. Error Capture & Reporting"
    change: "- The product's `thiserror::Error` derives read 9: `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError`. `AgentError`'s variants include `BatchScriptChild` and `ProgramNotFound`, their two messages fixed (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "§7 detail gains the census of 9 `thiserror::Error` derives and `AgentError`'s two variants."
    rationale: "Report, Changes, Counts / qualifiers moved and Symbols / APIs: the count, the nine names and the two variants. The record details the body's §7 mechanism line; that line's own enum list is the strong readers' edit under pattern `enum-rule`."
  - detector: D-new-detail
    severity: warning
    fact: F10
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 39
    section: "## 7. Error Capture & Reporting"
    change: "- The exit-2 paths of `viola answer` and `viola send` write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says, with no document; they are named in `NOT_ERROR_BODIES` (`tests/contract_error_bodies.rs`) (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "§7 detail gains the exit-2 paths of `answer` and `send`: a fixed stderr line under `--json`, no document."
    rationale: "Report, Changes, Spec claims disproved by measurement: the oversize `answer` claim of one `--json` error document measured false."
    basis: src/cmd/answer.rs:91
  - detector: D-new-detail
    severity: warning
    fact: F11
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 18
    section: "## 7. Error Capture & Reporting"
    change: "  - **`-32603` proof:** no `-32603` is driven in a scanned home: no failpoint or counted panic was added, and the unit layer holds that fault (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`) (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "§7 detail gains where `-32603` is proved: the unit layer, not a scanned home."
    rationale: "Report, Changes, Reverted / negative API facts, which states the negative and names both unit cases."
note: "F8's nine names stand beside the body's §7 enum list at .andromeda/obs-plan.md:496, which pattern `enum-rule` moves: the two bind, and that edit is the strong readers'."
