# obs · Telemetry Strategy — category read

## Carried
- (.andromeda/obs-plan-detail/telemetry-strategy.md:10) The served `link` / `unlink` methods sit on the `viola-channel` seam, so the verbs' client call and the wrapper's dispatch of both methods must be inside spans.
  > The 6 must-trace paths (Section 4) and every crate seam: `viola-pty` spawn/write/wait/kill, `viola-channel` client call and server dispatch, `viola-state` snapshot/heartbeat/tail, `viola-agent-claude` hook parse, `viola-mcp` tool calls
- (.andromeda/obs-plan-detail/telemetry-strategy.md:10) Every new span on the verbs and methods must use the `skip_all` + static `<area>.<operation>` form, and a link call is joined across processes by `corr` + `conn`, never by a trace id.
  > Declared in code with tracing 0.1.44 `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`. They are **not serialized** in v1, because the output sets `with_current_span(false)` + `with_span_list(false)`. They give the code its structure and are ready for the deferred tracing-opentelemetry 0.34.0 bridge. Cross-process joins use `corr` + `conn`, not W3C trace IDs
- (.andromeda/obs-plan-detail/telemetry-strategy.md:11) The verbs' boundary calls, the link-set state transition and every refusal they make must be logged as closed `event` values in the per-role file, which is the primary signal.
  > Every boundary call, state transition, refusal, fault/heal and process lifecycle, as the closed `event` enum (Section 3) | **The primary signal.** tracing-subscriber 0.3.23 JSON-per-line, one file per process role under `<home>/diagnostics/`, merged with `events.ndjson` by `agent-run logs`
