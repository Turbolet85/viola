# obs · cascade read · group 1 of 9

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:165) This trigger is the chunk's own obligation: each exit-21 and exit-1 cause must carry a distinct code, and a check refused before any log opens must write no line, its cause held by the pair or the document.
  > - Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code, except a strict-modes check refused before any log opens, which writes no line: its cause is carried only by the fixed stderr pair or the `--json` document (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, exit 21; `run`, `viola revive` and `viola verify`, exit 1).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:163)
  > - Refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text (design excerpt, CLI refusals / CLI faults).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:164)
  > - Exit codes 0, 1, 2, 10–14, 20 and 21 are the machine-readable outcome and should be a log field (arch Standard Contracts, CLI `--json` + exit codes).
  (.andromeda/obs-plan.md:253)
  > - **Required telemetry signal:** `release-from-driver` is its own logged event, and each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid, except a strict-modes check refused before any log opens, which writes no line (its cause is the fixed stderr pair or the `--json` document).
  (.andromeda/obs-plan.md:161)
  > - **Required log fields:** exit-1 causes use a distinct `detail` code, with no path or pid in it.
  (.andromeda/obs-plan.md:140)
  > The distinct detail codes for exit 21 and exit 1 can be an additive `detail` field.
  (.andromeda/obs-plan.md:111)
  > refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:159) The table's cases must hold each verb's refusal as this text rules it, opening nothing and writing no line where it says so, and the chunk's contract choice must amend the exact `--json` document it states if that document moves.
  > Without an instance there is no process-log file, and human stderr carries only fixed-message lines (design excerpt, CLI faults). Network OTLP is excluded: security says "viola makes no outbound network calls". The stdout exporter is excluded because stdout carries `--json` results. `viola revive <name> --list` logs as `cli` into the caller's `cli-<VIOLA_NAME>.ndjson` once its strict-modes check has passed (none with no `VIOLA_NAME`; a refused check opens none; chunk 2026-10-17-cli-machine-contract). `viola link` / `viola unlink` log as `cli` into the driven instance's `cli-<driven>.ndjson` (the instance their `<driven>` argument names, never the caller's `VIOLA_NAME`) once their instance check of it has passed; a refused check opens none. `send`, `wait`, `last`, `answer`, `pause` and `release` log as `cli` once their instance check (`refuse_untrusted`: `check_instance` on the home and `instances/<name>`) has passed; a refused check exits 21 with the instance pair, or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document, opens nothing under the home and writes no line. `viola revive`'s start arm is not a short-lived verb: it is the wrapper surface below (`process="run"`, `diagnostics/run-<name>.ndjson`). `viola verify` logs as `cli` (with an instance) once its home-tree check (`check_home_tree` on the home, `ledger/`, `bin/` and `plugin/`) has passed; a refused check exits 1 with the home pair, opens no log and writes no line, and a refused `update_stamps` (`StampsError::StrictModes`) exits 1 with the same pair.
  (.andromeda/obs-plan.md:264)
  > Section 1 is a verbatim copy of obs-scope.md, and it is kept current (the founder's ruling of 2026-10-04): a proposal or a cascade hit inside it is judged like any body amendment, and its label stays.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:123) The server-check cause must keep its logged form on the CLI and the hook and no line on MCP, whatever code it gains, and any strict-modes refusal's line must follow the trigger this text names.
  > - **Reason if not fully instrumentable:** Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals. The home's strict-modes checks (`check_instance`, `check_home_tree`, `check_diagnostics`: viola's stat and, on Windows, owner and DACL reads) are such call sites; which of their refusals write a line is the strict-modes refusal-telemetry trigger's (below).
  (.andromeda/obs-plan.md:215)
  > plus refusal telemetry for pinned-exe hash mismatches and ignored symlinks; a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate and on a `conpty/` companion its owner/DACL check refuses (`process-start`'s `sideload_fallback:"strict-modes-failed"`, the child on the inbox ConPTY, exit unchanged); a refused `diagnostics/` folder or file is a failed open, its line dropped.
  (.andromeda/obs-plan.md:91)
  > - **Instrumentability:** boundary-only — viola's call sites, not the OS internals.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:18) `viola mcp`'s reached code must keep this sink for any line it gains, and its refused home check's row or named exclusion must be read against it, nothing on stderr being the scope's stated behaviour there.
  > - `mcp` logging to stderr when `VIOLA_DIR` is unset — reconciled by D-09 and landed (chunk 2026-10-17-mcp-server-for-drivers): `mcp` always writes `mcp.ndjson`, stderr JSON only when that file cannot be opened;
  (.andromeda/obs-plan.md:127)
  > always the file sink `<home>/diagnostics/mcp.ndjson`, stderr JSON only when that file cannot be opened; stdout is forbidden because it carries MCP frames, so the writer is set explicitly; `isError` results and the log lines for rejected calls carry codes only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:78) The new codes must pass through `viola-mcp`'s results with stdout carrying MCP frames alone and no telemetry on it.
  > - **Reason if not fully instrumentable:** N/A. stdout is reserved for MCP frames (arch Stack and Obs-Relevant Conventions), so no telemetry may go to stdout.
  (.andromeda/obs-plan.md:71)
  > - **Instrumentability:** instrumentable; stdout is reserved for MCP frames, so no telemetry may go to stdout.
  (.andromeda/obs-plan.md:205)
  > - **Required telemetry signal:** rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
  (.andromeda/obs-plan.md:313)
  > - `mcp` writes only MCP frames to stdout.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:99) The catalogue's MCP codes must be recorded at MCP tool-call handling, and the hook's cause record must stay `hook-decision`, with no hook cause added.
  > Instrument `hook-invoked` / `hook-decision`, MCP tool-call handling and the `viola mcp` stdin cut (one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}`, no byte of the line). No trace context arrives from Claude Code.
  (.andromeda/obs-plan.md:79)
  > - **Instrumentability:** boundary-only — viola sees only the inbound stdin payload and its own response; no trace context arrives from Claude Code.
  (.andromeda/obs-plan.md:119)
  > without `VIOLA_NAME`, on an instance the instance check refuses, or on a `VIOLA_DIR` naming an instance directory that does not exist, the hook is a silent no-op (exit 0, no file opened, no line, no frame)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:28) The `internal-error` cause and every new code crossing the root bin must hold this boundary: a serde-sourced chain is a fixed message before stderr, and the full chain stays in `diagnostics/`.
  > - **Reason if not fully instrumentable:** N/A. This is the only crate that uses anyhow, so it is the error-capture boundary. Arch Stack says "context chains only at dispatch". Security Vector 6 says a chain that contains a serde source must become a fixed message before it reaches stderr. The full chain goes to `diagnostics/` only.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:27)
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan.md:47)
  > - **Instrumentability:** instrumentable — the only crate that uses anyhow, so the error-capture boundary: a chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to `diagnostics/` only.
  (.andromeda/obs-plan.md:210)
  > a serde-sourced anyhow chain becomes a fixed message before stderr
- (.andromeda/obs-plan-detail/obs-scope-summary.md:36) New closed codes added to `viola-core` must stay pure values the crate supplies, with no I/O and no line of its own.
  > - **Reason if not fully instrumentable:** It is a pure type crate with no I/O and no runtime behaviour of its own. It supplies the kebab-case values used as log fields (`refusal`, `detail`, event kinds) but emits nothing. Excluded from sections 2 to 4.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:35)
  > - **Instrumentability:** not-instrumentable
  (.andromeda/obs-plan.md:51)
  > - **Instrumentability:** not-instrumentable — a pure type crate with no I/O that supplies log-field values but emits nothing; excluded from sections 2 to 4.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:52) The endpoint-lost cause edited in the channel client must log, if it logs there, synchronously, with channel `-32603` keeping its null `data`.
  > - **Reason if not fully instrumentable:** N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:51)
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan.md:59)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Tokio is forbidden in this crate's graph unless its feature is enabled.
  (.andromeda/obs-plan.md:123)
  > `-32603` is `"internal error"` with `data:null`
- (.andromeda/obs-plan-detail/obs-scope-summary.md:134) The table's not-running, stale-heartbeat and already-live cases must read the unreachable result and the heartbeat-only collision check this states.
  >   - A `snapshot.json` the replay rebuilds from `events.ndjson` (`read_snapshot_or_replay`) carries no `endpoint`, `pid`, `started_at` or `statusline_command`: `live_endpoint` over it is `None` (unreachable), hook `connect` is unreachable (fail open, `channel-unreachable`), `hook statusline` runs and prints nothing, and `run`'s collision check reads the heartbeat alone.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:133)
  >   - Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
  (.andromeda/obs-plan.md:95)
  > the absolute paths in `snapshot.json` and config values are scrubbed from external output.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:171) Any log form a start refusal gains in `run` must go through the JSON logger synchronously, with no Tokio.
  > - **Backend instrumentation hooks:** `tracing` spans (`#[tracing::instrument]`) on the start sequence, the `viola-pty` seam operations, the `viola-channel` sync server handlers, vt100 readiness-gate outcomes and `viola-state` writes, all written through the tracing-subscriber JSON logger. Instrumentation must be sync: std threads, no Tokio (arch Stack, Hybrid concurrency; `cargo deny` ban).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:173) A logged `run` or revived-start cause must reach `run-<name>.ndjson` only, and no cause may write to the terminal while the child runs.
  > - **Exporter:** A file sink only, to `diagnostics/run-<name>.ndjson` (tests excerpt, Log format). The terminal is forbidden: "`run` writes nothing to the terminal while the child runs" (arch Observability Hints).
  (.andromeda/obs-plan.md:115)
  > a file sink only, to `diagnostics/run-<name>.ndjson`; `run` writes nothing to the terminal while the child runs
