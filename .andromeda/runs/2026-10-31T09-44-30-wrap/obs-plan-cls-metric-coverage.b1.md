# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · yes · add-to · channel-response's detail is now three closed words, which the Counter's detail grouping key must carry (§5 label discipline)
metric-coverage#1.2 · no · channel-response gains only the closed detail word and its required fields do not move, so the raw duration_ms samples stand
metric-coverage#1.3 · yes · stale · src/run/budget.rs is reached whole and the budget.read_at snapshot it writes is not shown to hold on the changed file
metric-coverage#2.1 · no · the hook gate stays hyperfine max with no bound moved, and the hook per-role lines are unmoved
metric-coverage#2.2 · no · channel-response gains only the closed detail word, with its wire form, v 1 and WARN level unmoved, so duration_ms stays the sample
metric-coverage#2.3 · no · the send duration is read in src/run/send.rs, which the change does not reach
metric-coverage#2.4 · no · the counter still counts send-confirmed and send-refused lines by terminal state, and the screen moves only non-code refusal and detail values
metric-coverage#2.5 · no · the wait outcome field is unchanged, and -32602 wait answers gain only a detail word
metric-coverage#2.6 · no · viola-state is not reached, so state-recovered and its detail are unchanged
metric-coverage#2.7 · no · list --json's skipped count sits in a command the change does not reach, and it is no error document
metric-coverage#2.8 · no · the mcp-stdin oversize line measures as one parse-rejected with count 1, and no parse-rejected field moves
metric-coverage#2.9 · yes · add-to · F3: v_rejected now derives from channel-response -32602 with detail unsupported-version, so it counts version rejections alone and its over-count caveat falls, and compat.unknown now also takes non-code peer refusals
metric-coverage#2.10 · no · child and probe process-exit lines are unmoved, and the only process-exit change is the mcp role's self line, which this counter does not name
metric-coverage#2.11 · no · the panic hook in src/obs.rs and src/main.rs is not reached, and no counted panic was added
metric-coverage#3.1 · no · the cli server-verify process-exit at WARN is unmoved, and MCP still writes no line for it
metric-coverage#3.2 · no · refuse_untrusted still writes no line, with exit 21 and the instance-unreachable code unchanged
metric-coverage#4.0 · no · the snippet's diagnostics paths, hook-decision event and duration_ms, and the max gate are all unmoved
