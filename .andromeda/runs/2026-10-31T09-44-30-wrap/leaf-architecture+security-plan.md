group: architecture+security-plan
read: .claude/rules/api.md · 41 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/rules/api.md:14   class: omission
  leaf-text: «A new listener or channel method needs a Security Decisions Log entry first.»
  source: .andromeda/architecture-detail/cross-cutting-patterns.md:33 «CLI `--json` documents, MCP error results and channel error data carry codes and fixed messages only, no path and no upstream text»
  replace-with: «A new listener or channel method needs a Security Decisions Log entry first. CLI `--json` documents, MCP error results and channel error data carry codes and fixed messages only, no path and no upstream text; `tests/contract_error_bodies.rs` enumerates every error body with its writer and witness (`CLI_DOCUMENTS`, `MCP_RESULTS`, `CHANNEL_FAULTS`, with `NOT_ERROR_BODIES` and `PLUMBING` the named exclusions), and `product_error_writers_are_each_named` fails on an unnamed product error writer and `thiserror_messages_interpolate_nothing` on an interpolating `thiserror` message.»
- leaf: .claude/rules/api.md:21   class: omission
  leaf-text: «`-32603` is exactly `"internal error"`, `data: null`.»
  source: .andromeda/architecture-detail/standard-contracts.md:118 «`viola_channel::sanitised_fault(&Value) -> Option<Value>` is the one fault reader, of `reply_of` and `of_reply`: `None` without an integer `code`;»
  replace-with: «`-32603` is exactly `"internal error"`, `data: null`. `viola_channel::sanitised_fault` is the one fault reader, of `reply_of` and `of_reply`: `None` without an integer `code`; for each of the five codes `message` is the fault's fixed text and `data` null, save the two `-32602` bodies that keep `data` (an integer `data.supported` with a `data.wrapper` of 1 to 64 ASCII letters, digits, `.`, `-` or `+`; `data.reason` exactly `release-from-driver`), any other `-32602` reading `invalid params` with `data` null.»
- leaf: .claude/rules/api.md:22   class: omission
  leaf-text: «`unknown` (`#[serde(other)]`)»
  source: .andromeda/security-plan-detail/error-handling.md:19 «Exception by design: a code word outside the closed list stays visible in the document and in `refused: <word>`, and its exit is `unknown`'s 14 by `reason_of`.»
  replace-with: «`unknown` (`#[serde(other)]`; by design a code word outside the closed list stays visible in the document and in `refused: <word>`, and its exit is `unknown`'s 14 by `reason_of`)»
- leaf: .claude/rules/api.md:22   class: omission
  leaf-text: «Details are closed per reason;»
  source: .andromeda/architecture-detail/conventions.md:27 «**A peer's reply:** `reply_of` and `of_reply` read a `refusal` that is no code word as `unknown`, a string `detail` that is no code word as `null`, and a non-string `detail` as no known reply.»
  replace-with: «Details are closed per reason (`unknown`'s a code word or null, a code word being `viola_core::is_code_word`'s 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first); `reply_of` and `of_reply` read a `refusal` that is no code word as `unknown`, a string `detail` that is no code word as `null`, and a non-string `detail` as no known reply;»
- leaf: .claude/rules/api.md:24   class: omission
  leaf-text: «a `budget` that is not a bool (`null` included), → `-32602` with `data: null`.»
  source: .andromeda/architecture-detail/occupied-resources.md:130 «`channel-response`'s `detail` in `schemas/diag-line.v1.json` is the closed enum `unsupported-version` · `invalid-params` · `release-from-driver`, written at WARN»
  replace-with: «a `budget` that is not a bool (`null` included), → `-32602` with `data: null`. The three `-32602` faults read `channel-response`'s closed `detail` `unsupported-version` · `invalid-params` · `release-from-driver` (`ProtocolError::cause`, `schemas/diag-line.v1.json`), at WARN on the server's and the client's line, no param name or value on it; every other `channel-response` line has no `detail`.»
- leaf: .claude/rules/api.md:28   class: omission
  leaf-text: «`{"v":1,"error":"instance-unreachable"|"wrapper-fault",…}`).»
  source: .andromeda/architecture.md:227 «(exit 20): `message` a fixed text and `data` a closed object or null, read through `viola_channel::sanitised_fault`»
  replace-with: «`{"v":1,"error":"instance-unreachable"|"wrapper-fault",…}`, `wrapper-fault`'s `detail` `{code,message,data}` with `message` a fixed text and `data` a closed object or null, read through `viola_channel::sanitised_fault`).»
- leaf: .claude/rules/api.md:29   class: omission
  leaf-text: «the hidden `viola mcp`'s home or `diagnostics/` (each that exists) failing `viola_state::strict::check_path` (nothing on stdout or stderr, no role file)»
  source: .andromeda/architecture-detail/established-decisions.md:131 «`viola mcp` on a line past `MAX_FRAME` before `initialize` exits 1 with both streams empty and one `parse-rejected{mcp-stdin, oversize, count:1}` line»
  replace-with: «the hidden `viola mcp`'s home or `diagnostics/` (each that exists) failing `viola_state::strict::check_path` (nothing on stdout or stderr, no role file) or a stdin line past `MAX_FRAME` before `initialize` (both streams empty, one `parse-rejected{mcp-stdin, oversize, count:1}` line; exit 0 once initialized)»
- leaf: .claude/rules/api.md:29   class: omission
  leaf-text: «and `viola send` text over `MAX_FRAME` or not UTF-8»
  source: .andromeda/security-plan-detail/error-handling.md:23 «Exception: the exit-2 paths of `viola answer` and `viola send` write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says, and no document»
  replace-with: «and `viola send` text over `MAX_FRAME` or not UTF-8 (the exit-2 paths of `viola answer` and `viola send` write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says, and no document)»
- leaf: .claude/rules/api.md:29   class: omission
  leaf-text: «20 wrapper fault (a `-32602` from `link` / `unlink` included:»
  source: .andromeda/architecture-detail/standard-contracts.md:119 «A fault code outside the five reads `{"code": <code>, "message": "wrapper fault", "data": null}`: the literal `wrapper fault` is a fixed output word on `--json` and MCP»
  replace-with: «20 wrapper fault (a fault code outside the five reading `{"code": <code>, "message": "wrapper fault", "data": null}` on `--json` and MCP and `error: wrapper fault  <code>` on the human line; a `-32602` from `link` / `unlink` included:»
- leaf: .claude/rules/api.md:35   class: omission
  leaf-text: «refusal, unreachable, fault or invalid params: `isError:true` + codes only, text `refused: <code>`»
  source: .andromeda/security-plan.md:413 «`wrapper-fault` with `{code,message,data}` as `viola_channel::sanitised_fault` closes them, never the peer's own»
  replace-with: «refusal, unreachable, fault or invalid params: `isError:true` + codes only (`wrapper-fault` with `{code,message,data}` as `viola_channel::sanitised_fault` closes them, never the peer's own), text `refused: <code>`»
