# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · the counter's query filters send-refused lines only, and the session's socket-dir-refused details land on process-exit and hook-decision, outside that grouping
metric-coverage#1.2 · no · the raw duration_ms histogram per event is unchanged, and the session adds no timed event or sample rule
metric-coverage#1.3 · no · budget.read_at keeps the write rule and agent-run status is not among the session's changes, since the cleanup endpoint_gone change is a different subcommand
metric-coverage#2.1 · no · the hook gate stays hyperfine max, and the report says this run's hook means and max stay in band with no value moved
metric-coverage#2.2 · no · channel-response.duration_ms per method is unchanged, since refused frames write no channel-response and accepted calls' latency is not restated
metric-coverage#2.3 · no · send-confirmed duration_ms starts at the wrapper's arrival, the refusal checks run before connect or at accept for refused peers only, and the readiness gate is unchanged
metric-coverage#2.4 · no · the counter counts readback outcomes from send-confirmed and send-refused, and a socket-dir refused send writes a process-exit before any frame, reaching no readback state
metric-coverage#2.5 · no · wait outcome counts channel-response{method:"wait"} only, and a socket-dir refused wait writes no channel-response, as the row's existing counting already requires
metric-coverage#2.6 · no · state-recovered.detail comes from heal events, the report names no change to recovery, and the session moved only the home and socket-directory checks
metric-coverage#2.7 · no · list --json skipped counts skipped records, and the list home change moves who may read, with ScanRefused::Home, the check order and no-open unchanged
metric-coverage#2.8 · no · parse-rejected comes from parsers and viola-pty, and the session did not edit that crate or any parser
metric-coverage#2.9 · no · the unknown and v_rejected counters read send-refused and channel-response details the session did not change, and a socket-dir refusal sends no frame
metric-coverage#2.10 · no · viola.child.exit counts child and probe exits, and a socket-dir refusal is a self-subject exit with no child status
metric-coverage#2.11 · no · panics come from the panic hook, which the session did not touch
metric-coverage#3.1 · yes · stale · the row's "(Linux and Windows)" reach of server-verify-failed moves to Linux, Windows and macOS, where the macOS case is an euid mismatch alone
metric-coverage#3.2 · no · the strict-modes-failed refusal stays lineless, its line sites do not move, and the socket-dir refusal is a separate word that writes lines, which this row does not claim
metric-coverage#4.0 · no · the snippet selects hook-decision session-end duration_ms, and the session's new hook-decision lines sit on the dialog path with the snippet's fields unchanged
