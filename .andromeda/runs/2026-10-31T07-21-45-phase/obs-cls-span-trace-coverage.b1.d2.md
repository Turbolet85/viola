# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · a CLIENT span-kind list; the chunk opens or changes no span, and no error body depends on a span's kind
span-trace-coverage#1.2 · no · a SERVER span-kind list; the chunk opens or changes no span, and no error body depends on a span's kind
span-trace-coverage#1.3 · no · an INTERNAL span-kind list; the chunk opens or changes no span, and no error body depends on a span's kind
span-trace-coverage#2.1 · yes · the short-lived verbs' process-start and process-exit with exit_code and detail, the internal-error fault path, and the full anyhow chain going only to detail-cli.ndjson when an instance resolves
span-trace-coverage#2.2 · yes · portable-pty's target is OFF and its failures are captured at the seam, so its text must stay absent from all three surfaces
span-trace-coverage#2.3 · yes · the instance-check exit that reads no config.json and the hidden statusline capture arm that writes no lines decide whether the dialog-hold reader's parse and the statusline oversize print are observable
span-trace-coverage#2.4 · yes · the channel-request and channel-response fields with error_code, which the -32602 causes must be told apart by, and never params or result bodies; interprocess emits no log, so the viola seam keeps its text off the surfaces
span-trace-coverage#2.5 · yes · rmcp's target is OFF and its records can carry frame content, so its text stays off the surfaces, and codes-only send-refused and channel-response lines on MCP rejects
span-trace-coverage#3.1 · yes · each CLI frame verb's server check ends exit 21 with detail server-verify-failed during connect, so that closed code and code-shaped detail must reach the verb's error output
span-trace-coverage#3.2 · no · captured-span placement tests of the existing per-verb server check, which the chunk does not change
span-trace-coverage#3.3 · yes · the instance check of the six frame verbs, whose --json refusal is exactly instance-unreachable with a null detail and writes no line
span-trace-coverage#3.4 · no · the hook's fail-open server check and its hook-decision codes, on a path that is none of the three surfaces and reaches no body the chunk changes
span-trace-coverage#3.5 · yes · the MCP frame tools' instance and server checks, whose mismatch returns a codes-only isError instance-unreachable with no frame and no line
span-trace-coverage#4.1 · yes · the child's --settings argument, the source path and the statusline command appear in no line or span, which the statusline-command and absolute-path rules read
span-trace-coverage#4.2 · yes · the batch-script-child refusal logged as process-exit detail, pty.spawn's env_stripped_count, and endpoint, pinned_bin path and env values never logged
span-trace-coverage#4.3 · yes · the closed process-exit detail codes with no path or pid, the strict-modes-failed document, env_stripped_count, env_stripped_known and env_kept, the parse-rejected line, and the diag-line schema closure
span-trace-coverage#4.4 · no · run's child-exit and self-exit lines carry no error body, and the start-refusal codes sit in #4.3
span-trace-coverage#5.1 · no · the concurrent first-start placement of the pin copy, which the chunk does not edit, and whose no-line rule is a sub-case of the pinned_bin rule that #4.2 carries
span-trace-coverage#6.1 · yes · a failing implicit link record answers -32603 with nothing issued, and driver and driven reach no process-log line
span-trace-coverage#6.2 · no · send span attributes (method, outcome, text_bytes, window_ms), spans only; the send's codes are carried by #6.3 and #6.4
span-trace-coverage#6.3 · yes · the send-refused, send-issued and channel-request and channel-response lines, codes-only on rejects, with a self-reported from and the refusal and detail fields of channel-response
span-trace-coverage#6.4 · yes · the send exit codes with detail (wrapper-fault, server-verify-failed, internal-error on a reply with neither result nor error code) and channel-response emitted from a Drop guard
span-trace-coverage#7.1 · no · the wait and last span chain and server-check ordering, whose instance and verify codes #3.1, #3.5 and #7.3 carry
span-trace-coverage#7.2 · no · wait.block and run.wait_dispatch span attributes, which are not an error body
span-trace-coverage#7.3 · yes · the wait and last channel-request and channel-response fields, instance-unreachable on a missing reply, no result body on either line, and the exit 21 details
