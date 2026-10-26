# Classified — obs-plan · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · no · the span names and areas are unchanged and the new check sits inside the existing dispatch spans
telemetry-strategy#1.2 · no · the closed event enum and schemas are unedited and the new refusal lines use existing events
telemetry-strategy#1.3 · yes · stale · its hook-dispatch catch site is reached through src/cmd/hook.rs, which the report's "not reached" does not clear, and the exit-21 detail set gains server-verify-failed
telemetry-strategy#1.4 · no · the metric naming and label cardinality are unchanged and the moved hook timings sit in Section 10, not Section 5
telemetry-strategy#1.5 · no · the new process-log lines carry codes only and the events.ndjson sink and retention are unchanged
telemetry-strategy#2.1 · yes · stale · the bounded exemptions name hook arms in src/cmd/hook.rs, whose two snapshot reads became one and whose connect now verifies the server
telemetry-strategy#3.1 · no · the peer-pid read logs nothing and the absence of pid and start time in the new lines is asserted in Section 8, which this row cites
telemetry-strategy#3.2 · no · the server check reads no paste text and the new refusal writes process-exit, not send-refused
telemetry-strategy#3.3 · yes · add-to · the new hook-stdin fail-open cause (a server that is not the recorded wrapper) logs a hook-decision line and writes no frame, which this row does not yet carry
telemetry-strategy#3.4 · no · a frame-tool mismatch returns the codes-only isError and writes no log line, the same as the MCP unreachable path already covered
telemetry-strategy#3.5 · no · the new CLI refusal messages are fixed codes only and the env-stripped scenario is untouched
telemetry-strategy#3.6 · no · the filesystem refusal codes, the 0600/0700 modes and the strict-modes ledger rule are untouched
telemetry-strategy#3.7 · no · no child spawn or run start sequence changed
telemetry-strategy#3.8 · no · no dependency was added or bumped and viola-channel's normal graph stays tokio-free
telemetry-strategy#3.9 · yes · add-to · the dialog hook's server-verify refusal writes hook-invoked and hook-decision with corr null before any hook.dialog response, a corr-null case the propagation rule must now carry
telemetry-strategy#3.10 · no · the row cites Section 5 and the Section 10 budget table, both still in place, and the budget value is unchanged
telemetry-strategy#3.11 · no · the new process-exit line uses existing fields of the closed schema, so the instrumentation this row names still holds
telemetry-strategy#3.12 · no · the per-OS pointers (log location, concurrent append, endpoint kind, per-OS artifacts) are untouched and the macOS identity gap was already recorded
telemetry-strategy#3.13 · yes · stale · its zero-unlogged-panics claim rests on the hook-dispatch catch sites, and src/cmd/hook.rs is reached whole, so the report's "catch sites not reached" does not clear it
telemetry-strategy#3.14 · no · the log file location and the absence of rotation are untouched
telemetry-strategy#3.15 · no · the schema is not edited and the Section 6 entries it cites still hold
telemetry-strategy#3.16 · yes · add-to · server-verify-failed is now a written exit-21 cause distinct from instance-dead, with no path or pid, which the Founder Direction 4 coverage must carry
telemetry-strategy#3.17 · no · the CLI measurement and the ledger read are untouched and the new check reads the wrapper's pid, not the CLI's
telemetry-strategy#3.18 · no · the new check reads only the snapshot's pid and start time, never the screen, so the vt100 gate outcome claim holds
