# Classified — obs-plan · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · no CLIENT span is added, renamed or re-kinded, and the task's Untouched list keeps the mcp frame-tool spans as written
span-trace-coverage#1.2 · no · no SERVER span name or kind changes, and the task's Untouched list keeps mcp.tool_call as written
span-trace-coverage#1.3 · no · no run or state span is added or renamed, and the fs.rs compare read adds none (the task's Untouched list)
span-trace-coverage#2.1 · no · no short-lived CLI verb module is reached, and the cli spans and process lines are unchanged
span-trace-coverage#2.2 · no · the run start's spans and child process lines are unchanged, and the reached fs.rs change adds no span or line
span-trace-coverage#2.3 · no · no hook module is reached, and the hook spans and hook-decision lines are unchanged
span-trace-coverage#2.4 · no · viola-channel is not reached, and the MCP stdin cut leaves the channel-frame rule as it stands
span-trace-coverage#2.5 · no · the stdin cut is a read outside any tool call, so mcp.tool_call and its codes-only reject lines are unchanged
span-trace-coverage#3.1 · no · no CLI module is reached, so the frame verbs' connect path and server check are unchanged
span-trace-coverage#3.2 · no · the per-verb and hook captured-span tests and the refused_inside helper sit in no reached file
span-trace-coverage#3.3 · no · client::refuse_untrusted sits in no reached file, and the frame verbs' instance check is unchanged
span-trace-coverage#3.4 · no · no hook module is reached, and the task's Untouched list keeps the snapshot replay reader and its lines
span-trace-coverage#3.5 · no · the task's Untouched list says no frame tool's instance check, liveness pre-check or server check changed, and the stdin cut is a read, not a tool call
span-trace-coverage#4.1 · no · the start's span chain and steps are unchanged (the task's Untouched list), and the fs.rs change adds no span or line
span-trace-coverage#4.2 · no · no start span attribute or outcome set changed, and the fs.rs change adds no field or detail code
span-trace-coverage#4.3 · no · no process-start or process-exit field, detail or sideload value changed, and the schema change is the parse-rejected enum only
span-trace-coverage#4.4 · no · the start's close and the handle-wait process-exit lines are unchanged (the task's Untouched list)
span-trace-coverage#5.1 · no · place_private_new and the placement's no-span, no-line rule are unchanged, since the fs.rs change adds no event, field or detail code
span-trace-coverage#6.1 · no · the send's span chain and the MCP send's instance check are unchanged (the task's Untouched list keeps the frame tools' checks)
span-trace-coverage#6.2 · no · no send span attribute changed, vt100_panicked keeps its meaning, and the chaos test change asserts passthrough only
span-trace-coverage#6.3 · no · no send, channel or hook line field or detail changed, and the one new line is a parse-rejected line outside the send path
span-trace-coverage#6.4 · no · the confirm windows, the send's exit codes and its process-exit detail are unchanged, and no exit code was added
span-trace-coverage#7.1 · no · the wait and last span chain and their server check are unchanged (the task's Untouched list)
span-trace-coverage#7.2 · no · no wait or last span attribute changed, as the task's Untouched list keeps that scenario
span-trace-coverage#7.3 · no · no channel-request, channel-response or process-exit field changed, and last_assistant_message is untouched
