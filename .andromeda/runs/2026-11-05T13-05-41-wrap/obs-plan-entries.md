@amendment marker=2026-11-05-linux-only-code-and-ci
title: the ConPTY sideload and the Windows kill fallback retired; openpty the one PTY backend
section: §1 Obs Scope Summary (body: the viola-pty and OS-API entities, the Vector 7 signal; obs-scope-summary.md: the viola-pty reason, the chaos trigger, the Vector 7 trigger) · §2 Telemetry Strategy (telemetry-strategy.md: the Vector 7 row) · §3 (contracts/logging-stack) · §4 Span / Trace Coverage (body and span-trace-coverage.md: Scenario `viola run` start) · §7 Error Capture & Reporting (error-capture-reporting.md: the forced feed panic witness)
change:
  - The `viola-pty` entity reads portable-pty `=0.8.1` alone (was plus the windows-sys `TerminateProcess` fallback); the OS-API entity reads openpty, Unix socket, filesystem locks and atomic rename, sysinfo (was with ConPTY, the named pipe and `TerminateProcess`); the seam's boundary-only OS API is openpty (was ConPTY and openpty).
  - The `viola run` span chain reads `run.pin_copy` › `run.version_gate` (was with `run.conpty_sideload` (Windows) between); the span attributes carry no `run.conpty_sideload` row; `pty.spawn`'s `pty_backend` reads `openpty` (was `conpty-sideload|conpty|openpty`); `process-start` carries no `sideload_fallback` (was five words, D-36); the never-list drops its sideload clauses.
  - A strict-modes failure has a line in two places, the `ledger-stamps` read and verify's refused stamp write (was three, with a refused `conpty/` companion as `sideload_fallback` `strict-modes-failed`); the Vector 7 row drops that mapping.
  - Handle-detected child exit and the handle-wait `process-exit` lose their ConPTY qualifiers; the logging stack's spawn failure reads `pty.spawn` (openpty) (was / ConPTY); the forced feed panic witness reads the child's `e4 b8 ad` reaching the terminal, no OS split (was a Windows byte reading owed to the route entry Windows-only live measurements).
why: the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6) make Viola Linux-only, the Windows code cut and recoverable at the tag `multi-os-final`; the chunk removed the sideload, `src/conpty.rs`, the vendored companions, the `TerminateProcess` fallback and `windows-sys`, and `pty_backend()` is always `openpty`.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the `.cmd`/`.bat` start refusal retired; the refusal writer's site counts
section: §1 Obs Scope Summary (body: the viola-agent-claude entity; obs-scope-summary.md: the run path's exit-1 causes, the Founder Direction 4 trigger) · §3 Observability Harness Contract (body: the `run` terminal rule) · §4 (span-trace-coverage.md: Scenario `viola run` start, `run.pin_copy` and the failure set) · §6 Log Coverage (log-coverage.md: the exit-1 list, verify's codes) · §7 (error-capture-reporting.md: start refusals under `--json`, the typed-error census) · §11 Obs Anti-Patterns (obs-anti-patterns.md: Logs)
change:
  - `batch-script-child` leaves every list of `run`'s, revive's and verify's exit-1 causes and `refused_with`'s `--json` set; `AgentError`'s named variants read `ProgramNotFound` alone (was with `BatchScriptChild`); `run.pin_copy` loses its `.cmd`/`.bat` sentence; the `run` terminal rule names no `.cmd`/`.bat` child and no example pair.
  - The `viola-agent-claude` entity reads child program resolution (was npm-shim resolution).
  - The `human::refuse(` sites read `run` 7, `verify` 5 (was 8, 6), re-derived by `grep -rcE 'human::refuse\(' src` at chunk 2026-11-05-linux-only-code-and-ci, in §3 and §11; revive adds three refusals to `run`'s five (was six).
