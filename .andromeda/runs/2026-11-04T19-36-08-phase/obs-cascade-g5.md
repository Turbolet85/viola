@carried n=1 label=obs-plan/telemetry-strategy fact=fct-4joil3ca
why: The openpty backend must sit behind the viola-pty seam whose spawn/write/wait/kill keep their code-declared, unserialized spans on Unix as on Windows, read by G1 on both Unix OSes, with no per-chunk span in the pump.
quote: | Spans / Traces | The 6 must-trace paths (Section 4) and every crate seam: `viola-pty` spawn/write/wait/kill, `viola-channel` client call and server dispatch, `viola-state` snapshot/heartbeat/tail, `viola-agent-claude` hook parse, `viola-mcp` tool calls | Declared in code with tracing 0.1.44 `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`. They are **not serialized** in v1, because the output sets `with_current_span(false)` + `with_span_list(false)`.
@carried n=2 label=obs-plan/telemetry-strategy fact=fct-4ns7wzxz
why: The one diagnostics schema the chunk re-asserts requires the same per-role JSON-per-line files under diagnostics/ and the closed event enum on every OS, and a peer the Unix wrapper drops on its accept thread must write no line.
quote: | Structured Logs | Every boundary call, state transition, refusal, fault/heal and process lifecycle (the `mcp` role writes no self `process-exit`, Section 7; a peer the wrapper drops on the accept thread has no line, no frame existing to key one on, and MCP's `frame` has none on a refused socket directory or server euid, `isError` its record), as the closed `event` enum (Section 3) | **The primary signal.** tracing-subscriber 0.3.23 JSON-per-line, one file per process role under `<home>/diagnostics/`, merged with `events.ndjson` by `agent-run logs` |
@carried n=2 run=2 label=obs-plan fact=fct-i24bcxka
quote: Structured logs are the primary signal, one JSON-per-line file per process role; spans are declared in code but not serialized in v1, and cross-process joins use `corr` + `conn`;
@carried n=3 label=obs-plan/telemetry-strategy fact=fct-6c7xwzms
why: The Unix wrapper's PTY pump and handle-wait threads must each run inside catch_unwind behind the hand-written panic hook, so G2 reads green on both Unix OSes.
quote: | Errors | Panics (a one-line `event:"panic"` record), typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit` (the `mcp` role writes no self `process-exit`, Section 7) | No external reporter: security says "no outbound network calls". A hand-written `std::panic::set_hook`, plus `std::panic::catch_unwind` at the main-thread catch site of every role, around `hook` dispatch, in the `run` PTY pump and handle-wait threads, and around the vt100 feed (Section 7) |
@carried n=4 label=obs-plan/telemetry-strategy fact=fct-6lxvbxmt
why: The per-OS perf job the chunk re-reads green on ubuntu-latest and macos-latest must gate on hyperfine --export-json, with no in-process instrument added.
quote: No in-process instruments and no exporter. Metrics are derived by `jq` over the ndjson; the perf gates run on hyperfine 1.20.0 `--export-json` (Section 5) |
@carried n=4 run=2 label=obs-plan fact=fct-irnyaxwz
quote: Perf gates produce hyperfine `--export-json` output and CI produces nextest JUnit XML.
@carried n=5 label=obs-plan/telemetry-strategy fact=fct-6ukjckx6
why: G2 re-read on both Unix OSes counts this one-line panic record, and the Unix-reached refusals (a refused home tree on run, revive's start or verify) must stay inside the listed exemptions.
quote: Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance. The bounded exemptions (the pre-init window, `cli` without an instance, `hook` without `VIOLA_NAME`, a failed step-4 role-file open (a `diagnostics/` folder or file its strict-modes check refuses included), the no-obs-init hidden capture arms (`hook <event> --capture` and `hook statusline --capture`), and a refused instance check on either `viola revive` arm (start or `--list`) or on `viola hook statusline`, or a refused `viola list` scan, which opens none either, or a refused instance check on `viola link` / `viola unlink`, made before their role file opens, or a refused instance check on the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`), a refused or absent instance on `viola hook`'s event and dialog paths (a `VIOLA_DIR` naming no existing instance directory included), or a refused home tree on `viola run`, `viola revive`'s start or `viola verify`, each made before its role file opens) are listed in Section 10. The payload (one escaped JSON string) and the backtrace (a one-line JSON array of frames) are written only to the instance-scoped detail file (Section 7).
@carried n=5 run=2 label=obs-plan fact=fct-z3yuahd4
quote: Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance, except the bounded exemptions listed in Section 10; the payload and the backtrace are written only to the instance-scoped detail file (Section 7).
@carried n=6 label=obs-plan/telemetry-strategy fact=fct-pl6vsrqd
why: The channel on a Unix socket must keep the channel field allow-list and surface table coverage of the IPC vector under the one schema the chunk re-asserts.
quote: | logging-sensitive Vector 1 (IPC) | §4 surface table (channel), §6 channel field allow-list, §8 table |
@carried n=7 label=obs-plan/telemetry-strategy fact=fct-xumv4vtr
why: The openpty write is the paste path on Unix, so the paste-text vector's coverage (no text, send-refused side) must hold there as on ConPTY.
quote: | logging-sensitive Vector 2 (paste text) | §4 Scenario 2 (`send-refused` `side`), §8 |
@carried n=8 label=obs-plan/telemetry-strategy fact=fct-n6l5o3fm
why: The Unix fake-agent suite reaches the hook's socket-dir-refused and server-verify-failed lines, and on macOS the word must mean an euid mismatch alone, the carve-out the scope keeps.
quote: A refused socket directory is no such silent path: the hook's `connect` writes one `hook-decision{detail:"socket-dir-refused"}` at WARN, with exit 0, empty streams and no frame. The "server-verify mismatch" item writes one `hook-decision{detail:"server-verify-failed"}` at WARN on Linux, Windows and macOS (macOS compares no peer pid, the recorded gap, so there the word means an euid mismatch alone) |
@carried n=9 label=obs-plan/telemetry-strategy fact=fct-kgjd4mty
why: The openpty spawn on Unix must apply the R8 env strip and log its env_stripped_* names-only fields exactly as the ConPTY spawn does.
quote: | logging-sensitive Vector 6 (CLI/env) | §4 Scenario 1 (`env_stripped_*`), §7 fixed messages, §8 |
@carried n=10 label=obs-plan/telemetry-strategy fact=fct-ne5xp5cg
why: The Unix suite reaches the socket-directory refusal (one process-exit at run and revive's start) and the 0600/0700 diagnostics modes the secret scan reads on every OS, while the conpty companion leg stays Windows-only and unmoved.
quote: | logging-sensitive Vector 7 (filesystem) | §3 Log file location (0600/0700), §6 `state-recovered` / `skipped`, §6 detail catalog Filesystem refusals (`strict-modes-failed`, `pinned-hash-mismatch`, `symlink-ignored` via `parse-rejected{parser:"state-entry"}`), §9 secret-scan-before-upload, §4 Scenario 1 / §6 `process-start.sideload_fallback` `strict-modes-failed` (a `conpty/` companion its owner/DACL check refuses: the child on the inbox ConPTY, exit unchanged), §10 exemptions (a strict-modes check made before a read or open — the six frame verbs, `viola hook`'s event and dialog paths, `viola mcp`'s frame tools, `verify`, `run` / `revive`'s start, the `diagnostics/` opens — writes no line; the socket-directory check shares the read verdict yet is no such exemption, §6 `socket-dir-refused`: one `process-exit` at `run` / `revive`'s start and at the eight CLI frame verbs, one `hook-decision` at `viola hook`, and no line only at MCP's `frame`) |
@carried n=10 run=2 label=obs-plan fact=fct-26ufyipy
quote: a socket directory `socket_dir_ready` refuses (Unix, exit 1, before any snapshot, child or `Server::bind`), a home tree another user could write or read: the home pair for a home-level refusal, the instance pair for an instance-level one, before any log opens)
@carried n=11 label=obs-plan/telemetry-strategy fact=fct-wkatnlg5
why: The openpty spawn must give the child's process-start with the closed subject values on Unix, the same as ConPTY.
quote: | logging-sensitive Vector 8 (child spawn) | §4 Scenario 1 (`subject` values), §6 `parse-rejected` |
@carried n=12 label=obs-plan/telemetry-strategy fact=fct-pso667is
why: The lint job's cargo-deny gates re-read green on both Unix OSes must hold over the openpty backend's dependency graph (no Tokio in sync crates, no C-building crates).
quote: | logging-sensitive Vector 9 (supply chain) | §3 Logging stack (feature set), §9 cargo-deny gates, §11 |
@carried n=13 label=obs-plan/telemetry-strategy fact=fct-7hbkkb6o
why: channel-request and channel-response must carry corr and conn on both sides of the Unix-socket channel, the join key the one schema keeps.
quote: | cross-surface-trace-propagation | §3 Trace context propagation (`corr` + `conn`), §4 Scenarios 2/4 |
@carried n=13 run=2 label=obs-plan fact=fct-pcsvls55
quote: Every cross-surface call carries correlation through the tests `corr` field plus the additive `conn` discriminator (Section 3, Trace context propagation).
@carried n=14 label=obs-plan/telemetry-strategy fact=fct-olppdytk
why: The per-OS perf job re-read green on ubuntu-latest and macos-latest measures against the section 10 budget table.
quote: | perf-budget-instruments | §5, §10 budget table |
@carried n=15 label=obs-plan/telemetry-strategy fact=fct-4vc4xc3n
why: state-recovered detail words and process-exit exit_source (kill-fallback) and during must read the same on every OS under the re-asserted schema.
quote: | chaos-instrumentation | §6 `state-recovered` / `process-exit` `exit_source` + `during`, §10 |
@carried n=16 label=obs-plan/telemetry-strategy fact=fct-6isebxud
why: This trigger is the chunk's own re-assertion: the same file sink, modes and line schema on windows-2025, macos-latest and ubuntu-latest, pty_backend and endpoint_kind on process-start, and per-OS artifacts.
quote: | multi-platform-exporter-compat | §3 Log file location (0600/0700 vs home DACL), §3 Logging stack (concurrent-append check on windows-2025 / macos-latest / ubuntu-latest), §4 Scenario 1 (`endpoint_kind`, `pty_backend`), §9 per-OS artifacts |
@carried n=17 label=obs-plan/telemetry-strategy fact=fct-shii35kl
why: The zero-unlogged-panics error budget is what G2 must read green on both Unix OSes, each verdict an exit-code-setting assertion.
quote: | error-budget-SLO | §7, §10 zero-unlogged-panics |
@carried n=17 run=2 label=obs-plan fact=fct-i3sjkwgk
quote: No human-review-gated log analysis. Every verdict is a `jq` / harness assertion that sets an exit code.
@carried n=18 label=obs-plan/telemetry-strategy fact=fct-lrne4qjd
why: The schema verbatim and the section 6 catalog are what the chunk re-asserts as one diagnostics schema on every OS.
quote: | creator-explicit (Founder Directions 1/2/5) | §3 schema verbatim, §6 |
@carried n=19 label=obs-plan/telemetry-strategy fact=fct-4hckhgvs
why: socket-dir-refused is a Unix-reached logged detail (exit 1 at run and revive's start, exit 21 at the CLI frame verbs) with no path, pid or uid, while a strict-modes refusal before any log opens must write no line.
quote: a strict-modes refusal made before any process log opens (the six frame verbs and `link` / `unlink`, exit 21; `run`, `revive`'s start and `verify`, exit 1) writes no line and carries no logged `detail` code: its machine-readable form is the exit code, and under `--json` the closed `instance-unreachable` document with `detail` `strict-modes-failed` (the frame verbs and `link` / `unlink`) or the `strict-modes-failed` document (`revive`); a refused socket directory is outside that exception: it carries the logged `detail` `socket-dir-refused`, no path, pid or uid, exit 21 at the eight CLI frame verbs and exit 1 at `run` and `revive`'s start |
@binding n=1
**telemetry-strategy multi-platform-exporter-compat ↔ log-coverage required fields and null encoding**: the one sink and line schema on every OS (obs-plan/telemetry-strategy fct-6isebxud) must be met by the same required fields on every line, a field one OS cannot fill encoded by key absence, never an OS-only word (obs-plan fct-qaeihrqm), checked by G4 default-deny against both schemas (obs-plan fct-llmksgvm).
@binding n=2
**telemetry-strategy Errors ↔ error-capture-reporting and ci-integration G2**: the Unix run PTY pump and handle-wait threads each inside catch_unwind (obs-plan/telemetry-strategy fct-6c7xwzms) must meet the catch-site rule that no worker panic leaves main running without its process-exit and every profile keeps unwind (obs-plan fct-sue3fxzz), counted by G2 over home-level role files on each OS (obs-plan fct-eodqccd6).
@binding n=3
**telemetry-strategy Vector 4 / Founder Direction 4 ↔ log-coverage detail catalog and the wait/last scenario**: socket-dir-refused and server-verify-failed (obs-plan/telemetry-strategy fct-n6l5o3fm, obs-plan/telemetry-strategy fct-4hckhgvs) must read as the same closed detail words on Linux and macOS, macOS an euid mismatch alone with no peer-pid comparison, a CLI wait or last ending at exit 21 during connect and MCP returning instance-unreachable with no line (obs-plan fct-q67chfay).
@binding n=4
**telemetry-strategy Metrics / perf-budget-instruments ↔ slo-invariants-telemetry-budgets**: the per-OS perf job on hyperfine --export-json (obs-plan/telemetry-strategy fct-6lxvbxmt, obs-plan/telemetry-strategy fct-olppdytk) must gate the max of the five timed hook rows through gate --require perf, with every hook command run with VIOLA_NAME set and a home under target/e2e-home/ (obs-plan fct-onnnq5uj).
@binding n=5
**telemetry-strategy Spans / Traces ↔ ci-integration G1**: the viola-pty seam spans the openpty backend sits behind (obs-plan/telemetry-strategy fct-4joil3ca) must pass G1's bare-instrument gate on all three OSes, only rg exit 1 passing (obs-plan fct-a4sgiwyc).
@binding n=6
**telemetry-strategy error-budget-SLO ↔ slo-invariants-telemetry-budgets error budget**: zero panic lines, zero secret-scan hits and zero schema-conformance failures (obs-plan/telemetry-strategy fct-shii35kl) must be measured by in-job gate exit codes on each OS, never the uploaded artifact (obs-plan fct-2v5pxi7x).
