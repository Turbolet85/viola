# obs · cascade read · group 4 of 5

## Carried
- (.andromeda/obs-plan-detail/span-trace-coverage.md:30) The start chain must keep `run.collision_check` and `channel.bind` as the steps of `run.start`: the chunk re-asserts the squatted-name refusal in the first, edits `refuse_squatted` and asserts the listener behind the second, and any span it adds must follow the static-name, allow-listed `#[instrument]` form.
  > - **Must-trace spans:** `run.start` (INTERNAL, root) › `run.collision_check` › `run.pin_copy` › `run.conpty_sideload` (Windows) › `run.version_gate` (spawns the `version-probe`) › `channel.bind`
  (.andromeda/obs-plan-detail/span-trace-coverage.md:19)
  > | cli (`viola run`) | none (portable-pty 0.8.1 logs through `log`; its target is OFF and failures are captured at the seam, D-11) | Manual spans on the start sequence,
  (.andromeda/obs-plan.md:327)
  > Every span is declared with `#[instrument(skip_all, name = "...", fields(...))]` or `info_span!` with an explicit field allow-list, and the `err` / `ret` options are banned (Section 11).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:33) The re-asserted squat refusal must still record `outcome:"squatted-name"` on `run.collision_check`, and `channel.bind` must still record only the closed `endpoint_kind` (`named-pipe` on the Windows leg), never the pipe name, in any span or line the chunk's code or tests produce.
  > - `run.collision_check`: `outcome` (`free|already-live|squatted-name`) — a `live` or `stale` holder is `already-live`; a `gone` one is `free` (taken over, its `events.ndjson` appended to);
  (.andromeda/obs-plan-detail/span-trace-coverage.md:37)
  > - `channel.bind`: `endpoint_kind` (`named-pipe|unix-socket`);
  (.andromeda/obs-plan-detail/span-trace-coverage.md:40)
  > - Never the endpoint name, `pinned_bin` path or env values.
  (.andromeda/obs-plan.md:345)
  > - Never the endpoint name, the `pinned_bin` path or env values in a span, and never the sideload companions' paths or hashes; the child's `--settings` argument, the statusline source path and the command appear in no line or span; a sideload outcome is never a refusal or an exit change; a failure's `detail` carries no path or pid;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:45) A squatted endpoint must keep ending the start with one ERROR-level `process-exit{subject:"self", exit_code:1, detail:"squatted-name"}` that carries no path or pid. The detail stays inside the closed catalogue the schema enumerates, so neither a remote-client nor a first-instance refusal may add a detail value outside the plan. The refused start's `process-start` keeps `endpoint_kind` and never the name, and the terminal gets only the two fixed stderr lines.
  > - On failure, `process-exit{subject:"self", exit_code:1, detail}` with `detail` ∈ `already-live|squatted-name|pinned-hash-mismatch|batch-script-child` (Founder Direction 4; no path or pid).
  (.andromeda/obs-plan-detail/span-trace-coverage.md:43)
  > - `process-start{subject:"self", service_name, version, os, pid, endpoint_kind}`.
  (.andromeda/obs-plan-detail/log-coverage.md:10)
  > | `process-start` | `subject` (`self|claude-child|version-probe|verify-probe|verify-pty-probe|agents-probe|statusline-shell`), `service_name`, `version`, `os`, `pid`, `child_pid`, `endpoint_kind`,
  (.andromeda/obs-plan-detail/log-coverage.md:11)
  > | `process-exit` | `subject`, `exit_code` (self), `child_exit_status` (child/probes), `shell_exit_status` (statusline), `exit_source` (`handle-wait|kill-fallback`), `detail`, `during` (`connect|call`), `duration_ms` |
  (.andromeda/obs-plan-detail/log-coverage.md:26)
  > `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, `internal-error`,
  (.andromeda/obs-plan-detail/log-coverage.md:27)
  > `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 13 values (the enum's members; 12 before chunk 2026-10-16-unwrapped-sessions-on-the-board, 11 before chunk 2026-10-13-interrupted-verify-cleanup, 9 before chunk 2026-10-10-viola-revive).
  (.andromeda/obs-plan.md:459)
  > | `error` | `panic`; `process-exit` with `exit_code` 1 or 20; `channel-response` with `error_code` -32603 |
  (.andromeda/obs-plan.md:311)
  > a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint) writes exactly two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid) through `src/human.rs` `refuse` → `write_refusal`