why: the founder's Linux-only rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6) retired the `.cmd`/`.bat` refusal with `AgentError::BatchScriptChild` and the Windows resolution arms; the report re-measured the refusal sites at this wrap.
kept: the typed-error census still reads 9 derives: the report read 10 before the chunk and after, not this chunk's drift, so the family is routed to a CARRY on «Version done-check».
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the named-pipe endpoint and the macOS no-peer-pid carve-out retired; the server check on Linux
section: §1 (body: the ipc-internal surface; obs-scope-summary.md: the OS-API reason, the cli and hook notes, the dialog fields, the Vector 4 and Founder Direction 4 triggers, the multi-platform signal) · §2 (telemetry-strategy.md: the Vector 4 row) · §4 (body and span-trace-coverage.md: the cli, hook and MCP server checks, the euid check, the socket-directory span item, Scenarios send, wait/last, answer, pause/release) · §5 (metric-coverage.md) · §6 (log-coverage.md: `server-verify-failed`) · §7 (body: MCP rejects) · §10 (slo-invariants-telemetry-budgets.md: the spine-hook timed path)
change:
  - The ipc-internal surface reads ndjson on a Unix socket (was a named pipe or Unix socket); `channel.bind`'s `endpoint_kind` reads `unix-socket` (was `named-pipe|unix-socket`).
  - Each server-check statement drops its per-OS qualifiers and the macOS no-peer-pid carve-out (was `server-verify-failed` there an euid mismatch alone); the comparison reads the peer pid and its start time on Linux, the one target.
  - The multi-platform signal reads the peer pid by `SO_PEERCRED` and the euid by `peer_creds` (was per-OS reads behind `SERVER_PID_REPORTED` / `PEER_EUID_REPORTED`); `peer_admitted` refuses another euid and an unreadable one, `peer_euid_admitted(observed, own)` being `observed == Some(own)` (was where euids are reported, Windows reading none).
  - Founder Direction 4 has no carve-out outside the distinct-code rule (was the macOS one); the span recorder item no longer names the macOS-gated `refused_inside`.
why: the founder's Linux-only rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6) retired the named-pipe endpoint with its SDDL / SQOS controls and the macOS socket directory and carve-out; the chunk removed `server/win.rs`, folded both constants to their Linux value and dropped `server_verdict`'s macOS `Matches` arm.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: one CI leg, two workflows, one deny target and one harness shim
section: §1 (body: the snapshot / paste-to-AI line; obs-scope-summary.md: the Vector 9 trigger) · §2 (telemetry-strategy.md: the multi-platform row) · §3 (contracts/snapshot-paste-to-ai-integration, contracts/bootstrap-phases-derive-for-route-setup-project) · §8 (pii-scrubbing-compliance.md: unscanned uploads, the refusal witnesses) · §9 CI Integration (body: Platform, Gate commands; ci-integration.md: the Lint, census and Mutation rows, the cross-OS JUnit reader row)
change:
  - Platform reads `ci.yml` beside `nightly.yml`, matrix `[ubuntu-latest]` (was also the dispatch-only `windows-mutants.yml`, matrix `windows-2025` / `macos-latest` / `ubuntu-latest`); gates run on the `ubuntu-latest` leg (was all three OSes); lint-probes name no Linux leg.
  - The Mutation row drops the `windows-mutants.yml` leg and its `panic_frames` score; the `scripts/junit-os-sets.py` row of the three `junit-<os>` artifacts is retired; the multi-platform row reads §9 artifacts in `matrix.os` form on the one leg.
  - The OS-gated census also fails a gate whose predicate names `windows` or `target_os = "macos"` (`another OS: …`, a fault even when a row names it), its self-check firing on every planted class (was eight planted cases).
  - `cargo deny` runs on one target, `x86_64-unknown-linux-gnu` (was all three).
  - The `logs` path names `scripts/agent-run.sh` alone (was with the `.ps1` twin).
  - Unscanned uploads drop the Windows mutation scratch (`HOST_SCRATCH`, `scratch_bytes`); the three refusal witnesses read on Linux (was per-OS lists).
why: the founder's Linux-only rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6) retired the two non-Linux legs, `windows-mutants.yml` with its scratch, `scripts/junit-os-sets.py`, `scripts/agent-run.ps1` and two deny triples; the census gained its another-OS fault class in this chunk.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the Windows home owner/DACL strict-modes arm retired
section: §1 (body: the Vector 7 signal; obs-scope-summary.md: the OS-API reason, the hook note, log file location, the Vector 7 trigger, the multi-platform signal) · §2 (telemetry-strategy.md: the multi-platform row) · §3 (contracts/log-file-location, contracts/otel-sdk-init step 4) · §7 (error-capture-reporting.md: `check_diagnostics`, catch-site reach) · §8 (pii-scrubbing-compliance.md: the content sink) · §10 (body: failure conditions)
change:
  - `diagnostics/` files read 0600 in 0700 directories, the tests secret-scan test enforcing it (was with the Windows home DACL of user, SYSTEM, Administrators); §10's failure reads a file not 0600 (was or not DACL-protected on Windows).
  - `check_diagnostics` holds a folder and a file to the read rule, no group or other bit (was a Windows read grant to another SID and the write rule for a file); the strict-modes checks are viola's stat reads (was with Windows owner and DACL reads); `check_socket_dir` has no off-Unix arm.
  - Init step 4 creates the home at 0700 and checks owner, mode and not a symlink by `lstat` (was with the Windows protected DACL and owner/DACL checks).
  - Catch-site reach: a home under a regular file is a strict-modes refusal with no line (was a Windows `NotFound` arm reaching the catch-site `internal-error` line).
