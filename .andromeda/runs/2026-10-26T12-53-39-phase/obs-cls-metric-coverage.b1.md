# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · the generic Counter over every event, detail and refusal value, which the chunk's new values fall under unchanged
metric-coverage#1.2 · no · the raw-sample histogram type, which the chunk neither buckets nor adds a histogram row for (its samples bind through #2.2)
metric-coverage#1.3 · no · the gauge reads harness snapshots (budget.read_at, liveness via agent-run status) that the chunk does not touch
metric-coverage#2.1 · no · hook latency per hook_event, and the chunk changes no hook command or hook path
metric-coverage#2.2 · yes · the two methods become grouping values of viola.channel.duration_ms, so their channel-response.duration_ms samples bind
metric-coverage#2.3 · no · send confirm latency, on a send path the chunk does not change
metric-coverage#2.4 · no · send terminal state counts send-confirmed and send-refused lines, which the chunk neither adds nor changes
metric-coverage#2.5 · no · viola.wait.outcome groups only the method "wait" channel-response, which the new methods do not add to
metric-coverage#2.6 · no · the heal counter reads state-recovered lines from the events.ndjson append's torn-line heal, and the chunk adds no recovery path, chaos test or change to that append
metric-coverage#2.7 · no · the list --json skipped gauge, which the chunk does not touch (its link-name re-parse sits on the verbs' result path, not the board scan)
metric-coverage#2.8 · no · parse-rejected counts per parser, and the chunk names no parser or parse-rejected line (its link-name re-parse ends in a closed refusal or a skip)
metric-coverage#2.9 · yes · the two new -32602 refusals (a driven that is not the serving wrapper's name, a driver equal to driven) join the viola.compat.v_rejected over-count
metric-coverage#2.10 · no · viola.child.exit counts child and probe exits, and the verbs' process-exit lines are subject "self" in the existing shape with no child or probe process added
metric-coverage#2.11 · no · standing S7 · zero unlogged panics and no retry-once policy in any new test
metric-coverage#3.1 · yes · the CLI server-verification refusal is one process-exit at WARN in cli-<name>.ndjson with no channel line, the shape the verbs' connect path must write
metric-coverage#4.0 · no · the quantile snippet reads hook-decision session-end samples only, and the chunk changes no hook or gated latency
