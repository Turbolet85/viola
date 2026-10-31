# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · yes · the Counter row groups every `detail` value, so the moved process-exit details land in it, as #3.1 relies on for server-verify-failed
metric-coverage#1.2 · no · raw duration_ms latency per event, keyed on no exit-cause word
metric-coverage#1.3 · no · budget.read_at and liveness gauge, neither a word the chunk moves
metric-coverage#2.1 · no · hook-decision duration_ms latency; the hook edge adds no cause and the hook paths stay as they are
metric-coverage#2.2 · no · channel-response duration_ms latency per method, keyed on no moved word
metric-coverage#2.3 · no · send-confirmed duration_ms readback latency, keyed on no exit-cause word
metric-coverage#2.4 · no · counts send-confirmed and send-refused reasons, which exits 10–14 keep outside the catalogue
metric-coverage#2.5 · no · counts channel-response wait outcomes, which the chunk does not move and a refused wait never writes
metric-coverage#2.6 · no · state-recovered detail counter, a heal word outside the catalogue
metric-coverage#2.7 · no · list --json skipped gauge, a parse count the chunk does not change
metric-coverage#2.8 · no · parse-rejected counter per parser, outside the catalogue
metric-coverage#2.9 · no · counts unknown, unsupported-version, invalid-params and release-from-driver, none an exit-21 or exit-1 detail the chunk moves
metric-coverage#2.10 · no · counts process-exit by subject, child_exit_status and exit_source, none of them a detail the chunk moves
metric-coverage#2.11 · no · standing S6 · zero tolerance on event:"panic" lines across test homes
metric-coverage#3.1 · yes · states the server-verify-failed process-exit shape (exit 21, WARN, during connect, no channel-request line) that the chunk's exit-21 catalogue and client verify path carry
metric-coverage#3.2 · yes · states that a CLI wait refused by refuse_untrusted writes no line and that MCP's check_instance document has detail null, which the chunk's client.rs contract choice can move
metric-coverage#4.0 · no · hook session-end latency quantile snippet, keyed on hook-decision duration_ms; the hook paths and their gates stay
