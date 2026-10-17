
## 2026-10-17-mcp-server-for-drivers — MCP list builds the board and reads agents
**Section:** §Standard Contracts Session liveness, Channel methods (`list`); §Established Decisions [MCP] (its `list` clause); §Infrastructure Patterns → Project directory structure (`run/agents.rs`, `cmd/list.rs`, `cmd/mcp.rs`)
**Change:**
- Session liveness: was "`mcp`'s `list` is not built"; now the `mcp` process's `list` tool makes the same one `claude agents --json` read on each call — provider `list::board_document` (scan → read with the agents read and join → document) on a blocking thread, its `agents-probe` pair in `mcp.ndjson`, the board's refusal returned as `instance-unreachable`; the spawner list names it beside CLI `list`, `revive`'s preflight and verify's Run B.
- Channel methods `list`: MCP `list` builds the same board through `board_document` and returns the document's `ok`; CLI `list` keeps its own order (scan, its log opened, read, document).
- [MCP]: `list` builds the CLI's board and makes its one agents read.
- Directory structure: `run/agents.rs` also serves the `mcp` `list` tool; `cmd/list.rs` holds `board_document`; `cmd/mcp.rs` the hidden verb.
**Why:** a boundary widening (a new process spawning `claude`), ratified by the founder's own answer to P4 fork Q1, «Восьмой gap + agents», live in the overseer's dialog, relayed verbatim by the overseer. The wrap's answer-card confirm that these sections lie inside it is a delegate's (the Viola overseer) — PROVISIONAL, superseded by his own word at the Epoch 5 boundary batch.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
