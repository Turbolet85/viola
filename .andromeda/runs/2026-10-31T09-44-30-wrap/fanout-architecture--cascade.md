proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 130
    section: "## Established Decisions"
    change: "In the closing clause the exit reads 1: «or `ServeError::Initialize` (exit 1) when the cut lands before `initialize` completes.»"
    sidecar: "dec-mcp: the stdin cut before `initialize` is process exit 1, not 11."
    rationale: "Report, Spec claims disproved: `established-decisions.md:130` states exit 11 as `viola mcp`'s exit; `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` reads exit 1, both streams empty."
    basis: crates/viola-e2e/tests/mcp_tools.rs:522
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 130
    section: "## Established Decisions"
    change: "New bullet after this line: «- `viola mcp` on a line past `MAX_FRAME` before `initialize` exits 1 with both streams empty and one `parse-rejected{mcp-stdin, oversize, count:1}` line: `src/main.rs` `exit_code` gives `Role::Mcp` with `Failed` exit 1, and 11 is only the unit child's own code (`serve_child_entry`).»"
    sidecar: "dec-mcp: adds the measured exit of the stdin cut before `initialize`."
    rationale: "Report, Spec claims disproved, `exit-eleven`; the fact belongs beside the decision that states this exit."
    basis: crates/viola-e2e/tests/mcp_tools.rs:522
    fact: F10
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 267
    section: "## Established Decisions"
    change: "Replace «— the second enum `StampError` … harness. One open divergence … is open). A second open divergence, not an exception: `viola-mcp` holds» with «— the second enum `StampError` was folded into it by chunk 2026-09-29-verify-stamped-test-homes-and-harness, and `Refusal` by chunk 2026-10-31-sanitised-error-surfaces). A first open divergence, not an exception: `viola-mcp` holds», and replace «A third open divergence, not an exception: `viola-state` holds `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`) beside `StateError`; the rule stands, and its fold or ratification is open; anyhow» with «A second: `viola-state` holds `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`) beside `StateError`, the same. Three more the census of that chunk adds, the same: `PinError`, `StampsReadError` and `Refused`. The owner of all five: a CARRY on the working-route entry \"Version done-check\" (the delegate overseer's word, PROVISIONAL); anyhow»."
    sidecar: "dec-error-handling: `Refusal` folded and out; open divergences `ServeError`, `StampsError`, `PinError`, `StampsReadError`, `Refused`, owned by a CARRY on \"Version done-check\"."
    rationale: "Report, Retired: `viola_agent_claude::Refusal`; Counts: patterns `refusal-enum` and `enum-rule` change `established-decisions.md:267`; open divergences read two named plus three the census adds; owner per inputs#I2 card row, PROVISIONAL, the route edit P5's."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 267
    section: "## Established Decisions"
    change: "New bullet after this line: «- `viola_agent_claude::Refusal` was folded, not ratified, `StampError` the precedent: `AgentError` gains `BatchScriptChild` and `ProgramNotFound` (was `Refusal::NotFound`), messages and outputs unchanged, and `resolve_program` returns `Result<PathBuf, AgentError>` in the crate and in `src/run/mod.rs`.»"
    sidecar: "dec-error-handling: adds how `Refusal` was folded."
    rationale: "Report, Symbols / APIs and Decisions & corrections: «`Refusal` folded, not ratified, `StampError` the precedent»."
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 267
    section: "## Established Decisions"
    change: "New bullet after this line: «- The product sources hold 9 `thiserror::Error` derives: `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError` (the count's rule `command grep -rn \"thiserror::Error\" src crates --include=*.rs`).»"
    sidecar: "dec-error-handling: adds the census of 9 `thiserror` derives and its rule."
    rationale: "Report, Counts / qualifiers moved: the derives read 9, from 10, by that command at research, less `Refusal`."
    fact: F9
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture.md
    near: 65
    section: "## Established Decisions"
    change: "Replace «and three open divergences, `viola-agent-claude`'s `Refusal`, `viola-mcp`'s `ServeError` and `viola-state`'s `StampsError`, the rule standing» with «and five open divergences, `viola-mcp`'s `ServeError`, `viola-state`'s `StampsError`, and `PinError`, `StampsReadError` and `Refused`, the rule standing»."
    sidecar: "[Error Handling] broad line: `Refusal` out, five open divergences named."
    rationale: "Report, Retired and Counts: `refusal-enum` and `enum-rule` change `architecture.md:65`."
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture.md
    near: 106
    section: "## Conventions"
    change: "The enum list reads «(`PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`)», `CoreError` out; the parenthesis reads «([Error Handling]; `viola-mcp`'s `ServeError` beside `McpError`, `viola-state`'s `StampsError` beside `StateError`, and `PinError`, `StampsReadError` and `Refused` are the open divergences named there)»."
    sidecar: "Rust error types: `CoreError`, which never existed, and `Refusal` out; five open divergences named."
    rationale: "Report, Retired and Spec claims disproved: `CoreError` stated at `architecture.md:106`, `command grep -n \"CoreError\" -r crates src --include=*.rs` 0 hits; `Refusal` folded."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture.md
    near: 449
    section: "## Inherited Defaults"
    change: "The parenthesis reads «(except `viola-pty`'s hand-written `PtyError`; `viola-mcp`'s `ServeError` beside `McpError`, `viola-state`'s `StampsError` beside `StateError`, and `PinError`, `StampsReadError` and `Refused` are open divergences, [Error Handling])»."
    sidecar: "Inherited Defaults, Errors: `Refusal` out, five open divergences named."
    rationale: "Report, Retired and Counts: `refusal-enum` and `enum-rule` change `architecture.md:449`."
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 13
    section: "## Conventions"
    change: "The row reads «  - `unknown`: a code word (`viola_core::is_code_word`) or `null`», no longer any value."
    sidecar: "conv-refusal-reason: `unknown`'s detail is a code word or null."
    rationale: "Report, Expected amendments, `code-word`: `unknown`'s detail a code word or null; Symbols / APIs: `reply_of` and `of_reply`."
    basis: crates/viola-core/src/lib.rs:350
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 15
    section: "## Conventions"
    change: "The opening sentence reads «Readers treat an unrecognised code-word `detail` as opaque text, never as an error.»; the rest of the line stands."
    sidecar: "conv-refusal-reason: a reader's opaque rule kept, for a code word outside the closed set."
    rationale: "Report, Expected amendments: `code-word` changes `conventions.md:15`, «a reader's opaque rule kept»; a string `detail` that is no code word now reads null."
    basis: src/cmd/client.rs:77
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 15
    section: "## Conventions"
    change: "New bullet after this line: «- **Code word:** `viola_core::is_code_word(&str) -> bool`: 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first; its callers are `reply_of` and `of_reply`.»"
    sidecar: "conv-refusal-reason: adds what a code word is."
    rationale: "Report, Symbols / APIs: `viola_core::is_code_word`."
    basis: crates/viola-core/src/lib.rs:350
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 15
    section: "## Conventions"
    change: "New bullet after this line: «- **A peer's reply** is screened by `reply_of` and `of_reply`: a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads null, and a non-string `detail` is still no known reply.»"
    sidecar: "conv-refusal-reason: adds the reply screen of `reply_of` and `of_reply`."
    rationale: "Report, Symbols / APIs: the `reply_of` and `of_reply` bullet."
    basis: crates/viola-mcp/src/result.rs:71
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 98
    section: "## Conventions"
    change: "Replace «each `detail` from a closed set per reason, extended only in `viola-core`, an unknown one opaque text to a reader» with «each `detail` from a closed set per reason (`unknown`'s a code word or null), extended only in `viola-core`, an unrecognised code word opaque text to a reader»."
    sidecar: "`RefusalReason` broad line: `unknown`'s detail a code word or null."
    rationale: "Report, Expected amendments: `code-word` changes `architecture.md:98`."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 64
    section: "## Conventions"
    change: "Replace «`wrapper-fault` with the JSON-RPC fault's `{code,message,data}`» with «`wrapper-fault` with the JSON-RPC fault's `{code,message,data}` as `viola_channel::sanitised_fault` reads it: a fixed `message`, `data` a closed object or null»."
    sidecar: "conv-mcp-results: a wrapper fault's result reads through `sanitised_fault`, its fields closed."
    rationale: "Report, Expected amendments: `fault-reader` changes `conventions.md:64`; Symbols / APIs: `sanitised_fault` replaces the private `fault_detail` in `crates/viola-mcp/src/result.rs`."
    basis: crates/viola-channel/src/lib.rs:117
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 227
    section: "## Standard Contracts"
    change: "After «(exit 20)» add «: `message` a fixed text and `data` a closed object or null, read through `viola_channel::sanitised_fault`»."
    sidecar: "CLI `--json` output: `wrapper-fault`'s fields closed."
    rationale: "Report, Expected amendments: `fault-reader` changes `architecture.md:227`, §Standard Contracts `wrapper-fault`'s fields closed; the seven wire bodies are unchanged."
    basis: crates/viola-channel/src/lib.rs:117
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 111
    section: "## Standard Contracts"
    change: "New bullet after this line: «- `viola_channel::sanitised_fault(&Value) -> Option<Value>` is the one fault reader of `reply_of` and `of_reply`: `None` without an integer `code`; each of the five codes reads its fixed text; `-32602` keeps `supported` and `wrapper` only when `data.supported` is an integer and `data.wrapper` is version text (1 to 64 ASCII letters, digits, `.`, `-` or `+`), keeps `data.reason` only when exactly `release-from-driver`, and otherwise reads `invalid params` with `data` null; every other known code reads `data` null.»"
    sidecar: "contract-cli-json-output: adds `sanitised_fault`, the one fault reader."
    rationale: "Report, Symbols / APIs: `viola_channel::sanitised_fault`, both private `fault_detail` copies gone."
    basis: crates/viola-channel/src/lib.rs:117
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 111
    section: "## Standard Contracts"
    change: "New bullet after this line: «- A fault code outside the five reads `{\"code\": <code>, \"message\": \"wrapper fault\", \"data\": null}`: the literal `wrapper fault` is a fixed output word on `--json` and MCP, and the human line `error: wrapper fault  <code>` is unchanged.»"
    sidecar: "contract-cli-json-output: adds the body of a fault code outside the five."
    rationale: "Report, Coverage of new surfaces: the `message` literal `wrapper fault`."
    basis: crates/viola-channel/src/lib.rs:117
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 29
    section: "## Occupied Resources"
    change: "Replace «(exit 0 once initialized, 11 before `initialize`)» with «(exit 0 once initialized, 1 before `initialize`, both streams empty)»."
    sidecar: "res-binaries: `mcp`'s stdin cut before `initialize` is exit 1, not 11."
    rationale: "Report, Spec claims disproved: `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` reads exit 1; Expected amendments name §Occupied Resources (`mcp`), and this site states the exit in words the `exit-eleven` pattern did not read."
    basis: crates/viola-e2e/tests/mcp_tools.rs:522
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 124
    section: "## Occupied Resources"
    change: "In the `mcp.ndjson` list, replace «, and `process-exit` on stdin EOF or at that cut — none when Claude Code ends it by a signal, as measured live on claude 2.1.287 at that chunk;» with «, and no self `process-exit` on stdin EOF, at that cut or when Claude Code ends it by a signal;»."
    sidecar: "res-filesystem: `mcp.ndjson` holds no self `process-exit`."
    rationale: "Report, Spec claims disproved: `command grep -n 'ObsEvent::ProcessExit' -r src crates --include=*.rs` shows no self `process-exit` written on the `mcp` role; the `mcp-exit-line` pattern did not read this site, which states the same claim."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 124
    section: "## Occupied Resources"
    change: "New continuation line after this line: « `channel-response`'s `detail` in `schemas/diag-line.v1.json` is the closed enum `unsupported-version` · `invalid-params` · `release-from-driver`, written on both the server's and the client's line at WARN by `ResponseLine`'s `Drop`, the client's `fault_of(&Value)` reading `data` for `-32602`, and absent on every other `channel-response` line, holding no param value or name.»"
    sidecar: "res-filesystem: adds `channel-response`'s closed `detail`."
    rationale: "Report, Schema / config and Symbols / APIs: `detail` closes to the enum; `v` stays 1."
    basis: schemas/diag-line.v1.json:42
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 205
    section: "## Occupied Resources"
    change: "New bullet after this line: «- Test-only: `tests/support/events.rs` holds `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes; no after-every-test helper exists, the three witnesses, the enumeration and the census of writers replacing it. `viola` never reads it.»"
    sidecar: "res-filesystem: adds the error-body check and that no after-every-test helper exists."
    rationale: "Report, Spec claims disproved: `after-every-test`, 1 extended at `occupied-resources.md:205`; a new bullet, since the standing line is already long."
    basis: tests/support/events.rs:18
    fact: F14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 234
    section: "## Occupied Resources"
    change: "New continuation line after this line: « `v1-47`'s after every end-to-end test is concretized to after each job's end-to-end run, on `AGENT_RUN_KEEP_HOMES` `\"1\"` and `if: always()` in `.github/workflows/ci.yml`, the `viola-chaos-*` home out; accepted by the delegate overseer's word, PROVISIONAL, owed the founder's own word.»"
    sidecar: "res-repository: adds `v1-47`'s concretization over the homes kept in CI, PROVISIONAL."
    rationale: "Report, Decisions & corrections: `v1-47` concretized and accepted by the delegate word, measured."
    basis: .github/workflows/ci.yml:20
    fact: F15
note: "Left to other passes: F1, F6, F11, F12, F13 fit no picked entity (new-detail), and `crate-dependency-direction.md:6` @c1247 `Refusal` is a contract, no picked entity; two sites here state a disproved claim the report's patterns did not read: `occupied-resources.md` `11 before initialize` and `process-exit` on the `mcp` role, the latter against the task's Untouched line."
