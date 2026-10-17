
## 2026-10-17-mcp-server-for-drivers — the mcp sink is always mcp.ndjson
**Section:** §1 Obs Scope Summary (ipc-internal `viola mcp`: Exporter, Notes; Logging stack sink rules; Log file location; Obs tier justification items; the closing note)
**Change:** was "a file sink `diagnostics/mcp.ndjson` when `VIOLA_DIR` is set, otherwise stderr", the "stderr only for `mcp` without `VIOLA_DIR`" sink rule, the open misalignment flag and the reconciliation item; now `viola mcp` always writes `<home>/diagnostics/mcp.ndjson` (D-09), its home resolved through `resolved_home` and checked with `diagnostics/` before the sink opens, stderr JSON only when the file cannot be opened; the item reads reconciled; the closing note drops the `mcp` sink from the differences where §3, §6 and §12 win.
**Why:** the chunk built the verb on the D-09 form; §1 is a verbatim copy kept current (the founder's ruling of 2026-10-04), its label kept.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
