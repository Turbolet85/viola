group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »

- leaf: .claude/docs/conventions.md:15   class: retired
  leaf-text: «`McpError`, `CoreError`)»
  source: .andromeda/architecture.md:106 «one error enum per crate, named `<Crate>Error` (`PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`), each a `thiserror` enum except `PtyError`»
  replace-with: «`McpError`)»

- leaf: .claude/docs/conventions.md:15   class: omission
  leaf-text: «holds `Malformed` (the hook payload) and `StampsMalformed` (the stamps envelope).»
  source: .andromeda/architecture-detail/established-decisions.md:269 «`AgentError` holds `resolve_program`'s two refusals, `BatchScriptChild` and `ProgramNotFound`, each with a fixed message»
  replace-with: «holds `Malformed` (the hook payload), `StampsMalformed` (the stamps envelope) and `resolve_program`'s two refusals, `BatchScriptChild` and `ProgramNotFound`, each with a fixed message.»

- leaf: .claude/docs/conventions.md:15   class: retired
  leaf-text: «Three open divergences, not exceptions: the crate's pre-existing `Refusal` (`resolve_program`) beside `AgentError`, `viola-mcp`'s `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError` (`Internal`, Display `internal error`), and `viola-state`'s `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`) beside `StateError` (arch [Error Handling]).»
  source: .andromeda/architecture.md:65 «two open divergences, `viola-mcp`'s `ServeError` and `viola-state`'s `StampsError`, with `PinError`, `StampsReadError` and `Refused` the three the census adds, the rule standing»
  replace-with: «Five open divergences, not exceptions, the rule standing, their owner a CARRY on the working-route entry "Version done-check" (PROVISIONAL): `viola-mcp`'s `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError` (`Internal`, Display `internal error`), `viola-state`'s `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`) beside `StateError`, and the three the census adds, `PinError`, `StampsReadError` and `Refused`; nine `thiserror` derives over the product sources in all (arch [Error Handling]).»

- leaf: .claude/docs/conventions.md:32   class: omission
  leaf-text: «Channel: refusal is a normal `result.refusal` + `detail`;»
  source: .andromeda/architecture-detail/conventions.md:27 «`reply_of` and `of_reply` read a `refusal` that is no code word as `unknown`, a string `detail` that is no code word as `null`, and a non-string `detail` as no known reply.»
  replace-with: «Channel: refusal is a normal `result.refusal` + `detail` (`unknown`'s a code word, `viola_core::is_code_word`, or `null`; `reply_of` and `of_reply` read a `refusal` that is no code word as `unknown`, a string `detail` that is no code word as `null`, and a non-string `detail` as no known reply);»

- leaf: .claude/docs/conventions.md:36   class: omission
  leaf-text: «- Full detail only to `instances/<name>/diagnostics/detail-<process>.ndjson`.»
  source: .andromeda/architecture-detail/cross-cutting-patterns.md:33 «CLI `--json` documents, MCP error results and channel error data carry codes and fixed messages only, no path and no upstream text;»
  replace-with: «- Full detail only to `instances/<name>/diagnostics/detail-<process>.ndjson`: CLI `--json` documents, MCP error results and channel error data carry codes and fixed messages only, no path and no upstream text; a peer's `message`, `data`, `refusal` or `detail` reaches no document or text but as a closed value, and a code word outside the closed list stays visible by design. `tests/contract_error_bodies.rs` enumerates the error bodies and censuses the product's error writers, and holds that a `thiserror` message interpolates nothing.»

- leaf: .claude/docs/services/viola-agent-claude.md:11   class: retired
  leaf-text: «`resolve_program`, `Refusal`, `PLUGIN_DIR_FLAG`»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6 «`resolve_program` (returning `Result<PathBuf, AgentError>`), `PLUGIN_DIR_FLAG`»
  replace-with: «`resolve_program` (returning `Result<PathBuf, AgentError>`), `PLUGIN_DIR_FLAG`»

- leaf: .claude/docs/services/viola-agent-claude.md:26   class: retired
  leaf-text: «`AgentError` sits at the crate root; the pre-existing `Refusal` beside it is the one open divergence (arch [Error Handling]).»
  source: .andromeda/architecture-detail/established-decisions.md:269 «`AgentError` holds `resolve_program`'s two refusals, `BatchScriptChild` and `ProgramNotFound`, each with a fixed message»
  replace-with: «`AgentError` sits at the crate root and also holds `resolve_program`'s two refusals, `BatchScriptChild` and `ProgramNotFound`, each with a fixed message (arch [Error Handling]).»

- leaf: .claude/docs/services/viola-agent-claude.md:29   class: retired
  leaf-text: «any other `.cmd`/`.bat` → `Refusal::BatchScriptChild`.»
  source: .andromeda/architecture-detail/established-decisions.md:269 «`AgentError` holds `resolve_program`'s two refusals, `BatchScriptChild` and `ProgramNotFound`, each with a fixed message»
  replace-with: «any other `.cmd`/`.bat` → `AgentError::BatchScriptChild`; its other refusal is `AgentError::ProgramNotFound`.»

- leaf: .claude/docs/services/viola-channel.md:6   class: omission
  leaf-text: «the result / refusal / error shapes,»
  source: .andromeda/architecture-detail/standard-contracts.md:118 «`viola_channel::sanitised_fault(&Value) -> Option<Value>` is the one fault reader, of `reply_of` and `of_reply`:»
  replace-with: «the result / refusal / error shapes, the one fault reader `sanitised_fault` of `reply_of` and `of_reply` (`None` without an integer `code`; each of the five codes reads its fixed `message` and a closed `data` or null; a code outside the five reads `{"code": <code>, "message": "wrapper fault", "data": null}`),»

- leaf: .claude/docs/services/viola-channel.md:27   class: omission
  leaf-text: «result class, error code, `sender` — never params or bodies;»
  source: .andromeda/architecture-detail/occupied-resources.md:130 «`channel-response`'s `detail` in `schemas/diag-line.v1.json` is the closed enum `unsupported-version` · `invalid-params` · `release-from-driver`, written at WARN on both the server's and the client's»
  replace-with: «result class, error code, `sender` — never params or bodies; on a `-32602` the `channel-response` line also carries `detail`, the closed enum `unsupported-version` · `invalid-params` · `release-from-driver`, at WARN on both the server's and the client's line (`ProtocolError::cause`, the client's `fault_of(&Value)` reading `data`), absent on every other `channel-response` line and holding no param value or name;»
