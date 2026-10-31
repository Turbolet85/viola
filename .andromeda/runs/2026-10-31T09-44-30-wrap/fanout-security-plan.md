proposals:
  - detector: D-retired
    severity: warning
    section: "Error Handling"
    change: "The fixed-message bullet lists the enums that stand: the `Display` impls on `PtyError` (hand-written) and on `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError` and `ServeError` (`thiserror`) use fixed messages; `CoreError` is struck, it never existed in the code. Variant fields that hold paths or payloads stay excluded from `Display`."
    sidecar: "Error Handling: `CoreError` struck from the fixed-message enum list (it never existed in the code); `PinError`, `StampsError`, `StampsReadError`, `Refused` and `ServeError` named, the nine `thiserror` derives of chunk 2026-10-31-sanitised-error-surfaces."
    rationale: "The report's Retired bullet names `CoreError` as listed by security-plan and never in the code; Spec claims disproved measures 0 hits and states it at `security-plan.md:411`; Counts names the 9 derives after `Refusal` folded; Expected amendments carries the enum list for this section."
    basis: "tests/contract_error_bodies.rs:742-756"
  - detector: D-retired
    severity: warning
    section: "Bootstrap phases (derive for route / setup-project) → error-sanitization-wire"
    change: "The first bullet stops implying one enum per crate: the `Display` impls on the product error enums (`thiserror`, or hand-written for `PtyError`; the list stands in §Error Handling) must not interpolate upstream text or absolute paths."
    sidecar: "Bootstrap phases, error-sanitization-wire: the `<Crate>Error` class replaced by the product error enums, as §Error Handling lists them; viola-core has no such enum and five of the nine `thiserror` derives carry no crate name."
    rationale: "Same claim as the enum list, worded without its token: `<Crate>Error` reads as one enum per crate, viola-core's included, while the report's Retired bullet and Spec claims disproved measure no `CoreError` and Counts lists `PinError`, `StampsError`, `StampsReadError`, `Refused` and `ServeError` among the derives."
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "Error Handling"
    change: "The MCP `isError` bullet reads: `wrapper-fault` carries `viola_channel::sanitised_fault`'s closed `{code,message,data}`, never the peer's own (for the five known codes the fault's fixed `message`; `data` null except `-32602`'s unsupported-version body, an integer `supported` and a version-text `wrapper`, and its `release-from-driver` body; any other `-32602` `invalid params`; a code outside the five `wrapper fault` with `data: null`); on both CLI `--json` and MCP a `refusal` that is no `viola_core::is_code_word` reads `unknown` and a string `detail` that is no code word reads null, a code word outside the closed list staying visible in the document and in `refused: <word>`."
    sidecar: "Error Handling: `wrapper-fault`'s fields closed by `viola_channel::sanitised_fault`, replacing both private `fault_detail` copies, and the clients' `is_code_word` screen on `refusal` and `detail` stated."
    rationale: "The report's Retired bullet names both private `fault_detail` copies; the bullet's `the JSON-RPC fault's {code,message,data}` still describes that carry of the peer's fields. Symbols / APIs states the replacement and its two callers `reply_of` and `of_reply`, Outcome reads a peer's `message`, `data`, `refusal` or `detail` reaching no document or text, and the `fault-reader` sweep lists `security-plan.md:413` as changed."
    basis: "crates/viola-channel/src/lib.rs:113-142"
note: "D-security-input, D-security-auth and D-security-deps hold: the peer-reply screen and `channel-response.detail` are validated per the report's Coverage, no auth or secret handling moved, Dependencies lists none. The report's `exit-eleven` and `refusal-enum` sites in §Input Validation stand in .andromeda/security-plan-detail/input-validation.md, the cascade's; the body's MCP stdin row states no exit, so no body proposal."
