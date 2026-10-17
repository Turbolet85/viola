
## 2026-10-17-mcp-server-for-drivers — ServeError, a second open divergence
**Section:** §Established Decisions [Error Handling]; §Conventions Rust error types; §Inherited Defaults (Errors)
**Change:** was one open divergence (`viola-agent-claude`'s `Refusal`); now two: `viola-mcp` holds `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError` (`Internal`, Display `internal error`). The one-enum-per-crate rule stands; the fold or ratification of each is open.
**Why:** the chunk landed two enums in one crate. Recorded as an open divergence, not an exception, on the Viola overseer's answer at this wrap (a delegate's, PROVISIONAL), with no route work.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
