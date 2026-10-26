proposals:
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 28
    section: "## 5. Metric Coverage"
    change: "In the `unknown` fallback / higher-`v` rejections row, the over-count clause reads: over-counts: `-32602` also answers `send`'s invalid params (`ProtocolError::InvalidParams`, `\"invalid params\"`), `release` carrying `from`, and a `link` / `unlink` whose `driver` or `driven` is missing, not a string or not a valid name, whose `driven` is not the serving wrapper's own name, or whose `driver` equals `driven` (`ProtocolError::InvalidParams`, no line written), and the client's `fault_of` labels any `-32602` unsupported-version, until the line tells the causes apart"
    sidecar: "2026-10-26-explicit-session-links: metric-coverage §5 `viola.compat.v_rejected` over-count names the served `link` / `unlink` methods' `-32602` causes (bad or missing name, foreign `driven`, self-pair) beside `send`'s invalid params and `release` carrying `from`"
    rationale: "The report's Counts row `vrejected` names this line stale (`-32602` causes behind `viola.compat.v_rejected`'s over-count +2, a bad `link` / `unlink` name or pair); Symbols / APIs: `LinkSet::link` / `unlink` share `driver_of`, each refusal `ProtocolError::InvalidParams` (`-32602`, `data: null`), and the wrapper's `channel-response` carries `method:\"link\"|\"unlink\"`, so these answers land in the `channel-response{error_code:-32602}` count"
    basis: .andromeda/obs-plan-detail/metric-coverage.md:28
note: "The same stale over-count clause stands in the body at .andromeda/obs-plan.md:418 (report Counts `vrejected`, stale 2) — outside this category file, for the body's own pass; the Channel call latency row (line 21) and the Counter row (line 10) cover `link` / `unlink` by class (grouping by `method` / `detail`) and read true."
