# Classified — obs · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · no · standing S4 · every span is `#[instrument(skip_all, fields(...))]` or `info_span!` with an allow-list, without `err` or `ret`
telemetry-strategy#1.2 · yes · refusal and fault lines the chunk emits or changes (codes-only rejected call, internal-error process-exit, channel-response) must be closed-enum events
telemetry-strategy#1.3 · yes · the typed errors' Display (thiserror, hand-written PtyError and ServeError) must become fixed messages, and process-exit keeps a distinct detail code
telemetry-strategy#1.4 · yes · the derived viola.compat.v_rejected over-count is a jq count of channel-response values, so the -32602 cause split feeds it and no in-process instrument may be added
telemetry-strategy#1.5 · yes · the codes-only rejected-call and fault lines must copy no event or send payload into process logs
telemetry-strategy#2.1 · yes · a panic payload and backtrace stay in the instance detail file, so a panic reaching a surface may only give the fixed internal error
telemetry-strategy#3.1 · yes · IPC vector: the channel JSON-RPC error bodies and the channel-response line fall under the §6 channel field allow-list
telemetry-strategy#3.2 · yes · paste text: a refused send's isError and its codes-only line carry no paste text, serde-sourced text included
telemetry-strategy#3.3 · yes · the viola hook statusline path (bounded status-line read, oversize print, refused instance check leaving no trace) binds; the event and dialog paths do not
telemetry-strategy#3.4 · yes · MCP vector: the viola mcp isError bodies, the MCP stdin bound and the rejected-call codes-only line
telemetry-strategy#3.5 · yes · CLI/env vector: the R8-stripped CLAUDE* never-log floor, with env_stripped_count, env_stripped_known and env_kept as counts or names only
telemetry-strategy#3.6 · yes · filesystem vector: owner-readable detail files (0600 in 0700) carrying content and drift reports, and the secret scan before upload
telemetry-strategy#3.7 · yes · child spawn vector: the claude --version and claude agents --json pipe drain and its parse-rejected{parser:"claude-agents-json"} line
telemetry-strategy#3.8 · yes · supply-chain vector: rmcp, portable_pty, sysinfo and interprocess text stays off all three surfaces with their targets off in the sink; cargo-deny is not reached
telemetry-strategy#3.9 · no · the chunk changes no correlation scheme (Not needed) and no Need names corr or conn
telemetry-strategy#3.10 · yes · perf budget: the Path 1 timeout watch on path1_run_start_one_home_in_order is re-tallied on CI pushes against its kill line in the §10 budget table
telemetry-strategy#3.11 · yes · chaos: the internal-error process-exit carries the §6 exit_source and during fields; state-recovered lines are not reached
telemetry-strategy#3.12 · yes · multi-platform: the owner-readable detail-file leg of v1-47 (0600 in 0700, home DACL on Windows) that the chunk re-reads
telemetry-strategy#3.13 · yes · error-budget SLO: a panic that a verb or handler fault turns into internal error must still leave its role-file record
telemetry-strategy#3.14 · no · events.ndjson and diagnostics/ retention and rotation stay unchanged (Not needed)
telemetry-strategy#3.15 · yes · Founder Directions 1/2/5: the detail chain and drift_report fields and the channel-response cause value must be recorded in §6 and match the §3 schema verbatim
telemetry-strategy#3.16 · yes · Founder Direction 4: the frame verbs' --json instance-unreachable document with a null detail; the revive start document is another chunk's (Not needed)
telemetry-strategy#3.17 · yes · installed-CLI measured: the parse-rejected{parser:"claude-agents-json"} line the child pipe drain writes on a parse failure
telemetry-strategy#3.18 · no · the vt100 feed and screen read are outside the chunk (Not needed) and no error body reads screen text
