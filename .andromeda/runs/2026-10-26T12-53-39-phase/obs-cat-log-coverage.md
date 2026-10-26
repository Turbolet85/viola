# obs · Log Coverage — category read

## Carried
- (.andromeda/obs-plan-detail/log-coverage.md:12) The `link` / `unlink` channel methods, once the wrapper serves them, must log their `channel-request` with only the catalogued fields: no new field for `driver` / `driven`.
  > | `channel-request` | `method`, `conn` / `srv_conn`, `from`, `from_trust`, `sender`, `v`, `after`, `timeout_ms` |
- (.andromeda/obs-plan-detail/log-coverage.md:13) The served methods' answers (`{since, already_linked}`, the empty object, and `-32602` for a `driven` that is not the serving wrapper's name or a `driver` equal to `driven`) must log as `channel-response` with `result_class` and `error_code` taken from the closed sets. The `-32601` they answer today leaves the set when they are served.
  > | `channel-response` | `method`, `conn` / `srv_conn`, `result_class` (`ok|refusal|error`), `refusal`, `detail`, `error_code` (`-32700|-32600|-32601|-32602|-32603`), `outcome`, `duration_ms` |
- (.andromeda/obs-plan-detail/log-coverage.md:28) When `viola link` / `viola unlink` write their frame through `client::connect`, a server that fails verification must give the same exit-21 `server-verify-failed` line at WARN, with no pid, start time, path or endpoint. The count "six CLI frame verbs" moves with them.
  > - `process-exit` exit 21: `instance-dead`, `strict-modes-failed`, `server-verify-failed`.
  >  `server-verify-failed` is written by the six CLI frame verbs on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows): `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson`, with no pid, start time, path or endpoint.
- (.andromeda/obs-plan-detail/log-coverage.md:35) If the founder picks the second form (`check_instance` ahead of the snapshot read, as `viola revive` takes it), the verbs' refusal must follow revive's no-line shape. Writing `strict-modes-failed` on a `process-exit` would break "no arm of this build writes the word".
  > `viola revive`'s start arm and `viola revive --list` both refuse on `viola_state::strict::check_instance` with the stderr pair, or under `--json` the `strict-modes-failed` document, exit 1 and no log line; on `hook-decision` the schema's `detail` enum keeps the word and no arm writes it, since `viola hook statusline`'s refused check leaves no line and the nine hook events run no instance check; chunk 2026-10-10-budget-governor; `viola list`'s refused scan of the home, `instances/` or an instance exits 21 on both forms and writes no line either — no `process-start`, no `process-exit`, no `strict-modes-failed` — so on `process-exit` exit 21 no arm of this build writes the word;
