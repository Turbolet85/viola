# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · yes · the Counter groups send-refused by side and detail, and the chunk adds send-refused lines for control-character and empty-text on the CLI, MCP and wrapper sides that must carry both fields in a home-level role file
metric-coverage#1.2 · no · a generic raw-sample histogram type; the chunk adds no bucketing and no histogram of its own, and the duration-bearing lines it touches are bound by rows #2.1 and #2.2
metric-coverage#1.3 · yes · budget.read_at is set by a budget reading, and the chunk's checked budget.json read makes every failure, oversize included, no reading; the liveness half is not the chunk's (Not needed: heartbeat liveness)
metric-coverage#2.1 · yes · the chunk changes the hook stdin read and its fail-open hook-decision path, the hook-path change the row's perf trigger and hyperfine max gate cover
metric-coverage#2.2 · yes · the chunk's new -32602 and -32600 channel answers are channel-response lines this per-method histogram samples, so they must carry method and duration_ms
metric-coverage#2.3 · no · the sample is send-confirmed only; the chunk's paste refusals and the gate's input-not-ready degrade yield no confirmed line, and the gate.rs re-read checks only its catch_unwind site, not the GATE_MAX_WAIT wait the sample includes
metric-coverage#2.4 · yes · the outcome counter reads send-refused{refusal,detail,side}, which the chunk adds for control-character and empty-text on each deciding side and for the logged input-not-ready outcome
metric-coverage#2.5 · yes · the chunk's new invalid-params answers for method wait are channel-response lines this counter reads, and the no-body rule on channel-response must keep the outcome field the counter reads
metric-coverage#2.6 · yes · the Needs keep one state-recovered line per read_snapshot_or_replay call (snapshot-replayed or snapshot-unsupported-v) through the bounded snapshot read, which this counter reads
metric-coverage#2.7 · yes · the Needs keep the events.ndjson reader's three skip counts and the skip-and-count rule for unknown kinds and fields, the skipped records this gauge reports from list --json
metric-coverage#2.8 · yes · the chunk adds parse-rejected lines for parser config-json (one per bad key), claude-agents-json (oversize or malformed read) and vt100-feed (one per poisoning episode), all counted here by parser and detail
metric-coverage#2.9 · yes · the Needs name this over-count: each new invalid-params refusal from a channel name, driver or driven check adds to the -32602 count this row reads until the line tells the causes apart
metric-coverage#2.10 · yes · the chunk bounds the claude --version and claude agents --json spawns whose process-start lines the Needs name (names and counts only), and their process-exit lines are what this counter reads
metric-coverage#2.11 · yes · the Needs name the zero-unlogged-panics invariant, and a caught vt100 panic at the gate.rs and screen.rs catch_unwind sites still fires the hook's event:"panic" line this SLO counts, so the forced-panic test must run in the bounded exemption
metric-coverage#3.1 · yes · the MCP wait tool's instance-unreachable gets no line, the rule the chunk's mcp Need states, and the chunk's new wait invalid-params answers are channel-response lines inside this counter's boundary; the CLI server-verify line is owned by Unix endpoint and home hardening (Not needed)
metric-coverage#3.2 · yes · the MCP wait refused by check_instance writes no line, the rule the chunk's mcp Need states (instance-unreachable gets no line at all); the CLI client::refuse_untrusted half is not changed by the chunk
metric-coverage#4.0 · no · the snippet is a report-only quantile readout of session-end hook-decision samples (the gates use max, see #2.1), and no Need or changed line requires the chunk to honour it
