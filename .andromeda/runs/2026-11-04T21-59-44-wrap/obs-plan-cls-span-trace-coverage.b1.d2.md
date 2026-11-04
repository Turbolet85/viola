@answer n=1 entity=span-trace-coverage#1.1 v=no
why: the session renames no CLIENT span and moves no product source, only tests, a script and the nextest config
@answer n=2 entity=span-trace-coverage#1.2 v=no
why: the SERVER span names are unchanged and no test the session added asserts or alters them
@answer n=3 entity=span-trace-coverage#1.3 v=no
why: the run.* and state.* INTERNAL spans are unchanged, as no product file under src/ was touched
@answer n=4 entity=span-trace-coverage#2.1 v=no
why: the list witness runs viola list without changing its cli.list span or its strict-modes-gated process log
@answer n=5 entity=span-trace-coverage#2.2 v=no
why: the run start spans and pty seam are unchanged, and the child-end witness reads the process table, not a span
@answer n=6 entity=span-trace-coverage#2.3 v=no
why: the hook spans and fail-open paths are unchanged, and no hook source or hook test moved in the change
@answer n=7 entity=span-trace-coverage#2.4 v=no
why: the channel.request and channel.dispatch spans and their line fields are unchanged, and no channel code moved
@answer n=8 entity=span-trace-coverage#2.5 v=no
why: the accept-thread refusal is product code the session did not change, and no test the session added exercises it
@answer n=9 entity=span-trace-coverage#2.6 v=no
why: the viola-mcp mcp.tool_call span and its codes-only lines are unchanged, and no MCP source or test moved
@answer n=10 entity=span-trace-coverage#3.1 v=no
why: the cli server check and its macOS euid-only carve-out still hold unchanged, and the 43-case count is CI measurement this entity does not carry
@answer n=11 entity=span-trace-coverage#3.2 v=no
why: the client seams and peer_admitted are src code the session did not change
@answer n=12 entity=span-trace-coverage#3.3 v=no
why: the captured-span refusal tests named here live in src/ modules the change did not reach
@answer n=13 entity=span-trace-coverage#3.4 v=no
why: the socket-directory refusal tests named here sit in src/cmd/client.rs and src/cmd/hook.rs, which the change did not reach
@answer n=14 entity=span-trace-coverage#3.5 v=no
why: refuse_untrusted and its exit-21 refusal line are unchanged, and the change adds no refusal path
@answer n=15 entity=span-trace-coverage#3.6 v=no
why: check_endpoint_dir and its socket-dir-refused process-exit line are unchanged, as no src file moved
@answer n=16 entity=span-trace-coverage#3.7 v=no
why: the frame-verb refusal pair is unchanged, and the list unable/hint colour fact concerns list's own output, not this pair
@answer n=17 entity=span-trace-coverage#3.8 v=no
why: the hook server check and its macOS euid-only carve-out still hold unchanged, and the 43-case count is not carried here
@answer n=18 entity=span-trace-coverage#3.9 v=no
why: the hook socket-directory refusal line and its dialog-path order are unchanged, as no hook source moved
@answer n=19 entity=span-trace-coverage#3.10 v=no
why: ResponseLine and the client fault_of reading are src code the session did not change
@answer n=20 entity=span-trace-coverage#3.11 v=no
why: the MCP server check and its macOS euid-only carve-out still hold unchanged, and the 43-case count is not carried here
@answer n=21 entity=span-trace-coverage#3.12 v=no
why: reply_of and of_reply screen replies through is_code_word and sanitised_fault, which the change did not touch
@answer n=22 entity=span-trace-coverage#4.1 v=no
why: the run start sequence and its spans are unchanged, as no product source moved
@answer n=23 entity=span-trace-coverage#4.2 v=no
why: the collision-check and heartbeat outcome rules are unchanged, and the new witness is a list witness, not a collision check
@answer n=24 entity=span-trace-coverage#4.3 v=no
why: the process-start and process-exit log fields and their detail codes are unchanged, and no schema moved
@answer n=25 entity=span-trace-coverage#4.4 v=no
why: the handle-wait emission rule is src code the session did not change, and the new child-end witness reads the process table, not the log
