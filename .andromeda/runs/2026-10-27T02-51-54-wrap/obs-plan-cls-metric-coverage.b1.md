# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · the send-refused grouping is unchanged, since the session adds no event, detail code or label
metric-coverage#1.2 · no · no duration_ms field or event changed, and the perf gate stays green
metric-coverage#1.3 · no · the gauge's budget.read_at and harness status snapshot are unchanged
metric-coverage#2.1 · no · the hook-decision duration gate is unmoved, since the placement is a run-start path and no budget moved
metric-coverage#2.2 · no · the channel-response frame path is untouched, since no frame method's code path changed
metric-coverage#2.3 · no · the send readback path is untouched, since no confirmed-send code path changed
metric-coverage#2.4 · no · the send-confirmed and send-refused values are unchanged, since no confirmed-send path changed
metric-coverage#2.5 · no · the wait outcome is unchanged, since no wait code path changed
metric-coverage#2.6 · no · state-recovered.detail is unchanged, since place_private_new adds no line, field or detail code
metric-coverage#2.7 · no · the skipped count is the events.ndjson reader's, which the session does not change
metric-coverage#2.8 · no · parse-rejected lines and parser values are unchanged, since no config parse path changed
metric-coverage#2.9 · no · the unknown and -32602 counters read values the session does not change, since mcp_tools.rs changes only a test boot
metric-coverage#2.10 · no · process-exit keeps its subject and field set, since the reached run refusal is an existing site (count 7 unchanged)
metric-coverage#2.11 · no · the panic event is unchanged, since the zero-panic G2 gate is green and no seam was added or moved
metric-coverage#3.1 · no · the wait note and its server-verify-failed process-exit are unchanged, since no wait path or line changed
metric-coverage#3.2 · no · strict.rs changed only a cfg(windows) test oracle, so the refusal still writes no line and its verdict holds
metric-coverage#4.0 · no · the snippet reads hook-decision session-end durations, which the session does not change; the fake agent's receipt is not a hook-decision line
