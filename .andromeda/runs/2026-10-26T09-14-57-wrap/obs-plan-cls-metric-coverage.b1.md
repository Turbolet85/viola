# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · a generic counter over any detail value, which the new home-level server-verify-failed lines fall under
metric-coverage#1.2 · no · a generic raw-sample histogram per event that the session does not reshape
metric-coverage#1.3 · yes · stale · liveness.rs is a reached file and liveness now carries the server verdict, so the liveness reading this gauge names has moved
metric-coverage#2.1 · yes · stale · the hook perf rows' timed path now includes the server check, so the hyperfine max this row gates on moved to 2.2–2.4 ms from 1.23–1.6 ms
metric-coverage#2.2 · no · the check runs before the channel.request span, so channel-response.duration_ms still times only the request
metric-coverage#2.3 · no · its sample starts at the send's arrival at the wrapper, after the CLI check, so the duration is unchanged
metric-coverage#2.4 · no · a mismatch exits 21 before any frame and writes a process-exit line, not a send-confirmed or send-refused one
metric-coverage#2.5 · no · the check runs in wait.block before the wait frame, so a refused wait writes no channel-response line to count
metric-coverage#2.6 · no · it counts state-recovered lines from the E5 replay path, which the session leaves unchanged
metric-coverage#2.7 · no · skipped records come from list --json and the events reader, neither of which the session reaches
metric-coverage#2.8 · no · no reached file holds a parser that writes parse-rejected lines
metric-coverage#2.9 · no · the check sends no frame, so it adds no -32602 response for fault_of to label
metric-coverage#2.10 · no · the new line is a subject self CLI exit with exit_code and no child_exit_status or exit_source, so the child and probe count is unchanged
metric-coverage#2.11 · no · panic hooks and catch sites are not reached by the session, so the panic count is unchanged
metric-coverage#3.0 · no · a report-only quantile over hook-decision duration_ms for session-end lines, a field and event the session keeps in shape
