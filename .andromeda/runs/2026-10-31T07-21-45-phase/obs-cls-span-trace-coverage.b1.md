# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · the CLIENT span kind of the listed spans, which the chunk neither adds nor re-kinds and no error body reads
span-trace-coverage#1.2 · no · the SERVER span kind of channel.dispatch, mcp.tool_call, hook.handle and hook.statusline, which no error body reads
span-trace-coverage#1.3 · no · the INTERNAL span kind of run.*, state.*, hook.decision_emit and pty.paste_write, which no error body reads
span-trace-coverage#2.1 · yes · the cli verbs' process-start and process-exit detail codes and the rule that the full anyhow chain goes only to detail-cli.ndjson
span-trace-coverage#2.2 · yes · portable-pty's OFF target and failures captured at the viola seam (D-11), the upstream capture the chunk keeps off all three surfaces
span-trace-coverage#2.3 · yes · a refused or absent instance on the hook's dialog path exits 0 before any config.json read, which the dialog-hold BOUNDARIES row must respect
span-trace-coverage#2.4 · yes · the channel-request and channel-response line fields including error_code, which the -32602 cause split and the channel-response contract read rest on
span-trace-coverage#2.5 · yes · rmcp's OFF target and the codes-only send-refused and channel-response lines on MCP rejects
span-trace-coverage#3.1 · yes · the server-verify refusal's error body (exit 21, detail server-verify-failed, during connect, no channel-request line) on the eight frame verbs, its peer-credential check owned by hardening
span-trace-coverage#3.2 · no · captured-span placement cases of the server check in each verb's module and the hook helper, which the chunk neither edits nor moves, a re-check every change owes
span-trace-coverage#3.3 · yes · the instance check's refusal and its exact --json instance-unreachable document with null detail on the six frame verbs
span-trace-coverage#3.4 · no · the hook's fail-open hook-decision details channel-unreachable and server-verify-failed, which sit outside the three surfaces the chunk reaches
span-trace-coverage#3.5 · yes · the MCP instance-unreachable isError with null detail, codes-only, with no frame and no log line on send, wait, last and answer
span-trace-coverage#4.1 · yes · the run start span chain the Path 1 timeout watch runs through, and the statusline source and command kept out of every line and span
span-trace-coverage#4.2 · yes · the never-logged endpoint, pinned_bin path and env values, and pty.spawn's env_stripped_count recorded by count
span-trace-coverage#4.3 · yes · the claude-child process-start fields env_stripped_count, env_stripped_known and env_kept, the diag-line schema facts for pty_backend and sideload_fallback, and the start_records check
span-trace-coverage#4.4 · yes · run.start's close at pty.spawn return and the handle-wait and run process-exit cleanup, read through the Path 1 timeout watch
span-trace-coverage#5.1 · no · the concurrent first-start placement of the pin copy, which the chunk does not touch and no error body on the three surfaces reads
span-trace-coverage#6.1 · yes · the -32603 answer when the implicit link record fails, with nothing issued and nothing typed
span-trace-coverage#6.2 · no · the paste, vt100 and /compact confirm-window span attributes, which the Not needed list excludes, and the channel span attributes the chunk does not change
span-trace-coverage#6.3 · yes · the channel-request and channel-response fields and the send-refused and server-verify-failed details that the send error documents carry
span-trace-coverage#6.4 · yes · the send.client process-exit details internal-error, wrapper-fault, instance-dead and server-verify-failed, and the Drop-guard channel-response emission
span-trace-coverage#7.1 · no · the wait and last span chain and server-check placement, whose error body 3.1 binds and whose placement the chunk does not change
span-trace-coverage#7.2 · no · the wait and last span attributes after, timeout_ms, woken_kind and timed_out, which the chunk does not change
span-trace-coverage#7.3 · yes · the wait and last channel-request and channel-response fields, the derived outcome, and the instance-unreachable and no-result-body rules on both sides
