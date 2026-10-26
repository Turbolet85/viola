# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · a counter's jq derivation over existing home-level role lines; the chunk adds no metric and changes no event, detail or refusal line
metric-coverage#1.2 · no · a histogram of raw duration_ms samples per event; the chunk changes no event that carries duration_ms
metric-coverage#1.3 · no · a gauge over harness snapshots (budget.read_at, liveness via agent-run status); the chunk touches neither agent-run nor those snapshots
metric-coverage#2.1 · no · hook latency under a hyperfine max gate; the chunk re-runs no hyperfine gate and changes no hook code, and the scope lists the performance budgets as not needed
metric-coverage#2.2 · no · channel call latency from channel-response duration_ms; the chunk changes no channel-response emission and adds no metric
metric-coverage#2.3 · no · send-to-readback latency from send-confirmed duration_ms; the chunk changes no send path or its readiness gate
metric-coverage#2.4 · no · the send terminal-state counter from send-confirmed and send-refused lines; the chunk changes no send outcome
metric-coverage#2.5 · no · the wait outcome counter from channel-response method wait; the chunk changes no wait verb or its response
metric-coverage#2.6 · no · the heal and recovery counter from state-recovered lines; the chunk changes no recovery path in viola-state
metric-coverage#2.7 · no · the skipped-records gauge from list --json skipped; the chunk changes no list payload
metric-coverage#2.8 · no · the parse-rejection counter from parse-rejected lines; the chunk changes no parser
metric-coverage#2.9 · no · the compat counters from send-refused unknown and channel -32602 lines; the chunk changes no refusal or invalid-params path
metric-coverage#2.10 · no · the child and probe exit counter from process-exit lines; the chunk changes no spawn or exit path
metric-coverage#2.11 · no · the panic SLO counter from event panic lines; the chunk names no change to panic containment and adds no metric
metric-coverage#3.0 · no · the report-only quantile snippet over hook-decision lines; its gates use max, which the chunk does not re-measure
