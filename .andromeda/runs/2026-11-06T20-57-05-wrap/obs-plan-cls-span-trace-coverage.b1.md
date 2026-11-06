@answer n=1 entity=span-trace-coverage#1.1 v=no
why: the CLIENT span names, pty.spawn included, are unchanged and the PTY seam re-read on 0.9.0 keeps the openpty backend, with all gates green on 849aad18
@answer n=2 entity=span-trace-coverage#1.2 v=no
why: the SERVER span names are unchanged and none of the moved dependencies or cleaned remnants is a span, with all gates green on 849aad18
@answer n=3 entity=span-trace-coverage#1.3 v=no
why: the INTERNAL span names are unchanged and the pty.paste_write seam re-read on 0.9.0 keeps its openpty backend
@answer n=4 entity=span-trace-coverage#2.1 v=no
why: the cli verb dispatch spans and the process-start and process-exit rows are unchanged, clap is not among the moved dependencies, and the gates are green
@answer n=5 entity=span-trace-coverage#2.2 v=yes mode=stale
why: the session moved portable-pty from =0.8.1 to =0.9.0, so the 0.8.1 pin this PTY-seam row names is stale
@answer n=6 entity=span-trace-coverage#2.3 v=no
why: the hook spans and the instance-check order in hook.rs are unchanged and no hook behaviour moved, with all gates green
@answer n=7 entity=span-trace-coverage#2.4 v=no
why: viola-channel is not among the reached files and interprocess is not in the moved dependency set
@answer n=8 entity=span-trace-coverage#2.5 v=no
why: viola-channel is not reached and its accept_loop peer filter is unchanged
@answer n=9 entity=span-trace-coverage#2.6 v=no
why: viola-mcp is not among the reached files and rmcp is not in the moved dependency set
@answer n=10 entity=span-trace-coverage#3.1 v=no
why: the server-check order in client connect holds on the all-gates-green run, and its peer comparison already runs on Linux only
@answer n=11 entity=span-trace-coverage#3.2 v=no
why: the euid read at each client seam is unchanged, and the cleaned remnants (plain_drive, the send warning, EXE_SUFFIX, the .exe trim) are not this check
@answer n=12 entity=span-trace-coverage#3.3 v=no
why: the captured-span refusal cases sit inside the all-gates-green run on 849aad18 and none of the cleaned remnants is in them
@answer n=13 entity=span-trace-coverage#3.4 v=no
why: the loosened socket-directory span cases stay green on 849aad18 and the check they open around is unchanged
@answer n=14 entity=span-trace-coverage#3.5 v=no
why: the instance check client::refuse_untrusted is not among the cleaned remnants and the all-gates-green run covers its refusal
@answer n=15 entity=span-trace-coverage#3.6 v=no
why: the socket-dir refusal's detail, WARN level and no-frame behaviour are unchanged, and no PROVISIONAL mark on this row needs lifting
@answer n=16 entity=span-trace-coverage#3.7 v=no
why: the socket-directory pair's wording was confirmed on the founder's word and this row carries no PROVISIONAL mark, so nothing moves
@answer n=17 entity=span-trace-coverage#3.8 v=no
why: the hook connect sequence is unchanged, its peer comparison already runs on Linux only, and the all-gates-green run covers it
@answer n=18 entity=span-trace-coverage#3.9 v=no
why: the hook socket-dir refusal line in hook.rs is unchanged and the all-gates-green run covers it
@answer n=19 entity=span-trace-coverage#3.10 v=no
why: viola-channel is not reached and the -32602 closed-word detail on channel-response is not part of the session
@answer n=20 entity=span-trace-coverage#3.11 v=no
why: viola-mcp is not among the reached files and the frame tools' check order is unchanged
@answer n=21 entity=span-trace-coverage#3.12 v=no
why: the CLI reply_of and MCP of_reply screens are unchanged and the all-gates-green run covers them
@answer n=22 entity=span-trace-coverage#4.1 v=no
why: the start span chain holds with the budget gate staying so run.budget_eval stands, and pty.spawn stays on openpty after the 0.9.0 re-read
@answer n=23 entity=span-trace-coverage#4.2 v=no
why: the required span attributes, pty_backend openpty included, are unchanged on the 0.9.0 re-read and the version gate attribute names did not move
@answer n=24 entity=span-trace-coverage#4.3 v=no
why: the process-start claude-child row is re-read on 0.9.0 with pty_backend openpty and the boot start_records check green, so no value moved
@answer n=25 entity=span-trace-coverage#4.4 v=no
why: the handle-wait exit of the claude child is re-read on 0.9.0 and unchanged
