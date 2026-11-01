# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · the CLIENT span-kind list; the chunk adds no span name, and its client refusals run inside these existing spans without changing their kind
span-trace-coverage#1.2 · no · the SERVER span-kind list; the wrapper-side euid refusal opens no channel.dispatch, since a refused connection has no request behind it
span-trace-coverage#1.3 · no · the INTERNAL span-kind list for run.*, state.*, hook.decision_emit and pty.paste_write; the chunk adds no span and changes no kind
span-trace-coverage#2.1 · yes · the cli short-lived-verb row: dispatch span per verb, process-exit into cli-<name>.ndjson, no line on a refused check; the new client refusals and the link/unlink instance check sit beside it
span-trace-coverage#2.2 · yes · the cli `viola run` row: manual start-sequence spans and channel.dispatch, where the wrapper start refusal and the wrapper-side euid refusal fall
span-trace-coverage#2.3 · yes · the cli `viola hook` row: hook.handle, and the refused-instance exit 0 with no line, which the hook connect refusal as fail-open must match
span-trace-coverage#2.4 · yes · the viola-channel row: channel.request/dispatch spans and the channel-request/response field set, which the wrapper-side refusal must not extend (no pair, no params)
span-trace-coverage#2.5 · yes · the viola-mcp row: mcp.tool_call span and codes-only refusal lines, which the MCP client-side refusal (instance-unreachable with a detail) stays inside
span-trace-coverage#3.1 · yes · the cli server check: verdict on connect, server-verify-failed exit at WARN before any frame, and the macOS recorded gap that the euid check does not close
span-trace-coverage#3.2 · yes · the captured-span placement cases and the refused_inside helper: the new connect-time refusals must state and assert their place inside the dispatch span
span-trace-coverage#3.3 · yes · refuse_untrusted runs before any role file opens and writes no line on refusal, the ordering the new connect refusals must state against it
span-trace-coverage#3.4 · yes · the hook server check inside hook.handle failing open with hook-decision detail, which the hook connect refusal reaches as a fail-open
span-trace-coverage#3.5 · no · the -32602 closed word on channel-response; the chunk's refusals are pre-frame and write no response line, so it is a re-check every change owes
span-trace-coverage#3.6 · yes · the mcp server check: check_instance before live_endpoint, server verify after TokioClient::connect inside mcp.tool_call, and the codes-only isError the MCP client refusal maps to
span-trace-coverage#3.7 · no · screening of peer replies through is_code_word and sanitised_fault; the chunk's new codes are its own refusals, not peer replies
span-trace-coverage#4.1 · yes · the run start must-trace chain through channel.bind; the verified-directory check, the leftover-socket takeover and the start refusal must fit this order
span-trace-coverage#4.2 · yes · run.start and channel.bind attributes, with no endpoint name or socket path in any span, which the chunk's bind and directory failures must keep out
span-trace-coverage#4.3 · yes · process-start/process-exit fields and the closed exit-1 detail list, which the new pre-child socket-directory refusal and its detail must join
span-trace-coverage#4.4 · no · the handle-wait child exit and run's closing process-exit; the chunk's wrapper refusal writes no exit line and its takeover does not change when run.start closes
span-trace-coverage#5.1 · no · the concurrent-placement rule for the pinned binary copy (place_absent); the chunk's socket directory creation is not that placement
span-trace-coverage#6.1 · yes · the confirmed send connect: instance check first, server check inside send.client before channel.request, a refusal sends no frame; the euid check and socket-directory refusal reach it ahead of any frame
span-trace-coverage#6.2 · no · the send's channel.request, readiness, paste and confirm-window attributes past the connect, which the task keeps as they stand; the chunk opens no request span
span-trace-coverage#6.3 · yes · the client-side server-verification refusal: no channel lines on either side, CLI exit 21 process-exit at WARN, MCP writes no line
span-trace-coverage#6.4 · yes · the send.client exit list and the exit 21 detail list with during connect, where the new peer euid and socket-directory refusals take distinct closed codes
span-trace-coverage#7.1 · yes · wait.block and last.client run the server check before channel.request, the same connect path the new client refusals reach for every Unix client
