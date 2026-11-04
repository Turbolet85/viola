@carried n=1 label=obs-plan/obs-scope-summary fact=fct-2v5blby2
why: the openpty OS API the chunk puts behind the viola-pty seam on Linux and macOS must stay boundary-only, instrumented at the seam and never inside the OS API or the PTY byte stream.
quote: **Instrumentability:** boundary-only
@carried n=1 run=2 label=obs-plan fact=fct-m24p7svx
quote: instrumentable at the seam; the ConPTY and openpty OS APIs behind it are boundary-only, and the PTY byte stream is user content, never logged.
@carried n=1 run=3 label=obs-plan fact=fct-eokuiijb
quote: boundary-only — viola's call sites, not the OS internals.
@carried n=2 label=obs-plan/obs-scope-summary fact=fct-f3waaflx
why: the Unix call sites the parity suite exercises (kill-fallback use, the peer-euid read, the socket-directory check, accept_loop's euid refusal) must log exactly these closed words and no others on every OS, the MCP and accept_loop cases by no line.
quote: Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-euid (Unix), peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals. The home's strict-modes checks (`check_instance`, `check_home_tree`, `check_diagnostics`: viola's stat and, on Windows, owner and DACL reads) are such call sites; which of their refusals write a line is the strict-modes refusal-telemetry trigger's (below). The socket directory's check (`check_socket_dir`: viola's one `symlink_metadata`, off Unix no read) is such a call site, its refusal written under `socket-dir-refused` (MCP by no line), and the wrapper's read of a client's euid in `accept_loop` writes no line.
@carried n=2 run=2 label=obs-plan fact=fct-wrd7y3z2
quote: a refused socket directory, read by the strict-modes read verdict, is no strict-modes failure: its line, where it has one, carries its own `detail`, `socket-dir-refused`.
@carried n=3 label=obs-plan/obs-scope-summary fact=fct-a3522pml
why: the snapshot and --json documents the chunk re-asserts as one schema on every OS must keep the per-store scrub rules and the replayed-snapshot behaviour identically on Linux, macOS and Windows.
quote: Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
  - A `snapshot.json` the replay rebuilds from `events.ndjson` (`read_snapshot_or_replay`) carries no `endpoint`, `pid`, `started_at` or `statusline_command`: `live_endpoint` over it is `None` (unreachable), hook `connect` is unreachable (fail open, `channel-unreachable`), `hook statusline` runs and prints nothing, and `run`'s collision check reads the heartbeat alone.
@carried n=3 run=2 label=obs-plan/obs-scope-summary fact=fct-or7ezsji
quote: instrumentable (through the `viola-state` call sites)
@carried n=3 run=3 label=obs-plan fact=fct-5wgriooq
quote: `events.ndjson` payload content is never copied into process-log lines, and the absolute paths in `snapshot.json` and config values are scrubbed from external output.
@carried n=4 label=obs-plan/obs-scope-summary fact=fct-noyuw3eo
why: the JSON logger format and the per-process one-line panic hook are what G2 and G4 read on ubuntu-latest and macos-latest, so the Unix wrapper must run under the same logger and hook as Windows.
quote: Rust `tracing` facade with `#[tracing::instrument]` spans on the dispatch functions, plus a tracing-subscriber JSON structured logger (tests excerpt, Log format: `tracing-subscriber 0.3.23 fmt().json().flatten_event(true).with_current_span(false)`). Each process gets its own panic hook: a `std::panic::set_hook` replacement that writes one line.
@carried n=4 run=2 label=obs-plan fact=fct-ccfaed7l
quote: `tracing` with tracing-subscriber 0.3.23, `fmt().json().flatten_event(true).with_current_span(false)`; one file per process role under `diagnostics/`, no stdout sink on any role, stderr JSON only for `mcp` when `mcp.ndjson` cannot be opened; each line one `write` on an append-mode handle; the level filter is never read from `RUST_LOG`, and no verbosity control may bypass redaction; JSON-to-file is the only mode.
@carried n=5 label=obs-plan/obs-scope-summary fact=fct-oy7tgncm
why: the short-lived verbs must write the same file sink and the same strict-modes-failed --json document on every OS, which is what the one-diagnostics-schema re-assertion reads.
quote: A file sink only: JSON-per-line to `diagnostics/` when an instance is resolved (security Vector 6: "write the full error only to `diagnostics/` when an instance is resolved"). Without an instance there is no process-log file, and human stderr carries only fixed-message lines (design excerpt, CLI faults). Network OTLP is excluded: security says "viola makes no outbound network calls". The stdout exporter is excluded because stdout carries `--json` results. `viola revive <name> --list` logs as `cli` into the caller's `cli-<VIOLA_NAME>.ndjson` once its strict-modes check has passed (none with no `VIOLA_NAME`; a refused check opens none; chunk 2026-10-17-cli-machine-contract). `viola link` / `viola unlink` log as `cli` into the driven instance's `cli-<driven>.ndjson` (the instance their `<driven>` argument names, never the caller's `VIOLA_NAME`) once their instance check of it has passed; a refused check opens none. `send`, `wait`, `last`, `answer`, `pause` and `release` log as `cli` once their instance check (`refuse_untrusted`: `check_instance` on the home and `instances/<name>`) has passed; a refused check exits 21 with the instance pair, or under `--json` the `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` document, opens nothing under the home and writes no line.
@carried n=5 run=2 label=obs-plan fact=fct-dao4nifd
quote: the same file sink and line schema on every OS, with identical fields and no OS-specific exporter.
@carried n=6 label=obs-plan/obs-scope-summary fact=fct-uwlbnhdg
why: every line the parity suite produces on Unix must carry the required process and instance fields from the same sources as on Windows, so the one schema reads identical fields.
quote: The process role is the `process` field. The instance is `ViolaName` from an explicit argument or `VIOLA_NAME`.
@carried n=6 run=2 label=obs-plan/obs-scope-summary fact=fct-ivwsjbo2
quote: as for the short-lived verbs, with `process="run"` and `instance=<ViolaName>`.
@carried n=6 run=3 label=obs-plan fact=fct-uwfffcsu
quote: Per line, role and instance are carried by the tests-required fields `process` (`run|hook|mcp|cli`) and `instance` (`ViolaName` or null), matching `sender` / `writer` (arch Obs-Relevant Conventions, Service identity).
@carried n=7 label=obs-plan/obs-scope-summary fact=fct-oiibon7o
why: the scope holds the typed exit-code set and the macOS no-peer-pid carve-out, so Unix refusals must keep the fixed stderr lines, the distinct detail codes and the euid-alone reading of server-verify-failed on macOS.
quote: Refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text (design excerpt, CLI refusals / CLI faults). Without `--json` a catch-site fault prints the fixed `error: internal error` line; under `--json` stderr stays empty and the `{"v":1,"error":"internal-error","detail":null}` document goes to stdout (`internal_error_document` in `src/cmd/mod.rs`, when `json_asked` in `src/main.rs` reads `--json` among the arguments before a `--`).
  - Exit codes 0, 1, 2, 10–14, 20 and 21 are the machine-readable outcome and should be a log field (arch Standard Contracts, CLI `--json` + exit codes).
  - Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code, except a strict-modes check refused before any log opens, which writes no line: its cause is carried only by the fixed stderr pair or the `--json` document (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, exit 21; `run`, `viola revive` and `viola verify`, exit 1). A refused socket directory has its own code, `socket-dir-refused`, and writes its line; macOS compares no peer pid, the named carve-out from this rule, owned by no route entry, and there `server-verify-failed` means an euid mismatch alone (the Founder Direction 4 trigger below states them).
@carried n=7 run=2 label=obs-plan fact=fct-y6qmqcu2
quote: each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid, except a strict-modes check refused before any log opens, which writes no line (its cause is the fixed stderr pair or the `--json` document).
@carried n=8 label=obs-plan/obs-scope-summary fact=fct-cuxf35wq
why: the openpty backend the chunk puts behind the viola-pty seam must be traced through the same sync seam spans as ConPTY, with no Tokio, which the per-OS lint job's cargo deny re-reads.
quote: `tracing` spans (`#[tracing::instrument]`) on the start sequence, the `viola-pty` seam operations, the `viola-channel` sync server handlers, vt100 readiness-gate outcomes and `viola-state` writes, all written through the tracing-subscriber JSON logger. Instrumentation must be sync: std threads, no Tokio (arch Stack, Hybrid concurrency; `cargo deny` ban).
@carried n=8 run=2 label=obs-plan fact=fct-r2s52jjf
quote: obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass `cargo deny`.
@carried n=9 label=obs-plan/obs-scope-summary fact=fct-i2uu7hiy
why: the Unix wrapper must write only to its run file sink and nothing of its own to the terminal while the child runs, as the Windows wrapper does.
quote: A file sink only, to `diagnostics/run-<name>.ndjson` (tests excerpt, Log format). The terminal is forbidden: "`run` writes nothing to the terminal while the child runs" (arch Observability Hints).
@carried n=9 run=2 label=obs-plan fact=fct-ctz7espz
quote: a file sink only, to `diagnostics/run-<name>.ndjson`; `run` writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns; instrumentation is sync, with no Tokio; the child's `process-exit` comes from the handle wait, never from reader EOF.
@carried n=10 label=obs-plan/obs-scope-summary fact=fct-3ftavspi
why: under openpty the panic hook must be replaced before the child spawns, the 1 s heartbeat is the liveness tick the v1-40 stale-row witness reads, and the child's process-exit must come from the handle wait, never reader EOF.
quote: Rust's default panic hook prints to stderr, which is the child's terminal. It must be replaced before the child spawns.
  - The heartbeat file, touched every 1 s, is the existing liveness tick (arch Standard Contracts, Heartbeat).
  - Child exit is detected on the process handle, so `process-exit` for the child must come from the handle wait, not from reader EOF (arch Stack; brief §4.1 Windows exit detection).
@carried n=10 run=2 label=obs-plan fact=fct-avrouaxx
quote: boundary-only — a third-party binary run unmodified: its spawn and exit (exit detected on the process handle, never on EOF), paste writes and the vt100 readiness-gate outcome; its internal state and transcripts are never read as a contract.
@carried n=11 label=obs-plan/obs-scope-summary fact=fct-t4pdf4hq
why: the Unix fake-agent suite reads the required hook lines, and a hook panic on Unix must be caught and written as the one panic line G2 counts.
quote: `hook-invoked` / `hook-decision` lines are required, and panics are caught with `catch_unwind` and emitted as one `level:"ERROR", event:"panic"` line (tests excerpt, Log format, Agent parsing).
@carried n=12 label=obs-plan/obs-scope-summary fact=fct-d5lpbhzx
why: the scope holds the hook contract unchanged, so on Linux and macOS the hook must exit 0 with stderr empty, stdout only the decision body, and a failed log write swallowed.
quote: stderr is forbidden on both. stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output (arch Standard Contracts, Hook contract); the two hidden `--capture` arms (`verify`'s probe capture) write no sink and print nothing, except `--answers`' probe decision body. A failed log write is swallowed, and the hook still exits 0 (tests excerpt, Log format Constraints).
@carried n=12 run=2 label=obs-plan fact=fct-s5xlshqe
quote: fail-open paths log to `hook-<name>.ndjson` only, exit 0 with stderr empty, and drift reports go to `diagnostics/` only.
@carried n=13 label=obs-plan/obs-scope-summary fact=fct-7fgifl76
why: the Unix suite must read the E1 silent no-op and the instance check before obs init, and the fake-agent hook panic seam must stay after obs init, the one panic G2 exempts.
quote: If `VIOLA_NAME` is missing, `instance` is null and the hook is a silent no-op (tests excerpt, E1 R5 unwrapped session hooks; Coverage Triggers, Vector 4 "missing `VIOLA_NAME`"). The event and dialog paths resolve the instance directory (`canonical`) and run its strict-modes check (`trusted`: `check_instance`) before `read_diagnostics_level` and obs init: a refused or absent instance is a silent exit 0 with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append. The two `--capture` arms run ahead of the check; the `FAKE_AGENT_HOOK_PANIC` seam runs after obs init.
@carried n=14 label=obs-plan/obs-scope-summary fact=fct-kv6frzyp
why: the per-OS perf job the chunk re-reads green on ubuntu-latest and macos-latest times these five hook rows, with the Unix socket-directory and peer-euid checks inside the timed path.
quote: The latency budget is `max < 1.0 s` per hook (tests excerpt, Quality Gates). Logger init must not add blocking I/O beyond one file open and append; the instance check ahead of it (`canonical` + `trusted`: stat and, on Windows, owner and DACL reads) is in the timed path, and `run --perf` keeps the five hook rows under `SPINE_DEADLINE` with it, max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms (as measured at `.andromeda/runs/2026-10-26T19-02-07-implement/gate-2026-10-26-home-and-code-bearing-file-integrity.json`); the socket-directory check and the peer-euid check on the hook `connect` sit in that timed path too, the rows under `SPINE_DEADLINE` and at or under that band with them (as measured at `.andromeda/runs/2026-11-01T16-38-56-implement/outcome-2026-11-01-unix-endpoint-and-home-hardening.md`).
@carried n=15 label=obs-plan/obs-scope-summary fact=fct-kkjide4x
why: on the Unix socket endpoint channel-request and channel-response must carry corr on both sides, as on the Windows named pipe.
quote: Propagation uses the `corr` field in log lines (JSON-RPC `id` for `channel-request` / `channel-response`; `dialog_id`; send `cursor`), per the tests excerpt Log format.
@carried n=15 run=2 label=obs-plan fact=fct-m7cbk627
quote: the log file of whichever process is on each side; propagation is the `corr` field in log lines, since channel `params` declare no trace-context field
@carried n=15 run=3 label=obs-plan fact=fct-23yn4mby
quote: the channel join key (D-25: `(conn, corr)`, each side logs its own `instance`)
@binding n=1
**obs-scope-summary (OS APIs boundary-only) ↔ log-coverage (PTY seam boundary-call wrapper)**: openpty sits behind the seam as boundary-only (obs-plan/obs-scope-summary fct-2v5blby2), and the seam's own lines must read the same on Unix as on Windows: spawn gives `process-start{subject:"claude-child"}`, exit from the handle-wait thread, kill-fallback as `exit_source:"kill-fallback"`, paste `text_bytes` only (obs-plan fct-3epowryi).
@binding n=2
**obs-scope-summary (file sinks per role) ↔ triggers (multi-platform-exporter-compat) ↔ ci-integration (G4)**: every role's sink (obs-plan/obs-scope-summary fct-oy7tgncm, fct-i2uu7hiy, fct-d5lpbhzx) must be the same file sink and line schema on every OS (obs-plan fct-dao4nifd), and G4 must validate it against `schemas/diag-line.v1.json` and `schemas/diag-detail.v1.json` on ubuntu-latest and macos-latest as on windows-2025 (obs-plan fct-htkftgtl).
@binding n=3
**obs-scope-summary (run notes: heartbeat tick) ↔ slo-invariants-telemetry-budgets (liveness signal)**: the 1 s heartbeat (obs-plan/obs-scope-summary fct-3ftavspi) behind the v1-40 stale-row witness must flip at more than 5 s with no transition line (obs-plan fct-gqc4mh6b); §1's "Log liveness transitions … on change only" (obs-plan fct-zdbw2hss) yields to that D-13 rule, as the section-currency note requires (obs-plan fct-23yn4mby).
@binding n=4
**obs-scope-summary (Founder Direction 4 notes, macOS carve-out) ↔ span-trace-coverage (wait / last scenario) ↔ error-capture-reporting (MCP rejects)**: on macOS `server-verify-failed` must mean an euid mismatch alone (obs-plan/obs-scope-summary fct-oiibon7o), a CLI `wait` / `last` ends with `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}` and no `channel-request` line (obs-plan fct-q67chfay), and MCP's `instance-unreachable` writes no line (obs-plan fct-pqz5mgd5); no test reads macOS as a pid or start-time comparison.
@binding n=5
**obs-scope-summary (hook identity, fake-agent panic seam) ↔ ci-integration (G2 exemption)**: the `FAKE_AGENT_HOOK_PANIC` seam runs after obs init (obs-plan/obs-scope-summary fct-7fgifl76) and its panic at exactly `src/cmd/hook/seam.rs:<digits>` stays G2's one exemption on all three OSes (obs-plan fct-eodqccd6); neither may widen to make a Unix case pass.
@binding n=6
**obs-scope-summary (panic hooks per process) ↔ error-capture-reporting (catch sites) ↔ slo-invariants-telemetry-budgets (zero unlogged panics)**: the one-line panic hook (obs-plan/obs-scope-summary fct-noyuw3eo, fct-t4pdf4hq) must be backed on Unix by the same catch sites, the `run` PTY pump and handle-wait threads each inside `catch_unwind` (obs-plan fct-sue3fxzz), so G2 counts zero unlogged panics on each OS (obs-plan fct-3qkbv2ca).
@binding n=7
**obs-scope-summary (hook latency notes) ↔ slo-invariants-telemetry-budgets (performance budgets)**: the five hook rows timed with the Unix socket-directory and peer-euid checks in the path (obs-plan/obs-scope-summary fct-kv6frzyp) gate the hyperfine `max` through `gate --require perf` in the per-OS `perf` job, holding only with `VIOLA_NAME` set and a home under `target/e2e-home/` (obs-plan fct-onnnq5uj).
@binding n=8
**obs-scope-summary (persistent stores scrub rules) ↔ log-coverage (state-recovered details)**: the replayed snapshot (obs-plan/obs-scope-summary fct-a3522pml) must log `state-recovered{detail:"snapshot-replayed"}` and a healed torn append `state-recovered{detail:"torn-line-healed"}` with the same detail words on every OS (obs-plan fct-iyk74prg, obs-plan fct-ghtvc6rx).
