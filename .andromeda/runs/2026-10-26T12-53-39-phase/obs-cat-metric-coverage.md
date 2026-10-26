# obs · Metric Coverage — category read

## Carried
- (.andromeda/obs-plan.md:410) The chunk's served `link` / `unlink` methods and verbs must add no metrics pipeline: every metric about them is derived from the JSON-per-line logs only.
  > There is no in-process metrics pipeline. Every metric is derived from the JSON-per-line logs or from hyperfine output (obs-scope §5 perf-budget: "not by an in-process metrics pipeline (no listener, no exporter)")
- (.andromeda/obs-plan.md:698) The new verbs, methods and their wrapper handling must create no in-process instrument and no telemetry endpoint.
  > NEVER create in-process metrics instruments (`opentelemetry` meters, `metrics` 0.24.6). There is no reader, exporter or listener to flush them.
  > - NEVER add a `/metrics` or other telemetry endpoint without a Security Decisions Log entry. It is a new listener.
- (.andromeda/obs-plan-detail/metric-coverage.md:10) The new verbs' refusals and exits must land as closed `event` / `detail` / `refusal` values in the home-level role files, so the Counter derivation counts them by grouping alone.
  > | Counter | Occurrences of an `event` / `detail` / `refusal` value | `awk 1 diagnostics/*.ndjson | jq -R -n '[inputs|fromjson?|select(.event=="send-refused")]|group_by([.side,.detail])|map({side:.[0].side,d:.[0].detail,n:length})'`, over home-level role files only (detail files repeat lines; a torn line is skipped as in G2) |
- (.andromeda/obs-plan-detail/metric-coverage.md:21) A served `link` / `unlink` call must write its `channel-response` with `duration_ms` and the `method`, so `viola.channel.duration_ms` per `method` covers the two new methods.
  > | Channel call latency per `method` | `viola.channel.duration_ms` ← `channel-response.duration_ms` | histogram | ms | raw samples |
- (.andromeda/obs-plan-detail/metric-coverage.md:28) The `-32602` answers this chunk adds (a `driven` that is not the serving wrapper's name, a `driver` equal to `driven`) widen the `viola.compat.v_rejected` over-count; the row's named causes must take them in, or the line must tell the causes apart.
  > `viola.compat.v_rejected` ← `channel-response{error_code:-32602}` — over-counts: `-32602` also answers `send`'s invalid params (`ProtocolError::InvalidParams`, `"invalid params"`) and `release` carrying `from`, and the client's `fault_of` labels any `-32602` unsupported-version, until the line tells the causes apart
- (.andromeda/obs-plan.md:418) The body's broad line for the same over-count moves with the detail row when `link` / `unlink` add `-32602` causes.
  > - `viola.compat.v_rejected` over-counts: `-32602` also answers `send`'s invalid params and `release` carrying `from`, until the line tells the causes apart.
- (.andromeda/obs-plan-detail/metric-coverage.md:36) The two verbs take the server check through `client::connect` like the other CLI frame verbs, so their server-verification refusal must be the same one `process-exit` line, counted by the Counter row's `detail` grouping.
  > The CLI's server-verification refusal (Linux and Windows) is one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`, counted by the Counter row's `detail` grouping;
- (.andromeda/obs-plan.md:420) The new `method` values must be closed enum values usable as grouping keys, and the `driver` / `driven` instance names must never become a grouping key.
  > **Metric label cardinality discipline:** grouping keys are closed enums only: `event`, `detail`, `refusal`, `method`, `hook_event`, `subject`, `parser`, `process`. `corr`, `conn`, `pid`, `child_pid` and `instance` are join keys, never grouping keys in aggregate reports.
- (.andromeda/obs-plan.md:700) No derived metric over link lines may group by the instance names they carry.
  > NEVER group derived metrics by unbounded keys (`path`, `corr`, `conn`, `pid`, `instance`). Group by the closed enums only.

## Bindings
- **`viola.compat.v_rejected` over-count ↔ §6 Log Coverage**: the link methods' `-32602` answers are `channel-response` lines at WARN (body §6 log levels: "`channel-response` with `-32600`/`-32602`"), and the metric row (.andromeda/obs-plan-detail/metric-coverage.md:28) and its body line (.andromeda/obs-plan.md:418) name every `-32602` cause the counter folds in.
- **`viola.channel.duration_ms` per `method` ↔ §6 boundary-call wrappers and §4 E3**: the IPC wrapper's `channel-request` / `channel-response` lines with `method` (.andromeda/obs-plan.md:479) are the metric's source (.andromeda/obs-plan-detail/metric-coverage.md:21); §4's E3 line (.andromeda/obs-plan.md:392), which says the explicit methods are not built, moves with the served methods.
- **Counter `detail` grouping ↔ §4 wait/last scenario and §7**: the server-verify `process-exit` the new verbs write (.andromeda/obs-plan-detail/metric-coverage.md:36) is the same line §4 names for CLI frame verbs (.andromeda/obs-plan.md:361), and the `detail` code is from §6's closed catalog with no path or pid (.andromeda/obs-plan.md:735).
- **Cardinality ↔ security plan input validation**: `driver` / `driven` are `ViolaName` values (the logged link name re-parsed through `ViolaName::try_new`), join keys only, never grouping keys (.andromeda/obs-plan.md:420, .andromeda/obs-plan.md:700).
