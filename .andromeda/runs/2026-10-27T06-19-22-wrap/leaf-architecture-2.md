group: architecture-2
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»

- leaf: .claude/docs/services/viola-mcp.md:6   class: omission
  leaf-text: «It is the only crate allowed to build a Tokio runtime.»
  source: .andromeda/architecture-detail/established-decisions.md:118 «`viola mcp` hands rmcp its stdin through the crate-private `viola_mcp::stdin::CappedLines` at `MAX_FRAME` bytes per line, the `\n` counted, as the reader of rmcp's `(reader, writer)` stdio transport»
  replace-with: «It is the only crate allowed to build a Tokio runtime. It hands rmcp its stdin through the crate-private `viola_mcp::stdin::CappedLines` at `MAX_FRAME` bytes per line, the `\n` counted, as the reader of rmcp's `(reader, writer)` stdio transport (rmcp's features stay `server` + `transport-io`): a line of exactly the cap passes whole, and at the end of stdin a held unterminated line within the cap is released; a longer line releases no byte to rmcp, sets the reader's cut flag and then reads as end of stream (never an I/O error, so rmcp writes nothing of its own), so the line is refused whole, never truncated or skipped, and the session ends as at a closed stdin. The bound sits in this reader, never in rmcp's codec, because rmcp 3.4.1's stdio `AsyncRwTransport::receive` reads each line with an unbounded `read_until`.»

- leaf: .claude/docs/services/viola-mcp.md:23   class: omission
  leaf-text: «The `rmcp` log target is OFF.»
  source: .andromeda/architecture-detail/established-decisions.md:130 «After a session a cut ended, `viola_mcp::serve` writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN through `obs_event!`, holding no byte of the line, then returns what»
  replace-with: «The `rmcp` log target is OFF. After a session a stdin line past `MAX_FRAME` ended, `viola_mcp::serve` writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN through `obs_event!`, holding no byte of the line, then returns what the session gave: `Ok(())` once the session initialized (exit 0, with `process-exit`, as at a closed stdin), or `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes.»
