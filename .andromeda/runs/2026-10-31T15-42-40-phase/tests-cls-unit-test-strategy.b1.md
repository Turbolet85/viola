# Classified — tests · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · yes · the 1–64 byte lower-case `[a-z0-9-]` rule of `viola_core::is_code_word` binds every new closed code, since the reply screens read a non-code word as `unknown` or null through it
unit-test-strategy#1.2 · yes · the fixed -32603 "internal error" with `data:null` binds an `internal-error` code, and `sanitised_fault` binds every peer fault's reply screen on CLI and MCP
unit-test-strategy#1.3 · yes · the liveness `classify` (5 s stale bound), `server_verdict` and `live_endpoint` bind the not-running, stale-heartbeat and server-verification causes, and `check_instance` and revive's preflight bind the strict-modes and `session-live` refusals
unit-test-strategy#1.4 · yes · `resolve_program`'s `BatchScriptChild` and `ProgramNotFound` refusals and their fixed messages bind the script-child cause, the `claude`-not-on-`PATH` `internal-error` case and the `revive --json` gap
unit-test-strategy#1.5 · yes · `of_reply`'s screen (a non-code refusal reads `unknown`, a non-code detail reads null, a peer fault reads through `sanitised_fault`) binds the MCP reply surface in `crates/viola-mcp/src/result.rs` and `tools.rs`
unit-test-strategy#2.1 · yes · the exit-code mapping to 0/1/2/10–14/20/21 and the `reply_of` screen bind each new cause's exit and the typed exit set the code must stay inside
