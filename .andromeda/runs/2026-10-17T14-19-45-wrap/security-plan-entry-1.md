
## 2026-10-17-mcp-server-for-drivers — the eighth dated gap: MCP frame tools after the liveness-only pre-check
**Section:** Authentication & Authorization (IPC client-side server verification row; the `~/.viola/` strict-modes entry points); Security Anti-Patterns → Authentication; Input Validation (CLI arguments, the interim gaps reading `snapshot.json`)
**Change:**
- Server verification row: was seven dated gaps; now a sixth interim gap, the eighth dated gap overall — until "Server verification before any frame" and "Home and code-bearing file integrity" land, `viola mcp`'s `send` / `wait` / `last` / `answer` tools write their frames through the Tokio client after the liveness-only pre-check (`viola_state::liveness::live_endpoint`; else `{"error":"instance-unreachable","detail":null}`), checking neither the server's identity nor the snapshot's strict-modes, borrowing none of the seven earlier gaps.
- Anti-Patterns → Authentication: the dated exceptions add the MCP tool frames (the eighth).
- Strict-modes entry points: `mcp` checks at start only its home and `diagnostics/` (each that exists) through `check_path` before anything under the home opens (exit 1, nothing printed, no role file); its frame tools read `snapshot.json`'s `endpoint` under the eighth gap, without the check; the gap list adds the eighth.
- CLI arguments gap list: an eighth — the MCP frame tools read `snapshot.json` the same way.
**Why:** a boundary widening ratified by the founder's own answer to P4 fork Q1, «Восьмой gap + agents», live in the overseer's dialog on 2026-10-17, relayed verbatim by the overseer. The wrap's answer-card confirm is a delegate's (the Viola overseer) — PROVISIONAL, superseded by his own word at the Epoch 5 boundary batch. The gap closes with the other seven at the two Epoch 6 entries.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
