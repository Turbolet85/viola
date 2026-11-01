# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · yes · the Counter type counts detail values over home-level role files, and the chunk's new home-level refusal details fall under it
metric-coverage#1.2 · no · a generic raw-duration histogram; the chunk adds no duration-carrying event of its own and states no value
metric-coverage#1.3 · no · budget.read_at and status liveness are gauge sources the chunk does not move; last_error is a different field
metric-coverage#2.1 · yes · hook-decision.duration_ms is gated by hyperfine max, and the chunk adds peer-euid and directory checks to the hook connect path
metric-coverage#2.2 · no · no gate on channel-response duration, and the new checks run before the first frame
metric-coverage#2.3 · no · the duration starts at the send's arrival at the wrapper, after the connect checks; the readiness and paste path stay as they are
metric-coverage#2.4 · yes · the chunk's client refusal reaches the send verb ahead of any frame, so it must write no send-refused line that this counter would count
metric-coverage#2.5 · yes · the chunk's client refusal reaches the wait verb ahead of any frame, so it must write no channel-response{method:"wait"} that this counter would count
metric-coverage#2.6 · no · state-recovered and torn-line healing are not changed
metric-coverage#2.7 · no · list --json skipped counts parse skips, which the home refusal does not change
metric-coverage#2.8 · no · parse-rejected per parser is not touched by the scope
metric-coverage#2.9 · yes · the chunk's client refusal reaches link and unlink ahead of any frame, so it must not write the invalid-params channel-response this counter reads
metric-coverage#2.10 · yes · the chunk adds process-exit lines (CLI connect refusal, wrapper pre-child refusal, and the cleanup exit 1 measured after a force-kill)
metric-coverage#2.11 · no · a whole-product panic SLO every change owes, not a rule the scope names
metric-coverage#3.1 · yes · the CLI server-verify-failed line shape is the one a new client refusal sits beside, and the Counter row's detail grouping counts it
metric-coverage#3.2 · yes · a CLI wait refused by client::refuse_untrusted writes no line, which the chunk's client refusals must keep
metric-coverage#4.0 · no · report-only quantile snippet over session-end hook-decision lines; the gates use max, and this chunk moves no session-end line
