### obs-scope-summary#8.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 68–70
- **Reason if not fully instrumentable:** N/A. Its outputs are the most sensitive in the product:
  - Drift reports go to `diagnostics/` only (security Vector 4).
  - R8-stripped `CLAUDE*` values are never logged anywhere (security Vector 6).
### obs-scope-summary#9.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 76–76
- **Source:** arch excerpt, Stack (rmcp) and Workspace
### obs-scope-summary#9.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 77–77
- **Instrumentability:** instrumentable
### obs-scope-summary#9.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 78–78
- **Reason if not fully instrumentable:** N/A. stdout is reserved for MCP frames (arch Stack and Obs-Relevant Conventions), so no telemetry may go to stdout.
### obs-scope-summary#10.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 84–84
- **Source:** arch excerpt, Stack (portable-pty; "hosts the unmodified `claude`"); Project Intent
### obs-scope-summary#10.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 85–85
- **Instrumentability:** boundary-only
### obs-scope-summary#10.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 86–91
- **Reason if not fully instrumentable:** It is a third-party binary run unmodified. Instrument only the boundary:
  - spawn and exit (`process-start` / `process-exit`), with exit detected on the process handle, never on EOF (Stack; brief §4.1);
  - paste writes;
  - the vt100 readiness-gate outcome.

  Its internal state and transcripts are never read as a contract (creator Obs Anti-Patterns, brief §4).
### obs-scope-summary#11.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 97–97
- **Source:** arch excerpt, Standard Contracts (Hook contract; MCP tools)
### obs-scope-summary#11.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 98–98
- **Instrumentability:** boundary-only
### obs-scope-summary#11.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 99–99
- **Reason if not fully instrumentable:** The caller is Claude Code, so viola sees only the inbound stdin payload and its own response. Instrument `hook-invoked` / `hook-decision` and MCP tool-call handling. No trace context arrives from Claude Code.
### obs-scope-summary#12.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 105–105
- **Source:** arch excerpt, Stack (sysinfo + `claude agents --json`); security Vector 8
### obs-scope-summary#12.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 106–106
- **Instrumentability:** boundary-only
### obs-scope-summary#12.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 107–107
- **Reason if not fully instrumentable:** External CLI. Instrument the spawn/exit and the parse outcome. A parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.
### obs-scope-summary#13.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 113–113
- **Source:** arch excerpt, Standard Contracts (`viola hook statusline`, "the only shell-out in viola")
### obs-scope-summary#13.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 114–114
- **Instrumentability:** boundary-only
### obs-scope-summary#13.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 115–115
- **Reason if not fully instrumentable:** A user-written shell command. Instrument the shell-out duration and exit status only. The command string is never logged outside `diagnostics/` (security Data Classifications, `statusline_command`).
### obs-scope-summary#14.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 121–121
- **Source:** arch excerpt, Stack
### obs-scope-summary#14.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 122–122
- **Instrumentability:** boundary-only
### obs-scope-summary#14.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 123–123
- **Reason if not fully instrumentable:** Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals.
### obs-scope-summary#15.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 129–129
- **Source:** arch excerpt, Stack (ndjson + snapshots), Standard Contracts (event line, snapshot envelope, heartbeat); security Data Classifications
### obs-scope-summary#15.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 130–130
- **Instrumentability:** instrumentable (through the `viola-state` call sites)
### obs-scope-summary#15.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 131–134
- **Reason if not fully instrumentable:** N/A. Scrub rules per store (security Data Classifications):
  - `events.ndjson` is itself the product audit stream (brief §7, "the event log as the audit trail"). Its payload content is never copied into process-log lines.
  - Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
  - Operational metadata in `budget.json` (percentages, `resets_at`) may be logged as structured fields.
### obs-scope-summary#16.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 140–140
- **Source:** arch excerpt, Observability Hints (Config management); security Data Classifications (config values)
### obs-scope-summary#16.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 141–141
- **Instrumentability:** not-instrumentable
### obs-scope-summary#16.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 142–142
- **Reason if not fully instrumentable:** Static configuration and data. Only the parse at load time is observable, and it is logged through `viola-state` / root bin (skipped-key counts).
