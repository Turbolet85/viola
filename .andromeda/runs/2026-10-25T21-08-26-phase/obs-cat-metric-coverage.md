# obs · Metric Coverage — category read

## Carried
- (.andromeda/obs-plan-detail/metric-coverage.md:10) The CARRY's `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 asserts the `send-refused` line this counter derives from: its log-capture fix must leave the refused budget send writing that line, keyed by `side` and `detail`, in the home-level role file, never moved into a detail file or reshaped.
  > Counter | Occurrences of an `event` / `detail` / `refusal` value | `awk 1 diagnostics/*.ndjson | jq -R -n '[inputs|fromjson?|select(.event=="send-refused")]|group_by([.side,.detail])|map({side:.[0].side,d:.[0].detail,n:length})'`, over home-level role files only (detail files repeat lines; a torn line is skipped as in G2)
- (.andromeda/obs-plan-detail/metric-coverage.md:23) The same case_4 fix must keep the `send-refused` line carrying `refusal`, `detail` and `side`, the fields `viola.send.outcome` counts; a test-only capture change may not drop or rename any of them.
  > | Send terminal state (design readback words `read back` / `unable` / `unconfirmable`) | `viola.send.outcome` ← count of `send-confirmed{confirmed}` / `send-refused{refusal,detail,side}` | counter | 1 | n/a |
- (.andromeda/obs-plan-detail/metric-coverage.md:30) The chunk's new Windows listener tests (remote-client rejection, a second listener on a held name, the DACL's effect) and the three plain `cargo test` runs of case_4 must keep the panic count at zero in every home-level role file.
  > | Panics (SLO) | `viola.panic` ← `event:"panic"` | counter | 1 | n/a (must be 0) |
- (.andromeda/obs-plan.md:410) Hardening the wrapper's named-pipe listener must add no in-process metrics pipeline, listener or exporter. Any signal the admission work needs is derived from the ndjson lines.
  > There is no in-process metrics pipeline. Every metric is derived from the JSON-per-line logs or from hyperfine output (obs-scope §5 perf-budget: "not by an in-process metrics pipeline (no listener, no exporter)")
- (.andromeda/obs-plan.md:420) Any line the chunk adds or keeps for an admission outcome (the `squatted-name` start refusal, a rejected open) is grouped only by its closed enums: never by `pid`, `instance` or `conn`, and never by an endpoint name or path.
  > **Metric label cardinality discipline:** grouping keys are closed enums only: `event`, `detail`, `refusal`, `method`, `hook_event`, `subject`, `parser`, `process`. `corr`, `conn`, `pid`, `child_pid` and `instance` are join keys, never grouping keys in aggregate reports.
- (.andromeda/obs-plan.md:698) The chunk works on the product's only listener, so it must not introduce metric instruments or a telemetry endpoint beside the pipe. A second listener is admissible only with a Security Decisions Log entry, which this chunk's no-widening boundary does not grant.
  > - NEVER create in-process metrics instruments (`opentelemetry` meters, `metrics` 0.24.6). There is no reader, exporter or listener to flush them.
  > - NEVER add a `/metrics` or other telemetry endpoint without a Security Decisions Log entry. It is a new listener.
  > - NEVER group derived metrics by unbounded keys (`path`, `corr`, `conn`, `pid`, `instance`). Group by the closed enums only.

## Bindings
- **`send-refused` counter ↔ the chunk scope's case_4 CARRY and test-plan §3 log capture**: the counter reads `send-refused{refusal,detail,side}` from home-level role files. The CARRY's fix (three plain `cargo test` runs, or a repaired capture) must leave that line's emission and fields as the counter derives them (.andromeda/obs-plan-detail/metric-coverage.md:10), (.andromeda/obs-plan-detail/metric-coverage.md:23)
- **`viola.panic` ↔ the body's §10 zero-unlogged-panics invariant and §9 G2**: the counter must equal 0, and G2 measures it over the Windows CI leg the chunk's new tests run on (.andromeda/obs-plan-detail/metric-coverage.md:30)
- **No telemetry listener ↔ security-plan §Authentication & Authorization (the Windows pipe clause)**: the pipe stays the one listener, with no metrics endpoint beside it, and any new listener needs a Security Decisions Log entry (.andromeda/obs-plan.md:698), (.andromeda/obs-plan.md:410)
- **Cardinality discipline ↔ the body's §8 path-free home-level lines (`endpoint` never logged)**: admission outcomes group by closed `detail` / `refusal` only, never by an endpoint name, pid or instance (.andromeda/obs-plan.md:420)
