# obs · cascade read · group 2 of 5

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:233) `run`'s diagnostics file and subscriber must be in place before the start sequence, so the squatted-name collision refusal the chunk edits in `refuse_squatted` is logged, and no test capture may route a line to stdout.
  > 4. Open `diagnostics/<process file>` in append mode with 0600 permissions inside a 0700 directory.
  > 5. Install the subscriber with an explicit writer (never stdout).
  > 6. Emit `process-start`.
  >
  > For `run`, this completes before the start sequence, so the collision check and version gate are logged. For `mcp`, it completes before the rmcp stdio transport starts.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:243) The CARRY case_4 capture fix must make the wrapper's line visible through the pipeline itself: no stdout sink, one `write` per line, and no level filter read from `RUST_LOG` or any env var.
  > - **Logging stack:** `tracing` with tracing-subscriber 0.3.23, configured as `fmt().json().flatten_event(true).with_current_span(false)` (tests excerpt, verbatim). Version 0.3.23 satisfies the security Vector 9 floor of `>=0.3.20` (RUSTSEC-2025-0055). Sink rules:
  >   - one file per process role under `diagnostics/`;
  >   - no stdout sink on any role;
  >   - stderr JSON only for `mcp`, and only when `mcp.ndjson` cannot be opened (D-09).
  >
  > The writer must emit each formatted line as a single `write` on an append-mode handle (arch Crash-safe disk writes). Concurrent `hook` processes share one file.
  >
  > The level filter is not read from `RUST_LOG`. Arch says "environment variables are not a configuration channel", and security Anti-Patterns ban env, flag or config values that disable any control. Any verbosity control therefore comes from `--home/config.json` or a CLI flag and must never bypass redaction. There is no separate agent-mode flag: JSON-to-file is the only mode.
  (.andromeda/obs-plan.md:135)
  > the level filter is never read from `RUST_LOG`, and no verbosity control may bypass redaction; JSON-to-file is the only mode.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:264) The re-verified `process-exit` (squatted-name) and `send-refused` (budget-paused) lines must carry the required fields under the closed `event` enum; the chunk adds no `event` value without a Decisions Log entry and renames or removes no field.
  >   - `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums. Each value fixes what its `corr` holds:
  >     - `channel-request`, `channel-response`: `corr` = the JSON-RPC request `id`
  >     - `dialog-raised`, `dialog-answered`: `corr` = `dialog_id`
  >     - `hook-invoked`, `hook-decision`: `corr` = `dialog_id` for dialog hooks, otherwise null
  >     - `send-issued`, `send-confirmed`, `send-refused`: `corr` = the send `cursor`
  >     - `process-start`, `process-exit`, `panic`: `corr` = null
  >   - `process` (`run|hook|mcp|cli`) and `instance` (a `ViolaName` or null).
  >   - `corr` is copied unchanged as a JSON number or string. It is never renamed (for example to `correlation_id`).
  >
  >   obs-plan may add fields but must not rename or remove these. The harness greps on them.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:284) The squatted-name refusal the chunk re-verifies is read back from `run`'s role file, so the start tests' viola homes must hold that basename under `diagnostics/`.
  > - **Log file location:** Tests require `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:298) The start scenario's surfaces include the channel bind, which is the Windows listener whose admission the chunk asserts.
  > - **Surfaces involved:** cli (`run`) + ipc-internal (channel bind) + persistent stores (snapshot, heartbeat, `events.ndjson`)
  (.andromeda/obs-plan.md:121)
  > - **Surface:** ipc-internal (`viola-channel`: JSON-RPC 2.0 over ndjson on a named pipe or Unix socket, one endpoint per `viola run`)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:299) `run.collision_check` and `channel.bind` must stay traced over the squat refusal and the pipe bind the chunk asserts, and a start refused at the collision check must still write its `process-exit` line.
  > - **Must-trace spans:** `run.start` › `run.collision_check` › `run.pin_copy` › `run.version_gate` › `channel.bind` › `run.budget_eval` › `state.snapshot_write` › `state.heartbeat_start` › `pty.spawn`. Log events: `process-start` (run), `process-start` for the child, and `process-exit` on failure. The product event order to verify in `events.ndjson` is `wheel{cause:"start"}` → `budget-gate` → `session-start`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:300) The squatted-name exit-1 start line must keep its distinct `detail` code with no endpoint name, path or pid in it.
  > - **Required log fields:** `event`, `process="run"`, `instance`, `corr=null`, `cli_version`, `cli_verified`, `exit_code`. Exit-1 causes use a distinct `detail` code: already-live, squatted-name, sha-256 mismatch, `.cmd`/`.bat` child, and for a start made by `viola revive` also no-session, session-live and cwd-missing. A revive whose instance check refuses writes no line: the check precedes the log (chunk 2026-10-10-budget-governor). No path or pid appears in the detail.
  (.andromeda/obs-plan.md:161)
  > - **Required log fields:** exit-1 causes use a distinct `detail` code, with no path or pid in it.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:309) The CARRY case's `send-refused` line must carry its refusal and the self-reported `from`, and never the sent text.
  > - **Required log fields:** `event`, `process`, `instance`, `corr` (the JSON-RPC `id` on channel lines; the send `cursor` on send lines), `from` (self-reported, not an identity), `confirmed`, `refusal`, `detail` (`input-not-ready` / `no-prompt-submitted` / `control-character` / `empty-text` / `turn-running`), `duration_ms`. Sent text never appears.
  (.andromeda/obs-plan.md:165)
  > - **Required log fields:** sent text never appears.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:344) `send-refused` with `budget-paused` is the line the CARRY case_4 must capture under a plain `cargo test` (libtest threads), its red closed by a known cause or a fixed capture.
  > - **Must-trace spans:** `hook.statusline` › `state.budget_write` › `statusline.shell_out`, then `run.budget_eval`. Log events: `hook-invoked`, `hook-decision` for statusline, and `send-refused` with `budget-paused`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:345) The budget-paused `send-refused` line the CARRY case reads must carry the crossed window's figures and `refusal="budget-paused"`.
  > - **Required log fields:** `event`, `process`, `instance`, `corr`, `window` (`five-hour` / `seven-day`), `used_percentage`, `resets_at`, `paused`, `refusal="budget-paused"`, `exit_code` (11), `shell_exit_status`, `duration_ms`. The `statusline_command` string never appears outside `diagnostics/`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:362) The re-asserted `send-refused` line, and any capture the fix adds, must hold no sent text.
  >   - A never-log rule for `send` text on stderr, `--json`, MCP and every process-log field outside `instances/<name>/diagnostics/`.
  (.andromeda/obs-plan.md:195)
  > - **Required telemetry signal:** `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:412) Any instrumentation or test dependency the plan adds to `viola-channel` for the pipe-mode, first-instance or DACL assertions must stay Tokio-free in its normal graph, add no C-building crate and pass `cargo deny`.
  > - **Required telemetry signal:** The logger dependency is tracing-subscriber `>=0.3.20` (0.3.23 pinned by tests). Obs dependencies stay Tokio-free in the sync crates and add no C-building crates, and they pass `cargo deny` on all three targets.
  (.andromeda/obs-plan.md:59)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Tokio is forbidden in this crate's graph unless its feature is enabled.
  (.andromeda/obs-plan.md:225)
  > - **Required telemetry signal:** obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass `cargo deny`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:419) The CARRY case asserts a numeric `corr` on a refusal made before any cursor is given, so the plan must state what that `corr` joins on against this cursor join and the channel join key.
  >   - `corr` = send `cursor` joins `send-issued` / `send-confirmed` / `send-refused` across CLI, MCP and wrapper.
  (.andromeda/obs-plan.md:264)
  > the channel join key (D-25: `(conn, corr)`, each side logs its own `instance`)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:450) The Windows-only listener assertions (remote-client rejection, first-instance, DACL effect) must add no OS-specific exporter, field or event: a named-pipe line carries the same fields as a Unix-socket line.
  > - **Required telemetry signal:** The same file sink and line schema on every OS. Platform differences must still yield identical fields:
  >   - permission application: Windows DACL vs Unix 0600/0700;
  >   - append atomicity with concurrent hook writers: Windows append vs POSIX `O_APPEND`;
  >   - named pipe vs Unix socket in channel logs;
  >   - ConPTY vs openpty in `pty` spans.
  >
  >   No OS-specific exporter.
  (.andromeda/obs-plan.md:239)
  > - **Required telemetry signal:** the same file sink and line schema on every OS, with identical fields and no OS-specific exporter.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:462) A panic in the new Windows listener tests or in `run`'s start must leave exactly one single-line `event:"panic"` record, so G2 reads zero unlogged panics on the Windows leg.
  > - **Required telemetry signal:** A zero-unlogged-panics invariant for all four process roles. Each panic produces exactly one single-line `event:"panic"` record and no multi-line backtrace, and `hook` still exits 0. No formal latency or availability SLO beyond the tests perf gates.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:474) `send-refused` must stay a logged event of the verbatim tests list, which the CARRY case reads from the wrapper's structured lines.
  > - **Required telemetry signal:**
  >   - Structured JSON process logs from every process role (`run` / `hook` / `mcp` / `cli`) under `diagnostics/`, merged by harness `logs` with `events.ndjson`.
  >   - `send-issued` and `send-refused` are logged events.
  >   - The tests field and event list is adopted verbatim.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:485) The squatted-name start refusal keeps its own distinct exit-1 `detail` code with no path or pid, which the chunk re-verifies and does not change.
  >   - Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid.
  (.andromeda/obs-plan.md:253)
  > - **Required telemetry signal:** `release-from-driver` is its own logged event, and each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:502) The CARRY case's capture must see the refusal line deterministically: no sleep-based wait for the line may make case_4 green under libtest threads.
  >   - Events are emitted deterministically, keyed on hook events and `events.ndjson` offsets.
  >   - No telemetry is derived from screen content: vt100 is used for gate outcomes only.
  >   - No log-based waits on sleeps.

## Bindings
- **values/paths (start scenario) ↔ values/triggers (exit-1 causes) ↔ mirror/log-fields**: the squatted-name refusal's `process-exit` line is one line held by three rules at once: logged inside `run.collision_check` after log init (.andromeda/obs-plan-detail/obs-scope-summary.md:299, .andromeda/obs-plan-detail/obs-scope-summary.md:237), its distinct `detail` with no path or pid (.andromeda/obs-plan-detail/obs-scope-summary.md:300, .andromeda/obs-plan-detail/obs-scope-summary.md:485), and the required fields with `corr` = null (.andromeda/obs-plan-detail/obs-scope-summary.md:269), read back from `run-<name>.ndjson` (.andromeda/obs-plan-detail/obs-scope-summary.md:284).
- **values/logs (logging stack) ↔ values/triggers (determinism)**: the CARRY case_4 capture fix is bounded on both sides: no stdout sink and no `RUST_LOG` (.andromeda/obs-plan-detail/obs-scope-summary.md:250), and no sleep-based wait (.andromeda/obs-plan-detail/obs-scope-summary.md:504).
- **values/paths (budget governor) ↔ mirror/log-fields ↔ values/triggers (corr join)**: the budget-paused `send-refused` line's fields (.andromeda/obs-plan-detail/obs-scope-summary.md:345) sit under the `send-refused` corr = cursor rule (.andromeda/obs-plan-detail/obs-scope-summary.md:268) and the cursor join (.andromeda/obs-plan-detail/obs-scope-summary.md:419), while the refusal comes before any cursor is given; the plan states what its numeric `corr` carries, against the `(conn, corr)` channel join key (.andromeda/obs-plan.md:264).
- **values/triggers (multi-OS) ↔ values/paths (channel bind) ↔ values/triggers (supply chain)**: the Windows pipe assertions under `channel.bind` (.andromeda/obs-plan-detail/obs-scope-summary.md:299) add no OS-specific field (.andromeda/obs-plan-detail/obs-scope-summary.md:450) and no Tokio or C-building dependency in `viola-channel` (.andromeda/obs-plan-detail/obs-scope-summary.md:412).
