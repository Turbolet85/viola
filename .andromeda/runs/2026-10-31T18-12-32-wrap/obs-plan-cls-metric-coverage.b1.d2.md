# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · the Counter row counts any detail value and its query filters send-refused only, and no RefusalReason or send-refused word was added, so the grown process-exit enum leaves it standing
metric-coverage#1.2 · no · the histogram samples duration_ms per event, and the session moves no duration_ms or event it names
metric-coverage#1.3 · no · the gauge reads harness snapshots (budget.read_at, agent-run status), and no status shape moved
metric-coverage#2.1 · no · hook-decision latency is untouched, since src/cmd/hook.rs is not in the diff and the hook rules stand
metric-coverage#2.2 · no · channel-response latency is untouched, since nothing in viola-channel is in the diff
metric-coverage#2.3 · no · it counts send-confirmed lines whose rule lives in src/run/send.rs, which the session does not reach
metric-coverage#2.4 · no · the send terminal words are unchanged, since no RefusalReason was added and no send-refused value is named
metric-coverage#2.5 · no · the wait outcome is a channel-response field, and the channel wire is untouched
metric-coverage#2.6 · no · state-recovered is untouched, since the replay and appender code is not in the diff
metric-coverage#2.7 · no · list --json skipped is a success-output field, no field was added to list output, and only the exit-21 refusal document changed
metric-coverage#2.8 · no · parse-rejected is a parser detail the session does not touch
metric-coverage#2.9 · no · the unknown and v_rejected counters are send and channel words, all unchanged, and v stays 1
metric-coverage#2.10 · no · the counter groups process-exit by subject, child_exit_status and exit_source, none of which moved, and no new line kind is written
metric-coverage#2.11 · no · the panic event is untouched, since the session adds no panic path
metric-coverage#3.1 · no · the server-verify-failed process-exit keeps its code, WARN level and during connect, all standing under the session's log rules
metric-coverage#3.2 · yes · contradict · the MCP wait refused by check_instance now carries detail strict-modes-failed on its isError body, so detail null no longer holds
metric-coverage#4.0 · no · the quantile snippet reads hook-decision duration_ms, which the session does not touch
