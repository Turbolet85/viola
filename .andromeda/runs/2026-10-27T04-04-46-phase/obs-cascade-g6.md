# obs · cascade read · group 6 of 10

## Carried
- (.andromeda/obs-plan-detail/span-trace-coverage.md:18) The chunk re-verifies the paste rule at the CLI `send` client, so its client-side refusal must stay the one `send-refused{side:"client"}` line with the chain only in the detail file.
  > Client-side `send-refused{side:"client"}`. The full anyhow chain goes to `detail-cli.ndjson` only
- (.andromeda/obs-plan-detail/span-trace-coverage.md:19) The vt100 panic degrade the chunk re-verifies is observed through `run.readiness_gate`, and the bound on the child's PTY output into the feed lands on the `pty.*` seam, so both must keep these manual spans and add none per chunk.
  > Manual spans on the start sequence, `pty.*` seam, `channel.dispatch`, `run.readiness_gate` (vt100 has no tracing), `run.wheel_transition`, `run.budget_eval`, `state.*`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:20) The hook's stdin bound and the names-before-path-joins rule meet here: an instance that does not resolve from a valid `VIOLA_NAME` must exit 0 with no stdin read and no line, and an oversize stdin must fail open inside `hook.handle` with `catch_unwind` keeping exit 0.
  > `hook.handle` › `channel.request` › `hook.decision_emit`. `hook-invoked` / `hook-decision` (except the hidden capture arms, `hook <event> --capture` and `hook statusline --capture`: no obs init, no lines, §4 Edge flows; and except a refused or absent instance on the event and dialog paths: the instance check, `canonical` then `trusted`, runs before `read_diagnostics_level` and obs init, and a refused check, or an instance that does not resolve (no valid `VIOLA_NAME`, or a `VIOLA_DIR` naming no existing instance directory), exits 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append). `std::panic::catch_unwind` around dispatch keeps exit 0.
  (.andromeda/obs-plan.md:385)
  > the user's stdout passes through unchanged and is never logged;
  (.andromeda/obs-plan.md:310)
  > (nothing when no command is recorded, on a refused instance, on oversize or malformed stdin, or under the hidden `--capture` arm, which prints nothing)
