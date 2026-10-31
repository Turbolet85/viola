proposals:
  - detector: D-retired
    severity: warning
    section: "§Established Decisions → [Error Handling]"
    change: "Replace «three open divergences, `viola-agent-claude`'s `Refusal`, `viola-mcp`'s `ServeError` and `viola-state`'s `StampsError`» with: two open divergences, `viola-mcp`'s `ServeError` and `viola-state`'s `StampsError`, and three more the census adds, `PinError`, `StampsReadError` and `Refused`, the rule standing; `viola-agent-claude`'s `Refusal` is folded into `AgentError`."
    sidecar: "[Error Handling]: `Refusal` retired, folded into `AgentError`; open divergences now `ServeError` and `StampsError` plus the census's `PinError`, `StampsReadError` and `Refused`."
    rationale: "Report Retired names `viola_agent_claude::Refusal`; Symbols / APIs folds it into `AgentError` (`BatchScriptChild`, `ProgramNotFound`); Counts reads open divergences three named to two named plus three the census adds; Expected amendments list this site."
    basis: ".andromeda/architecture.md:65"
  - detector: D-retired
    severity: warning
    section: "§Conventions → Rust error types"
    change: "Drop `CoreError` from the `<Crate>Error` list, leaving `PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`; replace the parenthesis's divergences with: `viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are the two open divergences named there, with `PinError`, `StampsReadError` and `Refused` the three the census adds, nine `thiserror` derives over the product sources in all."
    sidecar: "Rust error types: `CoreError` and `Refusal` removed; the two open divergences and the census's three named; nine `thiserror` derives."
    rationale: "Report Retired names `Refusal` and `CoreError`; Spec claims disproved measures `CoreError` at 0 hits in crates and src, stating this site; Counts lists the 9 derives by name."
    basis: ".andromeda/architecture.md:106"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Inherited Defaults → Errors"
    change: "Replace «`viola-agent-claude`'s pre-existing `Refusal` beside `AgentError` and `viola-mcp`'s `ServeError` beside `McpError` are open divergences» with: `viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are open divergences, and `PinError`, `StampsReadError` and `Refused` the three the census adds."
    sidecar: "Inherited Defaults, Errors: `Refusal` removed; open divergences aligned with [Error Handling], `StampsError` and the census's three added."
    rationale: "Report Retired names `Refusal`; Counts pattern `refusal-enum` and `enum-rule` both mark this site changed; Expected amendments list §Inherited Defaults, Errors."
    basis: ".andromeda/architecture.md:449"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Infrastructure Patterns → Crate dependency direction"
    change: "In the `viola-agent-claude` bullet's thiserror list, remove `Refusal` and keep `resolve_program`, which now returns `Result<PathBuf, AgentError>`."
    sidecar: "Crate dependency direction: `Refusal` removed from `viola-agent-claude`'s surface; `resolve_program` returns `AgentError`."
    rationale: "Report Retired names `viola_agent_claude::Refusal`; Counts pattern `refusal-enum` marks this line changed at offset 1247, and offset 1229 `resolve_program` no-change as a name."
    basis: ".andromeda/registries/contracts/architecture/crate-dependency-direction.md:6"
    dependent-of: D-retired
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → CLI --json output"
    change: "After the `wrapper-fault` document (exit 20), state that `detail` is read through `viola_channel::sanitised_fault` and holds nothing of the peer but closed values: for the five codes `message` is the fault's fixed text; `-32602` with an integer `data.supported` and a version-text `data.wrapper` keeps those two values, `data.reason` exactly `release-from-driver` keeps that body, any other `-32602` reads `invalid params` with `data` null; every other known code reads `data` null; a code outside the five reads `message` `wrapper fault`, `data` null; MCP's `isError` fault result reads the same."
    sidecar: "CLI --json `wrapper-fault`: fields closed by `viola_channel::sanitised_fault`, the literal `wrapper fault` registered for a code outside the five."
    rationale: "Report Symbols / APIs defines `sanitised_fault` with callers `reply_of` and `of_reply`; Coverage of new surfaces names `wrapper fault` a new fixed output word on --json and MCP; Expected amendments: §Standard Contracts, CLI --json output, `wrapper-fault`'s fields closed, pattern `fault-reader` at this site."
    basis: ".andromeda/architecture.md:227"
  - detector: D-arch-resources
    severity: warning
    section: "§Conventions → Error handling schema, `RefusalReason`"
    change: "Replace «an unknown one opaque text to a reader» with: a `detail` outside the closed set opaque to a reader but always a code word (`viola_core::is_code_word`: 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first) or null: the clients' `reply_of` and `of_reply` read a `refusal` that is no code word as `unknown` and a string `detail` that is no code word as null, and a code word outside the closed list stays visible, its exit `unknown`'s 14."
    sidecar: "`RefusalReason`: an unrecognised `detail` is a code word or null, screened by `viola_core::is_code_word`; a reader's opaque rule kept."
    rationale: "Report Symbols / APIs adds `is_code_word` and states the `reply_of` / `of_reply` screen; Expected amendments: `unknown`'s detail a code word or null, a reader's opaque rule kept, pattern `code-word` marking this site changed."
    basis: ".andromeda/architecture.md:98"
note: "D-arch-decisions found no drift (no dependency, crate, runtime or manifest edge moved). The body and keyed contracts hold no exit-11 site; the report's detail sites `established-decisions.md:267`, `established-decisions.md:130`, `conventions.md:15`, `conventions.md:64` and `occupied-resources.md:205` are standing groups, so the cascade's; the CARRY on «Version done-check» is P5's route edit, not stated here."
