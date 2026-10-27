# obs · cascade read · group 8 of 10

## Carried
- (.andromeda/obs-plan-detail/metric-coverage.md:10) The chunk re-verifies `send-refused` for `control-character` and `empty-text` on the CLI client, the MCP client and the wrapper, so each deciding side's line must carry a closed `side` and `detail` that this Counter grouping counts apart, over home-level role files only.
  > | Counter | Occurrences of an `event` / `detail` / `refusal` value | `awk 1 diagnostics/*.ndjson | jq -R -n '[inputs|fromjson?|select(.event=="send-refused")]|group_by([.side,.detail])|map({side:.[0].side,d:.[0].detail,n:length})'`, over home-level role files only (detail files repeat lines; a torn line is skipped as in G2) |
  (.andromeda/obs-plan.md:420)
  > **Metric label cardinality discipline:** grouping keys are closed enums only: `event`, `detail`, `refusal`, `method`, `hook_event`, `subject`, `parser`, `process`. `corr`, `conn`, `pid`, `child_pid` and `instance` are join keys, never grouping keys in aggregate reports.
- (.andromeda/obs-plan-detail/metric-coverage.md:23) The paste refusals the chunk re-verifies on each side are the send terminal state `unable`; every one must land as one `send-refused{refusal,detail,side}` so `viola.send.outcome` counts it, with no text in the line.
  > | Send terminal state (design readback words `read back` / `unable` / `unconfirmable`) | `viola.send.outcome` ← count of `send-confirmed{confirmed}` / `send-refused{refusal,detail,side}` | counter | 1 | n/a |
- (.andromeda/obs-plan-detail/metric-coverage.md:27) The chunk's closed-type boundaries emit `parse-rejected` (one per bad `config.json` key under `config-json`, one per failed, oversize or malformed agents read under `claude-agents-json`, one per vt100 poisoning episode under `vt100-feed`); each must carry a closed `parser` and `detail` so this counter derives from the line alone, and a new `parser` or `detail` value is a closed enum fit for grouping.
  > | Parse rejections per parser (creator-explicit) | `viola.parse.rejected` ← `parse-rejected{parser, detail, count}` | counter | 1 | n/a |
  (.andromeda/obs-plan.md:410)
  > _[Standard: included. There is no in-process metrics pipeline. Every metric is derived from the JSON-per-line logs or from hyperfine output (obs-scope §5 perf-budget: "not by an in-process metrics pipeline (no listener, no exporter)")]_
  (.andromeda/obs-plan.md:412)
  > **Metric instrument types** (all are logical: computed by `jq` / the harness over `diagnostics/*.ndjson`, never OTel or `metrics` 0.24.6 instruments, because there is no reader or exporter to flush them):
- (.andromeda/obs-plan-detail/metric-coverage.md:28) A `name` / `driver` / `driven` check the chunk adds or re-verifies on a channel method answers `-32602`, which adds to the `viola.compat.v_rejected` over-count; the plan must either keep that over-count recorded as stated or make the line tell the cause apart, never leave a new `-32602` cause unnamed.
  > | `unknown` fallback / higher-`v` rejections | `viola.compat.unknown` ← `send-refused{refusal:"unknown"}`; `viola.compat.v_rejected` ← `channel-response{error_code:-32602}` — over-counts: `-32602` also answers `send`'s invalid params (`ProtocolError::InvalidParams`, `"invalid params"`), `release` carrying `from` and a `link` / `unlink` with a missing, non-string or invalid name, a `driven` that is not the serving wrapper or a `driver` equal to `driven`, and the client's `fault_of` labels any `-32602` unsupported-version, until the line tells the causes apart | counter | 1 | n/a |
  (.andromeda/obs-plan.md:418)
  > - `viola.compat.v_rejected` over-counts: `-32602` also answers `send`'s invalid params, `release` carrying `from` and a bad `link` / `unlink` name or pair, until the line tells the causes apart.
  (.andromeda/obs-plan.md:414)
  > **Naming conventions:** `viola.<area>.<measure>` for derived metrics. No OTel semconv instruments exist in v1.
- (.andromeda/obs-plan-detail/metric-coverage.md:25) The bounded snapshot read must keep `read_snapshot_or_replay`'s one `state-recovered` line per call, the line this counter derives from; a cap added on `snapshot.json` changes no `detail` value it counts.
  > | Heal / recovery (chaos) | `viola.state.recovered` ← `state-recovered.detail` | counter | 1 | n/a |
- (.andromeda/obs-plan-detail/metric-coverage.md:26) The chunk's closed-type reading of `events.ndjson` must keep skipping and counting an unknown kind or field, so `list --json`'s `skipped` stays the value this gauge reads; a cap or type rule that refused a line instead would drop it from the gauge.
  > | Skipped records | `viola.parse.skipped` ← `list --json` `skipped` | gauge | 1 | n/a |
- (.andromeda/obs-plan-detail/metric-coverage.md:30) The chunk re-verifies the vt100 panic degrade; a panic forced in that witness fires the panic hook's `event:"panic"` line, so the witness must run inside the bounded exemption home (outside `target/e2e-home/`) and every other new test must keep this count at 0.
  > | Panics (SLO) | `viola.panic` ← `event:"panic"` | counter | 1 | n/a (must be 0) |

## Bindings
- **metric-coverage `viola.parse.rejected` ↔ error-capture (vt100 feed)**: the counter reads one `parse-rejected{parser:"vt100-feed"}` per poisoning episode, the line §7 requires the feed thread's `catch_unwind` to write while `run` continues (.andromeda/obs-plan-detail/metric-coverage.md:27), (.andromeda/obs-plan.md:535)
- **metric-coverage `viola.parse.rejected` ↔ log-coverage (child spawns)**: an oversize or malformed `claude agents --json` read is one `parse-rejected{parser:"claude-agents-json"}` and the value `"unknown"`, counted here and never logged verbatim (.andromeda/obs-plan-detail/metric-coverage.md:27), (.andromeda/obs-plan.md:482)
- **metric-coverage `viola.compat.v_rejected` ↔ log-coverage (levels)**: an over-size or invalid frame or param answered `-32602` / `-32600` is a WARN `channel-response`, the line the over-counting metric reads (.andromeda/obs-plan-detail/metric-coverage.md:28), (.andromeda/obs-plan.md:460)
- **metric-coverage `viola.panic` ↔ slo-invariants (zero unlogged panics)**: the must-be-0 counter is the SLO's count over home-level role files, its exemptions bounded, so the forced vt100 panic witness sits in an exempt home (.andromeda/obs-plan-detail/metric-coverage.md:30), (.andromeda/obs-plan.md:637)
- **metric-coverage `viola.state.recovered` ↔ span-trace (E5 snapshot corruption)**: the counter reads the one WARN `state-recovered` per `read_snapshot_or_replay` call that the bounded snapshot read keeps (.andromeda/obs-plan-detail/metric-coverage.md:25), (.andromeda/obs-plan.md:394)
- **metric-coverage `viola.send.outcome` / Counter row ↔ pii-scrubbing**: each deciding side's `send-refused` is counted by `side` and `detail` and carries no text (.andromeda/obs-plan-detail/metric-coverage.md:23), (.andromeda/obs-plan.md:195)
