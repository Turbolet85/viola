### obs-scope-summary#1.1 · values · tier · ## 1. Obs Scope Summary · lines 8–8
- 5 telemetry surface entries across 4 process roles (`run` / `hook` / `mcp` / `cli`) and 2 product parts;
### obs-scope-summary#1.2 · values · tier · ## 1. Obs Scope Summary · lines 9–9
- 6 instrumentable crates;
### obs-scope-summary#1.3 · values · tier · ## 1. Obs Scope Summary · lines 10–10
- 6 must-trace paths plus 4 edge flows;
### obs-scope-summary#1.4 · values · tier · ## 1. Obs Scope Summary · lines 11–11
- 18 telemetry triggers, including cross-surface propagation through `corr`, perf budgets, chaos fault telemetry, multi-OS compat, and binding Founder Directions.
### obs-scope-summary#2.1 · pending · tier · ## 1. Obs Scope Summary · lines 17–17
- the `diagnostics/` directory root (tests `<home>/diagnostics/` vs arch and security `instances/<name>/diagnostics/`);
### obs-scope-summary#2.2 · pending · tier · ## 1. Obs Scope Summary · lines 18–18
- `mcp` logging to stderr when `VIOLA_DIR` is unset — reconciled by D-09 and landed (chunk 2026-10-17-mcp-server-for-drivers): `mcp` always writes `mcp.ndjson`, stderr JSON only when that file cannot be opened;
### obs-scope-summary#2.3 · pending · tier · ## 1. Obs Scope Summary · lines 19–19
- the `event` enum extensions (`release-from-driver` and the chaos signals);
### obs-scope-summary#2.4 · pending · tier · ## 1. Obs Scope Summary · lines 20–20
- `hook-invoked` `corr` timing relative to `dialog_id` assignment.
### obs-scope-summary#3.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 26–26
- **Source:** arch excerpt, Workspace / Modules; Stack (clap 4.6.7, vt100 0.16.2, thiserror/anyhow)
### obs-scope-summary#3.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 27–27
- **Instrumentability:** instrumentable
### obs-scope-summary#3.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 28–28
- **Reason if not fully instrumentable:** N/A. This is the only crate that uses anyhow, so it is the error-capture boundary. Arch Stack says "context chains only at dispatch". Security Vector 6 says a chain that contains a serde source must become a fixed message before it reaches stderr. The full chain goes to `diagnostics/` only.
### obs-scope-summary#4.1 · values · scope-entities · ## 1. Obs Scope Summary · lines 34–34
- **Source:** arch excerpt, Workspace / Modules
### obs-scope-summary#4.2 · values · scope-entities · ## 1. Obs Scope Summary · lines 35–35
- **Instrumentability:** not-instrumentable
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
