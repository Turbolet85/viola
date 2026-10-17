
## 2026-10-17-mcp-server-for-drivers — the MCP wait cap, a driver-only bound
**Section:** §8 Cognitive Accessibility → Timeout extensions
**Change:** beside CLI `viola wait`'s caller-passed timeout, the MCP `wait` tool clamps `timeout_ms` to `WAIT_CAP` 50 000 ms, also its default (`viola_mcp::wait_timeout`), so an MCP `wait` with no timeout is answered `timed-out` after 50 s; the bound falls on the driver only (the hidden, plugin-run `viola mcp` serves a driving session, never a person) — no key blocked, refused or delayed, the wheel untouched.
**Why:** the chunk closed the MCP `wait` bound below the client's tool-call timeout; a person's timing is not touched.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
