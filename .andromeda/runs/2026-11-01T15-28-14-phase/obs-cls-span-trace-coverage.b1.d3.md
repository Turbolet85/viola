# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · yes · the CLIENT spans send.client, wait.block, last.client, cli.link and cli.unlink hold the new client-side euid and socket-directory refusals before any frame
span-trace-coverage#1.2 · yes · channel.dispatch, mcp.tool_call and hook.handle are where the new refusals must open no request span, and where the Tokio and hook connect refusals sit
span-trace-coverage#1.3 · yes · run.* is the start sequence where the wrapper's socket-directory check, the bind and the wrapper-side euid refusal run as INTERNAL spans
span-trace-coverage#2.1 · yes · the cli verbs link, unlink and list gain refusals that open no process log and write no line, and the new client refusals follow that pattern
span-trace-coverage#2.2 · yes · the viola run start sequence carries the new pre-child socket-directory refusal and the wrapper-side euid refusal
span-trace-coverage#2.3 · yes · hook connect refusals become a fail-open hook-decision detail with exit 0 and no line beyond it
span-trace-coverage#2.4 · yes · viola-channel is where the euid check and the socket-directory check sit, and a refused connection writes no channel-request or channel-response pair
span-trace-coverage#2.5 · yes · the MCP Tokio client refusal maps to a codes-only instance-unreachable result kept inside mcp.tool_call, with no line
span-trace-coverage#3.1 · yes · the new peer euid check runs in the same connect as the server verdict, before any frame, beside the server-verify-failed refusal shape
span-trace-coverage#3.2 · yes · the new connect-time refusals must fall inside each verb's dispatch span, the place these captured-span cases assert
span-trace-coverage#3.3 · yes · client::refuse_untrusted is the shared site the new client-side refusals reach ahead of any frame, beside the instance check
span-trace-coverage#3.4 · yes · the hook connect's new refusal is a fail-open through hook-decision, at the same place before any frame
span-trace-coverage#3.5 · no · the -32602 channel-response detail is for protocol faults; the chunk's refusals write no channel-response line and add no -32602 path
span-trace-coverage#3.6 · yes · the MCP frame tools gain the Tokio client's euid refusal, mapped to a codes-only isError instance-unreachable with a detail and no line
span-trace-coverage#3.7 · no · covers screening of peer replies through is_code_word and sanitised_fault; the chunk's refusals are client-local, not peer replies, and it adds no reply path
span-trace-coverage#4.1 · yes · the start sequence's channel.bind and the new pre-child socket-directory refusal, before or after the log opens
span-trace-coverage#4.2 · yes · channel.bind's endpoint_kind, and the rule that no endpoint name or socket path appears in any span, with a bind failure's detail carrying no path or pid
span-trace-coverage#4.3 · yes · the pre-child refusal list and its no-detail, no-log rule, which the socket-directory refusal joins as the two fixed stderr lines
span-trace-coverage#4.4 · no · the run and child process-exit closing rules at pty.spawn; the chunk's force-kill measurement reads endpoint_gone, not these lines
span-trace-coverage#5.1 · no · pin_copy's concurrent placement and its outcome set; the chunk does not touch the pin copy
span-trace-coverage#6.1 · yes · send.client's server check and the new client euid refusal sit before channel.request, and a refused check opens no channel.request
span-trace-coverage#6.2 · no · the send's channel.request, readiness, paste and confirm-window attributes; a refused connection opens no channel.request and the chunk changes none of these
span-trace-coverage#6.3 · yes · the CLI send's client-side server-verification refusal shape (exit 21, WARN, no channel line on either side) is what the new client refusal matches
span-trace-coverage#6.4 · yes · send.client's exit-21 process-exit with during connect and a closed detail; the new euid refusal needs its own detail code in this list
span-trace-coverage#7.1 · yes · wait.block and last.client run the server check before channel.request, and the new client refusal sits beside it
