# Classified — obs-plan · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · the listed client span names are unchanged, and the socket-directory check and euid read sit inside existing spans with no new span name
span-trace-coverage#1.2 · no · the server span names are unchanged, and a dropped peer sends no request so every channel.request still has its channel.dispatch
span-trace-coverage#1.3 · no · no internal span is added, since the socket-directory step is spanless by design
span-trace-coverage#2.1 · yes · add-to · F3: a socket-directory refusal on a frame verb writes one exit-21 process-exit into the caller's cli log, a line this row's refused-check rule does not carry
span-trace-coverage#2.2 · yes · add-to · F2: run's start gains a spanless socket-directory step whose refusal writes one process-exit self line, which this start instrumentation row must carry
span-trace-coverage#2.3 · yes · add-to · F4: a refused socket directory writes one hook-decision with detail socket-dir-refused, a line the hook list must carry beside the instance-check exception
span-trace-coverage#2.4 · yes · add-to · F6: the wrapper's accept_loop drops a refused peer before any frame, so this channel row carries that no channel-request or channel-response line is written
span-trace-coverage#2.5 · no · MCP's socket-directory refusal writes no line and adds no span, and the mcp.tool_call spans and codes-only rejects are unchanged
span-trace-coverage#3.1 · yes · stale · F7 adds the euid decision on macOS, so server-verify-failed reaches macOS and the Linux and Windows reach reads stale, while the pid gap stays
span-trace-coverage#3.2 · no · the named captured-span cases and the refused_inside helper still hold for the pid refusal, and the new euid refusal has no case that this row claims
span-trace-coverage#3.3 · yes · add-to · F9: a socket-directory refusal gets its own human pair on frame-verb refusals, joining the instance pair this row lists on stderr
span-trace-coverage#3.4 · yes · stale · F7 adds the euid decision to the hook's connect, so server-verify-failed reaches macOS and the "macOS on the recorded gap" reading stales
span-trace-coverage#3.5 · no · the session adds no -32602 fault or detail word, names no change to ResponseLine or fault_of, and a dropped peer sends no frame
span-trace-coverage#3.6 · yes · stale · F7 adds the euid decision after MCP's TokioClient connect, so server-verify-failed reaches macOS and the Linux and Windows reading stales
span-trace-coverage#3.7 · no · reply screening through is_code_word and sanitised_fault is unchanged, and the euid read adds no reply handling, span or line
span-trace-coverage#4.1 · yes · add-to · F2: the start sequence gains a spanless socket-directory step before any snapshot, child or bind, which the ordered start chain must carry
span-trace-coverage#4.2 · no · run.start, collision, pin, version, bind, snapshot and spawn attributes are unchanged, and the new step carries no span or attribute
span-trace-coverage#4.3 · yes · add-to · F2: the on-failure process-exit detail list gains socket-dir-refused for run's and revive's start refusals
span-trace-coverage#4.4 · no · the cleanup chain runs after pty.spawn, and the socket-directory refusal exits before any span or child, its line carried by span-trace-coverage#4.3
span-trace-coverage#5.1 · no · the session makes no change to the pin placement or its no-line rule
span-trace-coverage#6.1 · yes · stale · the directory check now precedes the connect (F3) and the euid decision reaches macOS (F7), so the "macOS on the recorded gap" reading stales
span-trace-coverage#6.2 · no · the send span attributes, text_bytes and window_ms are unchanged and no send span is added
span-trace-coverage#6.3 · yes · stale · F7 moves the client-side server-verification refusal's reach to macOS, so the "Linux and Windows" reading stales, and F3 adds a CLI send exit-21 line for a socket-directory refusal
span-trace-coverage#6.4 · yes · stale · F7 moves server-verify-failed's reach to macOS, so the "macOS on the recorded gap" reading stales, and F3 adds socket-dir-refused to the exit-21 detail list
span-trace-coverage#7.1 · yes · stale · F7 adds the euid decision inside wait.block and last.client, so the server check's Linux and Windows reach reads stale on macOS
