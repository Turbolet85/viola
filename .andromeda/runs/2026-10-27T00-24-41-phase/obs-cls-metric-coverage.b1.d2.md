# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · yes · the chunk's new closed `detail` codes and its once-logged WARN/ERROR pin-write line are occurrences this grouping counts over home-level role files only, since detail files repeat lines
metric-coverage#1.2 · no · a raw-duration histogram, and the chunk adds no timed event or `duration_ms` field
metric-coverage#1.3 · yes · the race witness reads `agent-run status` (state, last_error), the harness snapshot this gauge's liveness is read from; `budget.read_at` is not reached
metric-coverage#2.1 · no · the hook decision path and `hook-decision.duration_ms` are unchanged; the fix touches only the absent-copy pin write at start, so the hook latency gate is not reached
metric-coverage#2.2 · no · no channel method is added or changed, and the pin write is no channel call (Not needed: channel `corr` join, `-32603`)
metric-coverage#2.3 · no · no send path changes (Not needed: the Confirmed `send` scenario)
metric-coverage#2.4 · no · no send path changes (Not needed: the Confirmed `send` scenario, `text_bytes`)
metric-coverage#2.5 · no · no frame client changes (Not needed: the `wait` / `last` readback scenario)
metric-coverage#2.6 · no · the snapshot replay and torn-line healing readers and appenders are unchanged (Not needed: E5, torn-line healing), so `state-recovered` is not reached
metric-coverage#2.7 · no · the `events.ndjson` reader and `list`'s board read are unchanged (Not needed: no state reader changes)
metric-coverage#2.8 · no · the chunk adds no parser and the pin write parses nothing, so no `parse-rejected` line is new
metric-coverage#2.9 · no · no send or channel-method change, and no metric is added (Not needed: derived metrics, `viola.compat.v_rejected`)
metric-coverage#2.10 · no · the pin write's new exit is a self `process-exit`, and the child and probe exit lines this counter reads are unchanged
metric-coverage#2.11 · no · standing S5 · new test output counts toward the zero budgets (0 unlogged panics, G2)
metric-coverage#3.1 · no · its `wait.outcome` clause is unreached (no channel-response change), and the server-verify exit it cites is the CLI's own, which the pin write does not touch
metric-coverage#3.2 · no · the frame verbs' `refuse_untrusted` and the MCP `wait` refusal are untouched (Not needed: the `wait` readback, the `viola mcp` surface)
metric-coverage#4.0 · no · a report-only quantile snippet over `hook-decision` durations; the chunk measures no latency and its witness waits on events, not durations
