# Classified — obs-plan · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · yes · stale · the CLI verbs that hold these CLIENT spans (send, wait, last, answer, pause, link) are reached whole and the report clears none of the names
span-trace-coverage#1.2 · yes · stale · mcp.tool_call lives in reached viola-mcp (tools.rs), reached whole, and no report clears the SERVER span names
span-trace-coverage#1.3 · yes · stale · run.* sits in reached src/cmd/run.rs and state.* in reached viola-state liveness.rs; the report re-reads only the run.collision_check outcome
span-trace-coverage#2.1 · yes · stale · the cmd verbs that open cli-<name>.ndjson (send, list, plugin, link, revive, verify) are reached whole and the report clears none of their span and log rules
span-trace-coverage#2.2 · yes · stale · the run start sequence and its spans sit in reached src/cmd/run.rs, reached whole; the report clears only the start's human pairs and the collision_check outcome
span-trace-coverage#2.3 · no · src/cmd/hook.rs is not in the diff and the report's Untouched hook rules (fail-open, silent no-op, stdout) and hook-decision's seven words stand
span-trace-coverage#2.4 · no · viola-channel is not in the diff and the report's Untouched channel wire, conn join key and -32603 body stand
span-trace-coverage#2.5 · yes · stale · mcp.tool_call and the per-tool span rule live in reached viola-mcp (tools.rs, result.rs), reached whole, and no report clears them
span-trace-coverage#3.1 · yes · stale · cmd::client::connect in reached src/cmd/client.rs is the server check each CLI frame verb runs; reached whole and not cleared by the report
span-trace-coverage#3.2 · yes · stale · its test names and the cmd::client::refused_inside helper sit in reached cmd modules and client.rs, under the report's moved test names into reached files
span-trace-coverage#3.3 · yes · contradict · exit 21's --json document, given here with detail null, now names its cause in detail, and the no-endpoint reading via without_endpoint (F5) leaves its strict-modes check first and writes no line
span-trace-coverage#3.4 · no · the hook server check's words (channel-unreachable, server-verify-failed) did not move, src/cmd/hook.rs is not in the diff, and snapshot replay is untouched
span-trace-coverage#3.5 · no · channel-response's -32602 detail sits in viola-channel, which is not in the diff, and the report's Untouched channel wire stands
span-trace-coverage#3.6 · yes · contradict · MCP's check_instance refusal now carries its cause as detail, so the bare detail null and the codes-only isError body are no longer true
span-trace-coverage#3.7 · yes · stale · CLI reply_of and MCP of_reply screen through viola_core::is_code_word in reached viola-core lib.rs (UnreachableCause added), reached whole and not cleared
span-trace-coverage#4.1 · yes · stale · the run start's span chain sits in reached src/cmd/run.rs, reached whole; the report re-reads only the collision_check outcome
span-trace-coverage#4.2 · yes · stale · run.rs attributes (pin_copy, conpty_sideload, version_gate, snapshot_write) are reached whole; the report re-reads only collision_check's outcome, unmoved at line 45
span-trace-coverage#4.3 · yes · stale · a stale holder now logs stale-heartbeat rather than already-live, and revive --json's refusal codes grow, so the detail lists read moved
span-trace-coverage#4.4 · yes · stale · run.start's close and the process-exit pair sit in reached src/cmd/run.rs, reached whole, and the report does not clear them
span-trace-coverage#5.1 · yes · stale · the run start's pin-copy path is reached whole with the start code, and the report does not clear the concurrent-placement outcome
span-trace-coverage#6.1 · yes · stale · the send path's CLI instance check and server check sit in reached src/cmd/send.rs and client.rs, reached whole; the report clears no send step
span-trace-coverage#6.2 · yes · stale · run.readiness_gate and run.confirm_window sit in reached src/cmd/run.rs, reached whole, and the report clears neither
span-trace-coverage#6.3 · yes · stale · CLI send's server-verify process-exit and MCP send's no-line rule sit in reached send.rs and viola-mcp, reached whole; the channel lines are unmoved but the send side is not cleared
span-trace-coverage#6.4 · yes · contradict · exit 21 instance-dead with during call is retired and now reads endpoint-lost with during call (F4), while the no-endpoint instance-dead reading (F5) lands beside the connect group
span-trace-coverage#7.1 · yes · stale · wait.block and last.client's server check sit in reached wait.rs and last.rs, reached whole, and the report clears neither
