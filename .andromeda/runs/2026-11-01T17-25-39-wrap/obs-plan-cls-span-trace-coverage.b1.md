# Classified — obs-plan · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · the session adds no span name, and its new refusals sit inside the existing dispatch spans
span-trace-coverage#1.2 · no · the accept-loop peer drop opens no server span, so the SERVER span names hold
span-trace-coverage#1.3 · no · the new run start step carries no span, so the INTERNAL span names are unchanged
span-trace-coverage#2.1 · yes · add-to · F3: a frame verb meeting a refused socket directory writes one process-exit with detail socket-dir-refused at WARN, which the cli row's process-exit detail carries
span-trace-coverage#2.2 · yes · add-to · F2: viola run's start check writes one process-exit with exit_code 1 and detail socket-dir-refused before any snapshot, child or bind
span-trace-coverage#2.3 · yes · add-to · F4: a refused socket directory writes one hook-decision with detail socket-dir-refused at WARN, so it is not one of the silent no-ops
span-trace-coverage#2.4 · yes · add-to · F6: the wrapper's accept loop drops a refused peer before any frame, and the channel lines this row names stay unwritten for it
span-trace-coverage#2.5 · no · the MCP socket-directory refusal writes no line, as its other pre-frame refusals already do, so the codes-only rule holds
span-trace-coverage#3.1 · yes · stale · the server check now reaches macOS through the euid decision, so the Linux and Windows reach and the macOS gap reading moved
span-trace-coverage#3.2 · yes · add-to · F7: the euid refusal is a server-verify refusal inside connect that no committed captured-span case asserts
span-trace-coverage#3.3 · no · the instance check this row names still runs first and refuses with no line; the directory check is a later, separate refusal
span-trace-coverage#3.4 · yes · stale · the hook's server check now includes a euid decision on macOS, so "macOS on the recorded gap" no longer describes it
span-trace-coverage#3.5 · no · the -32602 closed words of §6 are unchanged; socket-dir-refused is a process-exit and hook-decision word, not a fault detail
span-trace-coverage#3.6 · yes · stale · the MCP frame server check now reaches macOS through the euid decision, so the Linux and Windows reach moved
span-trace-coverage#3.7 · no · reply screening is unchanged, and the new closed word is a local refusal word, not a peer reply word
span-trace-coverage#4.1 · yes · add-to · F2: the start gains a step before any snapshot, child or bind that carries no span and writes a process-exit on refusal
span-trace-coverage#4.2 · no · no span attribute is added or changed; the new refusal is a process-exit, not a span
span-trace-coverage#4.3 · yes · add-to · F2: the failure detail list of run's process-exit gains socket-dir-refused from the new pre-snapshot refusal
span-trace-coverage#4.4 · no · run.start's close and the child's process-exit on normal ends are unchanged by the refusal
span-trace-coverage#5.1 · no · the concurrent first-start pin-copy placement is not touched by the session
span-trace-coverage#6.1 · yes · stale · the server check on the send path now reaches macOS through the euid decision, so the "macOS on the recorded gap" reading moved
span-trace-coverage#6.2 · no · send's span attributes are unchanged
span-trace-coverage#6.3 · yes · add-to · F3: a CLI send meeting a refused socket directory writes process-exit exit 21 with detail socket-dir-refused, during connect, at WARN, a client-side pre-frame refusal this row lists
span-trace-coverage#6.4 · yes · add-to · F3: send.client's exit-21 detail list gains socket-dir-refused, with during connect
span-trace-coverage#7.1 · yes · stale · the wait and last server check now reaches macOS through the euid decision, so the Linux and Windows reach reading moved
