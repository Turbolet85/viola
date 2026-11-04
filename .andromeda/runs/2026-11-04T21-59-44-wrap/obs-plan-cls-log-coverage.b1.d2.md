@answer n=1 entity=log-coverage#1.1 v=no
why: the session changed no product source and no schemas/diag-line.v1.json, so the process-start field set holds.
@answer n=2 entity=log-coverage#1.2 v=no
why: the Unix wrapper-kill test reads pids and start time and asserts no exit_source, and the handle-wait|kill-fallback enum is unchanged.
@answer n=3 entity=log-coverage#1.3 v=no
why: no IPC method, channel request path or product code moved, so channel-request fields hold.
@answer n=4 entity=log-coverage#1.4 v=no
why: no channel-response method, detail or error code changed, the session being test-only.
@answer n=5 entity=log-coverage#1.5 v=no
why: no send path or wrapper send code changed; the stale-row witness drives viola list, not send.
@answer n=6 entity=log-coverage#1.6 v=no
why: no dialog path or product code moved, so dialog-raised and dialog-answered fields hold.
@answer n=7 entity=log-coverage#1.7 v=no
why: no hook arm or hook event code changed, so the hook-invoked and hook-decision fields hold.
@answer n=8 entity=log-coverage#1.8 v=no
why: viola_panic_hook, catch_unwind sites and the panic fields are untouched per the session's untouched §7 item.
@answer n=9 entity=log-coverage#1.9 v=no
why: no driver release path changed, so release-from-driver fields hold.
@answer n=10 entity=log-coverage#1.10 v=no
why: no state-recovery or snapshot replay code changed, so state-recovered details hold.
@answer n=11 entity=log-coverage#1.11 v=no
why: no parser or parse-rejected emitting code changed and the schema is untouched.
@answer n=12 entity=log-coverage#2.1 v=no
why: no run or revive refusal path changed; the stale-row witness runs list and not collision_check, so the exit-1 details hold.
@answer n=13 entity=log-coverage#2.2 v=no
why: no frame verb or server-verify path changed, so the exit-21 details hold.
@answer n=14 entity=log-coverage#2.3 v=no
why: no wrapper-fault path changed, so the exit-20 detail holds.
@answer n=15 entity=log-coverage#2.4 v=no
why: no hook fail-open path changed, so the hook-decision fail-open codes hold.
@answer n=16 entity=log-coverage#2.5 v=no
why: no parser's detail emission changed, so the parse-rejected details hold.
@answer n=17 entity=log-coverage#2.6 v=no
why: viola_mcp::serve's oversize path is unchanged, so the mcp-stdin parse-rejected line holds.
@answer n=18 entity=log-coverage#2.7 v=no
why: no refusal reason or send refusal path changed, so the refusal details hold.
@answer n=19 entity=log-coverage#2.8 v=no
why: the strict-modes and socket-dir checks are unchanged, and the witness reads only list's terminal stream, not its log.
@answer n=20 entity=log-coverage#3.1 v=no
why: no spawn site changed, so the spawn pairs for list, verify and the agents read hold; the witness runs list without changing its spawns.
