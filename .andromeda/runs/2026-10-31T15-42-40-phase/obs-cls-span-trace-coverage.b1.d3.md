# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · the CLIENT span-kind list; the chunk adds or re-kinds no span, so the kinds it names are untouched
span-trace-coverage#1.2 · no · the SERVER span-kind list; the chunk adds or re-kinds no span, so the kinds it names are untouched
span-trace-coverage#1.3 · no · the INTERNAL span-kind list; the chunk adds no span and its collision-check and start-refusal edits leave span kinds as they are
span-trace-coverage#2.1 · yes · the cli short-lived-verb row: process-exit{exit_code, detail} only after the instance resolves, a refused instance check (exit 21) opens no process log and writes no line, and the link, unlink, list, plugin install and revive --list refusals the chunk edits
span-trace-coverage#2.2 · no · the viola run row's manual start-sequence spans and handle-wait child lines; the chunk's start refusals are pre-spawn self lines and touch neither
span-trace-coverage#2.3 · yes · the hook row's refused or absent instance exits 0 with no line; the scope names the hook edge (exit 0, no stderr, hook-decision its only cause record) and the chunk must add no hook cause
span-trace-coverage#2.4 · yes · channel-request and channel-response fields including error_code and the -32602 detail; the vanished-server read in viola-channel client.rs and the new codes crossing channel error data write these fields
span-trace-coverage#2.5 · yes · viola-mcp codes-only send-refused and channel-response on rejects; the new codes pass through crates/viola-mcp/src/result.rs and tools.rs as codes only
span-trace-coverage#3.1 · yes · the cli server check on the frame verbs ends process-exit exit 21 server-verify-failed during connect with no channel.request; the chunk edits the server check in client.rs
span-trace-coverage#3.2 · yes · the refusal sits inside each verb's dispatch span as asserted by refused_inside; the chunk edits the server check, so each captured-span case must still read red with the check removed
span-trace-coverage#3.3 · yes · refuse_untrusted for the six frame verbs refuses before any role file with exit 21 and either the instance pair or the exact --json instance-unreachable document with detail null; the chunk edits refuse_untrusted and refused_document and may move that document
span-trace-coverage#3.4 · yes · the hook server check's hook-decision server-verify-failed; the scope names hook-decision as the hook's only cause record and the chunk must add no hook cause
span-trace-coverage#3.5 · yes · channel-response detail on -32602 only and absent for the other four faults; the chunk's channel -32603 with null data must keep that line free of detail
span-trace-coverage#3.6 · yes · viola-mcp server check: check_instance before live_endpoint, and a codes-only isError instance-unreachable with no frame and no log line; result.rs and tools.rs carry the new codes
span-trace-coverage#3.7 · yes · CLI reply_of and MCP of_reply screen replies through is_code_word and sanitised_fault; the new codes must pass that screen
span-trace-coverage#4.1 · yes · run.collision_check sits in the run start must-trace chain, and the chunk edits the collision check
span-trace-coverage#4.2 · yes · run.collision_check outcome set, run.pin_copy outcome, and the detail set already-live, squatted-name, pinned-hash-mismatch, batch-script-child; the chunk edits these start refusals and the collision check
span-trace-coverage#4.3 · yes · exit-1 process-exit detail set for run start, the revive preflight details no-session, session-live and cwd-missing, and the check_home_tree refusal before the log; the chunk edits these
span-trace-coverage#4.4 · no · the cleanup rule after pty.spawn (handle-wait child exit and run's exit line); the chunk edits only pre-spawn refusals
span-trace-coverage#5.1 · yes · a lost pin_copy placement re-hashes and reads pinned-hash-mismatch with no extra line; the chunk's pinned-hash-mismatch refusal and its exit-1 detail must hold on that path
span-trace-coverage#6.1 · yes · CLI send runs refuse_untrusted before any role file and the server check before channel.request; the chunk edits both in client.rs and send.rs is reached whole
span-trace-coverage#6.2 · no · readiness-gate, paste-write and confirm-window span attributes; no exit-21 cause reaches those spans and the chunk changes none of them
span-trace-coverage#6.3 · yes · the client-side server-verification refusal, CLI send's process-exit exit 21 server-verify-failed at WARN with no channel line and MCP with no line; the chunk edits that server check
span-trace-coverage#6.4 · yes · send.client exit codes with exit 21 details instance-dead and server-verify-failed and exit 1 internal-error, plus the refuse_untrusted refusal with no span, process-exit or line; the chunk edits those paths
span-trace-coverage#7.1 · yes · wait.block and last.client run the server check after their client refusals and before channel.request; the chunk edits that server check, and wait.rs and last.rs are reached whole
