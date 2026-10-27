# obs · cascade read · group 1 of 5

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:28) The `run` start's `pin_bytes` call and `verify`'s `pin_exe` call the chunk edits sit in the root bin, so any io error the absent-copy write returns must reach stderr only as a fixed message, its full chain written to `diagnostics/` alone.
  > - **Reason if not fully instrumentable:** N/A. This is the only crate that uses anyhow, so it is the error-capture boundary. Arch Stack says "context chains only at dispatch". Security Vector 6 says a chain that contains a serde source must become a fixed message before it reaches stderr. The full chain goes to `diagnostics/` only.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:27)
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan.md:47)
  > a chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to `diagnostics/` only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:60) Whatever instrumentation the race fix adds in `crates/viola-state/src/pin.rs` or `fs.rs` must be sync, std-thread only, with no Tokio entering `viola-state`'s graph.
  > - **Reason if not fully instrumentable:** N/A. Stack bans Tokio here, so instrumentation must be sync.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:59)
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan.md:63)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Stack bans Tokio here.
  (.andromeda/obs-plan.md:225)
  > - **Required telemetry signal:** obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass `cargo deny`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:86) The pin step runs before the child's spawn in `run`'s start, and the two-sided witness drives the wrapped child: its failure must be read at viola's boundary (hook events, `events.ndjson`), never from the child's internal state or transcripts.
  > - **Reason if not fully instrumentable:** It is a third-party binary run unmodified. Instrument only the boundary:
  >   - spawn and exit (`process-start` / `process-exit`), with exit detected on the process handle, never on EOF (Stack; brief §4.1);
  (.andromeda/obs-plan-detail/obs-scope-summary.md:91)
  > Its internal state and transcripts are never read as a contract (creator Obs Anti-Patterns, brief §4).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:85)
  > - **Instrumentability:** boundary-only
  (.andromeda/obs-plan.md:75)
  > - **Instrumentability:** boundary-only — a third-party binary run unmodified: its spawn and exit (exit detected on the process handle, never on EOF), paste writes and the vt100 readiness-gate outcome; its internal state and transcripts are never read as a contract.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:99) The race breaks Claude Code's spawn of the pinned `viola hook`: that failure must be observed only through the hook boundary's own telemetry and the missing `session-start`, the hook's fail-open shape unchanged.
  > - **Reason if not fully instrumentable:** The caller is Claude Code, so viola sees only the inbound stdin payload and its own response. Instrument `hook-invoked` / `hook-decision` and MCP tool-call handling. No trace context arrives from Claude Code.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:98)
  > - **Instrumentability:** boundary-only
  (.andromeda/obs-plan.md:79)
  > - **Instrumentability:** boundary-only — viola sees only the inbound stdin payload and its own response; no trace context arrives from Claude Code.
  (.andromeda/obs-plan.md:119)
  > a failed log write is swallowed and the hook still exits 0; without `VIOLA_NAME`, on an instance the instance check refuses, or on a `VIOLA_DIR` naming an instance directory that does not exist, the hook is a silent no-op (exit 0, no file opened, no line, no frame)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:123) The absent-copy write's temp-file, lock and rename are OS-API call sites the chunk edits: any telemetry goes at viola's call site, never the OS internals, and the pinned-exe hash-mismatch refusal telemetry a concurrent start's re-hash reaches keeps its shape.
  > - **Reason if not fully instrumentable:** Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals. The home's strict-modes checks (`check_instance`, `check_home_tree`, `check_diagnostics`: viola's stat and, on Windows, owner and DACL reads) are such call sites; which of their refusals write a line is the strict-modes refusal-telemetry trigger's (below).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:122)
  > - **Instrumentability:** boundary-only
  (.andromeda/obs-plan.md:89)
  > - **Entity:** OS APIs: ConPTY / openpty, named pipe / Unix socket, filesystem locks and atomic rename, sysinfo, `TerminateProcess`
  (.andromeda/obs-plan.md:215)
  > plus refusal telemetry for pinned-exe hash mismatches and ignored symlinks; a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate and on a `conpty/` companion its owner/DACL check refuses (`process-start`'s `sideload_fallback:"strict-modes-failed"`, the child on the inbox ConPTY, exit unchanged); a refused `diagnostics/` folder or file is a failed open, its line dropped.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:132) The pinned copy is recorded as `snapshot.json`'s `pinned_bin`: its absolute path must stay out of every external output and process line the fix adds, and the witness reads `events.ndjson`'s `session-start` without copying its payload into a process-log line.
  > - `events.ndjson` is itself the product audit stream (brief §7, "the event log as the audit trail"). Its payload content is never copied into process-log lines.
  >   - Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:130)
  > - **Instrumentability:** instrumentable (through the `viola-state` call sites)
  (.andromeda/obs-plan.md:95)
  > - **Instrumentability:** instrumentable through the `viola-state` call sites; `events.ndjson` payload content is never copied into process-log lines, and the absolute paths in `snapshot.json` and config values are scrubbed from external output.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:159) The chunk edits `verify`'s `pin_exe` call: its pin must stay behind the home-tree check, logging as `cli` only once that check has passed, and a refused tree or `update_stamps` must still open no log and write no line.
  > `viola verify` logs as `cli` (with an instance) once its home-tree check (`check_home_tree` on the home, `ledger/`, `bin/` and `plugin/`) has passed; a refused check exits 1 with the home pair, opens no log and writes no line, and a refused `update_stamps` (`StampsError::StrictModes`) exits 1 with the same pair.
  (.andromeda/obs-plan.md:111)
  > a file sink only, JSON-per-line to `diagnostics/` when an instance resolves
