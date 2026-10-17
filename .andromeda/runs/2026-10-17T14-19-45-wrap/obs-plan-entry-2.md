
## 2026-10-17-mcp-server-for-drivers — the mcp list tool's lines, the refused home check, invalid-params
**Section:** §6 Log Coverage (Boundary-call wrappers → Child / shell spawns; `detail` catalog → Filesystem refusals, the `state-entry` call site); §4 E5; §3 → OTel SDK init (steps 5–6); §3 → Logging stack (`mcp.ndjson`); §7 (the pre-step-4 window); §10 Zero unlogged panics → Bounded exemptions (items 1 and 7); §1 (Vector 5 trigger; `viola mcp` Notes)
**Change:**
- The `agents-probe` pair adds a writer: `viola mcp`'s `list` tool, into `mcp.ndjson`; the board read's `parse-rejected{parser:"state-entry"}`, `state-recovered` and per-key `config-json` lines go into `mcp.ndjson` for that tool — was `cli-<name>.ndjson` under a `VIOLA_NAME` only; the Logging stack key lists `mcp.ndjson`'s lines.
- §7 and exemption 1: the pre-init window adds `viola mcp`'s check of its home and `diagnostics/`; exemption 7 adds its refused check (exit 1, nothing printed, no role file, no line), the plan's lean approved by the operator at P5.
- Vector 5 and the mcp Notes: the codes-only set adds `invalid-params`, its `detail` a closed field word.
**Why:** the chunk landed the `list` tool (its agents read the founder's own answer to P4 fork Q1) and the start-time home check; no line carries row text, a path or user content.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
