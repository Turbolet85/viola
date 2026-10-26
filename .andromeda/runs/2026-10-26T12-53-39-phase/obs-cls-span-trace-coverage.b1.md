# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · yes · the verbs' new client span (cli.link / cli.unlink or a new area) and the channel.request client span each new frame opens are CLIENT spans this list must record
span-trace-coverage#1.2 · yes · channel.dispatch is the SERVER span every channel.request runs in, and the new link and unlink methods are dispatched there
span-trace-coverage#1.3 · no · the chunk names no run.* or state.* span: the link and unlink work sits under channel.dispatch, and the verbs' spans are client spans
span-trace-coverage#2.1 · yes · the new verbs need a cli.<verb> dispatch span, process-start and process-exit into cli-<name>.ndjson when the driven instance resolves, and the detail-cli chain for internal faults, which the scope names
span-trace-coverage#2.2 · yes · the run's manual channel.dispatch span is the one the new methods dispatch under, with the Drop-guard response the scope names
span-trace-coverage#2.3 · no · the viola hook spans and their channel.request path are unchanged (Not needed: the hook scenarios)
span-trace-coverage#2.4 · yes · channel.request and channel.dispatch carry method, corr, conn, result_class, error_code and sender for the new methods, never params or result bodies, and the -32602 refusals log at WARN
span-trace-coverage#2.5 · no · the chunk adds no MCP tool, so viola-mcp's mcp.tool_call spans and codes-only rejects are unchanged (v1-13's MCP half is P3's read)
span-trace-coverage#3.1 · yes · each frame verb, the new link and unlink included, verifies the wrapper through cmd::client::connect inside its dispatch span before any frame, and a refusal ends with exit 21 server-verify-failed (during connect, at WARN) and no channel-request line
span-trace-coverage#3.2 · yes · each frame verb's module needs its request_refuses_an_unverified_server_inside_its_dispatch_span case through the shared cmd::client::refused_inside helper, which the two new verbs also need
span-trace-coverage#3.3 · no · the hook's connect and verify_server path belongs to viola hook, which the chunk does not reach (Not needed: the hook scenarios)
span-trace-coverage#3.4 · no · the viola-mcp frame tools' server check is unchanged, since the chunk adds no MCP tool
span-trace-coverage#4.1 · no · the run start sequence's span chain is not changed by the chunk (Not needed: the start scenario)
span-trace-coverage#4.2 · no · the run start's required span attributes are not changed, and the chunk adds no start span (Not needed: the start scenario)
span-trace-coverage#4.3 · no · the run start's log fields, product order and schema closure are not changed, and the chunk writes no start line (Not needed: the start scenario)
span-trace-coverage#4.4 · no · the run start's cleanup and the run's own process-exit are not changed (Not needed: the start scenario)
span-trace-coverage#5.1 · yes · driver and driven reach no process-log line, the rule the new link and unlink lines must keep; the send chain and its implicit-link ordering are not reached
span-trace-coverage#5.2 · yes · channel.request and channel.dispatch carry method and conn, which the new methods' channel spans carry with their own method value; the send-only readiness, paste and confirm-window attributes are not reached
span-trace-coverage#5.3 · yes · the channel-request and channel-response field shapes the new methods log, and the self-reported trust label that keeps a link or unlink driver name from reading as an authenticated identity; the send-issued, send-confirmed and send-refused lines are not reached
span-trace-coverage#5.4 · yes · channel-response is written from channel.dispatch's Drop guard on every return, and a verb closes with a process-exit code (1 internal-error, 21 server-verify-failed during connect) that the new verbs honour; the confirm-window close is send-only
span-trace-coverage#6.1 · yes · the frame-verb server check runs inside the verb's span and before channel.request, with no channel.request on refusal, which the new verbs also run; the wait.block and run.wait_dispatch chain is wait's own
span-trace-coverage#6.2 · no · wait.block's after and timeout_ms and run.wait_dispatch's woken_kind are wait-only attributes the new verbs do not carry
span-trace-coverage#6.3 · yes · the channel-request and channel-response shapes with the closed method values, the client's instance-unreachable reply line, and the WARN process-exit with exit 21 instance-dead (during connect or call) or server-verify-failed (during connect) are what the new verbs log; wait's after, timeout_ms and outcome fields are not reached
span-trace-coverage#6.4 · yes · a frame verb's span closes on its response, a connection error, or a pre-frame server refusal with no channel.request opened, and a mid-call disconnect is logged once by the client; the new verbs close the same way
span-trace-coverage#7.1 · no · the dialog-to-answer scenario and its implicit link on a taken answer stand as built (Not needed: the dialog scenario)