- (.andromeda/obs-plan-detail/obs-scope-summary.md:160) The pinned `bin/<key>/` is keyed by version and hash, and the chunk re-measures two clean release builds of one tree: `service.version` must stay the compile-time `CARGO_PKG_VERSION`, and no build change the founder may rule on the measurement may inject CI or env metadata into it.
  > `service.version` is compile-time `env!("CARGO_PKG_VERSION")`, the same value already used as `sender` and `writer`.
  (.andromeda/obs-plan.md:133)
  > `service.version` is compile-time `env!("CARGO_PKG_VERSION")`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:162) A new exit-1 cause the absent-copy write may add (a lost race that cannot be settled) must get its own `detail` code with no path or pid, and its refusal line must be a fixed string, while a strict-modes refusal before the log still writes no line.
  > - The instance comes from `VIOLA_NAME` or an argument, never from the working directory (arch Instance identity rule).
  >   - Refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text (design excerpt, CLI refusals / CLI faults).
  >   - Exit codes 0, 1, 2, 10–14, 20 and 21 are the machine-readable outcome and should be a log field (arch Standard Contracts, CLI `--json` + exit codes).
  >   - Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code, except a strict-modes check refused before any log opens, which writes no line: its cause is carried only by the fixed stderr pair or the `--json` document (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, exit 21; `run`, `viola revive` and `viola verify`, exit 1).
  (.andromeda/obs-plan.md:161)
  > - **Required log fields:** exit-1 causes use a distinct `detail` code, with no path or pid in it.
  (.andromeda/obs-plan.md:253)
  > each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid, except a strict-modes check refused before any log opens, which writes no line (its cause is the fixed stderr pair or the `--json` document).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:171) The pin write is a `viola-state` write inside `run`'s start sequence, so whatever span or line it gains must ride the sync tracing JSON logger.
  > - **Backend instrumentation hooks:** `tracing` spans (`#[tracing::instrument]`) on the start sequence, the `viola-pty` seam operations, the `viola-channel` sync server handlers, vt100 readiness-gate outcomes and `viola-state` writes, all written through the tracing-subscriber JSON logger. Instrumentation must be sync: std threads, no Tokio (arch Stack, Hybrid concurrency; `cargo deny` ban).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:173) Any line the fix adds in `run`'s start must go only to `run-<name>.ndjson`, the terminal holding nothing of `run`'s own but the pre-spawn refusal pair.
  > - **Exporter:** A file sink only, to `diagnostics/run-<name>.ndjson` (tests excerpt, Log format). The terminal is forbidden: "`run` writes nothing to the terminal while the child runs" (arch Observability Hints).
  (.andromeda/obs-plan.md:115)
  > a file sink only, to `diagnostics/run-<name>.ndjson`; `run` writes nothing to the terminal while the child runs
- (.andromeda/obs-plan.md:311) The tampered-pinned-copy refusal sits at the `run` and `verify` pin call sites the chunk edits: it must stay the fixed two-line pair through `human::refuse`, with no path or pid, and the per-verb site counts (`run` 7, `verify` 6) must be re-derived if the fix adds or removes a refusal site.
  > a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint, a home tree another user could write or read: the home pair for a home-level refusal, the instance pair for an instance-level one, before any log opens) writes exactly two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid) through `src/human.rs` `refuse` → `write_refusal`, one `write_all` on the locked stderr, a closed pipe swallowed — the root bin's human-stderr refusal writer, called by `run` (7 sites), `verify` (6), `list` (2), `revive` (1), `plugin install` (1) and `client` (1, `client::refuse_untrusted`, the one site `viola link` / `viola unlink` and the six frame verbs `send`, `wait`, `last`, `answer`, `pause` and `release` share), as re-derived by `grep -rcE 'human::refuse\(' src` at chunk 2026-10-26-home-and-code-bearing-file-integrity;
- (.andromeda/obs-plan.md:312) A typed error from the pin write that the fix logs must keep the home-level role file codes-only, its content-bearing chain going only to the instance's detail file.
  > - Every role's codes-only process log goes to its home-level role file under `diagnostics/`, and content-bearing detail goes only to the instance's `diagnostics/detail-<role>.ndjson`.

## Bindings
- **obs-scope-summary (root bin error-capture boundary) ↔ error-capture-reporting**: a pin-write error crossing to `run` / `verify` dispatch keeps a fixed message on stderr (.andromeda/obs-plan-detail/obs-scope-summary.md:28), and its full chain lands as `chain:[...]` only in the instance detail file (.andromeda/obs-plan.md:511), with `StateError`'s `Display` fixed (.andromeda/obs-plan.md:509).
- **obs-scope-summary (exit-1 `detail` codes) ↔ log-coverage**: a new exit-1 cause on the absent-copy write takes a distinct `detail` code (.andromeda/obs-plan-detail/obs-scope-summary.md:162), which the `detail` catalog and the Obs Decisions Log record (.andromeda/obs-plan.md:767).
- **obs-scope-summary (`viola-state` sync instrumentation) ↔ ci-integration**: the sync, Tokio-free rule over `viola-state` (.andromeda/obs-plan-detail/obs-scope-summary.md:60) is enforced by the per-sync-crate deny ban (.andromeda/obs-plan.md:225).
- **obs-scope-summary (service identity) ↔ telemetry-strategy**: `service.version` as the compile-time value (.andromeda/obs-plan-detail/obs-scope-summary.md:160) is the same `viola_core::VERSION` emitted on `process-start` (.andromeda/obs-plan.md:297), and it keys the pinned copy the build measurement compares.
- **obs-scope-summary (OS-API call sites, Vector 7) ↔ the harness contract**: the pinned-exe mismatch telemetry (.andromeda/obs-plan.md:215) and the tampered-copy refusal pair with its site counts (.andromeda/obs-plan.md:311) hold together at the pin call sites the chunk edits.
