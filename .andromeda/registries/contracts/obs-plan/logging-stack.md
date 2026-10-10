### Logging stack

- **Library:** tracing 0.1.44 (facade; Tokio-free, no C) + tracing-subscriber 0.3.23 (`fmt` JSON formatter; meets security Vector 9's `>=0.3.20`).
  - Every line is emitted through **one crate-local macro `obs_event!`**. It is defined in viola-core as a `macro_rules!` that expands to `::tracing::event!`, so viola-core itself stays I/O-free and tracing-free.
  - The macro always attaches `event` (the `ObsEvent` enum's kebab `Display`), plus `process` and `instance` from a process-global `OnceLock<ProcessCtx>`. Each is key absence when it has no value.
  - `corr` is an ordinary typed `key = value` field that the caller passes on the events that carry one. It is never `?`/`%`, and it is absent when null. The macro has no dedicated `corr` arm, because in `macro_rules` such an arm is ambiguous with the generic `key = value` arm. The schema confines `corr` to the corr-bearing events and types it `number|string`. As measured at chunk 2026-09-24-diagnostics-plane (`crates/viola-core/src/obs.rs`, `tests/contract_diag_schema.rs`).
  - `message` is a static literal equal to the event name, so it never carries interpolated content.
- **Format:** structured JSON-per-line. `on_event` formats into a thread-local buffer and makes one `write_all` per line on an append-mode handle, which meets the arch "one `write` per line" rule. The line is codes-only and stays well under 4 KiB. Every role file can have concurrent writers in separate processes, and the same rule covers all of them:
  - `hook-<name>.ndjson`, shared by hook processes;
  - `mcp.ndjson`, shared by one `viola mcp` per Claude session;
  - `cli-<name>.ndjson`, shared by concurrent short-lived verbs;
  - `run-<name>.ndjson` and `ui-<port>.ndjson`, briefly: a second `viola run <name>` or a `viola revive <name>` on a live or `stale` name, or `viola ui` on a live port, opens the same file at init step 4. It appends its `process-start` and its exit-1 `process-exit` (for example `detail:"already-live"`, Scenario 1) next to the holder's lines. A `gone` name is taken over by `run` instead, with no exit-1 line; a `viola revive` on a `gone` name appends its own lines to the same file and either starts the child or ends with an exit-1 `process-exit` (`strict-modes-failed`, `no-session`, `cwd-missing`). The concurrent-append check covers this collision path too (D-29);
  - the instance detail files `detail-hook.ndjson`, `detail-cli.ndjson` and `detail-mcp.ndjson`, shared by those same processes. Their lines (chains, drift reports, one-line backtraces) are not codes-only and can exceed 4 KiB, so the size argument below does not apply to them. They rely only on the single `write_all` and the torn-line tolerance, and the concurrent-append check must include a detail line larger than 4 KiB (D-28). As landed for the hook files: the check (8 concurrent `viola hook` processes: 16 whole lines in `hook-<name>.ndjson`, 8 in `detail-hook.ndjson`, all 3 CI OSes) carries no line over 4 KiB, and its over-4 KiB half is `hook_panics_append_whole_lines_over_4_kib_side_by_side`: 8 concurrent forced panics through the `fake-agent` seam `FAKE_AGENT_HOOK_PANIC` leave 8 whole role lines and 8 whole `detail-hook.ndjson` lines each over 4 096 B, all 3 CI OSes (chunk 2026-09-28-hook-perf-gate). The panic payload + backtrace is the only hook detail line that large.

  Each line is one `write_all` of one complete line on an append-mode handle. POSIX `O_APPEND` fixes the offset per write; on Windows, `append(true)` opens with `FILE_APPEND_DATA` only. Non-interleaving at this size is an assumption, not a POSIX guarantee, because the 4 KiB `PIPE_BUF` rule applies to pipes. Two things back it:
  - readers tolerate a torn line: the harness emits `{"torn":true}`, and G2 skips it through `fromjson?`;
  - a tests-owned concurrent-append check on windows-2025, macos-latest and ubuntu-latest asserts that every line of each shared file parses. This is a multi-platform-exporter-compat item. A torn line fails that check.
- **Mandatory builder settings beyond the tests line** (obs-research, tracing-subscriber, findings 2–4). Each is additive and renames or removes no required field:
  - `.log_internal_errors(false)`: the default `true` falls back to `eprintln`, which would break the Hook contract and the `run` terminal rule;
  - `.with_span_list(false)`: closes the `spans:[…]` leak path;
  - `.with_writer(...)` always explicit: the default writer is stdout;
  - `.with_ansi(false)`;
  - `with_file(false)` / `with_line_number(false)` left at their defaults, so no source paths.
- **Sink:** **file-only, one file per role.** There is no stdout sink on any role, and no pretty or dev sink. The files are:
  - `run`: `<home>/diagnostics/run-<name>.ndjson`;
  - `hook`: `<home>/diagnostics/hook-<name>.ndjson` (shared by concurrent hook processes; a write failure is swallowed);
  - `mcp`: `<home>/diagnostics/mcp.ndjson`, with stderr JSON only when the file cannot be opened (D-09);
  - `ui`: `<home>/diagnostics/ui-<port>.ndjson`;
  - `cli`: `<home>/diagnostics/cli-<name>.ndjson`, only when an instance resolves (D-06).

  Content-bearing records (full anyhow chains, serde_path_to_error drift reports, panic payload + backtrace) go **only** to `<home>/instances/<name>/diagnostics/detail-<process>.ndjson` (0600/0700), and only when an instance resolves (D-08).
  - They never pass through the tracing subscriber, whose one writer holds only the home-level role file.
  - The root-bin `viola::obs` detail writer formats each detail line itself, as the panic hook does: same `timestamp` (`MillisUtc` format), `level`, `target`, `message`, `event`, `process`, `instance`, and `corr` / `conn` keys. It writes each line with one `write_all` on the lazily opened detail handle.
  - No `obs_event!` call ever carries `chain`, `drift_report`, `panic_payload` or `backtrace`, and G4's `diag-line.v1.json` rejects a home-level line that does (D-30).
- **Agent-mode flag:** **none.** JSON-to-file is the only mode (obs-scope §3).
  - The level comes from `config.json` key `diagnostics_level` (`"info"` default | `"debug"`), applied through a `tracing_subscriber::filter::Targets` layer built in code.
  - It is never read from `RUST_LOG`, and `EnvFilter` / the `env-filter` feature is never enabled.
  - Level changes volume only. Every level uses the same field allow-list, so no setting can disable redaction (security Anti-Patterns).
  - Third-party targets (`rmcp`, `axum`, `tower_http`, `hyper`, `portable_pty`, `notify`, `sysinfo`, `interprocess`) are `OFF` in every sink, because their records lack the required fields. Their failures are captured at viola's seam from the returned `Result`s (D-11).
  - With `tracing-log` off and no hand-installed `LogTracer`, `log` records from portable-pty 0.8.1 and notify 8.2.0 are dropped; the failure classes they flag are captured from returned values or handles instead: a `pty.spawn` / ConPTY spawn failure is an `Err` from the seam and becomes `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` with the chain in `detail-run.ndjson`; a child exit without EOF is detected by the handle-wait thread (`exit_source`), independent of `log`; a notify watcher error reaches the handler as `Err(notify::Error)` and surfaces through `state.tail_notify` as `sse-closed{close_cause:"tail-error"}`. The accepted loss is only third-party `log` context emitted without a returned error. The chaos cases "child exit without EOF" and "notify watcher error" must each find a viola line; one that does not is a seam gap, answered by a seam fix or the researched fallback json-subscriber 0.3.0, never by re-enabling `tracing-log` or installing `LogTracer` (D-11, D-23).
