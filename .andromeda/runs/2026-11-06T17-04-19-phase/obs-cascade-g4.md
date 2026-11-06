@carried n=1 label=obs-plan/telemetry-strategy fact=fct-4joil3ca
why: The `portable-pty` bump must leave the `viola-pty` seam's spawn/write/wait/kill spans declared with `skip_all` and a static name, so G1 stays green at the wrap.
quote: every crate seam: `viola-pty` spawn/write/wait/kill, `viola-channel` client call and server dispatch, `viola-state`
  snapshot/heartbeat/tail, `viola-agent-claude` hook parse, `viola-mcp` tool calls | Declared in code with tracing 0.1.44
  `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`.
@carried n=2 label=obs-plan/telemetry-strategy fact=fct-4ns7wzxz
why: The live sessions (v1-53's `viola run`, the overseer driving a builder) must be read through the per-role files that `agent-run logs` merges with `events.ndjson`, each verdict a `jq` / harness assertion.
quote: **The primary signal.** tracing-subscriber 0.3.23 JSON-per-line, one file per process role under
  `<home>/diagnostics/`, merged with `events.ndjson` by `agent-run logs`
@carried n=2 run=2 label=obs-plan fact=fct-i3sjkwgk
quote: No human-review-gated log analysis. Every verdict is a `jq` / harness assertion that sets an exit code.
@carried n=3 label=obs-plan/telemetry-strategy fact=fct-6c7xwzms
why: The thiserror census re-measure and each one-enum fold or ratification must keep every typed error's fixed `Display`, and the bumped PTY seam must keep the `run` PTY pump and handle-wait threads inside `catch_unwind`.
quote: typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit` (the
  `mcp` role writes no self `process-exit`, Section 7) | No external reporter: security says "no outbound network calls".
  A hand-written `std::panic::set_hook`, plus `std::panic::catch_unwind` at the main-thread catch site of every role,
  around `hook` dispatch, in the `run` PTY pump and handle-wait threads, and around the vt100 feed (Section 7)
@carried n=3 run=2 label=obs-plan/telemetry-strategy fact=fct-kgjd4mty
quote: | logging-sensitive Vector 6 (CLI/env) | §4 Scenario 1 (`env_stripped_*`), §7 fixed messages, §8 |
@carried n=4 label=obs-plan/telemetry-strategy fact=fct-6ukjckx6
why: Every gate green at the done-check includes G2 and the zero error budget, which must witness exactly one single-line panic record per panic after init step 4.
quote: Every panic after init step 4 produces exactly one single-line structured record in its home-level role file:
  location, thread, role and instance.
@carried n=4 run=2 label=obs-plan/telemetry-strategy fact=fct-shii35kl
quote: | error-budget-SLO | §7, §10 zero-unlogged-panics |
@carried n=5 label=obs-plan/telemetry-strategy fact=fct-ne5xp5cg
why: The founder's ratification or re-wording of the socket-directory pair must keep where `socket-dir-refused` writes a line consistent across the trigger rows that state it.
quote: the socket-directory check shares the read verdict yet is no such exemption, §6 `socket-dir-refused`: one
  `process-exit` at `run` / `revive`'s start and at the eight CLI frame verbs, one `hook-decision` at `viola hook`, and
  no line only at MCP's `frame`)
@carried n=5 run=2 label=obs-plan/telemetry-strategy fact=fct-n6l5o3fm
quote: A refused socket directory is no such silent path: the hook's `connect` writes one
  `hook-decision{detail:"socket-dir-refused"}` at WARN, with exit 0, empty streams and no frame.
@carried n=5 run=3 label=obs-plan/telemetry-strategy fact=fct-4hckhgvs
quote: a refused socket directory is outside that exception: it carries the logged `detail` `socket-dir-refused`, no
  path, pid or uid, exit 21 at the eight CLI frame verbs and exit 1 at `run` and `revive`'s start
@carried n=6 label=obs-plan/telemetry-strategy fact=fct-wkatnlg5
why: The bumped PTY seam and the live sessions' child spawns must keep the closed `subject` values on `process-start` that Vector 8 maps to Scenario 1.
quote: | logging-sensitive Vector 8 (child spawn) | §4 Scenario 1 (`subject` values), §6 `parse-rejected` |
@carried n=7 label=obs-plan/telemetry-strategy fact=fct-pso667is
why: The `portable-pty` move off `=0.8.1` must leave the logging stack's feature set and the §9 cargo-deny gates passing, with no ignore-list widening.
quote: | logging-sensitive Vector 9 (supply chain) | §3 Logging stack (feature set), §9 cargo-deny gates, §11 |
@carried n=8 label=obs-plan/telemetry-strategy fact=fct-4vc4xc3n
why: The killed-wrapper reading and the kill fallback through the bumped seam must keep `process-exit` carrying `exit_source` and `during`.
quote: | chaos-instrumentation | §6 `state-recovered` / `process-exit` `exit_source` + `during`, §10 |
@carried n=9 label=obs-plan/telemetry-strategy fact=fct-6isebxud
why: The `process-start` row's `pty_backend` cut to `openpty` alone and the Windows remnant removal must leave this trigger's coverage on the one `ubuntu-latest` leg, with no OS-specific exporter, artifact or gate branch.
quote: | multi-platform-exporter-compat | §3 Log file location (0600/0700), §3 Logging stack (concurrent-append check on
  ubuntu-latest), §4 Scenario 1 (`endpoint_kind`, `pty_backend`), §9 artifacts (their names in `matrix.os` form, on the
  one `ubuntu-latest` leg) |
@carried n=10 label=obs-plan/telemetry-strategy fact=fct-lrne4qjd
why: The `process-start` row re-word lands in §6 and must keep the §3 schema verbatim, conforming under G4.
quote: | creator-explicit (Founder Directions 1/2/5) | §3 schema verbatim, §6 |
@binding n=1
**telemetry-strategy Vector 9 ↔ log-coverage per-module levels**: the `portable-pty` bump must pass `cargo deny` and keep the sync graph Tokio-free and C-free (obs-plan/telemetry-strategy fct-pso667is), and the `portable_pty` target must stay `OFF`, with failures captured at the viola seam (obs-plan fct-7e4jvssp).
@binding n=2
**telemetry-strategy multi-platform-exporter-compat ↔ obs-scope-summary required signal ↔ log-coverage `process-start` row**: the row re-worded to `pty_backend` `openpty` alone, with `sideload_fallback` gone, must keep the trigger's Scenario 1 coverage (obs-plan/telemetry-strategy fct-6isebxud) and its signal of one file sink and one line schema on Linux (obs-plan fct-dao4nifd).
@binding n=3
**telemetry-strategy Errors row ↔ error-capture platform pick**: the thiserror census (10 measured where 9 is stated) and each one-enum fold or ratification must be re-worded in the platform pick's list (obs-plan fct-nwrbbhk2), and each type must keep the fixed `Display` the strategy row and Vector 6's §7 fixed messages require (obs-plan/telemetry-strategy fct-6c7xwzms), (obs-plan/telemetry-strategy fct-kgjd4mty).
@binding n=4
**telemetry-strategy socket-directory rows ↔ §3 harness contract start refusals**: if the founder re-words the socket-directory pair, the Vector 7, Vector 4 and Founder Direction 4 rows (obs-plan/telemetry-strategy fct-ne5xp5cg), (obs-plan/telemetry-strategy fct-n6l5o3fm), (obs-plan/telemetry-strategy fct-4hckhgvs) must move with the `run` start-refusal text (obs-plan fct-26ufyipy).
@binding n=5
**telemetry-strategy chaos and child-spawn rows ↔ log-coverage PTY seam wrapper**: through the bump, spawn must still give `process-start{subject:"claude-child"}` and exit must still come from the handle-wait thread, with `exit_source:"kill-fallback"` on the fallback (obs-plan fct-3epowryi), as the Vector 8 and chaos rows map it (obs-plan/telemetry-strategy fct-wkatnlg5), (obs-plan/telemetry-strategy fct-4vc4xc3n).
@binding n=6
**telemetry-strategy panic record ↔ SLO error budget**: the done-check's green gates must show 0 panic lines, 0 secret-scan hits and 0 schema failures (obs-plan fct-zze2ezbr), so that each panic leaves exactly its one home-level record (obs-plan/telemetry-strategy fct-6ukjckx6).
