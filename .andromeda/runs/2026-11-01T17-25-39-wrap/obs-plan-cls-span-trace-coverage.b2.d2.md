# Classified — obs-plan · span-trace-coverage.b2

## Answers
span-trace-coverage#7.2 · no · the session adds no span attribute to wait.block or run.wait_dispatch; the new refusal is a log line, not an attribute
span-trace-coverage#7.3 · yes · add-to · F3 adds socket-dir-refused to exit 21's detail list for wait and last, and F7 extends server-verify-failed's reach to macOS's euid mismatch
span-trace-coverage#7.4 · yes · add-to · F3 adds a socket-dir refusal before any frame that closes wait.block with its process-exit and no channel.request, and F6 adds a wrapper peer refusal seen by the client as end-of-file
span-trace-coverage#8.1 · yes · stale · F7 puts the euid read on macOS too, so the "(Linux and Windows; macOS on the recorded gap)" reading of the server check has moved, only the pid gap staying named
span-trace-coverage#8.2 · no · the span attributes of run.dialog_register, run.dialog_await and hook.decision_emit are unchanged; F4 is a log field, not a span attribute
span-trace-coverage#8.3 · yes · add-to · F4 adds a socket-dir-refused hook-decision to the fail-open paths before dialog_id, and F3 adds the same refusal to CLI answer's process-exit lines
span-trace-coverage#8.4 · no · fail-open before dialog_id keeps the hook-invoked{corr:null} plus hook-decision shape for F4's new code, and the code joins Section 6's list, which this line does not enumerate
span-trace-coverage#9.1 · yes · stale · F7 puts the euid read on macOS, so the "(Linux and Windows; macOS on the recorded gap)" reading of pause and release's server check has moved, only the pid gap staying named
span-trace-coverage#9.2 · no · the wheel_from, wheel_to and cause attributes of run.wheel_transition are unchanged
span-trace-coverage#9.3 · yes · add-to · F3 adds socket-dir-refused as a pre-frame process-exit (exit 21, during connect, no channel-request) for pause and release
span-trace-coverage#9.4 · no · run.wheel_transition stays a point-in-time span and the wheel event stays the audit record; the session changes neither
span-trace-coverage#10.1 · no · the statusline spans (hook.statusline, state.budget_write, statusline.shell_out, run.budget_eval) are unchanged; F4 is stated for the dialog path only
span-trace-coverage#10.2 · no · the span attributes of state.budget_write, statusline.shell_out and run.budget_eval are unchanged
span-trace-coverage#10.3 · no · the statusline arm's line rules and its detail set (oversize-stdin, malformed-json, deadline) are unchanged; F4 is the dialog path only
span-trace-coverage#10.4 · no · the statusline.shell_out bound and its deadline line are unchanged
span-trace-coverage#11.1 · no · the R8 strip and its env_stripped, env_stripped_known and env_kept fields are not touched by the session
span-trace-coverage#12.1 · no · a refused directory sends no frame, so the driven wrapper records no implicit link, as the entity already states for refused calls
span-trace-coverage#13.1 · no · the explicit link and unlink rules (-32602, -32603, no line on refusal) hold; a refused peer never reaches the method, and F6 writes no line
span-trace-coverage#14.1 · no · live_endpoint still writes one state-recovered line per call; the directory check does not change the snapshot read or its callers
span-trace-coverage#15.1 · no · the torn-append heal in the appender is unchanged; the session touched no append path
span-trace-coverage#16.1 · no · the ledger read inside run.version_gate and its parse-rejected lines are unchanged; the session moves no stamp read
span-trace-coverage#17.1 · no · verify and plugin install's spans, pairs and home-tree checks are unchanged; the session moved neither check
