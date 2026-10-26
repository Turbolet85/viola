# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · yes · the CLIENT spans the edited send, wait, last, answer, pause, release and channel clients open, so the new verification spans join this list
span-trace-coverage#1.2 · yes · mcp.tool_call and hook.handle wrap the edited MCP pre-check and the hook fail-open path, while channel.dispatch and hook.statusline are unchanged
span-trace-coverage#1.3 · yes · hook.decision_emit records decision_emitted on the edited hook.dialog fail-open path, while the run.* and pty.paste_write spans are unchanged
span-trace-coverage#2.1 · yes · the dispatch spans and the cli-<name>.ndjson process-exit{exit_code, detail} lines of the edited verbs take the new exit-21 server-verify-failed outcome
span-trace-coverage#2.2 · no · the viola run row (start-sequence spans, channel.dispatch) is unchanged, run.rs is only re-read, and this row states no collision-check outcome
span-trace-coverage#2.3 · yes · the hook.handle, channel.request and hook.decision_emit spans and the hook-invoked and fail-open hook-decision lines are the edited hook.rs paths
span-trace-coverage#2.4 · yes · the edited client.rs, tokio_client.rs and endpoint.rs emit the client channel.request spans and channel-request/response lines, which carry corr and conn but never params or result bodies
span-trace-coverage#2.5 · yes · the edited tools.rs runs send, wait, last and answer under mcp.tool_call, whose codes-only instance-unreachable rejects are mirrored to mcp.ndjson
span-trace-coverage#3.1 · no · the viola run start chain is unchanged, its run.collision_check is only re-read, and the chain states no read outcome
span-trace-coverage#3.2 · yes · run.collision_check's outcome (free, already-live, squatted-name) is the behaviour the P3 re-read of run.rs's read_snapshot call must witness
span-trace-coverage#3.3 · yes · the already-live and squatted-name refusal details of run's collision check are the behaviour the P3 re-read of run.rs's read_snapshot call must witness
span-trace-coverage#3.4 · no · run's cleanup (run.start closing at pty.spawn return, the claude-child exit, run's self process-exit) is unchanged and states nothing about the exiting-wrapper pipe window
span-trace-coverage#4.1 · yes · send.client to channel.request (cli and mcp send) and hook.handle to channel.request(hook.event) are the chains the new pre-frame verification sits in
span-trace-coverage#4.2 · yes · channel.request (client) carries method and conn, which the edited client.rs connect sets for every call made
span-trace-coverage#4.3 · yes · the client-side send-refused line for a verification refusal (codes only, no sent text) and the client channel-request/response lines are new lines this scenario must list
span-trace-coverage#4.4 · yes · send.client's exit-21 process-exit, with detail instance-dead during connect, is the path a failed server verification now takes under its own detail
span-trace-coverage#5.1 · yes · wait.block and last.client run channel.request, which the edited wait.rs and last.rs pre-frame check must precede without changing the chain
span-trace-coverage#5.2 · no · wait.block's after and timeout_ms come from the wait arguments and run.wait_dispatch's woken_kind from the server dispatch, none of which the pre-frame check touches, so it is a re-check every change owes
span-trace-coverage#5.3 · yes · the wait and last scenario note and failure lines the task's Needs change, with the exit-21 client process-exit at WARN and the channel-request/response lines on both sides
span-trace-coverage#5.4 · yes · the connect-time verification refusal is a connection error that wait.block must close on, beside the response and connection error this cleanup names
span-trace-coverage#6.1 · yes · hook.handle to channel.request(hook.dialog) and answer.client to channel.request(answer) are the edited clients, which must verify before each request
span-trace-coverage#6.2 · yes · hook.decision_emit's decision_emitted is false on the hook.dialog fail-open path this chunk adds in the edited hook.rs
span-trace-coverage#6.3 · yes · hook-invoked is written only after the hook.dialog response, which a refused verification never gets, and the answer client's driver-side refusals are edited
span-trace-coverage#6.4 · yes · the hook's own backstop read and its fail-open hook-decision{corr:null, detail} path are the edited hook.dialog code
span-trace-coverage#7.1 · yes · pause.client and release.client run channel.request, and both edited clients must refuse with exit 21 before their frames
