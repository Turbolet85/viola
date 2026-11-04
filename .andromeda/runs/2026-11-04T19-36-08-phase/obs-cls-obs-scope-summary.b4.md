@answer n=1 entity=obs-scope-summary#22.3 v=yes
why: mcp's file sink (<home>/diagnostics/mcp.ndjson, stdout forbidden) is the one file sink and line schema the chunk must keep identical on every OS (Needs 3).
@answer n=2 entity=obs-scope-summary#22.4 v=yes
why: the process="mcp" and instance identity fields on every line are the required fields the one schema re-asserts on every OS (Needs 4, 5).
@answer n=3 entity=obs-scope-summary#22.5 v=yes
why: the closed instance-unreachable detail list names server-verify-failed, socket-dir-refused and strict-modes-failed, the Unix-reached closed words (Needs 5, 6).
@answer n=4 entity=obs-scope-summary#23.1 v=yes
why: the per-process init order (panic hook first, strict-modes check before the open, explicit writer, process-start) is what the Unix panic hook, refusals and process-start logging must follow (Needs 2, 8, 10); its OTel clause is not needed but the order binds.
@answer n=5 entity=obs-scope-summary#24.1 v=yes
why: the tracing-subscriber JSON config, one file per role, no stdout sink, RFC 3339 millisecond UTC timestamps and one write per line are what the same line schema on every OS must keep (Needs 3, 4).
@answer n=6 entity=obs-scope-summary#25.0 v=yes
why: the required line fields (timestamp, level, target, message, event, corr, process, instance) and the closed event enum with its corr mapping are the schema the chunk re-asserts on every OS (Needs 4, 5).
@answer n=7 entity=obs-scope-summary#26.1 v=yes
why: the home-level role file locations and the 0600 in 0700 modes are what the log file location and modes on every OS must match (Needs 20).
@answer n=8 entity=obs-scope-summary#27.1 v=no
why: a surfaces label for the run-start flow that states no rule the chunk must honour; its rules sit in #27.2 and #27.3.
@answer n=9 entity=obs-scope-summary#27.2 v=yes
why: the must-trace pty.spawn span and the process-start for run and the child with process-exit on failure are what the Unix pty seam must emit (Needs 1, 2).
@answer n=10 entity=obs-scope-summary#27.3 v=yes
why: the run and revive-start exit-1 detail codes including socket-dir-refused, and the rule that a home-tree refusal writes no line, are the Unix pre-child refusals and the revive start arm (Needs 8, 9).
@answer n=11 entity=obs-scope-summary#27.4 v=no
why: a provenance pointer to the Critical Path 1 sources that states no rule the chunk must honour.
@answer n=12 entity=obs-scope-summary#28.1 v=no
why: a surfaces label for the send path that states no rule; its rules sit in #28.2 and #28.3.
@answer n=13 entity=obs-scope-summary#28.2 v=yes
why: the must-trace pty.paste_write span and the send-issued and send-confirmed events are what the Unix pty seam paste and its channel telemetry must emit (Needs 2, 7).
@answer n=14 entity=obs-scope-summary#28.3 v=yes
why: the send line fields and closed detail words (input-not-ready, turn-running and the rest) are the closed catalog the chunk re-asserts on every OS (Needs 5, 7).
@answer n=15 entity=obs-scope-summary#28.4 v=no
why: a provenance pointer to the send-path sources that states no rule the chunk must honour.
@answer n=16 entity=obs-scope-summary#29.1 v=no
why: a surfaces label for the wait path that states no rule; its rules sit in #29.2 and #29.3.
@answer n=17 entity=obs-scope-summary#29.2 v=yes
why: the channel-request and channel-response events on the wait path are what the Unix channel endpoint telemetry must emit (Needs 7).
@answer n=18 entity=obs-scope-summary#29.3 v=yes
why: wait's exit 21 with detail socket-dir-refused during connect and no channel-request, and endpoint-lost on a killed wrapper, are the Unix channel refusals the chunk adds (Needs 5, 7).
@answer n=19 entity=obs-scope-summary#29.4 v=no
why: a provenance pointer to the Critical Path 3 sources that states no rule the chunk must honour.
@answer n=20 entity=obs-scope-summary#30.1 v=no
why: a surfaces label for the dialog path that states no rule; its rules sit in #30.2 and #30.3.
@answer n=21 entity=obs-scope-summary#30.2 v=yes
why: the channel.request(hook.dialog) span and the hook-invoked, dialog-raised, dialog-answered and hook-decision events are what the Unix channel and hook path must emit (Needs 7, 19).
@answer n=22 entity=obs-scope-summary#30.3 v=yes
why: the server-verify-failed refusal on Linux and macOS (an euid mismatch alone on macOS), socket-dir-refused, and the fail-open hook that writes no line on a refused instance check are the Unix dialog-path refusals (Needs 5, 6, 7, 19).
@answer n=23 entity=obs-scope-summary#30.4 v=no
why: a provenance pointer to the Critical Path 4 and 7 sources that states no rule the chunk must honour.
@answer n=24 entity=obs-scope-summary#31.1 v=no
why: a surfaces label for the pause and release path that states no rule; its rules sit in #31.2.
@answer n=25 entity=obs-scope-summary#31.2 v=yes
why: the release-from-driver event admitted by schemas/diag-line.v1.json and the channel-request and channel-response lines for pause and release are the closed event enum and channel telemetry the chunk re-asserts on every OS (Needs 5, 7).
