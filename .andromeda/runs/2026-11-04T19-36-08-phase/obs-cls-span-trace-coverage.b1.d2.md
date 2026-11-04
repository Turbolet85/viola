@answer n=1 entity=span-trace-coverage#1.1 v=yes
why: the CLIENT span kinds include pty.spawn, which the chunk's openpty seam on Unix opens with the same kind
@answer n=2 entity=span-trace-coverage#1.2 v=yes
why: channel.dispatch is the wrapper-side server span that the Unix channel telemetry must carry (channel-request and channel-response with corr and conn)
@answer n=3 entity=span-trace-coverage#1.3 v=yes
why: the run.* and state.* INTERNAL spans and pty.paste_write are opened by the Unix run path, and paste logs text_bytes only
@answer n=4 entity=span-trace-coverage#2.1 v=yes
why: the viola revive start arm logs as process run after its check and carries the Unix revive refusals, and the frame and list verbs' refusal lines are the cli surface the Unix suite runs
@answer n=5 entity=span-trace-coverage#2.2 v=yes
why: the viola run manual spans, the pty.* seam and the child process-start and process-exit from the handle-wait thread are the chunk's openpty seam
@answer n=6 entity=span-trace-coverage#2.3 v=yes
why: the hook contract (exit 0, empty stderr, failed log write swallowed, silent no-op without VIOLA_NAME) is what the Unix suite must hold, and this entity states it
@answer n=7 entity=span-trace-coverage#2.4 v=yes
why: it fixes the field list of channel-request and channel-response, which the scope names with corr and conn on both sides
@answer n=8 entity=span-trace-coverage#2.5 v=yes
why: a refused euid is dropped by accept_loop with no span and no line, the Unix channel endpoint behaviour the scope names
@answer n=9 entity=span-trace-coverage#2.6 v=yes
why: the mcp.tool_call span, codes-only rejects and the explicit file-only writer bind the MCP macOS instance-unreachable carve-out and the same file sink on every OS
@answer n=10 entity=span-trace-coverage#3.1 v=yes
why: the server check before any frame ends in process-exit exit 21 server-verify-failed, and macOS's euid-only reading is the named carve-out the scope states
@answer n=11 entity=span-trace-coverage#3.2 v=yes
why: the client euid read and peer_admitted refusal are the Unix euid reporting the chunk's connect seams must honour, including macOS
@answer n=12 entity=span-trace-coverage#3.3 v=yes
why: the per-verb captured-span refusal cases through refused_inside are tests the full fake-agent suite runs on ubuntu-latest and macos-latest
@answer n=13 entity=span-trace-coverage#3.4 v=yes
why: the socket-directory span cases sit beside the macOS-gated refused_inside helper, a cfg(target_os) gate the chunk must carry a reason for
@answer n=14 entity=span-trace-coverage#3.5 v=yes
why: the instance check before any role file and its strict-modes-failed instance-unreachable --json document are closed words the scope requires on every OS
@answer n=15 entity=span-trace-coverage#3.6 v=yes
why: a refused socket directory gives process-exit exit 21 detail socket-dir-refused before any channel-request line, which the Unix channel telemetry names
@answer n=16 entity=span-trace-coverage#3.7 v=yes
why: the plain-ASCII two-line refusal pair is the same fixed pair that run's pre-child refusal writes through human::refuse on Unix
@answer n=17 entity=span-trace-coverage#3.8 v=yes
why: the hook server check, fail-open server-verify-failed and macOS euid-only carve-out are the hook behaviour the chunk's Unix suite and the macOS carve-out name
@answer n=18 entity=span-trace-coverage#3.9 v=yes
why: the hook's socket-dir-refused WARN line in hook.handle is the Unix socket-directory detail the scope names
@answer n=19 entity=span-trace-coverage#3.10 v=yes
why: the -32602 closed detail word on channel-response is a closed word the chunk re-asserts on every OS
@answer n=20 entity=span-trace-coverage#3.11 v=yes
why: the MCP frame tools' check order, codes-only instance-unreachable server-verify-failed and macOS euid-only carve-out are what the MCP path must honour on Unix
@answer n=21 entity=span-trace-coverage#3.12 v=no
why: CLI reply_of and MCP of_reply screen incoming peer replies with no span or line; the chunk neither changes nor calls that screen, so it is a re-check every change owes
@answer n=22 entity=span-trace-coverage#4.1 v=yes
why: the run start sequence includes the Unix-only settings override write and the socket_dir_ready refusal that the chunk's run and revive starts must honour
@answer n=23 entity=span-trace-coverage#4.2 v=yes
why: pty.spawn's pty_backend (which takes the openpty value on Unix) and the never-list of endpoint, pinned path and env values are the required attributes of the chunk's seam
@answer n=24 entity=span-trace-coverage#4.3 v=yes
why: the process-start claude-child fields, the socket-dir-refused and home-tree refusals, and the diag-line pty_backend string are the log fields every OS must carry
@answer n=25 entity=span-trace-coverage#4.4 v=yes
why: the handle-wait process-exit with exit_source and never on reader EOF is the child exit rule the chunk's openpty seam must act on
