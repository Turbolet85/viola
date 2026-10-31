# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · the counter row's claim and its send-refused query hold; the session moved process-exit detail values, not send-refused's
metric-coverage#1.2 · no · raw duration_ms histogram of hook, channel and send events; the session adds no duration samples
metric-coverage#1.3 · no · gauge sources budget.read_at and liveness are not reached
metric-coverage#2.1 · no · hook-decision latency; hook events and src/cmd/hook.rs are not in the diff
metric-coverage#2.2 · no · channel-response latency; the channel wire is not reached
metric-coverage#2.3 · no · send-confirmed duration; src/run/send.rs is not in the reached files
metric-coverage#2.4 · no · send-confirmed and send-refused keep their fields and values; the session moved exit documents and process-exit codes only
metric-coverage#2.5 · no · channel-response wait outcome; the channel wire is not reached
metric-coverage#2.6 · no · state-recovered is not touched; snapshot replay and the appender are not in the diff
metric-coverage#2.7 · no · list --json skipped field holds; the session changes only the exit-21 document's cause in list
metric-coverage#2.8 · no · parse-rejected is not touched by the session
metric-coverage#2.9 · no · the unknown and v_rejected counts read send-refused and channel-response, both untouched; invalid-params is a channel-response detail
metric-coverage#2.10 · no · process-exit counted by subject, child_exit_status and exit_source; those fields are unchanged and no line is added
metric-coverage#2.11 · no · event panic is not touched
metric-coverage#3.1 · no · the CLI server-verify-failed process-exit at WARN during connect still holds; that value is not among the six new codes
metric-coverage#3.2 · yes · stale · MCP's check_instance refusal now carries detail strict-modes-failed in its isError body, so "detail: null" is no longer true
metric-coverage#4.0 · no · hook-decision quantile snippet; hook events are untouched