- (.andromeda/obs-plan-detail/span-trace-coverage.md:21) interprocess emits no logging, so a listener failure the chunk provokes (a remote client or a second first-instance listener) must be captured at viola's own seam as a viola event, and any new or changed `ChannelError` must keep a fixed `Display` and end in a codes-only logged line.
  > | ipc-internal (viola-channel) | none (interprocess 2.4.4 emits no `log`/`tracing`) | `channel.request` (client) / `channel.dispatch` (server) spans. `channel-request` / `channel-response` with `method`, `corr`, `conn`, `result_class`, `error_code`, `sender`. Never `params` or result bodies |
  (.andromeda/obs-plan.md:470)
  > | `rmcp`, `portable_pty`, `sysinfo`, `interprocess` | `OFF` (D-11; failures are captured at the viola seam as viola events) |
  (.andromeda/obs-plan.md:496)
  > The mechanism is typed errors with fixed-message `Display` impls — hand-written on `PtyError` (`viola-pty` takes no thiserror), thiserror 2.0.20 on `ChannelError`,
  (.andromeda/obs-plan.md:509)
  > 1. fixed `Display` strings on every thiserror type (no fields holding paths or payloads);
  (.andromeda/obs-plan.md:533)
  > - Module-boundary errors (Standard+): every typed error that crosses a crate seam ends in a logged `process-exit{detail}`, `channel-response{result_class:"error", error_code}`, `hook-decision{detail}` or `send-refused`. Each carries codes only, at `WARN`/`ERROR` per Section 6.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:146) The CARRY's case_4 reads the budget-paused `send-refused` line, so the capture fix must still see this line with its window word and paused fields. It must also see the wrapper's `conn`/`rpc_id` pair and no cursor. The capture must work through the code-set level filter, never through `RUST_LOG` or a stdout or stderr sink.
  > - `send-refused{corr:<cursor>, refusal:"budget-paused", detail:"five-hour"|"seven-day", window, used_percentage, resets_at, paused:true}` with client `exit_code:11`.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:74)
  > - every wrapper `send-*` line (here and in Scenarios 5 and 6) also carries `conn` (or `srv_conn`) and `rpc_id`, so `(conn, rpc_id)` on the send line equals `(conn, corr)` on this call's `channel-*` lines (D-30). The client-side `send-refused{side:"client"}` carries neither.
  (.andromeda/obs-plan-detail/log-coverage.md:14)
  > | `send-issued` / `send-confirmed` / `send-refused` | `from`, `from_trust`, `text_bytes` (the typed text's length: the sent text with every CR LF pair and every other CR as one LF, and without its trailing CR and LF characters), `confirmed`, `refusal`, `detail`, `side` (`client|wrapper`), `wheel`, `window`, `used_percentage`, `resets_at`, `paused`, `duration_ms`, `conn` / `srv_conn` and `rpc_id` (wrapper side only: the originating `send` request's connection and JSON-RPC `id`, D-30) |
  (.andromeda/obs-plan-detail/log-coverage.md:32)
  > - Refusal details are the arch `RefusalReason` details verbatim: `manual-pause`, `five-hour`, `seven-day`,
  (.andromeda/obs-plan.md:462)
  > Enabled only by `config.json` `diagnostics_level:"debug"`, never by `RUST_LOG`.
  (.andromeda/obs-plan.md:476)
  > - Agent-mode flag: none. JSON-to-file is the only mode, and there is no stdout pretty sink on any role.
