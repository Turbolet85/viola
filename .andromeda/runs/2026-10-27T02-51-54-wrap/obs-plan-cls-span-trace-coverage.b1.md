# Classified — obs-plan · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · the session adds no span and the report names no change on a CLIENT path
span-trace-coverage#1.2 · no · the session adds no span and the report names no change on a SERVER path
span-trace-coverage#1.3 · no · the session adds no span, and run.pin_copy keeps its run.* INTERNAL name
span-trace-coverage#2.1 · no · no CLI verb's code is in the change (verify, revive, plugin and run are unchanged) and the placement adds no span
span-trace-coverage#2.2 · no · the start's manual span set is unchanged, since the placement adds no span and src/cmd/run.rs is unchanged
span-trace-coverage#2.3 · no · the hook's spans, instance check and refusals hold, and strict.rs changed only a cfg(windows) test oracle
span-trace-coverage#2.4 · no · viola-channel is not a reached file and the placement adds no span
span-trace-coverage#2.5 · no · viola-mcp is not a reached file, and the mcp_tools.rs edit is a test-only path 7 setup that emits no product span
span-trace-coverage#3.1 · no · the server check is unchanged, and strict.rs changed only a cfg(windows) test oracle
span-trace-coverage#3.2 · no · the captured-span refusal cases are unchanged and no CLI frame verb's refusal path is in the change
span-trace-coverage#3.3 · no · the instance check holds: strict.rs's product verdict (Err(Writable)) and its lines are unchanged, only a cfg(windows) test oracle moved
span-trace-coverage#3.4 · no · the hook's connect is unchanged, and strict.rs changed only a cfg(windows) test oracle
span-trace-coverage#3.5 · no · the MCP check_instance and server check are unchanged, and strict.rs changed only a cfg(windows) test oracle
span-trace-coverage#4.1 · no · the start's span names and order are unchanged, since the placement adds no span
span-trace-coverage#4.2 · yes · add-to · per F1, a lost first-start placement re-hashes the standing copy, so run.pin_copy reads ok when equal and pinned-hash-mismatch when different
span-trace-coverage#4.3 · no · the closed detail set and the process-exit line hold (the lost placement reuses pinned-hash-mismatch, which 4.2 carries) and pin_companions' body is unchanged for sideload_fallback
span-trace-coverage#4.4 · no · the start's cleanup is unchanged, since the placement adds no span and sits before pty.spawn
span-trace-coverage#5.1 · no · the confirmed-send path is untouched (the report's Untouched section names it) and strict.rs changed only a cfg(windows) test oracle
span-trace-coverage#5.2 · no · the send span attributes are unchanged and no send-path code is in the change
span-trace-coverage#5.3 · no · the send log fields and refusal details are unchanged and no send-path code is in the change
span-trace-coverage#5.4 · no · the send cleanup and confirm windows are unchanged and no send-path code is in the change
span-trace-coverage#6.1 · no · the wait/last spans and server check are untouched (the report's Untouched section names the scenario) and strict.rs changed only a test oracle
span-trace-coverage#6.2 · no · the wait.block and run.wait_dispatch attributes are unchanged and no wait/last code is in the change
span-trace-coverage#6.3 · no · the wait/last log fields and outcomes are unchanged and no wait/last code is in the change
span-trace-coverage#6.4 · no · the wait.block cleanup is unchanged and no wait/last code is in the change
