# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · yes · the detail and refusal grouping counts codes this chunk closes (process-exit internal-error, send-refused detail and refusal), in home-level files only
metric-coverage#1.2 · no · a generic raw-sample histogram of duration_ms that names no event the chunk changes and no sample the chunk re-reads
metric-coverage#1.3 · no · harness snapshot gauges (budget.read_at, liveness); liveness flip timing is Not needed and budget.read_at is outside the chunk
metric-coverage#2.1 · no · hook-decision latency and its hyperfine gate; the chunk names no hook-decision line and re-measures no hook latency
metric-coverage#2.2 · yes · channel-response.duration_ms is a field of the line the chunk contract-reads, Drop guard emission included, against the §6 catalog and schema
metric-coverage#2.3 · no · send-confirmed duration_ms; the chunk changes no send-confirmed line and src/run/send.rs is not reached
metric-coverage#2.4 · yes · send-refused{refusal,detail,side} carries the refusal and detail codes the chunk closes (the Refusal carry, MCP send isError codes)
metric-coverage#2.5 · yes · channel-response{method:"wait"}.outcome is a field of the line the chunk contract-reads against the §6 catalog and schema
metric-coverage#2.6 · no · state-recovered lines are crash-recovery telemetry the chunk does not change (Not needed)
metric-coverage#2.7 · no · list --json skipped is a success-document count the chunk does not change; the chunk changes only list's error codes
metric-coverage#2.8 · yes · the child pipe drain's parse failure writes one parse-rejected{parser:"claude-agents-json"} that this counter groups by parser
metric-coverage#2.9 · yes · names the -32602 causes (ProtocolError::InvalidParams, fault_of) and the v_rejected over-count that the carry must tell apart
metric-coverage#2.10 · yes · the fault path's new process-exit{detail:internal-error} is a process-exit line this counter groups by subject, child_exit_status and exit_source
metric-coverage#2.11 · no · standing S6 · A test run leaves zero event:"panic" lines in home-level role files and uses no retry-once policy
metric-coverage#3.1 · yes · the CLI server-verification refusal's error body and process-exit detail server-verify-failed fall in the chunk's closed --json and detail catalogue
metric-coverage#3.2 · yes · the instance-unreachable refusal writes no line, which the chunk's rejected-MCP rule and its instance-check --json document with null detail must keep
metric-coverage#4.0 · no · a report-only session-end quantile snippet over hook-decision; nothing it reads changes
