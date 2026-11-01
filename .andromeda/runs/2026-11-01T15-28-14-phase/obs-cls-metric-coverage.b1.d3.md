# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · the send-refused recipe counts home-level role-file lines, and the chunk's connect-time refusals end before any role file opens
metric-coverage#1.2 · no · a generic duration_ms latency histogram, and the chunk adds no duration_ms line and no metric
metric-coverage#1.3 · no · the gauge reads budget.read_at and status liveness snapshots, which the chunk does not change
metric-coverage#2.1 · yes · the chunk's hook connect check and its fail-open hook-decision line feed this duration_ms histogram under its hyperfine max perf gate
metric-coverage#2.2 · no · channel-response latency runs from the first frame, and a refused connection writes no channel-request or channel-response pair
metric-coverage#2.3 · no · send-confirmed duration runs from the wrapper's arrival past the connect, which the scope keeps as it stands
metric-coverage#2.4 · yes · the send verb's ahead-of-frame refusal through client::refuse_untrusted is a send terminal state, so any send-refused detail it writes must be a closed code the counter admits
metric-coverage#2.5 · yes · a wait refused at the connect check before any frame must write no channel-response, so the wait outcome counter stays answered-only
metric-coverage#2.6 · no · state-recovered heal and replay are excluded by the scope
metric-coverage#2.7 · no · the skipped count is the record parse result, which the home check does not change for an admitted home
metric-coverage#2.8 · no · parse-rejected covers parser rejections, the scope excludes the vt100 feed's parse-rejected lines, and the chunk adds no parser
metric-coverage#2.9 · no · compat counts unknown refusals, unsupported-version and invalid-params frames, and the chunk's refusals produce none of them
metric-coverage#2.10 · yes · the chunk's new process-exit lines (CLI connect refusal, wrapper pre-child refusal) are counted by subject, child_exit_status and exit_source
metric-coverage#2.11 · no · a generic panic SLO every change owes, and the chunk adds no panic site
metric-coverage#3.1 · yes · the new client euid refusal sits beside the server-verify-failed CLI exit that this counter groups by detail
metric-coverage#3.2 · yes · the connect-time refusals must state where they fall against the instance check, and this rule fixes that the instance check refusal writes no line
metric-coverage#4.0 · no · a report-only quantile snippet over session-end hook-decision samples with no gate, which the chunk does not change
