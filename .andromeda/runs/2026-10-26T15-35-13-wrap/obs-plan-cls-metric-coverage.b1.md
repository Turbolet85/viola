# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · the generic occurrence counter names no value the session changes, and the diagnostics files it reads keep their shape (§5 Untouched: no derived-metric source changed except v_rejected)
metric-coverage#1.2 · no · the histogram row is a generic raw-sample rule over duration_ms and names no event the session adds or changes (§5 Untouched)
metric-coverage#1.3 · no · the gauge reads budget.read_at and liveness, and the session changes neither the budget gate nor the heartbeat (§10 and §4 Untouched)
metric-coverage#2.1 · no · hook latency is hook-decision.duration_ms and no hook path changed (§10 and §5 Untouched)
metric-coverage#2.2 · no · the row is per method with no enumerated method set, so link and unlink join it as new label values without any change to its rule
metric-coverage#2.3 · no · send confirm latency is the duration_ms of send-confirmed lines, and the session changes no send path (its changes are the link and unlink verbs and their channel methods)
metric-coverage#2.4 · no · send outcome counts send-confirmed and send-refused lines, which the session neither adds nor changes
metric-coverage#2.5 · no · wait outcome counts channel-response lines with method wait only, and the session does not touch wait
metric-coverage#2.6 · no · state recovery is counted from state-recovered lines, and the torn-append heal path is not reached (§4 Untouched)
metric-coverage#2.7 · no · the skipped count comes from the list read model, and the new unlink event kind is already in the closed set so the skip count sees no new skip
metric-coverage#2.8 · no · parse-rejected lines come from config and state parsers the session does not add or change
metric-coverage#2.9 · yes · stale · -32602 now also answers a bad link or unlink name or pair and a driven that is not the serving wrapper, so the cause list this over-count gives has moved
metric-coverage#2.10 · no · the counter groups process-exit lines by their existing fields, and the link and unlink exit lines take that same kind and shape (no source changed, §5 Untouched)
metric-coverage#2.11 · no · panics are event panic lines and no panic path or hook changed (§7 Untouched)
metric-coverage#3.1 · no · link and unlink write the same one process-exit line at WARN in cli-<driven>.ndjson on server-verify failure, so the claim about the CLI's server-verification refusal still holds
metric-coverage#4.0 · no · the quantile snippet reads hook-decision session-end lines, and no hook path changed (§10 Untouched)
