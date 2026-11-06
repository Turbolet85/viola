@carried n=1 label=obs-plan/obs-scope-summary fact=fct-cuxf35wq
why: the `portable-pty` bump moves the `viola-pty` graph, so the seam's instrumentation must stay sync and Tokio-free under the `cargo deny` ban, and the §1 `viola-pty` row naming `=0.8.1` must be re-worded to the new pin as a body amendment.
quote: `tracing` spans (`#[tracing::instrument]`) on the start sequence, the `viola-pty` seam operations, the `viola-channel` sync server handlers, vt100 readiness-gate outcomes and `viola-state` writes, all written through the tracing-subscriber JSON logger. Instrumentation must be sync: std threads, no Tokio (arch Stack, Hybrid concurrency; `cargo deny` ban).
@carried n=1 run=2 label=obs-plan/obs-scope-summary fact=fct-foy4up7k
quote: `cargo deny` bans Tokio in the sync crates' graph and bans C-building crates (arch Observability Hints, Build policy).
@carried n=1 run=3 label=obs-plan fact=fct-r2s52jjf
quote: obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass `cargo deny`.
@carried n=1 run=4 label=obs-plan fact=fct-t2iihkln
quote: **Entity:** `viola-pty`: PTY seam (spawn, read, write, resize, wait, kill) over portable-pty `=0.8.1`
@carried n=1 run=5 label=obs-plan fact=fct-23yn4mby
quote: Section 1 is a verbatim copy of obs-scope.md, and it is kept current (the founder's ruling of 2026-10-04): a proposal or a cascade hit inside it is judged like any body amendment, and its label stays.
@carried n=2 label=obs-plan/obs-scope-summary fact=fct-he3oql74
why: Cargo.lock moves under the `portable-pty` bump and `cargo deny check advisories` must read green at the wrap, so the locked tracing-subscriber must stay at a version meeting the Vector 9 floor.
quote: `tracing` with tracing-subscriber 0.3.23, configured as `fmt().json().flatten_event(true).with_current_span(false)` (tests excerpt, verbatim). Version 0.3.23 satisfies the security Vector 9 floor of `>=0.3.20` (RUSTSEC-2025-0055).
@carried n=3 label=obs-plan/obs-scope-summary fact=fct-3ftavspi
why: the `portable-pty` bump reaches the wait and kill path under `viola-pty`, and the killed-wrapper reading of item 11 rests on the child's `process-exit` still coming from the handle wait.
quote: Child exit is detected on the process handle, so `process-exit` for the child must come from the handle wait, not from reader EOF (arch Stack; brief §4.1 Windows exit detection).
@carried n=3 run=2 label=obs-plan fact=fct-ctz7espz
quote: the child's `process-exit` comes from the handle wait, never from reader EOF.
@carried n=3 run=3 label=obs-plan fact=fct-avrouaxx
quote: its spawn and exit (exit detected on the process handle, never on EOF), paste writes and the vt100 readiness-gate outcome
@carried n=4 label=obs-plan/obs-scope-summary fact=fct-i2uu7hiy
why: the live `viola run` sessions read their evidence from the run file sink through the harness, `run` must write nothing to the terminal while the child runs, and the remnant removal must leave one Linux sink and schema with no OS-specific exporter.
quote: A file sink only, to `diagnostics/run-<name>.ndjson` (tests excerpt, Log format). The terminal is forbidden: "`run` writes nothing to the terminal while the child runs" (arch Observability Hints).
@carried n=4 run=2 label=obs-plan fact=fct-dao4nifd
quote: one file sink and one line schema on Linux, with no OS-specific exporter.
@carried n=4 run=3 label=obs-plan fact=fct-q2zh72zo
quote: The agent-readable consumption path is the tests harness `logs` command (`scripts/agent-run.sh logs`, via `viola-harness`).
@carried n=5 label=obs-plan/obs-scope-summary fact=fct-f3waaflx
why: kill-fallback use is a call site the `portable-pty` bump reaches, and the founder's ruling on the socket-directory pair's wording must keep the `socket-dir-refused` code consistent wherever it is recorded.
quote: Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use, and the server check's peer-euid, peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line), not the OS internals. The home's strict-modes checks (`check_instance`, `check_home_tree`, `check_diagnostics`: viola's stat reads) are such call sites; which of their refusals write a line is the strict-modes refusal-telemetry trigger's (below). The socket directory's check (`check_socket_dir`: viola's one `symlink_metadata`) is such a call site, its refusal written under `socket-dir-refused` (MCP by no line)
@carried n=5 run=2 label=obs-plan/obs-scope-summary fact=fct-oiibon7o
quote: A refused socket directory has its own code, `socket-dir-refused`, and writes its line (the Founder Direction 4 trigger below states it).
@carried n=5 run=3 label=obs-plan/obs-scope-summary fact=fct-k56v7yhe
quote: `instance-unreachable` (its `detail` one closed cause: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`; the CLI's document less `v`, its text `refused: instance-unreachable`)
@carried n=5 run=4 label=obs-plan fact=fct-wrd7y3z2
quote: a refused socket directory, read by the strict-modes read verdict, is no strict-modes failure: its line, where it has one, carries its own `detail`, `socket-dir-refused`.
@carried n=6 label=obs-plan/obs-scope-summary fact=fct-t4pdf4hq
why: the dialog hold near the 3600 s ceiling is read live here, and its evidence is the required `hook-invoked` / `hook-decision` lines in the hook file sink.
quote: `hook-invoked` / `hook-decision` lines are required, and panics are caught with `catch_unwind` and emitted as one `level:"ERROR", event:"panic"` line (tests excerpt, Log format, Agent parsing).
@carried n=6 run=2 label=obs-plan/obs-scope-summary fact=fct-d5lpbhzx
quote: A file sink only, to `diagnostics/hook-<name>.ndjson`. stderr is forbidden on both.
@carried n=7 label=obs-plan/obs-scope-summary fact=fct-ogkwmfvl
why: a live real-`claude` session's diagnostics are non-synthetic and must still hold the `Cookie` / `CLAUDE*` floor the re-run secret scan enforces, and the live evidence is read by `jq`, never by human review.
quote: - **Constraints:**
    - The hook trace is the `hook-<name>.ndjson` file. A failed write there is swallowed, so the hook still exits 0 and never writes to stderr.
    - No line may contain a `Cookie` header or any stripped `CLAUDE*` value. This is enforced by the secret-scan test in §6.
  - **Agent parsing:** lines parseable with `jq -c 'select(.event=="dialog-raised")'` or equivalent; NEVER multi-line stack traces.
@carried n=7 run=2 label=obs-plan/obs-scope-summary fact=fct-khwnpdov
quote: Every file is 0600 inside a 0700 directory, and the tests secret-scan test enforces it.
@carried n=7 run=3 label=obs-plan fact=fct-xvmeqbmc
quote: a never-log floor for every R8-stripped `CLAUDE*` value across all sinks and fixtures
@binding n=1
**obs-scope-summary `viola-pty` row and sync instrumentation ↔ telemetry triggers (Vector 9) and log-coverage per-module levels**: the `portable-pty` release that drops `pin-utils` must keep `viola-pty`'s graph Tokio-free and free of C-building crates and pass `cargo deny` with no ignore-list widening, and the §1 `viola-pty` row must name the new pin, landed through the store's whole-fact form like any body amendment (obs-plan/obs-scope-summary fct-cuxf35wq) (obs-plan fct-t2iihkln) (obs-plan fct-23yn4mby).
@binding n=2
**obs-scope-summary logging stack ↔ ci-integration supply-chain gate**: the Cargo.lock move under the bump must leave tracing-subscriber at a version meeting the `>=0.3.20` floor so `cargo deny check advisories` and CI's `supply-chain` job read green at the wrap (obs-plan/obs-scope-summary fct-he3oql74).
@binding n=3
**obs-scope-summary child-exit and kill-fallback call sites ↔ log-coverage PTY seam wrapper**: after the bump the child's `process-exit` must still come from the handle-wait thread and kill-fallback use must still be a logged viola call site, since the killed-wrapper reading of item 11 rests on that exit path (obs-plan/obs-scope-summary fct-3ftavspi) (obs-plan/obs-scope-summary fct-f3waaflx).
@binding n=4
**obs-scope-summary `socket-dir-refused` records ↔ observability harness contract start refusals**: whatever wording the founder rules for the socket-directory refusal pair, the `socket-dir-refused` code must read the same in the CLI's logged line, the MCP `instance-unreachable` cause list and the Vector 7 trigger (obs-plan/obs-scope-summary fct-oiibon7o) (obs-plan/obs-scope-summary fct-k56v7yhe) (obs-plan fct-wrd7y3z2).
@binding n=5
**obs-scope-summary run and hook sinks ↔ span-trace-coverage dialog scenario and pii-scrubbing-compliance**: the live sessions' evidence (v1-53's `viola run`, the overseer driving a builder, the dialog hold near 3600 s) must be read from `run-<name>.ndjson` and `hook-<name>.ndjson` through `agent-run logs` / `status` and `jq`, and those non-synthetic diagnostics must hold no `Cookie` or `CLAUDE*` value before any upload (obs-plan/obs-scope-summary fct-i2uu7hiy) (obs-plan/obs-scope-summary fct-t4pdf4hq) (obs-plan/obs-scope-summary fct-ogkwmfvl).