- (.andromeda/obs-plan-detail/log-coverage.md:27) Any new exit cause the verbs add must move the closed `process-exit` `detail` enum and the count recorded here together.
  > `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 13 values (the enum's members; 12 before chunk 2026-10-16-unwrapped-sessions-on-the-board, 11 before chunk 2026-10-13-interrupted-verify-cleanup, 9 before chunk 2026-10-10-viola-revive).
- (.andromeda/obs-plan.md:109) `link` and `unlink` are already named on the short-lived cli surface. Their logs must go only to the `cli-<name>.ndjson` file sink when an instance resolves, never to stdout, and their stderr refusals must stay fixed strings.
  > - **Surface:** cli (short-lived verbs: `send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install · revive --list`)
  >
  > - **Exporter:** a file sink only, JSON-per-line to `diagnostics/` when an instance resolves — no network OTLP (viola makes no outbound network calls) and no stdout exporter (stdout carries `--json` results); the instance comes from `VIOLA_NAME` or an argument, never from the working directory; refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text.
- (.andromeda/obs-plan.md:190) The `link` / `unlink` channel lines must never carry their `params` (the driver and driven names) or their result bodies, and any `from` must be logged as unauthenticated.
  > - **Required telemetry signal:** channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
- (.andromeda/obs-plan.md:392) E3's statement says the explicit methods are not built and no link has a process-log line. Serving the methods must keep "no `event` value" unless a Decisions Log entry adds one, and the statement must be amended when they land.
  > - **E3** (`link` / `unlink`): only the implicit link exists, recorded by the driven wrapper inside an issued `send` or a taken `answer`; it has no process-log line, no `event` value and no span, and the explicit `link` / `unlink` methods are not built.
- (.andromeda/obs-plan.md:459) The verbs' and methods' lines must take the levels mapped here: `-32602` and exit 21 at warn, `-32603` and exit 1 at error, ok and refusal outcomes at info.
  > | `error` | `panic`; `process-exit` with `exit_code` 1 or 20; `channel-response` with `error_code` -32603 |
  > | `warn` | `state-recovered`; `parse-rejected`; `hook-decision` with a fail-open `detail`; `process-exit` exit 21; `channel-response` with `-32600`/`-32602` |
  > | `info` | All other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes |
- (.andromeda/obs-plan.md:479) The new IPC methods must go through the boundary wrapper with no argument digest.
  > - **IPC invocation** (viola-channel): `channel-request` / `channel-response` with method, `corr`, `conn`, result class, error code and `sender`. **No argument digest**: `params` and result bodies are never recorded, not even hashed.
- (.andromeda/obs-plan.md:576) Every field on a line the chunk writes must already be named in the §6 catalog, and G4 must pass against the schema.
  > **Default-deny posture:** a field is logged only if it is named in this plan's Section 6 catalog. Unknown fields are a build failure (§10), caught by gate G4 (§9) against `schemas/diag-line.v1.json`.
- (.andromeda/obs-plan.md:644) Both sides of a `link` / `unlink` call must log `corr` and `conn`.
  > - **Correlation propagation:** every channel call logs `corr` + `conn` on both sides;
- (.andromeda/obs-plan.md:709) The new verbs and methods must log only through `obs_event!`, never through a raw tracing macro.
  > - NEVER call `tracing::{event,info,warn,error,debug,trace}!` directly outside `viola_core::obs`. Only `obs_event!` guarantees `event`, `process` and `instance`; the caller sets `corr` explicitly as a typed field on the events that carry one.
- (.andromeda/obs-plan.md:715) No `link` / `unlink` `event` value may be added without a Decisions Log entry.
  > - NEVER rename tests fields (for example `corr` → `correlation_id`, `event` → `event_type`) or add `event` values without a Decisions Log entry.

## Bindings
- **channel-request / channel-response ↔ test-plan §6 Scenario E3 and architecture §Standard Contracts (channel frames)**: the served `link` / `unlink` methods, and their `-32602` cases, are logged with the closed fields and error codes on this side, and the frame shapes and E3 steps are asserted on the other. (.andromeda/obs-plan-detail/log-coverage.md:13), (.andromeda/obs-plan.md:392)
- **server-verify-failed on the frame verbs ↔ security-plan / rules/security.md (the dated frame-writing exceptions and the server check)**: the "six CLI frame verbs" count here and the security exception list both grow if the founder's form lets the verbs write frames. (.andromeda/obs-plan-detail/log-coverage.md:29)
- **strict-modes-failed no-line rule ↔ security-plan `check_instance` (the second form, as `viola revive` takes it) and "Home and code-bearing file integrity"**: the founder's P4 form decides whether the verbs refuse before any log opens (no line) or later. (.andromeda/obs-plan-detail/log-coverage.md:35)
- **process-exit `detail` enum / default-deny ↔ `schemas/diag-line.v1.json` (G4)**: a new code or field moves the schema and this catalog together. (.andromeda/obs-plan-detail/log-coverage.md:27), (.andromeda/obs-plan.md:576)
- **E3 statement ↔ obs §4 span scenarios (span area list)**: the "no span" clause and the span name the verbs take move together. (.andromeda/obs-plan.md:392)
