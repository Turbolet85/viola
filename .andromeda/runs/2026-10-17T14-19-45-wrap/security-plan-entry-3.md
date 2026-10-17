
## 2026-10-17-mcp-server-for-drivers — MCP tool input validation and error bodies as landed
**Section:** Input Validation (MCP tool inputs row; Paste text row, Where it runs); Error Handling (External responses, the MCP `isError` bullet)
**Change:**
- MCP tool inputs: each handler takes the raw argument object (never rmcp's typed deserialisation): `target` through `ViolaName::try_new`, `text` a string, `after` / `timeout_ms` absent, `null` or a `u64` (`wait`'s clamped to `WAIT_CAP` 50 000 ms, also its default), `dialog_id` a `u64`, `response` through `Response::parse`, then `validate_paste_text` on `send`'s text and every free-text answer value; a failure is `isError: true`, `{"error":"invalid-params","detail":"<field>"}` (closed field words `target` · `text` · `after` · `timeout_ms` · `dialog_id` · `response`), text `refused: invalid-params`, no frame; an unknown tool `-32602`.
- Paste text: was "MCP when it lands"; now the MCP `send` tool runs `validate_paste_text` then the empty-typed-text check client-side, each refused `not-delivered` / `control-character` | `empty-text` with one client-side `send-refused` line, before any frame.
- `isError` bullet: was the closed list `instance-unreachable | wrapper-fault`; now it adds `invalid-params`, `instance-unreachable` carrying `detail: null`, `wrapper-fault` the fault's `{code,message,data}`, every text `refused: <code>`; any other handler fault, a reply of no known shape or a panicked call is JSON-RPC `-32603` `internal error`, `data: null`.
**Why:** the chunk landed the validation the plan required; the bodies stay codes only.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