- (.andromeda/obs-plan-detail/span-trace-coverage.md:21) Channel frames on both sides are a boundary the chunk bounds, so an over-size or invalid frame's refusal must stay inside these spans with lines that never record `params` or result bodies.
  > `channel.request` (client) / `channel.dispatch` (server) spans. `channel-request` / `channel-response` with `method`, `corr`, `conn`, `result_class`, `error_code`, `sender`. Never `params` or result bodies
  (.andromeda/obs-plan.md:333)
  > channel lines never record `params` or result bodies.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:22) MCP stdin and tool arguments are bounded and validated by the chunk, and MCP `send` is a client where the paste rule is re-verified, so every reject must be codes-only inside `mcp.tool_call` with rmcp's own target kept OFF.
  > rmcp 3.4.1 emits native `tracing`, but its target is OFF in the sink (records lack required fields and can carry frame content) | `mcp.tool_call` span per tool (`send · wait · last · answer · list`). Codes-only `send-refused` / `channel-response` on rejects.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:30) A CLI positional name reaches `instances/<name>` through this check, so the chunk's name witness and its client paste witness must keep it ahead of any role file, with a refusal writing no line.
  > the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) run `client::refuse_untrusted`, the instance check `link` / `unlink` share, on their named instance before any role file opens; a refused check (exit 21, the instance pair on stderr, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`) opens no process log and writes no line.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:31) The hook `connect` reads `snapshot.json`, one of the files the chunk bounds, so that bounded read must keep its `state.snapshot_recover` span under `hook.handle` and go on only over a snapshot read whole.
  > (`read_snapshot_or_replay`, its span `state.snapshot_recover` under `hook.handle`, then `connect_by`, then `verify_server`) before any frame. Only a snapshot read whole goes on: a replayed snapshot (it carries no `endpoint`, `pid` or `started_at`) or an absent one is unreachable, so no frame is sent and the hook fails open with `hook-decision{detail:"channel-unreachable"}`, a SessionEnd taking the direct `append_session_end`.
  (.andromeda/obs-plan.md:394)
  > - **E5** (snapshot corruption): `state-recovered{detail:"snapshot-replayed"|"snapshot-unsupported-v"}` at WARN, written by `viola_state::replay::read_snapshot_or_replay`, one line per call;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:32) An MCP tool's `name` argument reaches `instances/<name>` and the chunk re-asserts that a rejected call is codes-only and that `instance-unreachable` writes no line, so this order and silence must hold.
  > the frame tools `send`, `wait`, `last` and `answer` first run `check_instance` on their instance, before `live_endpoint` (a refusal returns `instance-unreachable` with `detail: null`, no frame and no line)
- (.andromeda/obs-plan-detail/span-trace-coverage.md:40) The start's readers the chunk bounds (the `version-probe` output, the snapshot, the statusline source and settings files, the `events.ndjson` restore) must keep this span chain, and the steps that carry no span and no line must stay silent once bounded.
  > `run.start` (INTERNAL, root) › `state.snapshot_recover` (the start's one snapshot read, `read_snapshot_or_replay`, which the collision check takes) › `run.collision_check` › `run.pin_copy` › `run.conpty_sideload` (Windows) › `run.version_gate` (spawns the `version-probe`) › `channel.bind` › `run.budget_eval` (INTERNAL, `trigger="start"`; its attributes are under Scenario: Budget governor; chunk 2026-10-10-budget-governor) › `state.snapshot_write` › `state.heartbeat_start` › `pty.spawn` (CLIENT). Two start steps carry no span and no log line, by design (the operator's answer at the chunk 2026-10-10-statusline-pass-through wrap; that chunk's plan added no span name): the statusline source read, whose result is the snapshot's `statusline_command`, and the settings override write (`instances/<name>/settings.json` through `replace_private`, Unix only), which sits between the first snapshot and `pty.spawn`. The child's `--settings` argument, the source path and the command appear in no line or span. A third start step carries none either, as landed (chunk 2026-10-10-session-links): the link set's restore, one read pass over the instance's `events.ndjson` through `viola_state::events::read_from`, made beside the wait feed's rebuild and before the endpoint serves. It appends nothing and logs nothing,
- (.andromeda/obs-plan-detail/span-trace-coverage.md:46) The ledger read inside `run.version_gate` is a bounded reader the chunk covers, so an unreadable, oversize or refused `ledger/stamps.json` must read as `cli_verified:false`, and the bounded `claude --version` spawn must keep names and counts only.
  > `run.version_gate`: `cli_version`, `cli_verified`;
  (.andromeda/obs-plan-detail/span-trace-coverage.md:49)
  > `pty.spawn`: `pty_backend` (`conpty-sideload|conpty|openpty`, from `viola_pty::pty_backend()`; `conpty-sideload` after a successful pre-load), `env_stripped_count`.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:50)
  > Never the endpoint name, `pinned_bin` path or env values.
  (.andromeda/obs-plan.md:398)
  > - **Capability ledger** (`ledger/stamps.json`): read inside `run.version_gate`; a missing stamp, or an unreadable, malformed or strict-modes-refused ledger, counts as unverified (`cli_verified:false`); the path and stamp contents never appear.
  (.andromeda/obs-plan.md:390)
  > names only, never values, and no env map is passed to a span or event.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:54) The `claude --version` and `claude agents --json` spawns are readers the chunk bounds, so each must keep its one `process-start` / `process-exit` pair and an oversize or malformed agents answer exactly one `parse-rejected{parser:"claude-agents-json"}`, passing on a failed read.
  > `process-start` / `process-exit{subject:"version-probe", child_exit_status, duration_ms}`.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:55)
  > its `session-live` reading (skipped under `--fork`) writes one `process-start` / `process-exit{subject:"agents-probe"}` pair in `run-<name>.ndjson`, plus one `parse-rejected{parser:"claude-agents-json"}` on an oversize or malformed answer, and passes on a failed read;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:61) The chunk bounds the child's PTY output into the vt100 feed, so that bound must never turn a cap into a reader EOF that ends the child's exit path, which stays the handle-wait thread's.
  > The handle-wait thread emits `process-exit{subject:"claude-child", child_exit_status, exit_source:"handle-wait"|"kill-fallback"}`, never on reader EOF (ConPTY chaos class).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:75) The paste rule is re-verified on this path, so the client refusal must come before `channel.request` and the wrapper's before `run.readiness_gate`, and CLI `send`'s text read must precede its instance check.
  > `send.client` (CLIENT; `cli` or `mcp.tool_call`) › `channel.request` (CLIENT) → `channel.dispatch` (SERVER, `run`) › `run.readiness_gate` › `pty.paste_write` › `run.confirm_window`.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:76)
  > The instance check comes first: CLI `send` runs `client::refuse_untrusted` after its text read and before any role file opens, and MCP `send` runs `check_instance` before `live_endpoint`; a refusal sends no frame and writes no line.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:79) The vt100 panic degrade witness reads `run.readiness_gate`'s `outcome` of `input-not-ready` with `vt100_panicked` true, and the paste's only trace on the wrapper is `text_bytes` of the typed text, never the text.
  > `run.readiness_gate`: `outcome` (`ready|input-not-ready`), `vt100_panicked` (bool);
  (.andromeda/obs-plan-detail/span-trace-coverage.md:80)
  > `pty.paste_write`: `text_bytes` (the length of the typed text: the sent text with every CR LF pair and every other CR as one LF, and without its trailing CR and LF characters), `paste_mode="bracketed"`;
  (.andromeda/obs-plan-detail/span-trace-coverage.md:82)
  > Never text.
  (.andromeda/obs-plan.md:353)
  > - Never the sent text, only its `text_bytes`;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:89) This is the line set the chunk re-verifies on each deciding side: one codes-only `send-refused` per side for `control-character` and `empty-text`, the client's before any frame, the wrapper's ahead of both wheel reads with no `send-issued`.
  > client-side validation refusal, made before any frame: `send-refused{corr:null, refusal:"not-delivered", detail:"control-character"|"empty-text", side:"client"}`, with a mirror line on the wrapper side if the frame reached it. `empty-text` is an empty typed text, read after `control-character` and ahead of both wheel reads; the wrapper, deciding it on a frame sent straight to it, writes its own `send-refused{side:"wrapper"}` line with the same pair and `corr`, `conn` and `rpc_id`, and no `send-issued`. One line per deciding side, never the text;
  (.andromeda/obs-plan-detail/span-trace-coverage.md:92)
  > every wrapper `send-*` line (here and in Scenarios 5 and 6) also carries `conn` (or `srv_conn`) and `rpc_id`, so `(conn, rpc_id)` on the send line equals `(conn, corr)` on this call's `channel-*` lines (D-30). The client-side `send-refused{side:"client"}` carries neither.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:93) A paste refusal on the CLI client must close `send.client` with its exit code 13 in `process-exit`, and a send its instance check refuses must open no span and write no line.
  > `send.client` closes with `process-exit{subject:"self", exit_code}` (0 / 10 / 11 / 12 / 13 / 14 / 20 / 21, or 1 `internal-error` on a reply with neither a parseable `result` nor an `error.code`);
  (.andromeda/obs-plan-detail/span-trace-coverage.md:94)
  > A `send` its instance check refuses (`client::refuse_untrusted`: exit 21, the instance pair, or under `--json` the `instance-unreachable` document) is refused before any role file opens: no `send.client` span, no `process-exit` and no line.

## Bindings
- **span-trace-coverage ↔ log-coverage**: the `send-refused` pair (`control-character` / `empty-text`) per deciding side and the `parse-rejected{parser:"claude-agents-json"}` on an oversize agents read are §6 catalog values the spans carry, (.andromeda/obs-plan-detail/span-trace-coverage.md:89) and (.andromeda/obs-plan-detail/span-trace-coverage.md:55).
- **span-trace-coverage ↔ error-capture-reporting**: the vt100 panic degrade is read on `run.readiness_gate`'s `vt100_panicked` while the forced panic still fires the panic hook's one line under the chaos-feed exemption, and the hook's `catch_unwind` keeps exit 0, (.andromeda/obs-plan-detail/span-trace-coverage.md:79) and (.andromeda/obs-plan-detail/span-trace-coverage.md:20).
- **span-trace-coverage ↔ pii-scrubbing-compliance**: no span or line carries the paste text, channel `params`, rmcp frame content or env values; only `text_bytes` and counts cross, (.andromeda/obs-plan-detail/span-trace-coverage.md:82), (.andromeda/obs-plan-detail/span-trace-coverage.md:21) and (.andromeda/obs-plan-detail/span-trace-coverage.md:50).
- **span-trace-coverage ↔ obs-anti-patterns (body §11)**: the PTY output bound adds no per-chunk span or event to the pump or the vt100 feed, (.andromeda/obs-plan-detail/span-trace-coverage.md:19) and (.andromeda/obs-plan.md:682).
- **span-trace-coverage ↔ the security plan's input validation**: the order text read → instance check → client paste refusal → frame → wrapper refusal before the wheel reads is the paste rule's both-sides placement, (.andromeda/obs-plan-detail/span-trace-coverage.md:76).
