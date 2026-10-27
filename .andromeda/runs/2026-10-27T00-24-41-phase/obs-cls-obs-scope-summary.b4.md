# Classified — obs · obs-scope-summary.b4

## Answers
obs-scope-summary#22.3 · no · the mcp sink (mcp.ndjson, its stderr fallback, its resolved-home check before the open), which no chunk path reaches (task Not needed: the viola mcp surface)
obs-scope-summary#22.4 · no · the mcp process identity (process "mcp", the server's VIOLA_NAME as from), which no chunk path reaches (task Not needed: the viola mcp surface)
obs-scope-summary#22.5 · no · the mcp isError codes and the codes-only rule for rejected mcp calls, which the chunk does not change (task Not needed: the viola mcp surface)
obs-scope-summary#23.1 · yes · the sync and Tokio-free rule the Needs name for viola-state instrumentation, and run's and verify's init order (home check before the log, process-start before the pin step)
obs-scope-summary#24.1 · yes · the one-file-per-role sink, single append write per line and no-stdout rule that the new WARN or ERROR line and verify's cli-<name> lines take (its format and RUST_LOG rule stay unchanged)
obs-scope-summary#25.0 · yes · the required fields, closed event enum and corr rule (null on process-start and process-exit) that run's and verify's process lines and the new absent-copy detail code carry
obs-scope-summary#26.1 · yes · the run, cli and detail-<process> file basenames, the logs wrapper the witness reads through status and logs, and the check_diagnostics refusal the typed-error detail chain goes through
obs-scope-summary#27.1 · yes · the surfaces of the run start-to-spawn scenario the pin step sits in (cli run, channel bind, snapshot, heartbeat, events.ndjson), which the witness reads
obs-scope-summary#27.2 · yes · the run.pin_copy must-trace span, process-start for the child and process-exit on failure, and the wheel, budget-gate, session-start event order the witness reads
obs-scope-summary#27.3 · yes · run's required process fields and exit-1 detail codes (the sha-256 mismatch telemetry and the new absent-copy cause), with a home-tree refusal writing no line
obs-scope-summary#27.4 · yes · names Critical Path 1 (tests excerpt), the run-start scenario check the scope names and the pin step sits in
obs-scope-summary#28.1 · no · the Confirmed send surfaces (CL-1, SessionStart after /clear or /compact), which the task marks Not needed (no send path changes)
obs-scope-summary#28.2 · no · the send must-trace spans and log events, which the task marks Not needed (no send path changes)
obs-scope-summary#28.3 · no · the send required fields and refusal details, which the task marks Not needed (no send path changes)
obs-scope-summary#28.4 · no · names Critical Path 2 (send), a scenario the task marks Not needed
obs-scope-summary#29.1 · no · the wait and last readback surfaces, which the task marks Not needed (no frame client changes)
obs-scope-summary#29.2 · no · the wait.block and run.wait_dispatch spans, which the task marks Not needed (no frame client changes)
obs-scope-summary#29.3 · no · the wait required fields (after, timeout_ms, outcome), which the task marks Not needed (no frame client changes)
obs-scope-summary#29.4 · no · names Critical Path 3 (wait), a scenario the task marks Not needed
obs-scope-summary#30.1 · no · the dialog surfaces (PreToolUse, PermissionRequest, hook.dialog, answer), which the task marks Not needed (no dialog path changes)
obs-scope-summary#30.2 · no · the dialog must-trace spans (hook.handle, run.dialog_register, run.dialog_await), which the task marks Not needed (no dialog path changes)
obs-scope-summary#30.3 · yes · the hook fail-open rule the Needs keep, a refused or absent instance exits 0 with empty streams and no line, with no pid, path or endpoint in a line (its dialog fields are not needed)
obs-scope-summary#30.4 · no · names Critical Paths 4 and 7 (dialog answering, the unverified-CLI answer branch), which the task marks Not needed
obs-scope-summary#31.1 · no · the wheel and release surfaces, which the task marks Not needed (no wheel change)
obs-scope-summary#31.2 · no · the wheel and release spans and release-from-driver event, which the task marks Not needed (no wheel change)
