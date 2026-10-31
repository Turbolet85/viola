group: architecture-2
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
- leaf: .claude/docs/services/viola-core.md:6   class: omission
  leaf-text: «the shared validators (`validate_paste_text`)»
  source: .andromeda/architecture-detail/conventions.md:21 «`viola_core::is_code_word(&str) -> bool`: 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first»
  replace-with: «the shared validators (`validate_paste_text`, and `is_code_word`: 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first)»
- leaf: .claude/docs/services/viola-core.md:14   class: retired
  leaf-text: «newtypes, validators, constants, `CoreError`.»
  source: .andromeda/architecture.md:106 «one error enum per crate, named `<Crate>Error` (`PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`)»
  replace-with: «newtypes, validators, constants.»
- leaf: .claude/docs/services/viola-core.md:20   class: stale
  leaf-text: «`detail` values are kebab-case strings from a closed set per reason.»
  source: .andromeda/architecture.md:98 «each `detail` from a closed set per reason (`unknown`'s a code word or null)»
  replace-with: «`detail` values are kebab-case strings from a closed set per reason (`unknown`'s a code word or null).»
- leaf: .claude/docs/services/viola-core.md:25   class: retired
  leaf-text: «`CoreError` `Display` strings are fixed messages.»
  source: .andromeda/architecture-detail/established-decisions.md:270 «`AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError`»
  replace-with: «The [Error Handling] census of `thiserror::Error` derives names no enum of this crate.»
- leaf: .claude/docs/services/viola-core.md:28   class: stale
  leaf-text: «`detail` for `unknown` is opaque text — readers never treat an unrecognised detail as an error.»
  source: .andromeda/architecture-detail/conventions.md:13 «`unknown`: a code word (`viola_core::is_code_word`) or `null`»
  replace-with: «`detail` for `unknown` is a code word (`viola_core::is_code_word`) or `null`; readers treat an unrecognised code-word `detail` as opaque text, never as an error.»
- leaf: .claude/docs/services/viola-mcp.md:22   class: omission
  leaf-text: «refusal = `isError:true` + `{refusal, detail}` + text `refused: <reason>`;»
  source: .andromeda/architecture-detail/conventions.md:27 «`reply_of` and `of_reply` read a `refusal` that is no code word as `unknown`, a string `detail` that is no code word as `null`, and a non-string `detail` as no known reply.»
  replace-with: «refusal = `isError:true` + `{refusal, detail}` + text `refused: <reason>` (a peer's `refusal` that is no code word reads `unknown`, a string `detail` that is no code word `null`, and a non-string `detail` no known reply);»
- leaf: .claude/docs/services/viola-mcp.md:22   class: omission
  leaf-text: «unreachable / fault = `isError:true` + `{"error":"instance-unreachable"|"wrapper-fault","detail":…}`;»
  source: .andromeda/architecture-detail/conventions.md:76 «`wrapper-fault` with the JSON-RPC fault's `{code,message,data}` as `viola_channel::sanitised_fault` reads it: a fixed `message`, `data` a closed object or null»
  replace-with: «unreachable / fault = `isError:true` + `{"error":"instance-unreachable"|"wrapper-fault","detail":…}` (`wrapper-fault` with the JSON-RPC fault's `{code,message,data}` as `viola_channel::sanitised_fault` reads it: a fixed `message`, `data` a closed object or null);»
- leaf: .claude/docs/services/viola-mcp.md:22   class: omission
  leaf-text: «Every `isError` text is `refused: <code>`.»
  source: .andromeda/architecture-detail/cross-cutting-patterns.md:33 «CLI `--json` documents, MCP error results and channel error data carry codes and fixed messages only, no path and no upstream text»
  replace-with: «Every `isError` text is `refused: <code>`, and every MCP error result carries codes and fixed messages only, no path and no upstream text, each body a row of `MCP_RESULTS` in `tests/contract_error_bodies.rs`.»
- leaf: .claude/docs/services/viola-mcp.md:23   class: stale
  leaf-text: «(exit 0, with `process-exit`, as at a closed stdin)»
  source: .andromeda/architecture-detail/occupied-resources.md:124 «and no self `process-exit`: none on stdin EOF or at that cut»
  replace-with: «(exit 0, with no self `process-exit`, as at a closed stdin)»
- leaf: .claude/docs/services/viola-mcp.md:23   class: stale
  leaf-text: «`ServeError::Initialize` (exit 11)»
  source: .andromeda/architecture-detail/established-decisions.md:130 «or `ServeError::Initialize` (exit 1) when the cut lands before `initialize` completes.»
  replace-with: «`ServeError::Initialize` (exit 1, both streams empty; 11 is only the unit child's own code, `serve_child_entry`)»
- leaf: .claude/docs/services/viola-mcp.md:36   class: omission
  leaf-text: «`mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli`);»
  source: .andromeda/architecture-detail/cross-cutting-patterns.md:35 «`mcp_tools_error_results_hold_no_path_or_upstream_text` in `crates/viola-e2e/tests/mcp_tools.rs`»
  replace-with: «`mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli`; the MCP error-body witness in `mcp_tools_error_results_hold_no_path_or_upstream_text`);»
