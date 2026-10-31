### obs-scope-summary#2.3 · pending · tier · ## 1. Obs Scope Summary · lines 19–19 · add-to
- the `event` enum extensions (`release-from-driver` and the chaos signals);
### obs-scope-summary#2.4 · pending · tier · ## 1. Obs Scope Summary · lines 20–20 · stale
- `hook-invoked` `corr` timing relative to `dialog_id` assignment.
### obs-scope-summary#4.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 36–36 · stale
- **Reason if not fully instrumentable:** It is a pure type crate with no I/O and no runtime behaviour of its own. It supplies the kebab-case values used as log fields (`refusal`, `detail`, event kinds) but emits nothing. Excluded from sections 2 to 4.
### obs-scope-summary#12.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 107–107 · stale
- **Reason if not fully instrumentable:** External CLI. Instrument the spawn/exit and the parse outcome. A parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.
### obs-scope-summary#13.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 115–115 · stale
- **Reason if not fully instrumentable:** A user-written shell command. Instrument the shell-out duration and exit status only. The command string is never logged outside `diagnostics/` (security Data Classifications, `statusline_command`).
### obs-scope-summary#14.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 123–123 · stale
- **Reason if not fully instrumentable:** Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals. The home's strict-modes checks (`check_instance`, `check_home_tree`, `check_diagnostics`: viola's stat and, on Windows, owner and DACL reads) are such call sites; which of their refusals write a line is the strict-modes refusal-telemetry trigger's (below).
### obs-scope-summary#15.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 131–135 · stale
- **Reason if not fully instrumentable:** N/A. Scrub rules per store (security Data Classifications):
  - `events.ndjson` is itself the product audit stream (brief §7, "the event log as the audit trail"). Its payload content is never copied into process-log lines.
  - Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
  - A `snapshot.json` the replay rebuilds from `events.ndjson` (`read_snapshot_or_replay`) carries no `endpoint`, `pid`, `started_at` or `statusline_command`: `live_endpoint` over it is `None` (unreachable), hook `connect` is unreachable (fail open, `channel-unreachable`), `hook statusline` runs and prints nothing, and `run`'s collision check reads the heartbeat alone.
  - Operational metadata in `budget.json` (percentages, `resets_at`) may be logged as structured fields.
