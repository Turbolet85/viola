# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · generic Counter over any event or detail value; its send-refused example is outside the catalogue, and the new process-exit details are counted with no code change
metric-coverage#1.2 · no · latency histogram of duration_ms per event; no exit-cause code touches a duration
metric-coverage#1.3 · no · gauge over budget.read_at and liveness; no exit-cause code or liveness word moves
metric-coverage#2.1 · no · hook latency per hook_event; the hook paths stay as they are and no hook cause is added
metric-coverage#2.2 · no · channel call latency keyed on channel-response.duration_ms per method; nothing it keys on moves
metric-coverage#2.3 · no · send confirm latency keyed on send-confirmed duration_ms; send-confirmed lines are unchanged
metric-coverage#2.4 · no · send terminal state keyed on send-refused refusal, detail and side; exits 10–14 and their send-refused details are outside the catalogue
metric-coverage#2.5 · no · wait outcome keyed on channel-response outcome for answered calls; the chunk moves no outcome and a refused wait writes no response
metric-coverage#2.6 · no · heal and recovery keyed on state-recovered detail; no recovery line changes
metric-coverage#2.7 · no · skipped gauge keyed on list --json skipped; the chunk moves no skipped field
metric-coverage#2.8 · no · parse-rejected detail is a parser word, not an exit-cause code
metric-coverage#2.9 · yes · reads channel-response error detail invalid-params and release-from-driver, which the error layer crossing channel error.data and the link and unlink unwrapped-name cause may move
metric-coverage#2.10 · no · child exit counter keyed on process-exit subject, child_exit_status and exit_source; the chunk changes none of those fields
metric-coverage#2.11 · no · standing S6 · zero tolerance on event:"panic" lines across test homes
metric-coverage#3.1 · yes · the server-verify-failed process-exit at WARN with during:"connect" and MCP's no-line return, both in the client.rs server check the chunk edits
metric-coverage#3.2 · yes · the strict-modes refusal in client::refuse_untrusted writes no line, so instance-unreachable's isError code is its only record, and the chunk edits that refusal
metric-coverage#4.0 · no · hook-decision session-end quantile snippet; the hook paths stay as they are and no hook cause is added
