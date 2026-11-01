# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · yes · the new connect-time refusals sit inside the CLIENT spans listed (send.client, wait.block, last.client, cli.link, cli.unlink, client-side channel.request), so their kind binds
span-trace-coverage#1.2 · yes · mcp.tool_call and hook.handle are where the MCP client refusal (instance-unreachable with detail) and the hook fail-open refusal sit, so their SERVER kind binds
span-trace-coverage#1.3 · yes · run.start and the wrapper's socket-directory check before bind sit under the INTERNAL run.* spans
span-trace-coverage#2.1 · yes · the CLI verbs' connect refusals and the viola list scan refusal open or skip cli-<name>.ndjson as this row states
span-trace-coverage#2.2 · yes · the wrapper start sequence (bind_endpoint directory refusal, pre-child refusals) is the manual span set of the viola run row
span-trace-coverage#2.3 · yes · the hook connect fail-open through hook-decision detail sits in hook.handle, under this row's span chain and line rules
span-trace-coverage#2.4 · yes · the euid checks live in viola-channel, a refused connection writes no channel-request line, and a verified euid does not make from authenticated
span-trace-coverage#2.5 · yes · the MCP client refusal sits in mcp.tool_call, codes-only with no line, as this row requires
span-trace-coverage#3.1 · yes · the new client euid refusal sits beside the server-verify-failed exit on the connect path, and the macOS pid gap is the carve-out the chunk records
span-trace-coverage#3.2 · yes · each new connect-time refusal must fall inside its dispatch span and be asserted through refused_inside as the server-check cases are
span-trace-coverage#3.3 · yes · client::refuse_untrusted is the shared instance check the new client refusals reach beside, before any role file opens and with no line
span-trace-coverage#3.4 · yes · the hook connect's server check shares one connect in hook.handle, and the chunk reaches hook only as a fail-open hook-decision
span-trace-coverage#3.5 · no · channel-response's -32602 detail word is not reached, since the chunk adds no -32602 reply and a refused connection writes no channel-response line
span-trace-coverage#3.6 · yes · the MCP frame tools' server check inside mcp.tool_call is where the Tokio client's euid and directory refusals map to instance-unreachable with a detail
span-trace-coverage#3.7 · no · peer-reply screening through is_code_word and sanitised_fault is not reached, since the chunk's refusals are local and produce no peer reply
span-trace-coverage#4.1 · yes · the run start chain runs the new socket-directory check before channel.bind, so the refusal lands in run.start before the bind span
span-trace-coverage#4.2 · yes · span attributes must never carry the endpoint name or socket path, which binds the new refusal spans
span-trace-coverage#4.3 · yes · run's process-exit detail list gains the refused socket directory and the pre-child refusal pair, with no path or pid, and the home check's pre-log refusal
span-trace-coverage#4.4 · yes · a pre-child refusal ends run.start and emits run's process-exit subject self, the cleanup rule the new refusal follows
span-trace-coverage#5.1 · no · the run.pin_copy concurrent placement is not reached, since the chunk touches no pin copy and no place_private_new
span-trace-coverage#6.1 · yes · send.client's instance check, then the server check before channel.request, is the connect path where the client euid and directory refusals sit
span-trace-coverage#6.2 · no · the send's channel.request, channel.dispatch and paste, readiness and confirm attributes sit past connect, which the chunk does not change or open on refusal
span-trace-coverage#6.3 · yes · the client-side server-verification refusal rule (no channel-request or channel-response line, CLI process-exit 21 at WARN with during connect, MCP no line) is the form the new connect refusals take
span-trace-coverage#6.4 · yes · send.client's exit-21 detail list gains the new refusal codes with during connect, under the process-exit close rule
span-trace-coverage#7.1 · yes · the wait and last connect path runs the server check inside wait.block and last.client before channel.request, where the new refusals sit
