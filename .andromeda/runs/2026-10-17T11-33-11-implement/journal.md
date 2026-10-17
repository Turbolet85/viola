# Journal — /implement 2026-10-17-mcp-server-for-drivers

One line per long run, written before waiting on it.

- 2026-10-17T11:55:36Z · P2 · `gate.py run --plan … --run-dir … --marker …` — the whole block (non-leg entries; the
  pre-push entry declares 7200 s), backgrounded; the verdict is its summary line.
  Read: green 20 · red 3 (15, 16, 23): the two mcp_tools test defects and the Windows mutation scope contract.
- 2026-10-17T12:06:24Z · P2 · the whole block again, after the three fixes (entry 15 alone read green at 15.4.log).
  Read: green 22 · red 1 (16): path7 timed out at the 20 s kill; cause read two-sided in
  `chunks/2026-10-17-mcp-server-for-drivers/evidence/unstamped-boot-race.md`, the test boots `builder` alone.
- 2026-10-17T12:15:40Z · P2 · the whole block a third time, after the path7 fix.
  Read: green 23 · red 0. Then the parity oracles added to `mcp_tools` (entries 1, 2, 15 green).
- 2026-10-17T12:20:50Z · step 15 · the one live start, ledgered (`evidence/live-sessions.ndjson`); read in
  `evidence/live-mcp.md`: the CLI refused `tools/list` (no `ttlMs` / `cacheScope`), no tool call. Not re-fired.
- 2026-10-17T12:23:54Z · P2 · the whole block a fourth time, after the `tools_result` fix.
  Read: green 22 · red 1 (23): every test passed; `llvm-profdata` refused one truncated `viola` profile
  (114688 B against 7361 siblings of 141984 B). The E2E client now owns the server child and never kills it.
- 2026-10-17T12:29:04Z · P2 · the whole block a fifth time, after that fix.
