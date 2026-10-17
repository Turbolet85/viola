# Answer card — 2026-10-17-mcp-server-for-drivers

The founder's own answer to P4 fork Q1 (`inputs#I2`, given live in the overseer's dialog at 2026-10-17T11:18Z, relayed
verbatim by the overseer): «Восьмой gap + agents» — option 1: MCP `send` / `wait` / `last` / `answer` frames go after the
same liveness-only pre-check the CLI verbs use, an eighth dated gap closed by "Server verification before any frame" and
"Home and code-bearing file integrity"; MCP `list` reads agents as CLI `list` does (the `mcp` process spawns
`claude agents --json`).

Escalated proposals lying wholly inside that answer (numbered as `fanout-results.md` numbers them):
- architecture 6 — §Standard Contracts → Session liveness: `mcp`'s `list` tool makes the one `claude agents --json` read.
- architecture 7 — §Standard Contracts → Channel methods (`list`): MCP `list` builds the board through `board_document`.
- architecture 8 — §Infrastructure Patterns → Project directory structure: `run/agents.rs` also serves the MCP `list` tool.
- architecture 31 (its `list` clause) — §Established Decisions → [MCP]: `list` built, reading agents as CLI `list`.
- security-plan 1 — Input Validation (Child executable resolution): `mcp`'s `list` makes the one read per call.
- security-plan 2 — Threat Model Summary: `mcp` spawns `claude agents --json` once per `list` call.
- security-plan 6 — Authentication & Authorization: the eighth dated gap (MCP frame tools after the liveness-only pre-check).
- security-plan 7 — Security Anti-Patterns → Authentication: the MCP frame tools join the dated exceptions.
- security-plan 8 — Input Validation (CLI arguments gap list): the eighth reads `snapshot.json` through `live_endpoint`.
- security-plan 9 — Authentication & Authorization (strict-modes entry points): `mcp`'s start check of its home and
  `diagnostics/`; its frame tools read the endpoint under the eighth gap.

## The halt's answers (received in the session at about 2026-10-17T14:36Z, verbatim)

Answered in the wrap's P2 dialog; the notes name the answerer as the Viola overseer — a delegate, so every answer below
is PROVISIONAL, superseded by the founder's own word in the batch at the Epoch 5 boundary. The ratification of the ten
card items stays the founder's own answer to Q1 (`inputs#I2`), relayed by the overseer.

1. Answer card — "Confirm (Recommended)"; notes: «Overseer, 2026-10-17: each of the ten records only the eighth dated gap
   or MCP list making the one agents read, both inside his answer.»
2. ServeError — "Record as open divergence (Recommended)"; notes: «Technical; no route work.»
3. Tokio feature — "Write as built (Recommended)"; notes: «Test-only enabler; the release graph stays tokio-free outside
   viola-mcp, proven by channel-tokio-free.sh.»
4. mcp home — "As built, NOT ratified (Recommended)"; notes: «A boundary widening waits for the founder own word; it joins
   the carried VIOLA_DIR item.»
