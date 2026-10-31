# Classified — obs-plan · span-trace-coverage.b1

## Answers

span-trace-coverage#1.1 · no · the session adds, retires and moves no span name, so the CLIENT span list still holds
span-trace-coverage#1.2 · no · the session adds, retires and moves no SERVER span name; mcp.tool_call, channel.dispatch, hook.handle and hook.statusline are unchanged
span-trace-coverage#1.3 · no · the session adds no INTERNAL span, and without_endpoint opens nothing
span-trace-coverage#2.1 · no · the row names no detail enum, no verify and no exit-21 document, and its refused-check and no-log statements still hold
span-trace-coverage#2.2 · no · the run row names only the start's spans and the handle-wait child lines, all unmoved
span-trace-coverage#2.3 · no · hook.rs is not in the diff, so the hook spans, lines and exit-0 rules stand
span-trace-coverage#2.4 · no · viola-channel is not in the diff, so channel-request and channel-response are unchanged
span-trace-coverage#2.5 · no · the MCP change is in the isError body, not in a log line, and the rejects stay codes-only
span-trace-coverage#3.1 · no · the server check's process-exit (server-verify-failed, during connect) is unchanged and names no exit-21 document
span-trace-coverage#3.2 · yes · stale · the verb-module tests and cmd::client::refused_inside sit in reached files (src/cmd/send.rs and the other verb modules, src/cmd/client.rs) whose lines moved, and the report does not show them unmoved
span-trace-coverage#3.3 · yes · contradict · the frame verbs' exit-21 --json document with detail null is retired, and a refusal now names its cause (strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost)
span-trace-coverage#3.4 · no · the hook's server check, its fail-open path and its hook-decision details are not in the diff
span-trace-coverage#3.5 · no · channel-response's -32602 detail is untouched and viola-channel is not in the diff
span-trace-coverage#3.6 · yes · contradict · the MCP refusal's isError body now carries the cause as detail, so instance-unreachable with detail null is no longer true
span-trace-coverage#3.7 · no · reply_of and of_reply add no span or line, and the session adds no span or line to reply screening
span-trace-coverage#4.1 · no · the run start's must-trace chain of spans is unchanged, and the session adds no span to it
span-trace-coverage#4.2 · no · run.collision_check's outcome stays already-live for a stale holder (the report re-read it unmoved) and the other required attributes are unchanged
span-trace-coverage#4.3 · yes · add-to · the start's exit-1 detail set gains stale-heartbeat, which a stale holder now logs in place of already-live
span-trace-coverage#4.4 · no · run's start still closes on pty.spawn, and the child's and run's process-exit lines are unchanged
span-trace-coverage#5.1 · no · a concurrent placement adds no span or line, and the outcome stays ok or pinned-hash-mismatch
span-trace-coverage#6.1 · yes · add-to · F5: the replayed-snapshot path that live_endpoint reads is without_endpoint's reading, which reads by symlink_metadata, opens nothing and writes no line
span-trace-coverage#6.2 · no · the send span attributes (text_bytes, window_ms, outcome) are not touched by the session
span-trace-coverage#6.3 · no · the send log fields, the server-verify-failed process-exit and the MCP no-line rule still hold, and nothing in the session changes them
span-trace-coverage#6.4 · yes · contradict · exit 21 instance-dead with during call is retired, and that case now reads endpoint-lost with during call (F4)
span-trace-coverage#7.1 · no · the wait and last server check and must-trace chain are unchanged
