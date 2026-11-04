@answer n=1 entity=span-trace-coverage#1.1 v=no
why: the session is test-only and moved no file that emits the CLIENT spans listed here
@answer n=2 entity=span-trace-coverage#1.2 v=no
why: the SERVER span names are unchanged and no product emitting call site moved
@answer n=3 entity=span-trace-coverage#1.3 v=no
why: the INTERNAL span names are unchanged and the session changed no span or emitting call site
@answer n=4 entity=span-trace-coverage#2.1 v=no
why: the cli short-lived verb spans, process-start and process-exit rules are unchanged by the test-only session
@answer n=5 entity=span-trace-coverage#2.2 v=no
why: the viola run span and pty seam rules are unchanged, product code untouched per the report
@answer n=6 entity=span-trace-coverage#2.3 v=no
why: the viola hook span chain and hook-invoked and hook-decision rules are unchanged by the session
@answer n=7 entity=span-trace-coverage#2.4 v=no
why: the channel.request and channel.dispatch span rules are unchanged, no ipc-internal product code moved
@answer n=8 entity=span-trace-coverage#2.5 v=no
why: the accept-loop peer filter and its absence of span and line are unchanged, no src file moved
@answer n=9 entity=span-trace-coverage#2.6 v=no
why: the viola-mcp mcp.tool_call span and codes-only reject rules are unchanged by the session
@answer n=10 entity=span-trace-coverage#3.1 v=no
why: the macOS euid-only carve-out for the wait and last server check is unchanged in behaviour and this line states no case count, so the 43-case recount leaves it true
@answer n=11 entity=span-trace-coverage#3.2 v=no
why: the client euid check is product code, unchanged per the report, and this line names no macOS carve-out count
@answer n=12 entity=span-trace-coverage#3.3 v=no
why: the captured-span refusal cases sit in src modules with the shared helper refused_inside, and the session moved no src file or refusal placement
@answer n=13 entity=span-trace-coverage#3.4 v=no
why: the two socket-directory captured-span cases named here are src unit tests the session did not move, and their count and paths hold
@answer n=14 entity=span-trace-coverage#3.5 v=no
why: the six frame verb instance check and its no-log refusal are unchanged, product code untouched per the report
@answer n=15 entity=span-trace-coverage#3.6 v=no
why: the socket-directory refusal line for the eight frame verbs is unchanged, and list is not a frame verb
@answer n=16 entity=span-trace-coverage#3.7 v=no
why: the plain-ASCII no-SGR refusal pair names frame verbs only, and the list colour witness does not print that pair
@answer n=17 entity=span-trace-coverage#3.8 v=no
why: the hook server check and its macOS euid-only carve-out are unchanged in behaviour and this line states no case count
@answer n=18 entity=span-trace-coverage#3.9 v=no
why: the hook socket-directory refusal line is unchanged, product code untouched per the report
@answer n=19 entity=span-trace-coverage#3.10 v=no
why: the channel-response detail rule for -32602 is unchanged, no ipc-internal product code moved
@answer n=20 entity=span-trace-coverage#3.11 v=no
why: the viola-mcp server check and its macOS euid-only carve-out are unchanged in behaviour and this line states no case count
@answer n=21 entity=span-trace-coverage#3.12 v=no
why: the CLI reply_of and MCP of_reply screening adds no span and no line, and no client code moved
@answer n=22 entity=span-trace-coverage#4.1 v=no
why: the run start sequence and its must-trace span chain are unchanged, no span or emitting call site moved
@answer n=23 entity=span-trace-coverage#4.2 v=no
why: the required span attributes, including the STALE_AFTER heartbeat rule, are unchanged, and the new witness changes no liveness rule
@answer n=24 entity=span-trace-coverage#4.3 v=no
why: the required log fields and detail codes are unchanged, the session changed no log field or closed detail value
@answer n=25 entity=span-trace-coverage#4.4 v=no
why: the handle-wait process-exit emission and its no-EOF rule are unchanged, and the new Unix test checks only that the child ends by pid and start time
