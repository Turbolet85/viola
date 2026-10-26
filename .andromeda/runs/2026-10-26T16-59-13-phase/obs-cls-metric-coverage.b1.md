# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · generic counter over event, detail and refusal values, a reporting method; the new refusal lines are fixed by 3.1 and the Needs, and the chunk adds no metric
metric-coverage#1.2 · no · generic raw-sample histogram, a reporting method; its hook latency bound is the gate in 2.1, and the chunk adds no histogram
metric-coverage#1.3 · no · generic gauge of budget.read_at and agent-run liveness, naming no unit the chunk edits, and budget.read_at is unchanged
metric-coverage#2.1 · yes · the strict-modes check on every hook path counts inside the hook perf budgets, gated by hyperfine max (Needs)
metric-coverage#2.2 · no · the new check runs client-side before the frame, so channel-response latency is unchanged
metric-coverage#2.3 · no · the send confirm sample starts at the wrapper's arrival, after the new pre-frame check, and the send path after the frame is untouched
metric-coverage#2.4 · no · a CLI send refused by the new check writes process-exit and no send-refused or send-confirmed line, so the outcome counts are unchanged
metric-coverage#2.5 · no · counts answered wait channel-responses only, and a wait refused before its frame writes none (the shape is bound by 3.1)
metric-coverage#2.6 · no · counts state-recovered lines the chunk does not write, the readers' move being the P4 fork neither planned in nor out
metric-coverage#2.7 · no · counts list --json skipped, and list.rs is not reached by the chunk
metric-coverage#2.8 · no · counts parse-rejected lines per parser, and the chunk adds or changes no parser rejection
metric-coverage#2.9 · no · counts send-refused unknown and -32602 responses, and the chunk adds neither (its MCP refusal is instance-unreachable, and link and unlink already run check_instance)
metric-coverage#2.10 · no · counts child and probe process-exit lines, and the new refusal exits carry subject self, so the child count is unchanged
metric-coverage#2.11 · yes · must be 0, so no new check path may end a role through an uncaught unwind (Needs)
metric-coverage#3.1 · yes · the new CLI frame-verb refusals take the WARN process-exit shape in cli-<name>.ndjson, a refused wait writes no channel-response, and MCP frame tools write no line (Needs)
metric-coverage#4.0 · no · report-only quantile snippet over session-end samples, the gate being hyperfine max as in 2.1, so no chunk gate reads quantiles