- (.andromeda/obs-plan.md:311) `run`'s and `revive`'s start refusals are the catalogue's exit-1 causes: each must stay a pair of fixed stderr lines through this writer, before any log where stated, and a moved site must re-derive these counts.
  > - `run` writes nothing to the terminal except the child's own output, with one exception before any child is spawned: a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint, a home tree another user could write or read: the home pair for a home-level refusal, the instance pair for an instance-level one, before any log opens) writes exactly two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid) through `src/human.rs` `refuse` → `write_refusal`, one `write_all` on the locked stderr, a closed pipe swallowed — the root bin's human-stderr refusal writer, called by `run` (7 sites), `verify` (6), `list` (2), `revive` (1), `plugin install` (1) and `client` (1, `client::refuse_untrusted`, the one site `viola link` / `viola unlink` and the six frame verbs `send`, `wait`, `last`, `answer`, `pause` and `release` share), as re-derived by `grep -rcE 'human::refuse\(' src` at chunk 2026-10-26-home-and-code-bearing-file-integrity; `list`'s colour layer adds no stderr writer and its `unable` / `hint:` pair carries no SGR. `viola revive` (its start arm logs as process `run` and makes `run`'s home-tree check before that log) adds three refusals to `run`'s five: state files another user could write, no logged session (or an `--id` the log does not hold) and a missing recorded directory; on a live name it prints `run`'s own collision pair. `viola revive --list` writes the pair for its two refusals and nothing else on stderr. While the child runs, neither `run` nor a revived start writes anything of its own.
  (.andromeda/obs-plan.md:314)
  > - Short-lived CLI verbs may use stderr for their human output.
