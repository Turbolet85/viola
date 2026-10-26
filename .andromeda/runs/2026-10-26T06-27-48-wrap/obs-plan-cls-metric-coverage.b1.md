# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · the session changed no send-refused event and no side or detail field, so the counter stands
metric-coverage#1.2 · no · the session changed no duration_ms event field, so the raw histogram samples stand
metric-coverage#1.3 · no · the session reached no harness file, so the budget.read_at and liveness gauge reads stand
metric-coverage#2.1 · no · no hook or gate changed, so hook-decision duration_ms against the hyperfine max gate stands
metric-coverage#2.2 · no · no channel source or event changed, so channel-response duration_ms still feeds the latency histogram
metric-coverage#2.3 · no · no send source changed, so the cited src/run/send.rs reading and the send-confirmed samples stand
metric-coverage#2.4 · no · no send event or field changed, so the send-confirmed and send-refused outcome counts stand
metric-coverage#2.5 · no · no channel response field changed, so the wait outcome counter stands
metric-coverage#2.6 · no · no viola-state event or source changed, so state-recovered detail still feeds the heal counter
metric-coverage#2.7 · no · no list JSON field changed, so the skipped gauge still reads its source
metric-coverage#2.8 · no · no parse-rejected event or parser changed, so the per-parser counter stands
metric-coverage#2.9 · no · no refusal, error code or channel source changed, so both compat counters stand
metric-coverage#2.10 · no · no process-exit event or spawn source changed, so the child-exit counter stands
metric-coverage#2.11 · no · no panic event or panic-hook source changed, so the panic counter stands
metric-coverage#3.0 · no · no event, field or path the quantile snippet names changed, so it still reads hook-decision lines
