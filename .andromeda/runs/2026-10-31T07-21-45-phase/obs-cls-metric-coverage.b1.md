# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · generic occurrence count over all event, detail and refusal values in home-level files, naming no line or code the chunk changes
metric-coverage#1.2 · no · raw duration samples per event with no bucketing, and the chunk changes no duration or error body
metric-coverage#1.3 · no · budget.read_at and liveness are harness snapshots the chunk does not touch, and liveness flip timing is in Not needed
metric-coverage#2.1 · no · the hyperfine hook gate is a generic perf re-check every hook change owes, and the chunk names no hook-decision line or latency value
metric-coverage#2.2 · yes · channel-response.duration_ms is a field of the line the chunk re-reads against the §6 catalog, including its Drop guard emission
metric-coverage#2.3 · no · send-confirmed duration is success-path timing in src/run/send.rs, which the chunk does not change
metric-coverage#2.4 · yes · send-refused refusal and detail carry the closed codes the chunk fixes for MCP send refusals
metric-coverage#2.5 · yes · channel-response method and outcome are fields the chunk re-reads against the §6 catalog, and its cause split may change that line
metric-coverage#2.6 · no · state-recovered lines are crash-recovery telemetry the chunk does not change, as Not needed states
metric-coverage#2.7 · no · list --json skipped is a success-path count the chunk does not change, and the list isError carries codes only
metric-coverage#2.8 · yes · the child pipe drain the chunk edits emits one parse-rejected{parser:"claude-agents-json"} that this counter groups by parser
metric-coverage#2.9 · yes · the -32602 causes channel-response cannot tell apart, which fault_of labels unsupported-version, are the over-count this row records
metric-coverage#2.10 · yes · the short-lived CLI fault path writes process-exit{detail:"internal-error"}, which this counter groups by subject, child_exit_status and exit_source
metric-coverage#2.11 · no · standing S6 · a test run leaves zero event:"panic" lines in home-level role files and uses no retry-once policy
metric-coverage#3.1 · yes · the CLI server-verify-failed detail must stay a closed code, and the codes-only rejected-call rule must settle MCP wait's no-line case for that refusal
metric-coverage#3.2 · yes · the chunk's own rule that instance-unreachable writes no line, with its isError code as the only record, is what this note states
metric-coverage#4.0 · no · a report-only session-end quantile snippet over hook-decision lines, and the chunk changes no session-end hook line
