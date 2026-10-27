# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · yes · the CLIENT kinds name send.client (the CLI paste refusal), channel.request (the client frame bound) and statusline.shell_out (the stdout bound)
span-trace-coverage#1.2 · yes · the SERVER kinds name channel.dispatch (over-size frames and the wrapper paste refusal), mcp.tool_call (codes-only rejects), hook.handle and hook.statusline (the stdin bounds)
span-trace-coverage#1.3 · yes · the INTERNAL kinds name run.readiness_gate (the input-not-ready degrade), run.version_gate and run.budget_eval (the ledger and budget.json reads), state.* (the snapshot and events readers) and pty.paste_write (text_bytes)
span-trace-coverage#2.1 · yes · send-refused{side:"client"} for control-character and empty-text is the chunk's client refusal, and revive --list reads the bounded events chain
span-trace-coverage#2.2 · yes · run.readiness_gate carries the vt100 degrade, and the start sequence, the pty seam and channel.dispatch carry the bounded readers
span-trace-coverage#2.3 · yes · hook stdin through take(MAX_FRAME + 1) with the fail-open hook-decision detail, read only past the instance check
span-trace-coverage#2.4 · yes · channel-request and channel-response never record params or result bodies, and the chunk's over-size and invalid-frame rejects (-32602 / -32600 at WARN) are such lines
span-trace-coverage#2.5 · yes · codes-only send-refused and channel-response on mcp rejects, and instance-unreachable with no line, are the chunk's mcp argument rule
span-trace-coverage#3.1 · yes · the send client's paste refusal is one of the client-side refusals that must run before the connect (the server-verify lines belong to Unix endpoint hardening)
span-trace-coverage#3.2 · no · re-check only: these captured-span cases pin the server check's refusal place, which the chunk does not touch (the server-identity check belongs to Unix endpoint hardening)
span-trace-coverage#3.3 · yes · a name the validator refuses resolves no instance, so no role file opens and no line is written, the names-before-path-joins rule
span-trace-coverage#3.4 · yes · only a whole snapshot read goes on, so an oversize snapshot must take the replay path and read as unreachable (the server-verify lines belong to Unix endpoint hardening)
span-trace-coverage#3.5 · yes · check_instance before live_endpoint, and instance-unreachable with no frame and no line, are the chunk's mcp instance and no-line rules
span-trace-coverage#4.1 · yes · the start's statusline-source read and events link restore, both bounded by the chunk, carry no span and no log line
span-trace-coverage#4.2 · yes · run.version_gate's cli_verified is the ledger read the chunk makes fail to false, and env values and pinned paths never reach attributes
span-trace-coverage#4.3 · yes · the version-probe and agents-probe pairs, and parse-rejected{parser:"claude-agents-json"} on an oversize or malformed agents read, are the chunk's spawn and agents-read rules
span-trace-coverage#4.4 · yes · the PTY output cap must never emit process-exit on reader EOF, which only the handle-wait thread writes for the child
span-trace-coverage#5.1 · no · the pinned-copy placement belongs to the completed pinned-copy chunk, and the chunk reads no pinned exe and edits no pin code
span-trace-coverage#6.1 · yes · the confirmed send chain runs through send.client, channel.dispatch and run.readiness_gate, the path whose paste and vt100 degrade rules the chunk re-verifies
span-trace-coverage#6.2 · yes · run.readiness_gate outcome input-not-ready and vt100_panicked are what the degrade witness reads, and pty.paste_write text_bytes is the paste length
span-trace-coverage#6.3 · yes · send-refused for control-character and empty-text on the client and wrapper sides, codes only, is the chunk's paste-rule log
span-trace-coverage#6.4 · yes · channel-response from the Drop guard must still log on the chunk's new early returns, and send.client's process-exit must carry the paste refusal's exit code
span-trace-coverage#7.1 · yes · wait and last are channel frames on both sides that the chunk bounds, and channel.dispatch answers their over-size or invalid params at WARN
span-trace-coverage#7.2 · no · re-check only: the wait.block and run.wait_dispatch attributes come from CLI args and the wake set, which the chunk does not name, and its frame bounds bind at 7.1 and 7.3
span-trace-coverage#7.3 · yes · channel-request and channel-response for wait and last never record params or result bodies, and after and timeout_ms appear only as u64
