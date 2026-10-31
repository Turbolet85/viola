# Classified — obs-plan · obs-scope-summary.b4

## Answers

obs-scope-summary#22.3 · no · the MCP file sink, its home path and the stderr fallback are untouched; the session changes isError codes and error documents, not the sink
obs-scope-summary#22.4 · no · service identity (process mcp, instance from VIOLA_NAME) is untouched by the error-code change
obs-scope-summary#22.5 · yes · add-to · isError's instance-unreachable detail now carries one of viola_core::UnreachableCause's five cause words (F11), as invalid-params' detail carries its closed field word
obs-scope-summary#23.1 · no · init order and the strict-modes check before each open are unchanged; the session adds only the catch-site document after init, and verify's refused home tree stays lineless
obs-scope-summary#24.1 · no · the tracing stack and file sink rules are unchanged; the cli catch site's stdout document is --json output, not a log sink
obs-scope-summary#25.0 · no · the required log fields and the closed event enum are unchanged; no event value or field was added and process-exit still carries corr null
obs-scope-summary#26.1 · no · log file basenames and the home-level directory rule are unchanged; no file moved
obs-scope-summary#27.1 · no · the surfaces run touches are unchanged; the session adds no surface
obs-scope-summary#27.2 · no · the must-trace span names and the process-start / process-exit events are unchanged; a stale holder still fails through process-exit
obs-scope-summary#27.3 · yes · stale · a stale holder's run exit 1 now logs stale-heartbeat, not already-live, so already-live names only a live holder; the sha-256 cause is the pinned-hash-mismatch code
obs-scope-summary#27.4 · no · the source citations are unchanged; the session moves no citation in this entity
obs-scope-summary#28.1 · no · the send path's surfaces are untouched; the session changes no send code
obs-scope-summary#28.2 · no · the send spans and channel-request / channel-response events are unchanged; the channel wire is not reached
obs-scope-summary#28.3 · no · the send fields and detail words are unchanged; the channel wire and its -32602 bodies are not reached
obs-scope-summary#28.4 · no · the source citations for the send path are untouched
obs-scope-summary#29.1 · no · the wait surfaces are unchanged; the session moves no wait path
obs-scope-summary#29.2 · no · the wait spans and channel events are unchanged; the channel wire is not reached
obs-scope-summary#29.3 · no · outcome instance-unreachable and exit_code 21 are unchanged; the new cause sits in the --json document, which this field list does not name
obs-scope-summary#29.4 · no · the wait source citations are untouched
obs-scope-summary#30.1 · no · the dialog surfaces are unchanged; the session moves no hook or answer path
obs-scope-summary#30.2 · no · the dialog spans and events are unchanged
obs-scope-summary#30.3 · yes · add-to · the server-verify-failed row names Linux and Windows, and macOS has no server check (F12), which joins that platform qualifier
obs-scope-summary#30.4 · no · the dialog source citations are untouched
obs-scope-summary#31.1 · no · the send-refused and release surfaces are unchanged
obs-scope-summary#31.2 · no · the pause and release spans and events, including release-from-driver, are unchanged
