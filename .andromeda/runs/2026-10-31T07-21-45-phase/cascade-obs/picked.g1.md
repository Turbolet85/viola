### obs-scope-summary#2.1 · pending · tier · ## 1. Obs Scope Summary · lines 17–17
- the `diagnostics/` directory root (tests `<home>/diagnostics/` vs arch and security `instances/<name>/diagnostics/`);
### obs-scope-summary#2.2 · pending · tier · ## 1. Obs Scope Summary · lines 18–18
- `mcp` logging to stderr when `VIOLA_DIR` is unset — reconciled by D-09 and landed (chunk 2026-10-17-mcp-server-for-drivers): `mcp` always writes `mcp.ndjson`, stderr JSON only when that file cannot be opened;
### obs-scope-summary#2.3 · pending · tier · ## 1. Obs Scope Summary · lines 19–19
- the `event` enum extensions (`release-from-driver` and the chaos signals);
### obs-scope-summary#3.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 26–26
- **Source:** arch excerpt, Workspace / Modules; Stack (clap 4.6.7, vt100 0.16.2, thiserror/anyhow)
### obs-scope-summary#3.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 27–27
- **Instrumentability:** instrumentable
### obs-scope-summary#3.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 28–28
- **Reason if not fully instrumentable:** N/A. This is the only crate that uses anyhow, so it is the error-capture boundary. Arch Stack says "context chains only at dispatch". Security Vector 6 says a chain that contains a serde source must become a fixed message before it reaches stderr. The full chain goes to `diagnostics/` only.
### obs-scope-summary#4.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 36–36
- **Reason if not fully instrumentable:** It is a pure type crate with no I/O and no runtime behaviour of its own. It supplies the kebab-case values used as log fields (`refusal`, `detail`, event kinds) but emits nothing. Excluded from sections 2 to 4.
### obs-scope-summary#5.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 42–42
- **Source:** arch excerpt, Stack (portable-pty) and Workspace
### obs-scope-summary#5.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 43–43
- **Instrumentability:** instrumentable
### obs-scope-summary#5.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 44–44
- **Reason if not fully instrumentable:** N/A for the seam. The ConPTY and openpty OS APIs behind it are boundary-only (see the OS PTY entry). The PTY byte stream is user content and is never logged (security Data Classifications, user content; brief §3.2 "Nobody parses the rendered screen for content").
### obs-scope-summary#6.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 50–50
- **Source:** arch excerpt, Stack (interprocess 2.4.4), Workspace, Standard Contracts (IPC methods `send · wait · last · answer · pause · release · link · unlink · hook.dialog · hook.event`)
### obs-scope-summary#6.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 51–51
- **Instrumentability:** instrumentable
### obs-scope-summary#6.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 52–52
- **Reason if not fully instrumentable:** N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.
### obs-scope-summary#7.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 58–58
- **Source:** arch excerpt, Stack (ndjson + atomic-write-file, sysinfo 0.39.6) and Workspace
### obs-scope-summary#7.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 59–59
- **Instrumentability:** instrumentable
### obs-scope-summary#7.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 60–60
- **Reason if not fully instrumentable:** N/A. Stack bans Tokio here, so instrumentation must be sync.
### obs-scope-summary#8.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 66–66
- **Source:** arch excerpt, Workspace; Stack (serde_path_to_error drift reports)
### obs-scope-summary#8.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 67–67
- **Instrumentability:** instrumentable
### obs-scope-summary#8.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 68–70
- **Reason if not fully instrumentable:** N/A. Its outputs are the most sensitive in the product:
  - Drift reports go to `diagnostics/` only (security Vector 4).
  - R8-stripped `CLAUDE*` values are never logged anywhere (security Vector 6).
### obs-scope-summary#9.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 78–78
- **Reason if not fully instrumentable:** N/A. stdout is reserved for MCP frames (arch Stack and Obs-Relevant Conventions), so no telemetry may go to stdout.
### obs-scope-summary#11.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 99–99
- **Reason if not fully instrumentable:** The caller is Claude Code, so viola sees only the inbound stdin payload and its own response; `viola mcp` reads that payload in lines of at most `MAX_FRAME`, a longer line refused whole. Instrument `hook-invoked` / `hook-decision`, MCP tool-call handling and the `viola mcp` stdin cut (one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}`, no byte of the line). No trace context arrives from Claude Code.
### obs-scope-summary#12.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 107–107
- **Reason if not fully instrumentable:** External CLI. Instrument the spawn/exit and the parse outcome. A parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.
### obs-scope-summary#13.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 115–115
- **Reason if not fully instrumentable:** A user-written shell command. Instrument the shell-out duration and exit status only. The command string is never logged outside `diagnostics/` (security Data Classifications, `statusline_command`).
### obs-scope-summary#14.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 123–123
- **Reason if not fully instrumentable:** Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals. The home's strict-modes checks (`check_instance`, `check_home_tree`, `check_diagnostics`: viola's stat and, on Windows, owner and DACL reads) are such call sites; which of their refusals write a line is the strict-modes refusal-telemetry trigger's (below).
