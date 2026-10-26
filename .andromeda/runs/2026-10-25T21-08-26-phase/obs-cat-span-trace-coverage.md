# obs · Span / Trace Coverage — category read

## Carried
- (.andromeda/obs-plan-detail/span-trace-coverage.md:30) The start sequence's must-trace span chain must stay whole with `run.collision_check` and `channel.bind` in place when the Windows listener's admission (DACL, remote rejection, first-instance) is asserted or changed.
  > `run.start` (INTERNAL, root) › `run.collision_check` › `run.pin_copy` › `run.conpty_sideload` (Windows) › `run.version_gate` (spawns the `version-probe`) › `channel.bind` ›
- (.andromeda/obs-plan-detail/span-trace-coverage.md:33) The squatted-name start refusal must keep `run.collision_check`'s closed `outcome` value `squatted-name`; no new outcome word is added for the first-instance assertion.
  > `run.collision_check`: `outcome` (`free|already-live|squatted-name`) — a `live` or `stale` holder is `already-live`; a `gone` one is `free` (taken over, its `events.ndjson` appended to);
- (.andromeda/obs-plan-detail/span-trace-coverage.md:37) The pipe listener's bind span must carry only the closed `endpoint_kind`; the Windows leg records `named-pipe`.
  > `channel.bind`: `endpoint_kind` (`named-pipe|unix-socket`);
- (.andromeda/obs-plan-detail/span-trace-coverage.md:40) No span added or touched for the pipe's DACL, remote-client mode or first-instance check may record the endpoint (pipe) name, a path or an env value.
  > Never the endpoint name, `pinned_bin` path or env values.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:43) The `run` self `process-start` must keep `endpoint_kind` as its endpoint field (a kind, never a name).
  > `process-start{subject:"self", service_name, version, os, pid, endpoint_kind}`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:45) A squatted pipe name must end the start as `process-exit{subject:"self", exit_code:1}` with the closed `detail` `squatted-name`, no path or pid, and no new detail code.
  > On failure, `process-exit{subject:"self", exit_code:1, detail}` with `detail` ∈ `already-live|squatted-name|pinned-hash-mismatch|batch-script-child` (Founder Direction 4; no path or pid).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:21) The `viola-channel` listener work must stay manual-instrumented through its own spans and codes-only channel lines; interprocess emits nothing, so any admission failure is observed at viola's seam, never in `params` or result bodies.
  > | ipc-internal (viola-channel) | none (interprocess 2.4.4 emits no `log`/`tracing`) | `channel.request` (client) / `channel.dispatch` (server) spans. `channel-request` / `channel-response` with `method`, `corr`, `conn`, `result_class`, `error_code`, `sender`. Never `params` or result bodies |
- (.andromeda/obs-plan-detail/span-trace-coverage.md:146) The CARRY case `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 asserts this wrapper line; fixing its log capture must keep the line's fields as stated here and the client exit 11.
  > `send-refused{corr:<cursor>, refusal:"budget-paused", detail:"five-hour"|"seven-day", window, used_percentage, resets_at, paused:true}` with client `exit_code:11`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:74) The budget-paused `send-refused` line the CARRY case captures is a wrapper `send-*` line of Scenario 6 and must also carry `conn` (or `srv_conn`) and `rpc_id`.
  > every wrapper `send-*` line (here and in Scenarios 5 and 6) also carries `conn` (or `srv_conn`) and `rpc_id`, so `(conn, rpc_id)` on the send line equals `(conn, corr)` on this call's `channel-*` lines (D-30).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:137) The send-arrival budget evaluation behind the CARRY case must keep its span shape: declared once on `BudgetGate::decide_at`, `skip_all`, with `trigger`, `paused` and `window` only when paused.
  > `run.budget_eval`: `trigger` (`start|send|turn-ended`), `paused`, and `window`, recorded only when paused. As landed (chunk 2026-10-10-budget-governor) it is declared once, on `BudgetGate::decide_at`, `skip_all`, and opened once per evaluation: at start, at a `send`'s arrival, and after a `turn-ended` hook line is appended.
- (.andromeda/obs-plan.md:327) Every span the chunk adds or touches in `viola-channel` (`server.rs`, `server/win.rs`, `endpoint.rs`) or `src/cmd/run.rs` must have a static `<area>.<operation>` name and an explicit field allow-list, with `err` / `ret` banned.
  > Names are static strings; instance names, dialog ids and paths never appear in a span name. Every span is declared with `#[instrument(skip_all, name = "...", fields(...))]` or `info_span!` with an explicit field allow-list, and the `err` / `ret` options are banned (Section 11).
- (.andromeda/obs-plan.md:345) The start scenario's bans hold for the Windows admission work: no endpoint name in a span, and a failure's `detail` carries no path or pid.
  > - Never the endpoint name, the `pinned_bin` path or env values in a span, and never the sideload companions' paths or hashes;
- (.andromeda/obs-plan.md:683) The start sequence is a must-trace path; the admission changes must not drop any of its spans or events.
  > - NEVER skip spans or events on the 6 must-trace paths (Section 4). The agent loses its ability to debug.
- (.andromeda/obs-plan.md:691) A function handling the pipe's security descriptor, SID or handle must not carry a bare `#[instrument]`.
  > - NEVER use a bare `#[instrument]`. It records every argument with `Debug` and leaks send text, `hook.dialog` payloads and tool `input`. Always use `#[instrument(skip_all, fields(...))]`.
- (.andromeda/obs-plan.md:692) A listener-creation or DACL read-back function that returns an error must not be instrumented with `err` or `ret`.
  > - NEVER use `#[instrument(err)]` or `#[instrument(ret)]` on functions whose error or return can carry a serde / serde_path_to_error source or user content.
- (.andromeda/obs-plan.md:693) The pipe name is a path-like, instance-bearing value and must never appear in a span name.
  > - NEVER use high-cardinality span names: no instance names, dialog ids or paths in span names.

## Bindings
- **`run.collision_check` `squatted-name` / `process-exit` detail ↔ security-plan §Authentication & Authorization (Windows pipe first-instance)**: the first-instance property the chunk asserts on the listener surfaces as the existing closed outcome and detail `squatted-name`, exit 1, unchanged (.andromeda/obs-plan-detail/span-trace-coverage.md:33), (.andromeda/obs-plan-detail/span-trace-coverage.md:45)
- **`channel.bind` / `process-start` `endpoint_kind` ↔ obs Log Coverage (§6 field catalog) and `schemas/diag-line.v1.json` (G4)**: only the kind crosses into a span or line, never the pipe name (.andromeda/obs-plan-detail/span-trace-coverage.md:37), (.andromeda/obs-plan-detail/span-trace-coverage.md:43), (.andromeda/obs-plan-detail/span-trace-coverage.md:40)
- **viola-channel manual spans ↔ obs Log Coverage per-module levels (interprocess target OFF)**: listener admission failures are observable only at viola's seam (.andromeda/obs-plan-detail/span-trace-coverage.md:21)
- **budget-paused `send-refused` ↔ test-plan (the CARRY case's log capture under libtest threads)**: the case's fix asserts this line's fields, `conn` / `rpc_id` and exit 11 (.andromeda/obs-plan-detail/span-trace-coverage.md:146), (.andromeda/obs-plan-detail/span-trace-coverage.md:74), (.andromeda/obs-plan-detail/span-trace-coverage.md:137)
- **§4 span declaration rule ↔ §9 gate G1 (bare `#[instrument]`)**: any new span in the chunk's surfaces passes G1 (.andromeda/obs-plan.md:327), (.andromeda/obs-plan.md:691)