### obs-scope-summary#18.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 160–160 · stale
- **Service identity:** `service.name` is hardcoded `"viola"`, the binary name. `service.version` is compile-time `env!("CARGO_PKG_VERSION")`, the same value already used as `sender` and `writer`. The process role is the `process` field. The instance is `ViolaName` from an explicit argument or `VIOLA_NAME`. `deployment.environment` is N/A: local-only, with no environments (arch Obs-Relevant Conventions, Service identity; arch Surfaces, "no hosting").
### obs-scope-summary#19.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 173–173 · stale
- **Exporter:** A file sink only, to `diagnostics/run-<name>.ndjson` (tests excerpt, Log format). The terminal is forbidden: "`run` writes nothing to the terminal while the child runs" (arch Observability Hints).
### obs-scope-summary#19.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 174–174 · stale
- **Service identity:** as for the short-lived verbs, with `process="run"` and `instance=<ViolaName>`.
### obs-scope-summary#20.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 186–186 · stale
- **Exporter:** A file sink only, to `diagnostics/hook-<name>.ndjson`. stderr is forbidden on both. stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output (arch Standard Contracts, Hook contract); the two hidden `--capture` arms (`verify`'s probe capture) write no sink and print nothing, except `--answers`' probe decision body. A failed log write is swallowed, and the hook still exits 0 (tests excerpt, Log format Constraints).
### obs-scope-summary#20.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 187–187 · stale
- **Service identity:** as for the short-lived verbs, with `process="hook"`. If `VIOLA_NAME` is missing, `instance` is null and the hook is a silent no-op (tests excerpt, E1 R5 unwrapped session hooks; Coverage Triggers, Vector 4 "missing `VIOLA_NAME`"). The event and dialog paths resolve the instance directory (`canonical`) and run its strict-modes check (`trusted`: `check_instance`) before `read_diagnostics_level` and obs init: a refused or absent instance is a silent exit 0 with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append. The two `--capture` arms run ahead of the check; the `FAKE_AGENT_HOOK_PANIC` seam runs after obs init.
### obs-scope-summary#20.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 188–191 · stale
- **Notes:**
  - Many hook processes can append to the same `hook-<name>.ndjson` at once. Each line must be one `write` call on an append-mode handle (arch Stack, atomic-write-file / `OpenOptions::append`).
  - The latency budget is `max < 1.0 s` per hook (tests excerpt, Quality Gates). Logger init must not add blocking I/O beyond one file open and append; the instance check ahead of it (`canonical` + `trusted`: stat and, on Windows, owner and DACL reads) is in the timed path, and `run --perf` keeps the five hook rows under `SPINE_DEADLINE` with it, max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms (as measured at `.andromeda/runs/2026-10-26T19-02-07-implement/gate-2026-10-26-home-and-code-bearing-file-integrity.json`). `viola hook statusline` is outside that budget: its shell-out of the user's command is bounded by the 5 s PROVISIONAL `STATUSLINE_DEADLINE`, and no `run --perf` row times it.
  - Where no instance directory resolves, file logging is impossible. That case gets no log and no stderr, by contract. A `VIOLA_DIR` naming an instance directory that does not exist resolves none, so the hook writes nothing.
### obs-scope-summary#21.4 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 200–200 · stale
- **Service identity:** Each side logs its own `process`. The peer version is available as `params.sender` (`CARGO_PKG_VERSION`).
### obs-scope-summary#22.3 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 212–212 · add-to
- **Exporter:** Always the file sink `<home>/diagnostics/mcp.ndjson` (tests excerpt; D-09), the home resolved through `resolved_home` like every verb's and checked with its `diagnostics/` by strict-modes before the sink opens; stderr JSON only when that file cannot be opened (as landed, chunk 2026-10-17-mcp-server-for-drivers). stdout is forbidden because it carries MCP frames. tracing-subscriber `fmt()` writes to stdout unless a writer is configured, so the writer must be set explicitly. rmcp reads stdin only through viola's line cap (`CappedLines` at `MAX_FRAME`): a line of at most `MAX_FRAME` bytes, its newline counted, passes whole; a longer line is refused whole, never truncated or skipped, and ends the session as a closed stdin does, stderr empty, its one record the §6 `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line.
### obs-scope-summary#25.0 · mirror · log-fields · ## 1. Obs Scope Summary · lines 259–279 · add-to
```markdown
- **Format:** JSON-per-line. There are two streams:
  - **Event stream:** product events in `instances/<name>/events.ndjson`, exactly the arch Event line `{"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}`.
  - **Process logs:** emitted by tracing-subscriber 0.3.23 `fmt().json().flatten_event(true).with_current_span(false)`, writing to `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`. Files are 0600, dirs 0700, with one `write` per line.
- **Required fields:**
  - For process logs: `timestamp` (RFC 3339 UTC with ms and `Z`), `level` (`DEBUG|INFO|WARN|ERROR`), `target`, and `fields.message` (or the flattened `message`).
  - `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums. Each value fixes what its `corr` holds:
    - `channel-request`, `channel-response`: `corr` = the JSON-RPC request `id`
    - `dialog-raised`, `dialog-answered`: `corr` = `dialog_id`
    - `hook-invoked`, `hook-decision`: `corr` = `dialog_id` for dialog hooks, otherwise null
    - `send-issued`, `send-confirmed`, `send-refused`: `corr` = the send `cursor`
    - `process-start`, `process-exit`, `panic`: `corr` = null
  - `process` (`run|hook|mcp|cli`) and `instance` (a `ViolaName` or null).
  - `corr` is copied unchanged as a JSON number or string. It is never renamed (for example to `correlation_id`).

  obs-plan may add fields but must not rename or remove these. The harness greps on them.
- **Constraints:**
  - The hook trace is the `hook-<name>.ndjson` file. A failed write there is swallowed, so the hook still exits 0 and never writes to stderr.
  - No line may contain a `Cookie` header or any stripped `CLAUDE*` value. This is enforced by the secret-scan test in §6.
- **Agent parsing:** lines parseable with `jq -c 'select(.event=="dialog-raised")'` or equivalent; NEVER multi-line stack traces. A panic in a hook is caught and logged as one `level:"ERROR", event:"panic"` line.
```
### obs-scope-summary#28.3 · values · paths · ## 1. Obs Scope Summary · lines 310–310 · add-to
- **Required log fields:** `event`, `process`, `instance`, `corr` (the JSON-RPC `id` on channel lines; the send `cursor` on send lines), `from` (self-reported, not an identity), `confirmed`, `refusal`, `detail` (`input-not-ready` / `no-prompt-submitted` / `control-character` / `empty-text` / `turn-running`), `duration_ms`. Sent text never appears.
### obs-scope-summary#29.3 · values · paths · ## 1. Obs Scope Summary · lines 319–319 · add-to
- **Required log fields:** `event`, `process`, `instance`, `corr` (JSON-RPC `id`), `after` (byte offset), `timeout_ms`, `outcome` (woken kind / `timed_out` / `instance-unreachable`), `exit_code` (21 when the wrapper is killed mid-wait), `duration_ms`. The `last_assistant_message` content never appears.
### obs-scope-summary#31.2 · values · paths · ## 1. Obs Scope Summary · lines 336–336 · add-to
- **Must-trace spans:** `run.wheel_transition` › `channel.request(send)` refused › `pause.client` / `release.client` › `channel.request(pause|release)`. Log events: `send-refused` and `channel-request` / `channel-response` for `pause` and `release`. `release-from-driver` is logged as its own event (Founder Direction 4), admitted by `schemas/diag-line.v1.json`.
### obs-scope-summary#33.1 · values · triggers · ## 1. Obs Scope Summary · lines 353–356 · add-to
- **Required telemetry signal:**
  - A scrub rule on `channel-request` / `channel-response`: method, `id` / `corr`, result class (`ok` / `refusal` / `error` code) and `sender` version only, never `params` or result bodies.
  - `-32603` is logged as the fixed "internal error".
  - `from` is logged as unauthenticated.
### telemetry-strategy#1.2 · values · strategy · ## 2. Telemetry Strategy · lines 11–11 · contradict
| Structured Logs | Every boundary call, state transition, refusal, fault/heal and process lifecycle, as the closed `event` enum (Section 3) | **The primary signal.** tracing-subscriber 0.3.23 JSON-per-line, one file per process role under `<home>/diagnostics/`, merged with `events.ndjson` by `agent-run logs` |
### telemetry-strategy#1.3 · values · strategy · ## 2. Telemetry Strategy · lines 12–12 · contradict
| Errors | Panics (a one-line `event:"panic"` record), typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit` | No external reporter: security says "no outbound network calls". A hand-written `std::panic::set_hook`, plus `std::panic::catch_unwind` at the main-thread catch site of every role, around `hook` dispatch, in the `run` PTY pump and handle-wait threads, and around the vt100 feed (Section 7) |
### telemetry-strategy#3.15 · values · triggers · ## 2. Telemetry Strategy · lines 42–42 · stale
| creator-explicit (Founder Directions 1/2/5) | §3 schema verbatim, §6 |