- (.andromeda/obs-plan-detail/telemetry-strategy.md:12) A failing `viola link` / `viola unlink` must end in an exit code with a distinct `detail` code on `process-exit`, its errors typed with fixed `Display`, inside the main-thread catch site.
  > Panics (a one-line `event:"panic"` record), typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit` | No external reporter: security says "no outbound network calls". A hand-written `std::panic::set_hook`, plus `std::panic::catch_unwind` at the main-thread catch site of every role
- (.andromeda/obs-plan-detail/telemetry-strategy.md:14) The `link` / `unlink` lines the methods append to `events.ndjson` are the audit record; their payload (driver, driven, since) must not be copied into any process-log line.
  > `instances/<name>/events.ndjson` is the product audit stream (arch-owned, never rotated) | Not a compliance trail (security Minimal, no compliance triggers). Obs never copies event payloads into process logs
- (.andromeda/obs-plan-detail/telemetry-strategy.md:20) The verbs fall under the panic invariant after init step 4; if the P4 form puts an instance check before any role file opens (the `viola revive` shape), a refused check opens no file and must be added to the bounded exemptions this line and Section 10 list.
  > Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance. The bounded exemptions (the pre-init window, `cli` without an instance, `hook` without `VIOLA_NAME`, a failed step-4 role-file open, the no-obs-init hidden capture arms (`hook <event> --capture` and `hook statusline --capture`), and a refused instance check on either `viola revive` arm (start or `--list`) or on `viola hook statusline`, or a refused `viola list` scan, which opens none either) are listed in Section 10.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:28) Two new channel methods fall under the IPC logging-sensitive trigger: their lines must stay within the §6 channel field allow-list.
  > logging-sensitive Vector 1 (IPC) | §4 surface table (channel), §6 channel field allow-list, §8 table
- (.andromeda/obs-plan-detail/telemetry-strategy.md:33) If the P4 form takes the strict-modes instance check for the verbs, its refusal must use the catalog's `strict-modes-failed` code, never a new one.
  > §6 detail catalog Filesystem refusals (`strict-modes-failed`, `pinned-hash-mismatch`, `symlink-ignored` via `parse-rejected{parser:"state-entry"}`)
- (.andromeda/obs-plan-detail/telemetry-strategy.md:36) A verb-to-wrapper `link` call is a cross-surface call and must carry `corr` + `conn` correlation.
  > cross-surface-trace-propagation | §3 Trace context propagation (`corr` + `conn`), §4 Scenarios 2/4
- (.andromeda/obs-plan-detail/telemetry-strategy.md:43) Each refusal cause of the new verbs must map to the §6 detail code catalog (Founder Direction 4).
  > creator-explicit (Founder Direction 4) | §6 detail code catalog, §12 D-01
- (.andromeda/obs-plan.md:109) `link` and `unlink` are already named on the short-lived cli surface; the verbs must log as that surface does.
  > - **Surface:** cli (short-lived verbs: `send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install · revive --list`)
- (.andromeda/obs-plan.md:111) The verbs' sink is the file only, their instance taken from an argument (the driven one) or `VIOLA_NAME`, never the cwd; stdout carries only `--json`, and stderr refusals are fixed strings.
  > a file sink only, JSON-per-line to `diagnostics/` when an instance resolves — no network OTLP (viola makes no outbound network calls) and no stdout exporter (stdout carries `--json` results); the instance comes from `VIOLA_NAME` or an argument, never from the working directory; refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text.
- (.andromeda/obs-plan.md:123) The `link` / `unlink` methods' `-32603` must be the fixed form, and their `from` / driver must never be logged as an authenticated identity.
  > `-32603` is `"internal error"` with `data:null`, and the self-reported `from` is never logged as an authenticated identity.
- (.andromeda/obs-plan.md:183) Test-plan E3's `link` / `unlink` transfer markers are owed with the verbs and are covered by the triggers plus any E2E tracing expansion.
  > Tests E1 (unwrapped hook no-op), E2 (R8 env strip), E3 (`link` / `unlink` transfer markers) and E5 (snapshot corruption → replay) are covered by the section 5 triggers and by E2E tracing expansion in Phase 3 if needed.
- (.andromeda/obs-plan.md:190) The new methods' channel lines must carry no `params` (driver, driven) or result body (`since`, `already_linked`).
  > channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
- (.andromeda/obs-plan.md:215) As written, a strict-modes failure is logged only in `run`'s version gate; a verb-side instance check (the second P4 form) must either write no line or amend this trigger.
  > a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate.
- (.andromeda/obs-plan.md:253) Each exit-21 / exit-1 cause of the verbs must carry its own distinct `detail` code with no path or pid.
  > each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid.
- (.andromeda/obs-plan.md:264) The channel join for a `link` call is `(conn, corr)`, the verb process and the wrapper each logging its own `instance`.
  > the channel join key (D-25: `(conn, corr)`, each side logs its own `instance`)
- (.andromeda/obs-plan.md:280) The §2 broad line the chunk must honour: logs primary, spans unserialized, `corr` + `conn` joins, no event payload in process logs.
  > Structured logs are the primary signal, one JSON-per-line file per process role; spans are declared in code but not serialized in v1, and cross-process joins use `corr` + `conn`; errors have no external reporter; metrics have no in-process instruments and no exporter and are derived by `jq` over the ndjson; `events.ndjson` is the product audit stream, not a compliance trail, and obs never copies event payloads into process logs.
- (.andromeda/obs-plan.md:290) The verbs' calls are cross-surface and must carry `corr` plus `conn`.
  > Every cross-surface call carries correlation through the tests `corr` field plus the additive `conn` discriminator (Section 3, Trace context propagation).
- (.andromeda/obs-plan.md:293) The `<area>` list holds no `link` / `unlink`: the verbs' spans must use an area on the list (`channel` for the client call, `cli.link` / `cli.unlink` for dispatch), or the list must be amended.
  > `<area>` is one of `run`, `pty`, `channel`, `state`, `hook`, `statusline`, `send`, `wait`, `last`, `answer`, `pause`, `release`, `cli`, `mcp`. `cli.<verb>` names the dispatch span of a short-lived verb that has no dedicated client span (for example `cli.list`, `cli.verify`)
- (.andromeda/obs-plan.md:294) The chunk may add no `event` value outside the closed enum and its Section 12 extensions.
  > the `event` field is a closed kebab-case enum (the tests list plus the Section 12 extensions).
- (.andromeda/obs-plan.md:327) The verbs take two instance names; neither may appear in a span name.
  > Names are static strings; instance names, dialog ids and paths never appear in a span name.
- (.andromeda/obs-plan.md:361) The CLI frame-verb shape of a server-verification mismatch, which the two verbs take through `client::connect`; their strict-modes check stays owed to "Home and code-bearing file integrity" unless the P4 form takes it.
  > on Linux and Windows a serving process that is not the recorded wrapper ends a CLI `wait` / `last` before any frame, with one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN inside the dispatch span and no `channel-request` line, and MCP's returns `instance-unreachable` with no line (macOS compares no peer pid, the recorded gap); `strict-modes-failed` cannot occur on them, its check owed to the route entry "Home and code-bearing file integrity".
- (.andromeda/obs-plan.md:392) E3's text says the explicit methods are not built and the implicit link has no line, event value or span; serving the methods requires this line amended to the target state.
  > - **E3** (`link` / `unlink`): only the implicit link exists, recorded by the driven wrapper inside an issued `send` or a taken `answer`; it has no process-log line, no `event` value and no span, and the explicit `link` / `unlink` methods are not built.
- (.andromeda/obs-plan.md:418) The two new `-32602` causes (`driven` not the serving name, `driver` equal to `driven`) add to this derived metric's over-count.
  > - `viola.compat.v_rejected` over-counts: `-32602` also answers `send`'s invalid params and `release` carrying `from`, until the line tells the causes apart.
- (.andromeda/obs-plan.md:420) `method` is a closed grouping key; `link` / `unlink` must be values of that closed set.
  > grouping keys are closed enums only: `event`, `detail`, `refusal`, `method`, `hook_event`, `subject`, `parser`, `process`.
- (.andromeda/obs-plan.md:460) The methods' `-32602` responses log at WARN, and a verb's exit 21 logs at WARN.
  > | `warn` | `state-recovered`; `parse-rejected`; `hook-decision` with a fail-open `detail`; `process-exit` exit 21; `channel-response` with `-32600`/`-32602` |
- (.andromeda/obs-plan.md:479) The IPC boundary wrapper logs each `link` / `unlink` call as `channel-request` / `channel-response` with no argument digest.
  > - **IPC invocation** (viola-channel): `channel-request` / `channel-response` with method, `corr`, `conn`, result class, error code and `sender`. **No argument digest**: `params` and result bodies are never recorded, not even hashed.
- (.andromeda/obs-plan.md:557) The verbs read `snapshot.json`'s `endpoint`; it must never reach a line.
  > Home-level lines stay path-free; `endpoint` / `pinned_bin` are never logged
- (.andromeda/obs-plan.md:576) Any new field (a driver or driven name, `already_linked`) is logged only if the §6 catalog names it; G4 fails otherwise.
  > a field is logged only if it is named in this plan's Section 6 catalog.
- (.andromeda/obs-plan.md:684) Telemetry Strategy ban: every line the chunk adds carries the closed `event` and `corr`.
  > - NEVER let the signal pyramid collapse into free text. Every line carries the closed `event` enum and `corr`.
- (.andromeda/obs-plan.md:715) A new `event` value for explicit links (none exists today, per E3) requires a Decisions Log entry.
  > - NEVER rename tests fields (for example `corr` → `correlation_id`, `event` → `event_type`) or add `event` values without a Decisions Log entry.
- (.andromeda/obs-plan.md:735) The verbs' `detail` codes carry no path or pid.
  > - NEVER put paths or pids inside `detail` codes (Founder Direction 4).

## Bindings
- **Channel seam spans and lines ↔ obs §6 Log Coverage (channel field allow-list, `method` values, detail catalog)**: the methods' `channel-request` / `channel-response` lines must hold only catalogued fields, with `link` / `unlink` as `method` values (.andromeda/obs-plan-detail/telemetry-strategy.md:28), (.andromeda/obs-plan.md:420), (.andromeda/obs-plan.md:479), (.andromeda/obs-plan.md:576).
- **Span `<area>` list ↔ the new verbs' code**: the verbs' spans must fit `channel` / `cli.<verb>` or the §2 naming line is amended (.andromeda/obs-plan.md:293), (.andromeda/obs-plan.md:327).
- **E3 edge flow ↔ test-plan §6 Scenario E3**: the obs E3 line states the explicit methods unbuilt and must move to target state with steps 1 and 3 of the test scenario (.andromeda/obs-plan.md:392), (.andromeda/obs-plan.md:183).
- **Panic exemptions and strict-modes line ↔ obs §10 and the security plan's instance check (P4 form two)**: a pre-log `check_instance` refusal on the verbs opens no file, so the §10 exemption list and the Vector 7 trigger line move with that form (.andromeda/obs-plan-detail/telemetry-strategy.md:20), (.andromeda/obs-plan.md:215), (.andromeda/obs-plan-detail/telemetry-strategy.md:33).
- **Server-verify mismatch ↔ security-plan server check before any frame**: the verbs take the CLI frame-verb `process-exit{detail:"server-verify-failed"}` shape (.andromeda/obs-plan.md:361).
- **Exit and detail codes ↔ architecture §Conventions (exit codes) and obs §6 detail catalog**: each verb refusal maps to a distinct, path-free `detail` (.andromeda/obs-plan-detail/telemetry-strategy.md:43), (.andromeda/obs-plan.md:253), (.andromeda/obs-plan.md:735).
- **`events.ndjson` link lines ↔ architecture §Standard Contracts (event line)**: the product audit lines stay out of process logs (.andromeda/obs-plan-detail/telemetry-strategy.md:14).
- **`-32602` causes ↔ obs §5 Metric Coverage**: the new causes widen the `viola.compat.v_rejected` over-count (.andromeda/obs-plan.md:418).
