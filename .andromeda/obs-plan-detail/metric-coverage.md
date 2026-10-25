# obs-plan · Metric Coverage
Read whole. The master's broad view is .andromeda/obs-plan.md; this file holds its Metric Coverage detail.

## 5. Metric Coverage

<!-- detail: values · metrics -->

| Type | Use case | Derived from |
|------|----------|---------------------|
| Counter | Occurrences of an `event` / `detail` / `refusal` value | `awk 1 diagnostics/*.ndjson | jq -R -n '[inputs|fromjson?|select(.event=="send-refused")]|group_by([.side,.detail])|map({side:.[0].side,d:.[0].detail,n:length})'`, over home-level role files only (detail files repeat lines; a torn line is skipped as in G2) |
| Histogram | Latency distribution | Raw `duration_ms` samples per event, with no bucketing; quantiles by sort/index in `jq` |
| Gauge | Current value | Harness snapshots: `budget.read_at`, `liveness` via `agent-run status` |

<!-- /detail -->

<!-- detail: values · metrics -->

| Operation | Derived metric | Type | Unit | Buckets (histograms) |
|-----------|-----------|------|------|----------------------|
| Hook latency per `hook_event` (Scenario 4/6; perf trigger) | `viola.hook.duration_ms` ← `hook-decision.duration_ms` | histogram | ms | raw samples; gate = hyperfine `max` |
| Channel call latency per `method` | `viola.channel.duration_ms` ← `channel-response.duration_ms` | histogram | ms | raw samples |
| Send → readback latency (design readback states `open` → `read back`) | `viola.send.confirm_ms` ← `duration_ms` of the `send-confirmed` lines whose `confirmed` is not `false` (an unconfirmable send's line carries `duration_ms` too, but it had no confirmation window and is no readback). `duration_ms` runs from the send's arrival at the wrapper, so a sample includes the readiness gate's wait, up to architecture's `GATE_MAX_WAIT`, ahead of the window (as read in `src/run/send.rs` at chunk 2026-10-07-send-waits-out-the-paste-hint, not changed there) | histogram | ms | raw samples |
| Send terminal state (design readback words `read back` / `unable` / `unconfirmable`) | `viola.send.outcome` ← count of `send-confirmed{confirmed}` / `send-refused{refusal,detail,side}` | counter | 1 | n/a |
| `wait` outcome (design CLI `viola wait`) | `viola.wait.outcome` ← `channel-response{method:"wait"}.outcome` | counter | 1 | n/a |
| Heal / recovery (chaos) | `viola.state.recovered` ← `state-recovered.detail` | counter | 1 | n/a |
| Skipped records | `viola.parse.skipped` ← `list --json` `skipped` | gauge | 1 | n/a |
| Parse rejections per parser (creator-explicit) | `viola.parse.rejected` ← `parse-rejected{parser, detail, count}` | counter | 1 | n/a |
| `unknown` fallback / higher-`v` rejections | `viola.compat.unknown` ← `send-refused{refusal:"unknown"}`; `viola.compat.v_rejected` ← `channel-response{error_code:-32602}` — over-counts: `-32602` also answers `send`'s invalid params (`ProtocolError::InvalidParams`, `"invalid params"`) and `release` carrying `from`, and the client's `fault_of` labels any `-32602` unsupported-version, until the line tells the causes apart | counter | 1 | n/a |
| Child / probe exits | `viola.child.exit` ← `process-exit{subject, child_exit_status, exit_source}` | counter | 1 | n/a |
| Panics (SLO) | `viola.panic` ← `event:"panic"` | counter | 1 | n/a (must be 0) |

<!-- /detail -->

<!-- detail: values · metrics -->

Canonical quantile snippet (report-only; the gates use `max`):
```sh
# -R + fromjson? skip a torn line as G2 does (jq -s aborts on one); awk 1 ends each file's last line with a newline
awk 1 diagnostics/hook-*.ndjson | jq -R -n '[inputs|fromjson?|select(.event=="hook-decision" and .hook_event=="session-end")|.duration_ms]|sort
  |{n:length,p50:.[length/2|floor],p95:.[length*0.95|floor],max:.[-1]}'
```

<!-- /detail -->