why: the founder's Linux-only rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6) retired the Windows home owner/DACL arm; the chunk removed `strict.rs`'s Windows arm, `windows_read_verdict`, the DACL arm of `create_private_dir` and `check_socket_dir`'s off-Unix arm.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the OS target, Linux only
section: §1 Obs Scope Summary (obs-scope-summary.md: the tier values, the triggers line, the multi-platform signal; body: the multi-platform source and signal) · §2 (telemetry-strategy.md: the multi-platform row) · §3 (contracts/logging-stack: append atomicity, the concurrent-append check)
change:
  - The tier values gain an OS target of Linux only: the core on any current distribution, CI on `ubuntu-latest`, the Windows and macOS code recoverable at the tag `multi-os-final` (`ecbe7ab3`); the triggers line reads Linux platform compat (was multi-OS compat), 18 triggers standing.
  - The multi-platform signal reads one file sink and line schema on Linux, each field fixed: Unix 0600/0700, POSIX `O_APPEND`, the Unix socket, openpty (was the same on every OS, with Windows and macOS alternatives); its source reads arch CI/CD (`ubuntu-latest`) (was the three legs and Multi-OS).
  - The logging stack drops the Windows `append(true)` / `FILE_APPEND_DATA` arm; the concurrent-append check runs on `ubuntu-latest` (was the three legs); the two chunk 2026-09-28-hook-perf-gate figures read then all 3 OS legs, now `ubuntu-latest` alone.
why: the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6): Viola is Linux-only, the Windows and macOS code cut, recoverable at `multi-os-final`; a dated three-OS measurement stays as history, never as current.
kept: the multi-platform-exporter-compat trigger and its row stand, stated for Linux.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the Windows host arms of the panic capture, the env strip, verify and the statusline retired
section: §4 Span / Trace Coverage (span-trace-coverage.md: Scenario Budget governor; Scenario `viola run` start, the settings override write) · §6 Log Coverage (log-coverage.md: `interrupted`) · §7 Error Capture & Reporting (error-capture-reporting.md: the panic backtrace)
change:
  - The panic backtrace reads `libc::backtrace` + `dladdr`, one path (was with the Windows `RtlCaptureStackBackTrace` arm); the retired symbolising cost reads as measured on that chunk's Windows runner.
  - The strip matches `CLAUDE*` names exactly, the persistent set `config.json` `claude_env_keep` (was case-insensitive on Windows with the `HKCU`/`HKLM` set and the registry reader).
  - A verify stop reads `SIGINT` / `SIGTERM` (was with Windows Ctrl-C / Ctrl-Break), in §4 and §6; the relied count reads all 24 rows of `ALL` (was 24 on Unix, 21 on Windows through `statusline::WRAPPED_HERE`); the probe `--settings`, the statusline capture arm and the settings override write lose their Unix-only qualifiers; SIGKILL and a crash leave the probe dir (was also a Windows close, logoff or shutdown).
  - The statusline-shell pair needs only a recorded `statusline_command` (was also a built shell, never on Windows).
why: the founder's Linux-only rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6); the chunk removed the OS arms of `src/panic_frames.rs`, the registry reader, `CASE_INSENSITIVE` and the console-control stop arm, folded `WRAPPED_HERE` to true and made `shell_argv` return `[&str; 3]`.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the closed words that left `diag-line.v1.json` with their emitter
section: §3 (contracts/service-identity) · §4 (span-trace-coverage.md: Scenario `viola run` start, the schema item) · §6 Log Coverage (log-coverage.md: the `process-exit` `detail` enum, the closed-word rule)
change:
  - `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 19 values, as measured at chunk 2026-11-05-linux-only-code-and-ci (was 20, at chunk 2026-11-01-unix-endpoint-and-home-hardening).
  - `os` reads `linux`, the schema's one word (was `windows|macos|linux`); the schema item reads `pty_backend` a string and `os` closed to `linux` (was `sideload_fallback` closed to five words).
  - §6 gains the rule: a closed word of `schemas/diag-line.v1.json` that no build can emit leaves the schema with its emitter.
why: the Viola overseer's technical answer of 2026-11-05 11:08Z (inputs#I2), narrowing the scope's boundary on closed words: `batch-script-child`, `sideload_fallback` and `os`'s `windows` and `macos` left with their code; the enum's own count reads 19.
kept: the log-coverage `process-start` row still names `sideload_fallback` and the conpty `pty_backend` words: its edit drops cell pipes the store refuses, so it stands, owned by a CARRY on «Version done-check» with its target text (`pty_backend` `openpty` alone, `sideload_fallback` gone), by the overseer's technical answer, founder-delegated, 2026-11-06 15:20Z.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