- (.andromeda/obs-plan-detail/metric-coverage.md:23) The send-outcome counter is computed from the `send-refused{refusal,detail,side}` line that case_4 re-asserts, so the CARRY's fix must leave that line's `refusal`, `detail` and `side` keys, which the counter groups by, unchanged on the home-level role file.
  > | Send terminal state (design readback words `read back` / `unable` / `unconfirmable`) | `viola.send.outcome` ← count of `send-confirmed{confirmed}` / `send-refused{refusal,detail,side}` | counter | 1 | n/a |
  (.andromeda/obs-plan-detail/metric-coverage.md:10)
  > | Counter | Occurrences of an `event` / `detail` / `refusal` value | `awk 1 diagnostics/*.ndjson | jq -R -n '[inputs|fromjson?|select(.event=="send-refused")]|group_by([.side,.detail])|map({side:.[0].side,d:.[0].detail,n:length})'`, over home-level role files only (detail files repeat lines; a torn line is skipped as in G2) |
- (.andromeda/obs-plan-detail/metric-coverage.md:30) Panics must stay at 0 over the homes of the re-asserted start-refusal tests and any new Windows listener test (G2 on the Windows leg). A panic in that code must leave exactly one `event:"panic"` line carrying only its location and thread, written outside the tracing subscriber.
  > | Panics (SLO) | `viola.panic` ← `event:"panic"` | counter | 1 | n/a (must be 0) |
  (.andromeda/obs-plan-detail/log-coverage.md:17)
  > | `panic` | `panic_location` (workspace-relative, or `<crate>-<ver>/src/...` for dependencies), `thread`. Payload and backtrace go to `detail-<process>.ndjson` only |
  (.andromeda/obs-plan.md:515)
  > It writes **one** JSON line via a single `write_all` on the pre-opened `Arc<File>` held in a `OnceLock`, **not** through the tracing subscriber

## Bindings
- **Span (collision check, channel bind) ↔ Log (exit-1 detail catalogue) ↔ the body's level map**: the squat refusal recorded as `run.collision_check` `outcome:"squatted-name"` must be the same closed word as the ERROR-level `process-exit` `detail`, inside the schema's 13-value enum, with no path, pid or pipe name on either side. (.andromeda/obs-plan-detail/span-trace-coverage.md:33) ↔ (.andromeda/obs-plan-detail/log-coverage.md:26) ↔ (.andromeda/obs-plan.md:459)
- **Span (channel surface) ↔ Log (Targets table) ↔ Error capture (module-boundary errors)**: because interprocess is silent and its target is OFF, a remote-client or first-instance failure is visible only as a viola-seam line built from a fixed-`Display` `ChannelError` that carries codes only. (.andromeda/obs-plan-detail/span-trace-coverage.md:21) ↔ (.andromeda/obs-plan.md:470) ↔ (.andromeda/obs-plan.md:533)
- **Span (budget send refusal) ↔ Log (send-* field set, refusal details) ↔ Metric (send outcome counter)**: the line case_4 reads is the line the counter groups by `side` and `detail`, so its shape must stay as both sections state it. The capture fix uses no env level and no stdout or stderr sink. (.andromeda/obs-plan-detail/span-trace-coverage.md:146) ↔ (.andromeda/obs-plan-detail/log-coverage.md:14) ↔ (.andromeda/obs-plan-detail/metric-coverage.md:23) ↔ (.andromeda/obs-plan.md:462)
- **Metric (panics = 0) ↔ Log (panic fields) ↔ Error capture (panic hook)**: any panic in the chunk's tests must surface as the hook's one line with the panic field set, and the G2 count over the chunk's homes must stay 0. (.andromeda/obs-plan-detail/metric-coverage.md:30) ↔ (.andromeda/obs-plan-detail/log-coverage.md:17) ↔ (.andromeda/obs-plan.md:515)
