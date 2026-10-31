# Classified — obs · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · no · standing S4 · every span is #[instrument(skip_all, fields(...))] or info_span! with an allow-list, a convention the task lists
telemetry-strategy#1.2 · yes · the refusal, fault and process-exit lines the chunk adds or changes must sit in the closed event enum and the per-role JSON-per-line file
telemetry-strategy#1.3 · yes · typed thiserror errors with fixed Display and process-exit detail codes are the chunk's Display list and its internal-error exit
telemetry-strategy#1.4 · yes · the derived metric viola.compat.v_rejected reads channel-response, and the chunk may add no in-process instrument
telemetry-strategy#1.5 · yes · obs never copies event payloads into process logs, so the codes-only rejected-call line and the channel-response line carry no payload
telemetry-strategy#2.1 · yes · the refused-instance exemptions bind the instance-unreachable path, which writes no line, and any panic the fault path catches still writes its one record
telemetry-strategy#3.1 · yes · the IPC vector, so channel error.data and channel-response fields must pass the channel field allow-list and the §8 table
telemetry-strategy#3.2 · yes · the paste vector, so paste text may not reach the codes-only rejected-call line for send or answer; paste instrumentation itself is Not needed
telemetry-strategy#3.3 · yes · the statusline refused-instance no-trace and the dialog path's no config.json read on a refused instance bind the statusline and dialog-hold BOUNDARIES rows the chunk edits
telemetry-strategy#3.4 · yes · the MCP vector, so isError results and the codes-only rejected-call line carry codes only, with no upstream text or path
telemetry-strategy#3.5 · yes · the CLI/env vector, so stripped CLAUDE* values stay out of every sink, the strip records only env_stripped_count, env_stripped_known and env_kept, and the §7 fixed messages apply
telemetry-strategy#3.6 · yes · the filesystem vector, so detail-file 0600 in 0700, the closed detail codes and the secret scan before upload bind the v1-47 legs the chunk re-reads
telemetry-strategy#3.7 · yes · the child-spawn vector, so the claude agents JSON pipe drain gives one parse-rejected{parser:"claude-agents-json"} and logs no row verbatim
telemetry-strategy#3.8 · yes · the supply-chain vector, so upstream rmcp, portable_pty, sysinfo and interprocess text must stay out of all three surfaces and the sink targets the chunk sets off
telemetry-strategy#3.9 · no · the chunk changes no correlation scheme (Not needed: hook-invoked corr timing and W3C trace context)
telemetry-strategy#3.10 · no · the chunk names no budget row, adds no timing instrument and re-measures none, so the §5 perf gates are a re-check every change owes
telemetry-strategy#3.11 · yes · the process-exit line the fault path writes with detail internal-error must carry exit_source and during as §6 defines them
telemetry-strategy#3.12 · yes · the Unix 0600 in 0700 leg of log file location binds the detail-file re-read, while the Windows DACL and per-OS append legs do not
telemetry-strategy#3.13 · yes · the §7 typed errors and the zero-unlogged-panics SLO bind the fault path, which must leave a process-exit record and a panic record for any panic it catches
telemetry-strategy#3.14 · no · events.ndjson and diagnostics/ retention and rotation are unchanged by the chunk (Not needed)
telemetry-strategy#3.15 · yes · founder directions 1, 2 and 5, so the chunk's new log fields and detail codes follow the §3 schema verbatim and are recorded in §6
telemetry-strategy#3.16 · yes · founder direction 4, so the closed instance-unreachable and strict-modes-failed --json documents, with no line and no detail code on the refusal, bind the chunk's CLI error bodies
telemetry-strategy#3.17 · yes · installed-CLI measured, so the parse-rejected{parser:"claude-agents-json"} line the child pipe drain writes feeds the §5 counters and the §6 parse-rejected entry
telemetry-strategy#3.18 · no · no screen scraping binds the vt100 gate outcome only, which the chunk does not touch (vt100 feed and PTY pump are Not needed)
