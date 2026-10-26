# obs · Metric Coverage — category read

## Carried
- (.andromeda/obs-plan-detail/metric-coverage.md:20) The verification added to `viola hook`'s client before `hook.event` / `hook.dialog` (its fail-open mismatch included) runs inside the hook's latency, so every hook must keep emitting `hook-decision.duration_ms` per `hook_event` and stay under the hyperfine `max` gate with the new check in place.
  > | Hook latency per `hook_event` (Scenario 4/6; perf trigger) | `viola.hook.duration_ms` ← `hook-decision.duration_ms` | histogram | ms | raw samples; gate = hyperfine `max` |
- (.andromeda/obs-plan.md:701) The hook-latency cost of the new check is judged by the gated statistic, never by a p50/p95 report.
  > NEVER report latency without the gate statistic. The `max` from hyperfine is the enforced number; p50/p95 are report-only.
- (.andromeda/obs-plan-detail/metric-coverage.md:23) A `send` refused because the server failed verification (CLI exit 21, MCP `instance-unreachable`) is a send terminal state: wherever the chunk logs it as `send-refused`, the line must carry the closed `refusal`, `detail` and `side` this counter groups by.
  > | Send terminal state (design readback words `read back` / `unable` / `unconfirmable`) | `viola.send.outcome` ← count of `send-confirmed{confirmed}` / `send-refused{refusal,detail,side}` | counter | 1 | n/a |
- (.andromeda/obs-plan-detail/metric-coverage.md:24) A `wait` refused on a failed server verification is a `wait` outcome, and this counter reads it only from `channel-response{method:"wait"}.outcome`, so the new outcome must land in that field.
  > | `wait` outcome (design CLI `viola wait`) | `viola.wait.outcome` ← `channel-response{method:"wait"}.outcome` | counter | 1 | n/a |
- (.andromeda/obs-plan.md:361) The `wait` / `last` outcome this chunk makes possible is named here as owned by this route entry, so the counter above must count `server-verify-failed` (and `strict-modes-failed` only if the strict-modes half lands here) once verification is built.
  > a mid-call disconnect is logged once, by the client; `strict-modes-failed` / `server-verify-failed` cannot occur on `wait` / `last` until client-side server verification lands, owned by the route entries "Server verification before any frame" and "Home and code-bearing file integrity".
- (.andromeda/obs-plan-detail/metric-coverage.md:10) Any new refusal or fail-open `detail` code the verification adds becomes a counted value, derived over the home-level role files only; the code must be a closed value a `group_by` can count.
  > | Counter | Occurrences of an `event` / `detail` / `refusal` value | `awk 1 diagnostics/*.ndjson | jq -R -n '[inputs|fromjson?|select(.event=="send-refused")]|group_by([.side,.detail])|map({side:.[0].side,d:.[0].detail,n:length})'`, over home-level role files only (detail files repeat lines; a torn line is skipped as in G2) |
- (.andromeda/obs-plan.md:420) The verification compares pids and start times; the mismatch it reports must be a closed grouping value, and the server pid, the recorded pid and the instance stay join keys, never a grouping key.
  > **Metric label cardinality discipline:** grouping keys are closed enums only: `event`, `detail`, `refusal`, `method`, `hook_event`, `subject`, `parser`, `process`. `corr`, `conn`, `pid`, `child_pid` and `instance` are join keys, never grouping keys in aggregate reports.
- (.andromeda/obs-plan.md:700) The same ban, stated as a never-do: no verification-failure count is grouped by pid or instance.
  > NEVER group derived metrics by unbounded keys (`path`, `corr`, `conn`, `pid`, `instance`). Group by the closed enums only.
- (.andromeda/obs-plan.md:698) The verification-failure count is derived from log lines by `jq`, never kept by an in-process counter in any client.
  > NEVER create in-process metrics instruments (`opentelemetry` meters, `metrics` 0.24.6). There is no reader, exporter or listener to flush them.
- (.andromeda/obs-plan-detail/metric-coverage.md:25) Under the held CARRY 7 / 8 (clients keep `read_snapshot`, no replay), no client adds a `state-recovered` line; if the founder's live word moves a client to the classified read or the replay, its lines become samples of this counter.
  > | Heal / recovery (chaos) | `viola.state.recovered` ← `state-recovered.detail` | counter | 1 | n/a |
- (.andromeda/obs-plan.md:394) The replay is the only snapshot reader that writes `state-recovered`; a client keeping today's reader writes none, which is what the hold keeps.
  > **E5** (snapshot corruption): `state-recovered{detail:"snapshot-replayed"|"snapshot-unsupported-v"}` at WARN, written by `viola_state::replay::read_snapshot_or_replay`, one line per call; the other snapshot readers write no `state-recovered` line.
- (.andromeda/obs-plan-detail/metric-coverage.md:30) The new client paths, the hook's fail-open on mismatch included, must leave the panic count at zero.
  > | Panics (SLO) | `viola.panic` ← `event:"panic"` | counter | 1 | n/a (must be 0) |

## Bindings
- **Hook latency histogram ↔ §10 performance budgets (body)**: the new pre-frame check lands inside the timed `viola hook` rows, and §10 gates their hyperfine `max` (session-end under 1.0 s, the spine hooks and `pre-tool-use` under the spine deadline) through `gate --require perf` (.andromeda/obs-plan-detail/metric-coverage.md:20) (.andromeda/obs-plan.md:649)
- **`viola.wait.outcome` ↔ §4 `wait` / `last` scenario (span-trace-coverage)**: the scenario names `server-verify-failed` as the outcome this entry makes possible; the counter reads it from `channel-response{method:"wait"}.outcome` (.andromeda/obs-plan-detail/metric-coverage.md:24) (.andromeda/obs-plan.md:361)
- **Counter / cardinality ↔ §6 `detail` code catalog (log-coverage)**: a new verification `detail` code joins the closed catalog, never a path or pid, and is then a grouping key here (.andromeda/obs-plan-detail/metric-coverage.md:10) (.andromeda/obs-plan.md:420) (.andromeda/obs-plan.md:451)
- **`viola.send.outcome` ↔ security §Local trust boundary (exit 21 / `instance-unreachable`)**: the refusal security names for a mismatch is the send terminal state this counter counts by `refusal`, `detail` and `side` (.andromeda/obs-plan-detail/metric-coverage.md:23)
- **`viola.state.recovered` ↔ the scope's held CARRY 7 / 8 and §4 E5**: no client replay, so no new samples, unless the founder rules otherwise (.andromeda/obs-plan-detail/metric-coverage.md:25) (.andromeda/obs-plan.md:394)
