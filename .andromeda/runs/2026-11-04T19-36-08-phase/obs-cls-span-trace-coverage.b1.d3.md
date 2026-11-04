@answer n=1 entity=span-trace-coverage#1.1 v=yes
why: the CLIENT list names pty.spawn, the seam spawn the openpty backend fills on Unix, and channel.request client side, which carries corr and conn on both sides
@answer n=2 entity=span-trace-coverage#1.2 v=yes
why: the SERVER list names channel.dispatch, the wrapper side of every channel.request, plus mcp.tool_call and hook.handle, which the macOS euid carve-out reaches
@answer n=3 entity=span-trace-coverage#1.3 v=yes
why: the INTERNAL list names run.* and state.* spans on the Unix start path and pty.paste_write, which logs text_bytes only
@answer n=4 entity=span-trace-coverage#2.1 v=yes
why: the cli verbs process-start and process-exit lines and the macOS wait and last exit 21 server-verify-failed are named by the scope
@answer n=5 entity=span-trace-coverage#2.2 v=yes
why: viola run start sequence, the pty seam and the child process-exit from the handle-wait thread are the chunk's openpty work
@answer n=6 entity=span-trace-coverage#2.3 v=yes
why: viola hook spans, the exit 0 catch_unwind and the io::sink fallback are the hook contract under the Unix suite
@answer n=7 entity=span-trace-coverage#2.4 v=yes
why: channel-request and channel-response carry corr and conn on both sides, which the Unix channel telemetry names
@answer n=8 entity=span-trace-coverage#2.5 v=yes
why: accept_loop dropping a peer by euid before any frame with no line is stated by the Unix channel endpoint telemetry
@answer n=9 entity=span-trace-coverage#2.6 v=yes
why: viola-mcp mcp.tool_call and codes-only lines bind, since macOS MCP returns instance-unreachable with no line
@answer n=10 entity=span-trace-coverage#3.1 v=yes
why: the eight frame verbs server check ends in process-exit exit 21 server-verify-failed, and its macOS euid-only carve-out is named by the scope
@answer n=11 entity=span-trace-coverage#3.2 v=yes
why: each client seam Unix euid read and peer_admitted refusal is the check the macOS carve-out and the Unix channel telemetry rest on
@answer n=12 entity=span-trace-coverage#3.3 v=yes
why: the captured-span refusal cases must run in the full fake-agent suite on macOS, so their cfg gates fall under the cfg reason rule
@answer n=13 entity=span-trace-coverage#3.4 v=yes
why: the socket-directory refusal captured-span cases sit behind the macOS-gated refused_inside helper, which the full macOS suite must reach
@answer n=14 entity=span-trace-coverage#3.5 v=yes
why: the instance check strict-modes-failed refusal and its --json document are Unix-reached details, and the refusal opens no process log
@answer n=15 entity=span-trace-coverage#3.6 v=yes
why: a frame verb meeting a refused socket directory writes process-exit detail socket-dir-refused before any channel-request line, as the scope names
@answer n=16 entity=span-trace-coverage#3.7 v=yes
why: the socket-directory refusal pair through human::refuse is the fixed stderr pair the scope names for run pre-child refusals
@answer n=17 entity=span-trace-coverage#3.8 v=yes
why: the hook server check, its server-verify-failed fail-open decision and the macOS euid-only meaning are the hook path the macOS carve-out names
@answer n=18 entity=span-trace-coverage#3.9 v=yes
why: a hook meeting a refused socket directory writes hook-decision socket-dir-refused before any channel-request line, as the scope names
@answer n=19 entity=span-trace-coverage#3.10 v=yes
why: the channel-response detail closed word on -32602 is a closed word on both sides, which the one-schema re-assertion covers
@answer n=20 entity=span-trace-coverage#3.11 v=yes
why: viola-mcp frame tools run the instance check, the socket-directory check and server verify, and macOS returns instance-unreachable with no line
@answer n=21 entity=span-trace-coverage#3.12 v=no
why: the peer reply screen adds no span and no line, and the scope neither names it nor changes the reply path it guards
@answer n=22 entity=span-trace-coverage#4.1 v=yes
why: the run start sequence reaches pty.spawn for the openpty backend and the socket_dir_ready refusal on run and revive starts
@answer n=23 entity=span-trace-coverage#4.2 v=yes
why: the pty_backend value set names openpty, the value the Unix seam must record, so the attribute list binds beyond the span convention
@answer n=24 entity=span-trace-coverage#4.3 v=yes
why: the process-start claude-child fields, the socket-dir-refused detail and the home-tree check before the log are the Unix pre-child refusals the scope names
@answer n=25 entity=span-trace-coverage#4.4 v=yes
why: run.start closes on pty.spawn return and process-exit claude-child comes from the handle-wait thread, never reader EOF, as the scope names
