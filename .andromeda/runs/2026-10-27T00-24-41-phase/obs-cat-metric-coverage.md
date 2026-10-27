# obs · Metric Coverage — category read

## Carried
- (.andromeda/obs-plan.md:410) The race witness (6-of-24 two-instance against 0-of-24 single-instance), the Windows wait-out and the build-comparison measurement must take every count or timing from the JSON-per-line logs, the harness documents or hyperfine output. The fix must not add an in-process counter for its retries or same-bytes acceptances.
  > There is no in-process metrics pipeline. Every metric is derived from the JSON-per-line logs or from hyperfine output
- (.andromeda/obs-plan.md:698) A wait-out or retry added to the absent-copy arm of `pin_bytes` / `pin_exe` must record no `metrics` or `opentelemetry` instrument. Any count it needs goes on a catalogued line field.
  > - NEVER create in-process metrics instruments (`opentelemetry` meters, `metrics` 0.24.6). There is no reader, exporter or listener to flush them.
- (.andromeda/obs-plan-detail/metric-coverage.md:10) A boot-failure or refusal count the two-sided witness derives from `diagnostics/` must count over the home-level role files only and skip a torn line. A count over detail files would double it.
  > over home-level role files only (detail files repeat lines; a torn line is skipped as in G2)
- (.andromeda/obs-plan.md:420) The two-instance witness must group its aggregate by closed enums only. Instance name, pid and child pid may join lines but must never group the red-before and clean-after counts.
  > **Metric label cardinality discipline:** grouping keys are closed enums only: `event`, `detail`, `refusal`, `method`, `hook_event`, `subject`, `parser`, `process`. `corr`, `conn`, `pid`, `child_pid` and `instance` are join keys, never grouping keys in aggregate reports.
- (.andromeda/obs-plan.md:700) The same ban, at its rule: no per-path or per-instance grouping of the concurrent-start outcomes, and no grouping by the `bin/<key>/` path.
  > - NEVER group derived metrics by unbounded keys (`path`, `corr`, `conn`, `pid`, `instance`). Group by the closed enums only.
- (.andromeda/obs-plan-detail/metric-coverage.md:11) A latency the chunk reports, such as the measured 5–11 ms `ERROR_SHARING_VIOLATION` wait-out or the window that shows the race is reachable, must come from raw `duration_ms` samples with no bucketing.
  > Raw `duration_ms` samples per event, with no bucketing; quantiles by sort/index in `jq`
- (.andromeda/obs-plan.md:701) Any latency figure in the chunk's evidence must report the max-sample gate statistic. p50 and p95 are report-only.
  > - NEVER report latency without the gate statistic. The `max` from hyperfine is the enforced number; p50/p95 are report-only.
- (.andromeda/obs-plan-detail/metric-coverage.md:30) The concurrent-first-start tests and the race reproduction must write zero `event:"panic"` lines in any home G2 counts. The two-instance boot reaching that state does not exempt it.
  > | Panics (SLO) | `viola.panic` ← `event:"panic"` | counter | 1 | n/a (must be 0) |

## Bindings
- **Counter derivation over home-level files ↔ §10 error-budget measurement**: the Counter row's home-level-only scope (.andromeda/obs-plan-detail/metric-coverage.md:10) and the `viola.panic` must-be-0 row (.andromeda/obs-plan-detail/metric-coverage.md:30) are the same counting rule the body's §10 states for panics, where a `jq` count must leave out `detail-*.ndjson` (.andromeda/obs-plan.md:657). Both sides bind the race witness's homes under `target/e2e-home/`.
- **Cardinality discipline ↔ §11 Metrics bans**: the body's grouping-key list (.andromeda/obs-plan.md:420) and its anti-pattern (.andromeda/obs-plan.md:700) bind together on any aggregate the two-instance witness reports.
- **No in-process instruments ↔ §11 Metrics bans**: the §5 broad line (.andromeda/obs-plan.md:410) and the instrument ban (.andromeda/obs-plan.md:698) both bind the pin fix's retry or wait-out.
- **Histogram raw samples ↔ §10 performance budgets (gate on `max`)**: raw-sample latencies (.andromeda/obs-plan-detail/metric-coverage.md:11) are reported with `max` as the gated tail (.andromeda/obs-plan.md:701). No new perf budget is set by this chunk.
