@task_prose part=chunk
Builds the fleet console's scaffold: its surface written into the masters where the 0.1.0 stub deferred an interactive terminal surface, and the minimal console process and frame behind it.
Fixes the quarter-screen layout and its smallest whole size with a named degrade rule, and reconciles the redraw-ban rows by an exception named for the console's own surface only, never generalised.
Adds a test-only keystroke driver that feeds the console keys through its own pseudo-terminal under a test-owned home and asserts on the console's state or its log lines, never screen bytes, over the fake agent and fixtures.
Gives the console process its own diagnostics role beside run, hook, mcp and ui, reachable through the logs command, with the obs event catalog's rows for it.
Folds two carries: re-reads RUSTSEC-2026-0543 (chrono, unmaintained) for a safe upgrade, and restructures the framing-turns per-paste probes into one step function and the relied-row count and stamped-set table into one source each.
@task_file n=1 path=deny.toml mark=edits
what: the cargo-deny advisory and licence policy the dependency gate reads — edits: the RUSTSEC-2026-0543 chrono advisory ignore entry and its reason line saying it is re-read at each boundary
@task_item n=1 part=needs
A new process role for the console: one diagnostics log file per process role, the role list and its count wherever a row pins it, the process field's closed values, and the role taken from argv before clap.
@task_item n=2 part=needs
The console role's diagnostics file paths and home resolution: owner-only file and directory modes, the home strict-modes check before diagnostics init, and whether its lines are home-level (path-free, content-free) or instance-scoped, given the instance source fixed per role.
@task_item n=3 part=needs
The harness logs command reaching the console's role file: its merged sources, filters, output record envelope and torn-line handling.
@task_item n=4 part=needs
The obs event catalog rows for the console process: its process-start and process-exit lines with service name and version from the compiled constants, pid and os, its closed exit detail codes, and corr by event.
@task_item n=5 part=needs
The interactive terminal interface absence (prompt, TUI or pager) and the redraw-ban reconciliation: the console becomes an interactive surface by a named exception for its own surface, the rule never widened for other CLI surfaces.
@task_item n=6 part=needs
Stdout and stderr by role for the console: the console's terminal carries its frame and no telemetry, no ANSI codes and no screen content in diagnostics lines, logs to its file sink only, and its error output keeps the fixed-message rule.
@task_item n=7 part=needs
Keys typed into the console are never logged and no telemetry is taken from screen content; the keystroke driver asserts on the console's structured state or its log lines, never on screen bytes.
@task_item n=8 part=needs
Panic capture in the console role: panic hook installed first and silent on stderr, one single-line panic record, exit decided at the console's catch site, panic = unwind kept, and a panic outcome by role entry for the console.
@task_item n=9 part=needs
Diagnostics init order for the console: panic hook, home tree check, then log open before the role's work; a refused or failed log open discards lines; level from config only, never from the environment.
@task_item n=10 part=needs
Any new dependency that draws or reads a terminal: obs dependencies Tokio-free and C-free, passing cargo deny, not on the banned telemetry and logging list, its log target off among third-party targets.
@task_item n=11 part=needs
The chrono advisory carry: RUSTSEC-2026-0543 re-read at the boundary, the ignore leaving with a safe upgrade or standing one more epoch with its reason line; a cargo deny failure is a failure, and chrono stays the direct timestamp dependency for millisecond UTC timestamps.
@task_item n=12 part=needs
The restructure's invariant: the framing-turns per-paste probes folded into one step function, the send-through guarded section only looked at, the relied-row count read from one source and the stamped-set table held once — what each does stays as it is, held by the existing send-path log-event, send-refused detail and confirm-window checks and the relied-row tests.
@task_item n=13 part=needs
Any console code that logs or declares spans goes through the obs_event macro with a closed event enum and the default-deny field allow-list, no raw tracing macros, instrument skipping all arguments, static low-cardinality span names, and no print macros in product paths.
@task_item n=14 part=needs
Console tests run under a test-owned home and their own pseudo-terminal, bounded under the 20 s lint, against the fake agent and fixtures only; no live start, no real claude, no real terminal or compositor, no read of the operator's real homes.
@task_item n=15 part=needs
The console scaffold makes no hot-path instrumentation of its redraw loop: no info logging in hot loops and no line per redraw or keystroke.
@task_item n=16 part=needs
The console's start verb or flag and its name (his fork): the CLI process-log file the verb writes, and its process-exit detail codes on refusal.
@task_standing n=1
convention: each new diagnostics line is written through the single obs_event macro with a closed event value, explicit corr and instance, and default-deny fields
home: 11-obs-anti-patterns-never-do-these
@task_standing n=2
convention: each new diagnostics line carries only the fixed line fields and is additive to the log schema, never renaming a frozen field
home: log-format-json-schema
@task_standing n=3
convention: each new integration or E2E test that runs a process has its emitted lines schema-checked and its home under the one integration-test root, kept until the gates run
home: 9-ci-integration · ci-integration
@task_standing n=4
convention: each new log line names no path, pid, typed content or secret in a home-level file; content-bearing records go to the instance detail file only
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_standing n=5
convention: each new obs code path is mutation-tested like product code, and the zero-panics and schema gates still read every role file
home: ci-integration
@task_standing n=6
convention: each new test waits on log lines or state by condition, never by sleep
home: 10-slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
The MCP stdio server's tools, frames, isError results and stdin MAX_FRAME rows: the scaffold adds no MCP tool and changes no MCP frame.
@task_item n=2 part=not_needed
Hook stdout, hook fail-open, hook latency and session-end and spine hook perf budgets: the console is no hook and touches no hook path.
@task_item n=3 part=not_needed
Channel JSON-RPC error codes, link and unlink methods, dialog raise and answer lines, and wheel pause and release lines: verbs and dialogs belong to Console actions, not the scaffold.
@task_item n=4 part=not_needed
viola run start span attributes, version gate, pinned copy, collision check and run-start failure detail codes, and revive and adopt preflight order: the scaffold starts no wrapped session.
@task_item n=5 part=not_needed
viola verify spawn subjects, verify stdout step and summary lines, and the real-CLI verify ban rows: the chunk adds no verify step.
@task_item n=6 part=not_needed
Heartbeat liveness, list liveness and stale threshold, and snapshot replay and state-recovered lines: the fleet rows and their state belong to Fleet console view.
@task_item n=7 part=not_needed
The future opt-in error reporter, OTel SDK, exporter, Sentry and network-egress rows: the console adds no reporter and no exporter.
@task_item n=8 part=not_needed
Metric derivation rows (counters, gauges, histograms, quantiles, grouping by closed enums): the scaffold adds no derived metric.
