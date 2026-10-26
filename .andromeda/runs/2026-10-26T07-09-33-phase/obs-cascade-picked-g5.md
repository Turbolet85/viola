# obs · picked accounting · group 5 of 7

## Picked
span-trace-coverage#1.1 · carried · the CLIENT kind list holds every verb span the chunk's pre-check runs inside and the client `channel.request` it must precede
span-trace-coverage#1.2 · carried · `mcp.tool_call` and `hook.handle` are the SERVER-kind spans the chunk's MCP and hook verification runs inside
span-trace-coverage#1.3 · carried · `state.*` and `hook.decision_emit` are the INTERNAL kinds a verification span in `viola-state` or the hook's fail-open emit takes
span-trace-coverage#2.1 · carried · the cli row rules the dispatch spans and the `process-exit{exit_code, detail}` the exit-21 mismatch outcome writes, and the chain-to-`detail-cli.ndjson` rule
span-trace-coverage#2.3 · carried · the hook row rules the `hook.handle` › `channel.request` path the chunk gates and the exit-0 fail-open it must keep
span-trace-coverage#2.4 · carried · the viola-channel row rules the client `channel.request` span and `channel-request` / `channel-response` lines on the clients' connect the chunk edits
span-trace-coverage#2.5 · carried · the viola-mcp row rules `mcp.tool_call` and the codes-only reject lines the `instance-unreachable` mismatch writes
span-trace-coverage#3.2 · carried · `run.collision_check`'s outcome is the behaviour of the collision check whose `read_snapshot` call the chunk re-reads
span-trace-coverage#3.3 · carried · the start sequence's `already-live|squatted-name` failure details record the re-read collision check's outcome
span-trace-coverage#4.1 · carried · the confirmed-send path's client spans and the hook's `channel.request(hook.event)` are what the verification precedes
span-trace-coverage#4.2 · carried · the client `channel.request` attributes hold only on a call the verification lets through
span-trace-coverage#4.3 · carried · `channel-request` / `channel-response` and the client-side before-any-frame refusal are the lines a verification refusal must write or withhold
span-trace-coverage#4.4 · carried · `send.client`'s exit-21 `detail` / `during` closing is the catalog the mismatch outcome joins
span-trace-coverage#5.1 · carried · `wait.block` / `last.client` › `channel.request` is the path the `wait` / `last` verification gates
span-trace-coverage#5.3 · carried · the `wait` / `last` exit-21 note that `server-verify-failed` cannot occur until this entry lands is changed by this chunk
span-trace-coverage#5.4 · carried · `wait.block` must close on, and the client log once, the new pre-connection refusal
span-trace-coverage#6.1 · carried · the dialog path's `hook.handle` › `channel.request(hook.dialog)` and `answer.client` › `channel.request(answer)` are gated by the chunk
span-trace-coverage#6.2 · carried · `hook.decision_emit`'s `decision_emitted` records the hook's fail-open on a mismatch
span-trace-coverage#6.3 · carried · a dialog hook's `hook-invoked` timing and the fail-open `corr:null` lines rule the refused `hook.dialog` verification
span-trace-coverage#6.4 · carried · fail-open before `dialog_id` is the `hook-invoked` + `hook-decision{corr:null, detail}` pair the mismatch writes
span-trace-coverage#7.1 · carried · `pause.client` / `release.client` › `channel.request` is the path the wheel clients' verification gates
span-trace-coverage#7.3 · carried · `channel-request` / `channel-response{method:"release"}` are written only on a call the verification lets through
span-trace-coverage#11.1 · carried · E5's rule that the five other snapshot readers stay on `read_snapshot` with no `state-recovered` line is the chunk's re-read, held by the founder
