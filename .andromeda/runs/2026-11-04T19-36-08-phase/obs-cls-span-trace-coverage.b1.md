@answer n=1 entity=span-trace-coverage#1.1 v=yes
why: the CLIENT list names pty.spawn, the span the new Unix openpty seam must carry behind spawn (needs 1 and 2)
@answer n=2 entity=span-trace-coverage#1.2 v=yes
why: the SERVER list names channel.dispatch and hook.handle, which the Unix channel telemetry and the Unix hook contract (needs 7 and 19) must sit under
@answer n=3 entity=span-trace-coverage#1.3 v=yes
why: the INTERNAL list names pty.paste_write, whose paste logs text_bytes only on every OS (needs 2)
@answer n=4 entity=span-trace-coverage#2.1 v=yes
why: the cli verb row sets the process-start, process-exit and refusal-order rules that the Unix run, revive and macOS server-verify paths must honour (needs 6, 8 and 9)
@answer n=5 entity=span-trace-coverage#2.2 v=yes
why: the viola run row puts the child process-start and process-exit on the handle-wait thread, never on reader EOF, and covers the pty seam (needs 1 and 2)
@answer n=6 entity=span-trace-coverage#2.3 v=yes
why: the viola hook row sets the hook.handle to channel.request to hook.decision_emit chain and the exit 0 hook contract the Unix suite must hold (needs 19)
@answer n=7 entity=span-trace-coverage#2.4 v=yes
why: the channel row's corr and conn fields on channel-request and channel-response on both sides are the Unix channel telemetry itself (needs 7)
@answer n=8 entity=span-trace-coverage#2.5 v=yes
why: a peer refused by euid is dropped before any frame and writes no line, the accept_loop behaviour the Unix channel telemetry must match (needs 7)
@answer n=9 entity=span-trace-coverage#2.6 v=yes
why: the MCP row's codes-only send-refused and channel-response on rejects, with no line on the macOS unreachable path, binds the macOS carve-out (needs 6)
@answer n=10 entity=span-trace-coverage#3.1 v=yes
why: the server check, its macOS euid-only carve-out and process-exit exit 21 with detail server-verify-failed during connect are what the chunk's macOS CLI wait and last must meet (needs 6)
@answer n=11 entity=span-trace-coverage#3.2 v=yes
why: the client euid seams and peer_admitted refusing a peer of another euid are the Unix verify the connect paths must act on (needs 6 and 7)
@answer n=12 entity=span-trace-coverage#3.3 v=yes
why: the captured-span refusal cases run in each OS's suite, so the Unix server check must sit inside the dispatch span those tests read (needs 6 and the full-suite run on macOS)
@answer n=13 entity=span-trace-coverage#3.4 v=yes
why: the socket-dir span cases read through refused_seen, which sits outside the macOS-gated refused_inside helper, so the chunk's macOS suite run meets that gate
@answer n=14 entity=span-trace-coverage#3.5 v=yes
why: the instance check that precedes any role file or process log on the six frame verbs, with its --json document, is a refusal order and closed-word rule the Unix paths must keep (needs 5 and 21)
@answer n=15 entity=span-trace-coverage#3.6 v=yes
why: a refused socket directory writes process-exit detail socket-dir-refused before any channel-request line, the Unix telemetry rule of needs 7
@answer n=16 entity=span-trace-coverage#3.7 v=yes
why: the socket-dir human pair is the fixed stderr pair that the run refusal on Unix writes through human::refuse (needs 8)
@answer n=17 entity=span-trace-coverage#3.8 v=yes
why: the hook server check and its macOS euid-only server-verify-failed detail govern the Unix hook path (needs 6 and 19)
@answer n=18 entity=span-trace-coverage#3.9 v=yes
why: a hook meeting a refused socket directory writes hook-decision socket-dir-refused with no frame, which the Unix telemetry must carry (needs 7)
@answer n=19 entity=span-trace-coverage#3.10 v=yes
why: the -32602 closed detail on channel-response lines written on both sides is a closed word the Unix channel lines must carry (needs 5 and 7)
@answer n=20 entity=span-trace-coverage#3.11 v=yes
why: the MCP tool server check order and its instance-unreachable details, with the macOS euid-only verdict, are what the Unix MCP path must meet (needs 5 and 6)
@answer n=21 entity=span-trace-coverage#3.12 v=no
why: the clients' reply screening adds no span and no line and is shared unchanged on every OS, so the chunk's code acts on none of it and the scope names no such behaviour
@answer n=22 entity=span-trace-coverage#4.1 v=yes
why: the Unix-only settings override write and the socket_dir_ready refusal before any snapshot, child or bind bind the run start on Unix (needs 8)
@answer n=23 entity=span-trace-coverage#4.2 v=yes
why: the required attributes include pty_backend openpty on the pty.spawn span and channel.bind endpoint_kind for the Unix seam (needs 1 and 2)
@answer n=24 entity=span-trace-coverage#4.3 v=yes
why: the required fields, the run and revive refusal details and the claude-child process-start with pty_backend bind the Unix start and revive paths (needs 4, 5, 8 and 9)
@answer n=25 entity=span-trace-coverage#4.4 v=yes
why: the handle-wait thread emits the claude-child process-exit with exit_source and never on reader EOF, which the Unix pty seam must follow (needs 2)
