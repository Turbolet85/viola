# Curation — 2026-10-17-mcp-server-for-drivers

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A harness-session case never assumes the fake agent's defaults …" (confidence 0.8)
  Proof: report Decisions & corrections 7 — the first `mcp_tools` firing went red on both premises (no `activity` / no turn end within the bound; no unwrapped row), read to its cause from the logs (`.andromeda/runs/2026-10-17T11-33-11-implement/journal.md`); signals: measurement +0.4, specific detail +0.2, no other home +0.2 (no master, route annotation or ledger note carries it).
  Tier 3 (.claude/docs/session-learnings.md): + "Where Claude Code logs a plugin MCP server" (confidence 0.8)
  Proof: report Decisions & corrections 2 — live start 1 (`evidence/live-mcp.md`): the CLI's `mcp-logs-plugin-viola-viola` log named `tools/list failed` on `ttlMs` / `cacheScope` in one read; signals: measurement +0.4, specific detail +0.2, no other home +0.2 (the masters record the requirement, not the diagnostic source).
                                              + "An absence check over `cargo tree -i` reads the printed line, never the exit" (confidence 0.7)
  Proof: report Decisions & corrections 8 and Deviations 2 — research measured exit 101 before rmcp/tokio entered `Cargo.lock`; the implemented form exits 0 `nothing to print` (`evidence/instrument-controls.md`); signals: measurement +0.4, "never" +0.3.
  Filters: 2 dup · 4 task-specific or at 0.6 · 0 conflict · 0 deferred
  CLAUDE.md size: see P7's health check 1.

Dispositions of the report's Decisions & corrections:
1. A live CLI can refuse what the protocol library's client accepts — rejected (Filter 4: 0.6; the requirement amended into architecture [MCP] / Conventions this wrap).
2. Diagnostic source for a plugin MCP server — applied, Tier 3.
3. Plugin MCP tool names and their allow — rejected (amended into architecture §Occupied Resources this wrap; 0.6).
4. rmcp's child-process transport kills — rejected (amended into test-plan §6 Drivers row and the architecture MCP row this wrap; 0.6).
5. The `pin_exe` race — not a learning: a route owner, P5 (the operator's direction at invocation, a CARRY on "Home and code-bearing file integrity").
6. Fixture gap (`PostToolUse.default`) — rejected (Filter 2: a fixture-specific value; 0.6).
7. Harness defaults a test must not assume — applied, Tier 2 `testing.md`.
8. `cargo tree -i` reads differently once the crate is in `Cargo.lock` — applied, Tier 3 (scripts are outside every rule file's `paths:`).
9. Windows mutation scope contract — rejected (0.6; the contract case states it, and test-plan carries the workflow's scope rule).
10. A red read to its cause before any re-run — rejected (Filter 1: the Tier 1 entry "A red stays open until its cause is known" — followed, not violated).
11. Rulings and words — not a learning (records of authority; carried in the report and the sidecars).
12. Guard refusals — not a learning (the hook refuses the act itself; the handoff's standing rules list them).
13. A sweep hazard (an unfired control's zero) — rejected (Filter 1: the skill's own P1 rule states it; no other home owed).
14. The live server logs no `process-exit` — rejected (amended into architecture §Occupied Resources Filesystem this wrap).
15. A Claude Code tool-call default over 50 s — rejected (amended into architecture [MCP] / [Delivery Confirmation] this wrap).
Conversation (this window): the halt's four answers — records, not learnings (`answer-card.md`).
