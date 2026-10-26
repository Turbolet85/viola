# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · a Counter's jq grouping over role files, a derivation the chunk adds no metric to (Not needed, derived viola.* metrics)
metric-coverage#1.2 · no · a generic histogram type (raw samples, no buckets) the chunk adds no histogram to; its hook gate is hyperfine max, which 2.1 carries
metric-coverage#1.3 · no · a gauge over budget.read_at and the harness's agent-run status, which names no unit the chunk edits or its scope names
metric-coverage#2.1 · yes · the hook latency gate is hyperfine max, and the strict-modes check added on every hook path must stay inside the hook perf budgets (Needs)
metric-coverage#2.2 · no · a raw channel-response latency with no gate; a refusal before the frame writes no channel-response and the chunk adds no metric
metric-coverage#2.3 · no · send confirm latency starts at the wrapper's arrival, after the CLI pre-check, and the send path after the frame is untouched (Not needed)
metric-coverage#2.4 · no · send outcome counts send-confirmed and send-refused lines, and a CLI pre-check refusal writes process-exit (Needs), not a send terminal line
metric-coverage#2.5 · no · wait outcome counts channel-response{method:"wait"}, which a refusal before any frame never writes, and the chunk changes no wait response
metric-coverage#2.6 · yes · viola.state.recovered counts state-recovered lines that only read_snapshot_or_replay writes, and the readers' move onto it is an open P4 fork that binds it (Needs E5)
metric-coverage#2.7 · no · the skipped-record gauge reads list --json skips, list is not reached and torn-line healing is unchanged (Not needed)
metric-coverage#2.8 · no · parse-rejected counts per parser, and the chunk adds no parser or parse-rejected line (Not needed, the vt100 and agents rows are unchanged)
metric-coverage#2.9 · no · the compat counters count send-refused unknown and -32602 channel errors, and the chunk adds neither (its refusals are exit 21 and instance-unreachable)
metric-coverage#2.10 · no · the child and probe exit counter reads process-exit, and the chunk's new process-exit lines are self-subject refusals with no child added
metric-coverage#2.11 · yes · the new check paths must end no role through an uncaught unwind (Needs panic bullet), which this must-be-0 panic SLO rules over
metric-coverage#3.1 · yes · states the server-verify refusal line and the MCP no-line rule that the new CLI and MCP refusals must match, and the wait-before-frame rule (Needs keeps their order)
metric-coverage#4.0 · no · a report-only quantile snippet (the gates use max), and the session-end budget is witnessed by hyperfine max through gate --require perf (Needs), not by this snippet
