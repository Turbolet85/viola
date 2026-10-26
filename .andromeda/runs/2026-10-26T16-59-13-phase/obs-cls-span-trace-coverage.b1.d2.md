# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · a span-kind list the chunk leaves unchanged, since it adds or renames no span and its new checks sit inside these CLIENT spans as they are
span-trace-coverage#1.2 · no · a span-kind list the chunk leaves unchanged, since it adds or renames no SERVER span and its refusals stay inside the spans as classed
span-trace-coverage#1.3 · no · a span-kind list the chunk leaves unchanged, since it names no new run.*, state.* or pty span and the INTERNAL kinds it states are not reached
span-trace-coverage#2.1 · yes · the cli verb rows bind, since the new strict-modes refusals on send, wait, last, answer, pause and release sit in these dispatch spans and the no-line rule for a refused check must be reconciled with their exit-21 WARN process-exit
span-trace-coverage#2.2 · yes · the run start manual spans bind, since the new home, instances, bin, plugin, ui and conpty checks are new start steps that each need a span or a listed no-span exemption
span-trace-coverage#2.3 · yes · the hook row binds, since the strict check on the nine events runs before the VIOLA_DIR snapshot read and fails open with no body, and catch_unwind must keep exit 0
span-trace-coverage#2.4 · no · viola-channel spans and line fields are not reached, since the chunk's refusals sit before any frame and write no channel line
span-trace-coverage#2.5 · yes · the viola-mcp row binds, since the frame tools' new refusal in tools.rs returns instance-unreachable inside mcp.tool_call and writes no line
span-trace-coverage#3.1 · yes · the cli server-check rule binds, since the new strict refusal precedes the liveness pre-check and this server check, which must keep their order and exit-21 WARN shape
span-trace-coverage#3.2 · yes · the per-verb captured-span cases bind, since the server-verify refusal must stay inside its root span behind the new pre-frame check and each case must read red with the check removed
span-trace-coverage#3.3 · yes · the hook server-check sequence binds, since the strict check must precede read_snapshot and a refused check may write no hook-decision line, unlike the server-verify path
span-trace-coverage#3.4 · yes · the viola-mcp server check binds, since the frame tools' new refusal precedes live_endpoint and the server check and returns instance-unreachable with no frame and no line
span-trace-coverage#4.1 · yes · the run start must-trace chain binds, since the new home, bin, plugin, ui and conpty checks and the pin re-hash land inside it and each step must be named or listed as no-span
span-trace-coverage#4.2 · yes · the run start span attributes bind, since run.pin_copy's outcome (ok or pinned-hash-mismatch) and run.conpty_sideload's closed outcome set are what the re-hash and companion checks produce
span-trace-coverage#4.3 · yes · the run start log fields bind, since the pinned-hash-mismatch exit detail and the sideload_fallback closed set are what the re-hash and companion checks write
span-trace-coverage#4.4 · yes · the run.start close and run's exit line bind over the start unit the chunk edits, since its pre-spawn refusals end run.start without pty.spawn
span-trace-coverage#5.1 · yes · the send must-trace chain binds on its pre-frame half, since the new strict refusal ends send.client before channel.request as the server check does
span-trace-coverage#5.2 · no · post-frame span attributes (readiness outcome, paste text_bytes, confirm window) that the chunk's Not-needed list leaves untouched, and its refusal is before the frame
span-trace-coverage#5.3 · yes · the client-side server-verification refusal shape binds, since the new strict refusal on CLI send writes no channel line, ends process-exit exit 21 at WARN, and MCP send writes no line
span-trace-coverage#5.4 · yes · send.client's exit-21 detail set binds, since strict-modes-failed joins instance-dead and server-verify-failed as a closed detail the new refusal must carry
span-trace-coverage#6.1 · yes · the wait and last must-trace chain binds on its pre-frame half, since the new strict refusal ends wait.block and last.client before channel.request
span-trace-coverage#6.2 · no · wait.block and run.wait_dispatch attributes set on the wait frame, which the chunk does not change, and its refusal is before the frame
span-trace-coverage#6.3 · yes · the wait and last log fields bind, since the statement that strict-modes-failed cannot occur on them is owned by this chunk and goes stale once the check lands
span-trace-coverage#6.4 · yes · the wait.block closing rules bind, since the new pre-frame strict refusal must close wait.block with its outcome line as the server-verification refusal does
span-trace-coverage#7.1 · yes · the dialog and answer must-trace chain binds, since the strict check must precede the dialog hook's channel.request inside hook.handle and the answer frame in answer.client