- (.andromeda/obs-plan.md:312) A cause's new log form must be a code in the role's home-level file, with any content kept to the instance's detail file.
  > - Every role's codes-only process log goes to its home-level role file under `diagnostics/`, and content-bearing detail goes only to the instance's `diagnostics/detail-<role>.ndjson`.
- (.andromeda/obs-plan.md:310) The hook's edge, which the scope holds outside the catalogue, must stay this contract: no stderr, and no code reaching its stdout.
  > - `hook` writes only its decision body to stdout and nothing to stderr. The one other thing a `hook` stdout carries is `viola hook statusline`'s: the user's status-line command's output byte for byte (nothing when no command is recorded, on a refused instance, on oversize or malformed stdin, or under the hidden `--capture` arm, which prints nothing). No telemetry reaches stdout on either arm.

## Bindings
- **Founder Direction 4 ↔ the log catalogue and schema**: each new logged code must join the closed `detail` catalogue (.andromeda/obs-plan.md:451) and `schemas/diag-line.v1.json`'s default-deny (.andromeda/obs-plan.md:576), as this trigger demands (.andromeda/obs-plan-detail/obs-scope-summary.md:165).
- **Frame-verb document ↔ the `wait` / `last` scenario**: both state the exact `detail: null` document, so a contract choice must move them together, (.andromeda/obs-plan-detail/obs-scope-summary.md:159) and (.andromeda/obs-plan.md:361).
- **`server-verify-failed` ↔ the scenario and error capture**: the CLI's line at WARN with no `channel-request` (.andromeda/obs-plan.md:361) and MCP's no-line outcome (.andromeda/obs-plan.md:537) must hold with this rule (.andromeda/obs-plan-detail/obs-scope-summary.md:123).
- **`internal-error` boundary ↔ error capture and the table**: the fixed `error: internal error` stderr with no hint (.andromeda/obs-plan.md:523) must be reconciled with the table's last-`hint:`-line rule, over the boundary (.andromeda/obs-plan-detail/obs-scope-summary.md:28).
- **Exit code as a log field ↔ the level mapping**: a logged `process-exit` must sit at ERROR for exit 1 (.andromeda/obs-plan.md:459) and WARN for exit 21 (.andromeda/obs-plan.md:460), with (.andromeda/obs-plan-detail/obs-scope-summary.md:164).
- **`verify`'s refused tree ↔ its edge flow**: no line before its log against the stamp write's own `process-exit` (.andromeda/obs-plan.md:400) must agree with (.andromeda/obs-plan-detail/obs-scope-summary.md:159).
- **New codes in `viola-core` ↔ the module-boundary rule**: a typed error crossing a seam must end logged once with a closed code (.andromeda/obs-plan.md:643), the codes supplied from (.andromeda/obs-plan-detail/obs-scope-summary.md:36).
