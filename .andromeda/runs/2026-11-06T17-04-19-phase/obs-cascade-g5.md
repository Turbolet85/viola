@carried n=1 label=obs-plan/span-trace-coverage fact=fct-xrkfnxjv
why: The portable-pty move off `=0.8.1` must re-word this row's version and keep the `portable_pty` target OFF, failures captured at the seam, and the child's process lines from the handle-wait thread.
quote: none (portable-pty 0.8.1 logs through `log`; its target is OFF and failures are captured at the seam, D-11) | Manual spans on the start sequence, `pty.*` seam, `channel.dispatch`, `run.readiness_gate` (vt100 has no tracing), `run.wheel_transition`, `run.budget_eval`, `state.*`. Child `process-start` / `process-exit` from the handle-wait thread
@carried n=1 run=2 label=obs-plan fact=fct-4sjmn35v
quote: Every span is manual — no surface has an auto-instrumentation library viola uses, and the rmcp and portable-pty targets are OFF in the sink
@carried n=2 label=obs-plan/span-trace-coverage fact=fct-kreejnxq
why: The live read of a dialog hold near the 3600 s ceiling must take its evidence from these hook lines, read through `agent-run logs` and `jq`.
quote: `hook.handle` › `channel.request` › `hook.decision_emit`. `hook-invoked` / `hook-decision`
@carried n=2 run=2 label=obs-plan fact=fct-trei2tnd
quote: The question, answer and tool `input` never appear; a dialog hook's `hook-invoked` is written only after the `hook.dialog` response.
@carried n=3 label=obs-plan/span-trace-coverage fact=fct-jvpve25b
why: The mutation audit's one survivor by construction is an always-admit `peer_admitted` at exactly these three client seams; the chunk must keep its dated note and untestable zone, and the `host.rs` key edit must not change the mutation result.
quote: each client seam (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) reads the server's euid after the connect and before the pid comparison (`verify_server`, MCP's `server_verified`), through `Client::server_euid` / `TokioClient::server_euid`, which write and log nothing; `peer_admitted` refuses a peer of another euid and a peer whose euid it cannot read (`peer_euid_admitted(observed, own)` is `observed == Some(own)`).
@carried n=3 run=2 label=obs-plan/span-trace-coverage fact=fct-kohas2np
quote: then the euid decision `peer_admitted(client.server_euid())`, then `verify_server`) before any frame.
@carried n=3 run=3 label=obs-plan/span-trace-coverage fact=fct-ts6ky5ry
quote: (the euid decision `peer_admitted(client.server_euid())`, then `server_verified`, the start time read on `tokio::task::spawn_blocking`)
@carried n=4 label=obs-plan/span-trace-coverage fact=fct-zklcva6d
why: The founder's ratify or re-word of the socket-directory pair's wording must leave this line and its `socket-dir-refused` detail code agreeing with the ruled wording.
quote: a frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) meeting a refused socket directory writes one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN inside its dispatch span, no `channel-request` line and no frame.
@carried n=4 run=2 label=obs-plan/span-trace-coverage fact=fct-elpncyv4
quote: a hook meeting a refused socket directory writes one `hook-decision{detail:"socket-dir-refused"}` at WARN inside `hook.handle`, on the dialog path after `hook-invoked{corr:null}` and with `decision_emitted:false`; exit 0, empty streams, no frame.
@carried n=5 label=obs-plan/span-trace-coverage fact=fct-ucfyu5c3
why: The socket-directory pair is the founder item this chunk takes his word on; whatever wording he rules must stay plain ASCII, `hint:` last, no SGR, naming only the parsed `ViolaName`.
quote: The socket-directory refusal's human pair (design-system §Surface: cli) is printed on start and frame-verb refusals in plain ASCII with `hint:` last and no SGR, and names only the parsed `ViolaName`.
@carried n=6 label=obs-plan/span-trace-coverage fact=fct-e6vtcesf
why: The portable-pty bump reaches the viola-pty spawn, so the start's span chain must still end at `pty.spawn` (CLIENT), and v1-53's live `viola run` walks this chain.
quote: `run.start` (INTERNAL, root) › `state.snapshot_recover` (the start's one snapshot read, `read_snapshot_or_replay`, which the collision check takes) › `run.collision_check` › `run.pin_copy` › `run.version_gate` (spawns the `version-probe`) › `channel.bind` › `run.budget_eval` (INTERNAL, `trigger="start"`; its attributes are under Scenario: Budget governor; chunk 2026-10-10-budget-governor) › `state.snapshot_write` › `state.heartbeat_start` › `pty.spawn` (CLIENT).
@carried n=7 label=obs-plan/span-trace-coverage fact=fct-gdmzm6bw
why: The log-coverage `process-start` row re-word to `pty_backend` `openpty` alone must match this attribute, still read from `viola_pty::pty_backend()` after the bump.
quote: `pty.spawn`: `pty_backend` (`openpty`, from `viola_pty::pty_backend()`), `env_stripped_count`.
@carried n=8 label=obs-plan/span-trace-coverage fact=fct-4g2at3oh
why: The re-worded `process-start` row (no `sideload_fallback`) must agree with this field list and with the schema's `pty_backend` / `os` typing.
quote: On success, `process-start{subject:"claude-child", child_pid, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept}` (D-34).
@carried n=9 label=obs-plan/span-trace-coverage fact=fct-mw7zqeom
why: The portable-pty bump must keep the child's exit on the handle-wait thread with its `exit_source`, which the killed-wrapper reading of item 11 rests on.
quote: The handle-wait thread emits `process-exit{subject:"claude-child", child_exit_status, exit_source:"handle-wait"|"kill-fallback"}`, never on reader EOF.
@carried n=9 run=2 label=obs-plan fact=fct-itztmsse
quote: the child's `process-exit` comes from the handle-wait thread, never on reader EOF.
@binding n=1
**span-trace-coverage ↔ log-coverage**: the `process-start` row re-word (fct-yb5jpaot: `pty_backend` `openpty` alone, `sideload_fallback` gone, through the whole-fact form) must agree with the `pty.spawn` attribute and the claude-child `process-start` field list (obs-plan/span-trace-coverage fct-gdmzm6bw) (obs-plan/span-trace-coverage fct-4g2at3oh).
@binding n=2
**span-trace-coverage ↔ obs-scope-summary and log-coverage**: the portable-pty version this row names goes stale with the bump, together with the `viola-pty` entity row (obs-plan fct-t2iihkln); the per-module table must keep `portable_pty` OFF (obs-plan fct-7e4jvssp) (obs-plan/span-trace-coverage fct-xrkfnxjv).
@binding n=3
**span-trace-coverage ↔ log-coverage (PTY seam wrapper)**: after the bump, spawn must still give `process-start{subject:"claude-child"}` and exit must still come from the handle-wait thread with `kill-fallback` logged (obs-plan fct-3epowryi) (obs-plan/span-trace-coverage fct-mw7zqeom).
@binding n=4
**span-trace-coverage ↔ slo-invariants-telemetry-budgets and the test-plan / matrix**: the `peer_admitted` survivor at the three client seams (obs-plan/span-trace-coverage fct-jvpve25b) must keep matrix#v1-44's dated note and test-plan §1's untestable zone, while any other surviving obs mutant stays a build failure (obs-plan fct-o5c3uxju).
@binding n=5
**span-trace-coverage ↔ design-system §Surface: cli**: the founder's wording of the socket-directory pair (obs-plan/span-trace-coverage fct-ucfyu5c3) must land in both masters, with the log lines' `socket-dir-refused` code kept (obs-plan/span-trace-coverage fct-zklcva6d).
@binding n=6
**span-trace-coverage ↔ the live sessions**: v1-53's live `viola run` and the dialog hold read take their evidence from the start chain and hook lines (obs-plan/span-trace-coverage fct-e6vtcesf) (obs-plan/span-trace-coverage fct-kreejnxq), read through `agent-run logs` / `status` and `jq`, never by human review (obs-plan fct-h4xijqlt).
